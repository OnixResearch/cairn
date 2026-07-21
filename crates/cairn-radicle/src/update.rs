use std::{
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
    str::FromStr,
};

use cairn_core::{change_cob_content_root, change_cob_state_hash, ChangeCobUpdatePlan};
use nonempty::NonEmpty;
use radicle::{
    cob::{store::access::WriteAs, store::Store as CobStore, ObjectId},
    crypto::{self, PublicKey},
    git,
    node::device::BoxedDevice,
    storage::{git::Repository, ReadStorage},
};

use crate::{
    import::{load_profile_at, parse_repository_id},
    inspect_change_status, Action, ChangeImportError, ChangeStatusInspection, RadicleChangeCob,
    TYPENAME,
};

const UPDATE_LOCK_DIRECTORY: &str = "cairn/locks/radicle-update";

// r[impl cairn.collaboration_update.radicle_execution]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeUpdateOutcome {
    pub object_id: String,
    pub content_root: String,
    pub state_hash: String,
}

#[derive(Debug)]
pub(crate) struct UpdateLock {
    path: PathBuf,
}

pub fn create_change_update<Signer>(
    repo: &Repository,
    signer: &Signer,
    plan: &ChangeCobUpdatePlan,
    expected: &ChangeStatusInspection,
) -> Result<ChangeUpdateOutcome, ChangeImportError>
where
    Signer: crypto::signature::Keypair<VerifyingKey = PublicKey>,
    Signer: crypto::signature::Signer<crypto::Signature>,
    Signer: crypto::signature::Signer<crypto::ssh::ExtendedSignature>,
    Signer: crypto::signature::Verifier<crypto::Signature>,
{
    if plan.actions.is_empty() {
        return Err(ChangeImportError::EmptyUpdate);
    }
    let signer_key = signer.verifying_key();
    let current = inspect_change_status(repo, &signer_key, &plan.change_id)?;
    validate_update_context(&current, expected)?;
    require_update_authority(&current)?;
    validate_plan_context(plan, &current)?;
    if signer_key.to_string() != current.authority.actor {
        return Err(ChangeImportError::SignerMismatch {
            signer: signer_key.to_string(),
            inspected: current.authority.actor,
        });
    }
    let identity_oid = git::Oid::from_str(&current.authority.identity_oid).map_err(|_| {
        ChangeImportError::UpdateContextDrift {
            field: "identity_oid".to_string(),
        }
    })?;
    let object_id = ObjectId::from_str(&current.object_id).map_err(|error| {
        ChangeImportError::InvalidCobObjectId {
            object_id: current.object_id.clone(),
            reason: error.to_string(),
        }
    })?;
    let actions = NonEmpty::from_vec(
        plan.actions
            .iter()
            .cloned()
            .map(Action::from)
            .collect::<Vec<_>>(),
    )
    .ok_or(ChangeImportError::EmptyUpdate)?;
    let title = format!("Update Cairn change {}", plan.change_id);
    let mut store = CobStore::<RadicleChangeCob, _, _>::open(repo, WriteAs::new(signer))?
        .identity(identity_oid);
    store.update(&TYPENAME, object_id, &title, actions, Vec::new())?;
    let Some(reloaded) = store.get(&object_id)? else {
        return Err(ChangeImportError::UpdatedObjectMissing {
            object_id: object_id.to_string(),
        });
    };
    let content_root = change_cob_content_root(reloaded.state())?;
    if content_root != plan.expected_content_root {
        return Err(ChangeImportError::UpdatedContentRootMismatch {
            planned: plan.expected_content_root.clone(),
            updated: content_root,
        });
    }
    Ok(ChangeUpdateOutcome {
        object_id: object_id.to_string(),
        content_root,
        state_hash: change_cob_state_hash(reloaded.state())?,
    })
}

pub fn execute_profile_update(
    rad_home: &Path,
    repository_id: &str,
    plan: &ChangeCobUpdatePlan,
    expected: &ChangeStatusInspection,
) -> Result<ChangeUpdateOutcome, ChangeImportError> {
    if plan.actions.is_empty() {
        return Err(ChangeImportError::EmptyUpdate);
    }
    let profile = load_profile_at(rad_home)?;
    let repository_id = parse_repository_id(repository_id)?;
    let repo = profile.storage.repository(repository_id)?;
    let preflight = inspect_change_status(&repo, &profile.public_key, &plan.change_id)?;
    validate_update_context(&preflight, expected)?;
    require_update_authority(&preflight)?;
    validate_plan_context(plan, &preflight)?;
    let lock = acquire_update_lock(
        profile.home.path(),
        &preflight.repository_id,
        &plan.change_id,
    )?;
    let guarded = inspect_change_status(&repo, &profile.public_key, &plan.change_id);
    let result = match guarded {
        Ok(inspection) => validate_update_context(&inspection, expected)
            .and_then(|()| require_update_authority(&inspection))
            .and_then(|()| validate_plan_context(plan, &inspection))
            .and_then(|()| {
                let signer: BoxedDevice = profile.signer()?;
                create_change_update(&repo, &signer, plan, &inspection)
            }),
        Err(error) => Err(error),
    };
    let release_result = lock.release();
    match (result, release_result) {
        (Ok(outcome), Ok(())) => Ok(outcome),
        (Err(error), _) => Err(error),
        (Ok(_), Err(error)) => Err(ChangeImportError::ProfileIo(error)),
    }
}

fn validate_plan_context(
    plan: &ChangeCobUpdatePlan,
    current: &ChangeStatusInspection,
) -> Result<(), ChangeImportError> {
    if plan.actor != current.authority.actor {
        return Err(ChangeImportError::UpdateActorMismatch {
            planned: plan.actor.clone(),
            current: current.authority.actor.clone(),
        });
    }
    if plan.change_id != current.state.change_id {
        return Err(ChangeImportError::UpdateContextDrift {
            field: "change_id".to_string(),
        });
    }
    let state_hash = change_cob_state_hash(&current.state)?;
    if plan.input_state_hash != state_hash {
        return Err(ChangeImportError::UpdateContextDrift {
            field: "typed_state_hash".to_string(),
        });
    }
    Ok(())
}

pub(crate) fn validate_update_context(
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
    for (field, current_value, expected_value) in fields {
        if current_value != expected_value {
            return Err(ChangeImportError::UpdateContextDrift {
                field: field.to_string(),
            });
        }
    }
    if current.authority.is_delegate != expected.authority.is_delegate {
        return Err(ChangeImportError::UpdateContextDrift {
            field: "delegate_authority".to_string(),
        });
    }
    if current.state != expected.state {
        return Err(ChangeImportError::UpdateContextDrift {
            field: "typed_state".to_string(),
        });
    }
    Ok(())
}

pub(crate) fn require_update_authority(
    inspection: &ChangeStatusInspection,
) -> Result<(), ChangeImportError> {
    if inspection.authority.is_delegate || inspection.authority.actor == inspection.state.creator {
        Ok(())
    } else {
        Err(ChangeImportError::CurrentDelegateRequired {
            actor: inspection.authority.actor.clone(),
            identity_oid: inspection.authority.identity_oid.clone(),
        })
    }
}

impl UpdateLock {
    pub(crate) fn release(self) -> io::Result<()> {
        fs::remove_file(self.path)
    }
}

pub(crate) fn acquire_update_lock(
    rad_home: &Path,
    repository_id: &str,
    change_id: &str,
) -> Result<UpdateLock, ChangeImportError> {
    let repository_lock_id = cairn_core::blake3_hex(repository_id.as_bytes());
    let lock_relative = PathBuf::from(UPDATE_LOCK_DIRECTORY)
        .join(repository_lock_id)
        .join(format!("{change_id}.lock"));
    let path = rad_home.join(&lock_relative);
    let Some(lock_directory) = path.parent() else {
        return Err(ChangeImportError::ProfileIo(io::Error::new(
            io::ErrorKind::InvalidInput,
            "update lock parent is missing",
        )));
    };
    fs::create_dir_all(lock_directory)?;
    match OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(_) => Ok(UpdateLock { path }),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            Err(ChangeImportError::ConcurrentUpdate {
                change_id: change_id.to_string(),
                repository_id: repository_id.to_string(),
                lock_relative: lock_relative.display().to_string(),
            })
        }
        Err(error) => Err(ChangeImportError::ProfileIo(error)),
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, error::Error};

    use cairn_core::{
        change_cob_import_content_root, plan_change_cob_update, ChangeArtifactId, ChangeCobInit,
        ChangeCollaborationUpdateSnapshot, RADICLE_CHANGE_COB_SCHEMA_V1,
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

    const CHANGE_ID: &str = "radicle-collaboration-update";
    const POLICY_HASH: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const PROPOSAL: &str = "# Proposal\n";
    const UPDATED_PROPOSAL: &str = "# Updated proposal\n";
    const TEST_DIRECTORY_PREFIX: &str = "cairn-radicle-update-lock";
    const PROFILE_TEST_DIRECTORY_PREFIX: &str = "cairn-radicle-update-profile";

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

    fn plan(
        inspection: &ChangeStatusInspection,
        actor: &str,
        is_delegate: bool,
    ) -> Result<ChangeCobUpdatePlan, cairn_core::ChangeCobError> {
        plan_change_cob_update(
            &inspection.state,
            ChangeCollaborationUpdateSnapshot {
                change_id: CHANGE_ID.to_string(),
                policy_hash: POLICY_HASH.to_string(),
                actor: actor.to_string(),
                is_delegate,
                proposal: Some(UPDATED_PROPOSAL.to_string()),
                design: None,
                tasks: None,
                metadata: None,
                delta_specs: BTreeMap::new(),
            },
        )
    }

    // r[verify cairn.collaboration_update.radicle_execution.success]
    #[test]
    fn signed_update_reloads_expected_content_and_rejects_stale_plan() -> Result<(), Box<dyn Error>>
    {
        let fixture = NodeWithRepo::default();
        let inspection = create_change(&fixture)?;
        let plan = plan(&inspection, &inspection.authority.actor, true)?;
        let outcome =
            create_change_update(&fixture.repo, &fixture.node.signer, &plan, &inspection)?;
        assert_eq!(outcome.content_root, plan.expected_content_root);
        let updated =
            inspect_change_status(&fixture.repo, fixture.node.signer.public_key(), CHANGE_ID)?;
        assert_ne!(updated.state, inspection.state);
        assert!(matches!(
            create_change_update(&fixture.repo, &fixture.node.signer, &plan, &inspection,),
            Err(ChangeImportError::UpdateContextDrift { .. })
        ));
        Ok(())
    }

    // r[verify cairn.collaboration_update.radicle_execution.blocked]
    #[test]
    fn current_authority_overrides_false_delegate_plan_claim() -> Result<(), Box<dyn Error>> {
        let fixture = NodeWithRepo::default();
        create_change(&fixture)?;
        let outsider = Device::<MockSigner>::default();
        let outsider_inspection =
            inspect_change_status(&fixture.repo, outsider.public_key(), CHANGE_ID)?;
        let plan = plan(
            &outsider_inspection,
            &outsider_inspection.authority.actor,
            true,
        )?;
        assert!(matches!(
            create_change_update(&fixture.repo, &outsider, &plan, &outsider_inspection),
            Err(ChangeImportError::CurrentDelegateRequired { .. })
        ));
        Ok(())
    }

    // r[verify cairn.collaboration_update.radicle_execution.success]
    #[test]
    fn explicit_profile_update_executes_and_reloads_end_to_end() -> Result<(), Box<dyn Error>> {
        let root = std::env::temp_dir().join(format!(
            "{PROFILE_TEST_DIRECTORY_PREFIX}-{}",
            std::process::id()
        ));
        match fs::remove_dir_all(&root) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        fs::create_dir_all(&root)?;
        let home = Home::new(root.join("rad-home"))?;
        let profile = Profile::init(
            home,
            "cairn-update".parse::<Alias>()?,
            None,
            Seed::generate(),
        )?;
        let signer = profile.signer()?;
        let (repository_id, _, _, _) =
            fixtures::project(root.join("working"), &profile.storage, signer.as_ref())?;
        let repo = profile.storage.repository(repository_id)?;
        let init = ChangeCobInit {
            schema: RADICLE_CHANGE_COB_SCHEMA_V1.to_string(),
            change_id: CHANGE_ID.to_string(),
            repository_id: repository_id.to_string(),
            base_git_oid: repo.head()?.1.to_string(),
            policy_hash: POLICY_HASH.to_string(),
            artifacts: BTreeMap::from([(ChangeArtifactId::proposal(), PROPOSAL.to_string())]),
        };
        let import = inspect_change_import(&repo, &profile.public_key, CHANGE_ID)?;
        let content_root = change_cob_import_content_root(&init, &import.authority.actor)?;
        create_change_import(&repo, &signer, init, &import, &content_root)?;
        let status = inspect_change_status(&repo, &profile.public_key, CHANGE_ID)?;
        let plan = plan(&status, &status.authority.actor, true)?;
        let home_path = profile.home.path().to_path_buf();
        let repository_id_text = repository_id.to_string();
        drop(repo);
        drop(signer);
        drop(profile);

        let outcome = execute_profile_update(&home_path, &repository_id_text, &plan, &status)?;
        assert_eq!(outcome.content_root, plan.expected_content_root);
        let reloaded = crate::inspect_profile_status(&home_path, &repository_id_text, CHANGE_ID)?;
        assert_eq!(
            change_cob_content_root(&reloaded.state)?,
            outcome.content_root
        );

        fs::remove_dir_all(&root)?;
        Ok(())
    }

    // r[verify cairn.collaboration_update.radicle_execution.blocked]
    #[test]
    fn update_lock_fails_closed_until_deliberately_released() -> Result<(), Box<dyn Error>> {
        let root =
            std::env::temp_dir().join(format!("{TEST_DIRECTORY_PREFIX}-{}", std::process::id()));
        match fs::remove_dir_all(&root) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        fs::create_dir_all(&root)?;
        let first = acquire_update_lock(&root, "rad:repository", CHANGE_ID)?;
        assert!(matches!(
            acquire_update_lock(&root, "rad:repository", CHANGE_ID),
            Err(ChangeImportError::ConcurrentUpdate { .. })
        ));
        first.release()?;
        let second = acquire_update_lock(&root, "rad:repository", CHANGE_ID)?;
        second.release()?;
        fs::remove_dir_all(&root)?;
        Ok(())
    }
}
