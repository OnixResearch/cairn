use std::{
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
    str::FromStr,
};

use cairn_core::ChangeCobState;
use radicle::{
    cob::{store::access::ReadOnly, store::Store as CobStore, ObjectId},
    crypto::{self, signature::Keypair, signature::Signer, PublicKey},
    node::device::BoxedDevice,
    storage::{git::Repository, ReadStorage},
};

use crate::{
    import::{load_profile_at, parse_repository_id},
    inspect_change_import, ChangeImportError, CurrentDelegateAuthority, RadicleChangeCob,
};

const CHECKOUT_LOCK_DIRECTORY: &str = "cairn/locks/radicle-checkout";

// r[impl cairn.radicle_status_checkout.status]
// r[impl cairn.radicle_status_checkout.checkout_authority]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeStatusInspection {
    pub authority: CurrentDelegateAuthority,
    pub repository_id: String,
    pub repository_head_oid: String,
    pub object_id: String,
    pub state: ChangeCobState,
}

#[derive(Debug)]
struct CheckoutLock {
    path: PathBuf,
}

#[derive(Debug)]
pub struct ChangeCheckoutGuard {
    inspection: ChangeStatusInspection,
    lock: CheckoutLock,
}

impl ChangeCheckoutGuard {
    pub fn inspection(&self) -> &ChangeStatusInspection {
        &self.inspection
    }

    pub fn release(self) -> io::Result<()> {
        self.lock.release()
    }
}

pub fn inspect_change_status(
    repo: &Repository,
    actor: &PublicKey,
    change_id: &str,
) -> Result<ChangeStatusInspection, ChangeImportError> {
    let import = inspect_change_import(repo, actor, change_id)?;
    let Some(object_id_text) = import.existing_object_id else {
        return Err(ChangeImportError::ChangeObjectMissing {
            change_id: change_id.to_string(),
        });
    };
    let object_id = ObjectId::from_str(&object_id_text).map_err(|error| {
        ChangeImportError::InvalidCobObjectId {
            object_id: object_id_text.clone(),
            reason: error.to_string(),
        }
    })?;
    let store = CobStore::<RadicleChangeCob, _, _>::open(repo, ReadOnly)?;
    let Some(object) = store.get(&object_id)? else {
        return Err(ChangeImportError::ChangeObjectReloadMissing {
            object_id: object_id_text,
        });
    };
    Ok(ChangeStatusInspection {
        authority: import.authority,
        repository_id: import.repository_id,
        repository_head_oid: import.base_git_oid,
        object_id: object_id.to_string(),
        state: object.into_state(),
    })
}

pub fn inspect_profile_status(
    rad_home: &Path,
    repository_id: &str,
    change_id: &str,
) -> Result<ChangeStatusInspection, ChangeImportError> {
    let profile = load_profile_at(rad_home)?;
    let repository_id = parse_repository_id(repository_id)?;
    let repo = profile.storage.repository(repository_id)?;
    inspect_change_status(&repo, &profile.public_key, change_id)
}

pub fn acquire_change_checkout_guard(
    rad_home: &Path,
    repo: &Repository,
    actor: &PublicKey,
    change_id: &str,
    expected: &ChangeStatusInspection,
) -> Result<ChangeCheckoutGuard, ChangeImportError> {
    let preflight = inspect_change_status(repo, actor, change_id)?;
    validate_checkout_context(&preflight, expected)?;
    require_checkout_authority(&preflight)?;
    let lock = acquire_checkout_lock(rad_home, &preflight.repository_id, change_id)?;
    let guarded = inspect_change_status(repo, actor, change_id);
    let inspection = match guarded {
        Ok(inspection) => inspection,
        Err(error) => {
            let _release_result = lock.release();
            return Err(error);
        }
    };
    if let Err(error) = validate_checkout_context(&inspection, expected)
        .and_then(|()| require_checkout_authority(&inspection))
    {
        let _release_result = lock.release();
        return Err(error);
    }
    Ok(ChangeCheckoutGuard { inspection, lock })
}

pub fn acquire_profile_checkout_guard(
    rad_home: &Path,
    repository_id: &str,
    change_id: &str,
    expected: &ChangeStatusInspection,
) -> Result<ChangeCheckoutGuard, ChangeImportError> {
    let profile = load_profile_at(rad_home)?;
    let repository_id = parse_repository_id(repository_id)?;
    let repo = profile.storage.repository(repository_id)?;
    let preflight = inspect_change_status(&repo, &profile.public_key, change_id)?;
    validate_checkout_context(&preflight, expected)?;
    require_checkout_authority(&preflight)?;
    let signer: BoxedDevice = profile.signer()?;
    let signer_key = signer.verifying_key();
    if signer_key != profile.public_key {
        return Err(ChangeImportError::SignerMismatch {
            signer: signer_key.to_string(),
            inspected: profile.public_key.to_string(),
        });
    }
    let challenge = checkout_authority_challenge(expected)?;
    let proof: crypto::Signature = signer
        .try_sign(challenge.as_bytes())
        .map_err(|_| ChangeImportError::CheckoutSignerProof)?;
    profile
        .public_key
        .verify(challenge.as_bytes(), &proof)
        .map_err(|_| ChangeImportError::CheckoutSignerProof)?;
    acquire_change_checkout_guard(
        profile.home.path(),
        &repo,
        &profile.public_key,
        change_id,
        expected,
    )
}

fn checkout_authority_challenge(
    expected: &ChangeStatusInspection,
) -> Result<String, ChangeImportError> {
    let state_hash = cairn_core::change_cob_state_hash(&expected.state)?;
    Ok(format!(
        "cairn-radicle-checkout-v1\n{}\n{}\n{}\n{}\n",
        expected.repository_id, expected.object_id, expected.authority.identity_oid, state_hash,
    ))
}

impl CheckoutLock {
    fn release(self) -> io::Result<()> {
        fs::remove_file(self.path)
    }
}

fn acquire_checkout_lock(
    rad_home: &Path,
    repository_id: &str,
    change_id: &str,
) -> Result<CheckoutLock, ChangeImportError> {
    let repository_lock_id = cairn_core::blake3_hex(repository_id.as_bytes());
    let lock_relative = PathBuf::from(CHECKOUT_LOCK_DIRECTORY)
        .join(repository_lock_id)
        .join(format!("{change_id}.lock"));
    let path = rad_home.join(&lock_relative);
    let Some(lock_directory) = path.parent() else {
        return Err(ChangeImportError::ProfileIo(io::Error::new(
            io::ErrorKind::InvalidInput,
            "checkout lock parent is missing",
        )));
    };
    fs::create_dir_all(lock_directory)?;
    match OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(_) => Ok(CheckoutLock { path }),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            Err(ChangeImportError::ConcurrentCheckout {
                change_id: change_id.to_string(),
                repository_id: repository_id.to_string(),
                lock_relative: lock_relative.display().to_string(),
            })
        }
        Err(error) => Err(ChangeImportError::ProfileIo(error)),
    }
}

fn require_checkout_authority(
    inspection: &ChangeStatusInspection,
) -> Result<(), ChangeImportError> {
    if inspection.authority.is_delegate {
        Ok(())
    } else {
        Err(ChangeImportError::CurrentDelegateRequired {
            actor: inspection.authority.actor.clone(),
            identity_oid: inspection.authority.identity_oid.clone(),
        })
    }
}

fn validate_checkout_context(
    current: &ChangeStatusInspection,
    expected: &ChangeStatusInspection,
) -> Result<(), ChangeImportError> {
    let fields = [
        (
            "actor",
            current.authority.actor.as_str(),
            expected.authority.actor.as_str(),
        ),
        (
            "identity_oid",
            current.authority.identity_oid.as_str(),
            expected.authority.identity_oid.as_str(),
        ),
        (
            "repository_id",
            current.repository_id.as_str(),
            expected.repository_id.as_str(),
        ),
        (
            "object_id",
            current.object_id.as_str(),
            expected.object_id.as_str(),
        ),
    ];
    for (field, current, expected) in fields {
        if current != expected {
            return Err(ChangeImportError::CheckoutContextDrift {
                field: field.to_string(),
            });
        }
    }
    if current.authority.is_delegate != expected.authority.is_delegate {
        return Err(ChangeImportError::CheckoutContextDrift {
            field: "delegate_authority".to_string(),
        });
    }
    if current.state != expected.state {
        return Err(ChangeImportError::CheckoutContextDrift {
            field: "typed_state".to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, error::Error, fs, path::PathBuf};

    use cairn_core::{
        change_cob_import_content_root, ChangeArtifactId, ChangeCobInit,
        RADICLE_CHANGE_COB_SCHEMA_V1,
    };
    use radicle::{
        crypto::{test::signer::MockSigner, Seed},
        node::{device::Device, Alias},
        prelude::ReadRepository,
        profile::{Home, Profile},
        storage::ReadStorage,
        test::{fixtures, setup::NodeWithRepo},
    };

    use super::*;
    use crate::{create_change_import, inspect_change_import};

    const CHANGE_ID: &str = "radicle-status-checkout-cli";
    const POLICY_HASH: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const PROPOSAL: &str = "# Proposal\n";
    const PROFILE_TEST_DIRECTORY_PREFIX: &str = "cairn-radicle-checkout-profile";

    fn profile_test_directory() -> PathBuf {
        std::env::temp_dir().join(format!(
            "{PROFILE_TEST_DIRECTORY_PREFIX}-{}",
            std::process::id()
        ))
    }

    fn remove_profile_test_directory(path: &Path) -> Result<(), Box<dyn Error>> {
        match fs::remove_dir_all(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    fn create_change(fixture: &NodeWithRepo) -> Result<ChangeStatusInspection, Box<dyn Error>> {
        let repository_id = fixture.repo.id().to_string();
        let base_git_oid = fixture.repo.head()?.1.to_string();
        let init = ChangeCobInit {
            schema: RADICLE_CHANGE_COB_SCHEMA_V1.to_string(),
            change_id: CHANGE_ID.to_string(),
            repository_id,
            base_git_oid,
            policy_hash: POLICY_HASH.to_string(),
            artifacts: BTreeMap::from([(ChangeArtifactId::proposal(), PROPOSAL.to_string())]),
        };
        let import =
            inspect_change_import(&fixture.repo, fixture.node.signer.public_key(), CHANGE_ID)?;
        let content_root = change_cob_import_content_root(&init, &import.authority.actor)?;
        create_change_import(
            &fixture.repo,
            &fixture.node.signer,
            init,
            &import,
            &content_root,
        )?;
        Ok(inspect_change_status(
            &fixture.repo,
            fixture.node.signer.public_key(),
            CHANGE_ID,
        )?)
    }

    // r[verify cairn.radicle_status_checkout.status.typed]
    #[test]
    fn typed_status_loads_without_signing_and_reports_current_authority(
    ) -> Result<(), Box<dyn Error>> {
        let fixture = NodeWithRepo::default();
        let allowed = create_change(&fixture)?;
        assert!(allowed.authority.is_delegate);
        assert_eq!(allowed.state.change_id, CHANGE_ID);
        assert_eq!(allowed.repository_id, fixture.repo.id().to_string());

        let outsider = Device::<MockSigner>::default();
        let readable = inspect_change_status(&fixture.repo, outsider.public_key(), CHANGE_ID)?;
        assert!(!readable.authority.is_delegate);
        assert_eq!(readable.object_id, allowed.object_id);
        assert_eq!(readable.state, allowed.state);
        Ok(())
    }

    // r[verify cairn.radicle_status_checkout.checkout_authority.allowed]
    // r[verify cairn.radicle_status_checkout.checkout_authority.denied]
    #[test]
    fn checkout_guard_serializes_and_rejects_non_delegate_or_drift() -> Result<(), Box<dyn Error>> {
        let fixture = NodeWithRepo::default();
        let expected = create_change(&fixture)?;
        let guard = acquire_change_checkout_guard(
            &fixture.node.root,
            &fixture.repo,
            fixture.node.signer.public_key(),
            CHANGE_ID,
            &expected,
        )?;
        assert_eq!(guard.inspection(), &expected);
        assert!(matches!(
            acquire_change_checkout_guard(
                &fixture.node.root,
                &fixture.repo,
                fixture.node.signer.public_key(),
                CHANGE_ID,
                &expected,
            ),
            Err(ChangeImportError::ConcurrentCheckout { .. })
        ));
        guard.release()?;

        let outsider = Device::<MockSigner>::default();
        let outsider_status =
            inspect_change_status(&fixture.repo, outsider.public_key(), CHANGE_ID)?;
        assert!(matches!(
            acquire_change_checkout_guard(
                &fixture.node.root,
                &fixture.repo,
                outsider.public_key(),
                CHANGE_ID,
                &outsider_status,
            ),
            Err(ChangeImportError::CurrentDelegateRequired { .. })
        ));

        let mut drifted = expected;
        drifted.state.policy_hash = cairn_core::blake3_hex(b"drifted policy");
        assert!(matches!(
            acquire_change_checkout_guard(
                &fixture.node.root,
                &fixture.repo,
                fixture.node.signer.public_key(),
                CHANGE_ID,
                &drifted,
            ),
            Err(ChangeImportError::CheckoutContextDrift { .. })
        ));
        Ok(())
    }

    // r[verify cairn.radicle_status_checkout.status.typed]
    // r[verify cairn.radicle_status_checkout.checkout_authority.allowed]
    #[test]
    fn explicit_profile_status_and_signer_proof_guard_work_end_to_end() -> Result<(), Box<dyn Error>>
    {
        let root = profile_test_directory();
        remove_profile_test_directory(&root)?;
        fs::create_dir_all(&root)?;
        let home = Home::new(root.join("rad-home"))?;
        let profile = Profile::init(home, "cairn-test".parse::<Alias>()?, None, Seed::generate())?;
        let signer = profile.signer()?;
        let (repository_id, _, _, _) =
            fixtures::project(root.join("working"), &profile.storage, signer.as_ref())?;
        let repo = profile.storage.repository(repository_id)?;
        let base_git_oid = repo.head()?.1.to_string();
        let init = ChangeCobInit {
            schema: RADICLE_CHANGE_COB_SCHEMA_V1.to_string(),
            change_id: CHANGE_ID.to_string(),
            repository_id: repository_id.to_string(),
            base_git_oid,
            policy_hash: POLICY_HASH.to_string(),
            artifacts: BTreeMap::from([(ChangeArtifactId::proposal(), PROPOSAL.to_string())]),
        };
        let import = inspect_change_import(&repo, &profile.public_key, CHANGE_ID)?;
        let content_root = change_cob_import_content_root(&init, &import.authority.actor)?;
        create_change_import(&repo, &signer, init, &import, &content_root)?;
        let home_path = profile.home.path().to_path_buf();
        let repository_id_text = repository_id.to_string();
        drop(repo);
        drop(signer);
        drop(profile);

        let status = inspect_profile_status(&home_path, &repository_id_text, CHANGE_ID)?;
        assert!(status.authority.is_delegate);
        let guard =
            acquire_profile_checkout_guard(&home_path, &repository_id_text, CHANGE_ID, &status)?;
        assert_eq!(guard.inspection(), &status);
        guard.release()?;

        remove_profile_test_directory(&root)?;
        Ok(())
    }

    // r[verify cairn.radicle_status_checkout.receipts.blocked]
    #[test]
    fn status_rejects_a_missing_change_object() {
        let fixture = NodeWithRepo::default();
        assert!(matches!(
            inspect_change_status(&fixture.repo, fixture.node.signer.public_key(), CHANGE_ID,),
            Err(ChangeImportError::ChangeObjectMissing { .. })
        ));
    }
}
