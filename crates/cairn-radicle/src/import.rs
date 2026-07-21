use std::{
    collections::BTreeSet,
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
    str::FromStr,
};

use cairn_core::{
    change_cob_import_content_root, change_cob_state_hash, validate_change_cob_init,
    ChangeCobAction, ChangeCobError, ChangeCobInit,
};
use nonempty::NonEmpty;
use radicle::{
    cob::{
        self,
        store::{access::ReadOnly, access::WriteAs, Store as CobStore},
        ObjectId,
    },
    crypto::{self, ssh::keystore, ssh::Keystore, PublicKey},
    git,
    identity::RepoId,
    node::device::BoxedDevice,
    prelude::ReadRepository,
    profile::{self, Config, Home, Profile},
    storage::{
        self,
        git::{Repository, Storage},
        ReadStorage, RepositoryError,
    },
};
use thiserror::Error;

use crate::{
    inspect_current_delegate_authority, Action, CurrentAuthorityError, CurrentDelegateAuthority,
    RadicleChangeCob, TYPENAME,
};

// r[impl cairn.radicle_import_cli.current_authority]
// r[impl cairn.radicle_import_cli.signed_creation]

const IMPORT_LOCK_DIRECTORY: &str = "cairn/locks/radicle-import";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeImportInspection {
    pub authority: CurrentDelegateAuthority,
    pub repository_id: String,
    pub base_git_oid: String,
    pub existing_object_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ImportLock {
    path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeImportOutcome {
    pub object_id: String,
    pub content_root: String,
    pub state_hash: String,
}

#[derive(Debug, Error)]
pub enum ChangeImportError {
    #[error(transparent)]
    Authority(#[from] CurrentAuthorityError),
    #[error("failed to access the Radicle repository: {0}")]
    Repository(#[from] RepositoryError),
    #[error("failed to access the typed Radicle COB store: {0}")]
    Store(#[from] cob::store::Error),
    #[error("failed to enumerate Radicle COB refs: {0}")]
    Git(#[from] git::raw::Error),
    #[error("invalid Radicle COB object ref {reference}: {reason}")]
    InvalidCobObjectRef { reference: String, reason: String },
    #[error("invalid Radicle COB object ID {object_id}: {reason}")]
    InvalidCobObjectId { object_id: String, reason: String },
    #[error("Cairn rejected the import state: {0}")]
    Core(#[from] ChangeCobError),
    #[error("Radicle adapter rejected the import state: {0}")]
    Adapter(#[from] crate::Error),
    #[error("failed to load the explicit Radicle home: {0}")]
    ProfileIo(#[from] io::Error),
    #[error("failed to load the explicit Radicle profile key: {0}")]
    Keystore(#[from] keystore::Error),
    #[error("failed to load the explicit Radicle profile config: {0}")]
    ProfileConfig(#[from] profile::config::LoadError),
    #[error("failed to open Radicle storage: {0}")]
    Storage(#[from] storage::Error),
    #[error("failed to load the Radicle signer: {0}")]
    Signer(#[from] profile::SignerError),
    #[error("failed to prove possession of the checkout signer key")]
    CheckoutSignerProof,
    #[error("no Radicle profile key exists under {path}")]
    ProfileKeyMissing { path: String },
    #[error("invalid Radicle repository ID {repository_id}: {reason}")]
    InvalidRepositoryId {
        repository_id: String,
        reason: String,
    },
    #[error("profile actor {actor} is not a delegate in current identity {identity_oid}")]
    CurrentDelegateRequired { actor: String, identity_oid: String },
    #[error("another local import is active for change {change_id} in repository {repository_id}; after confirming no importer is running, remove stale lock {lock_relative}")]
    ConcurrentImport {
        change_id: String,
        repository_id: String,
        lock_relative: String,
    },
    #[error("another local checkout is active for change {change_id} in repository {repository_id}; after confirming no checkout is running, remove stale lock {lock_relative}")]
    ConcurrentCheckout {
        change_id: String,
        repository_id: String,
        lock_relative: String,
    },
    #[error("change {change_id} already has Radicle object {object_id}")]
    ExistingChangeObject {
        change_id: String,
        object_id: String,
    },
    #[error("change {change_id} has multiple Radicle objects: {object_ids:?}")]
    DuplicateChangeObjects {
        change_id: String,
        object_ids: Vec<String>,
    },
    #[error("change {change_id} has no Radicle object")]
    ChangeObjectMissing { change_id: String },
    #[error("Radicle change object {object_id} disappeared before typed reload")]
    ChangeObjectReloadMissing { object_id: String },
    #[error("checkout context changed before execution: {field}")]
    CheckoutContextDrift { field: String },
    #[error("import context changed before execution: {field}")]
    ContextDrift { field: String },
    #[error("import repository mismatch: planned {planned}, current {current}")]
    RepositoryMismatch { planned: String, current: String },
    #[error("import base Git object mismatch: planned {planned}, current {current}")]
    BaseGitObjectMismatch { planned: String, current: String },
    #[error("signer actor {signer} does not match inspected actor {inspected}")]
    SignerMismatch { signer: String, inspected: String },
    #[error("planned content root changed before execution: planned {planned}, current {current}")]
    ContentRootDrift { planned: String, current: String },
    #[error("created Radicle object {object_id} could not be reloaded")]
    CreatedObjectMissing { object_id: String },
    #[error("created content root mismatch: planned {planned}, created {created}")]
    CreatedContentRootMismatch { planned: String, created: String },
}

pub fn inspect_change_import(
    repo: &Repository,
    actor: &PublicKey,
    change_id: &str,
) -> Result<ChangeImportInspection, ChangeImportError> {
    let authority = inspect_current_delegate_authority(repo, actor)?;
    let repository_id = repo.id().to_string();
    let base_git_oid = repo.head()?.1.to_string();
    let store = CobStore::<RadicleChangeCob, _, _>::open(repo, ReadOnly)?;
    let mut object_ids = Vec::new();
    for object_id in change_object_ids(repo)? {
        if let Some(change) = store.get(&object_id)? {
            if change.state().change_id == change_id {
                object_ids.push(object_id.to_string());
            }
        }
    }
    object_ids.sort();
    if object_ids.len() > 1 {
        return Err(ChangeImportError::DuplicateChangeObjects {
            change_id: change_id.to_string(),
            object_ids,
        });
    }
    Ok(ChangeImportInspection {
        authority,
        repository_id,
        base_git_oid,
        existing_object_id: object_ids.into_iter().next(),
    })
}

pub fn inspect_profile_import(
    rad_home: &Path,
    repository_id: &str,
    change_id: &str,
) -> Result<ChangeImportInspection, ChangeImportError> {
    let profile = load_profile_at(rad_home)?;
    let repository_id = parse_repository_id(repository_id)?;
    let repo = profile.storage.repository(repository_id)?;
    inspect_change_import(&repo, &profile.public_key, change_id)
}

pub fn create_change_import<Signer>(
    repo: &Repository,
    signer: &Signer,
    init: ChangeCobInit,
    expected: &ChangeImportInspection,
    expected_content_root: &str,
) -> Result<ChangeImportOutcome, ChangeImportError>
where
    Signer: crypto::signature::Keypair<VerifyingKey = PublicKey>,
    Signer: crypto::signature::Signer<crypto::Signature>,
    Signer: crypto::signature::Signer<crypto::ssh::ExtendedSignature>,
    Signer: crypto::signature::Verifier<crypto::Signature>,
{
    validate_change_cob_init(&init)?;
    let signer_key = signer.verifying_key();
    let current = inspect_change_import(repo, &signer_key, &init.change_id)?;
    validate_execution_context(&current, expected)?;
    require_import_authority(&current, &init.change_id)?;
    if init.repository_id != current.repository_id {
        return Err(ChangeImportError::RepositoryMismatch {
            planned: init.repository_id,
            current: current.repository_id,
        });
    }
    let planned_base_git_oid = crate::radicle_base_git_oid(&init)?;
    if planned_base_git_oid != current.base_git_oid {
        return Err(ChangeImportError::BaseGitObjectMismatch {
            planned: planned_base_git_oid,
            current: current.base_git_oid,
        });
    }
    if signer_key.to_string() != current.authority.actor {
        return Err(ChangeImportError::SignerMismatch {
            signer: signer_key.to_string(),
            inspected: current.authority.actor,
        });
    }
    let current_content_root = change_cob_import_content_root(&init, &signer_key.to_string())?;
    if current_content_root != expected_content_root {
        return Err(ChangeImportError::ContentRootDrift {
            planned: expected_content_root.to_string(),
            current: current_content_root,
        });
    }
    let identity_oid = git::Oid::from_str(&current.authority.identity_oid).map_err(|_| {
        ChangeImportError::ContextDrift {
            field: "identity_oid".to_string(),
        }
    })?;
    let title = format!("Import Cairn change {}", init.change_id);
    let action = Action(ChangeCobAction::Init { state: init });
    let mut store = CobStore::<RadicleChangeCob, _, _>::open(repo, WriteAs::new(signer))?
        .identity(identity_oid);
    let (object_id, created) = store.create(&title, NonEmpty::new(action), Vec::new())?;
    let Some(reloaded) = store.get(&object_id)? else {
        return Err(ChangeImportError::CreatedObjectMissing {
            object_id: object_id.to_string(),
        });
    };
    let created_content_root = cairn_core::change_cob_content_root(reloaded.state())?;
    if created_content_root != expected_content_root {
        return Err(ChangeImportError::CreatedContentRootMismatch {
            planned: expected_content_root.to_string(),
            created: created_content_root,
        });
    }
    debug_assert_eq!(created.state(), reloaded.state());
    Ok(ChangeImportOutcome {
        object_id: object_id.to_string(),
        content_root: created_content_root,
        state_hash: change_cob_state_hash(reloaded.state())?,
    })
}

pub fn execute_profile_import(
    rad_home: &Path,
    repository_id: &str,
    init: ChangeCobInit,
    expected: &ChangeImportInspection,
    expected_content_root: &str,
) -> Result<ChangeImportOutcome, ChangeImportError> {
    let profile = load_profile_at(rad_home)?;
    let repository_id = parse_repository_id(repository_id)?;
    let repo = profile.storage.repository(repository_id)?;
    let preflight = inspect_change_import(&repo, &profile.public_key, &init.change_id)?;
    validate_execution_context(&preflight, expected)?;
    require_import_authority(&preflight, &init.change_id)?;
    let lock = acquire_import_lock(
        profile.home.path(),
        &preflight.repository_id,
        &init.change_id,
    )?;
    let result = match profile.signer() {
        Ok(signer) => {
            let signer: BoxedDevice = signer;
            create_change_import(&repo, &signer, init, expected, expected_content_root)
        }
        Err(error) => Err(ChangeImportError::Signer(error)),
    };
    let release_result = lock.release();
    match (result, release_result) {
        (Ok(outcome), Ok(())) => Ok(outcome),
        (Err(error), _) => Err(error),
        (Ok(_), Err(error)) => Err(ChangeImportError::ProfileIo(error)),
    }
}

impl ImportLock {
    fn release(self) -> io::Result<()> {
        fs::remove_file(self.path)
    }
}

fn acquire_import_lock(
    rad_home: &Path,
    repository_id: &str,
    change_id: &str,
) -> Result<ImportLock, ChangeImportError> {
    let repository_lock_id = cairn_core::blake3_hex(repository_id.as_bytes());
    let lock_relative = PathBuf::from(IMPORT_LOCK_DIRECTORY)
        .join(repository_lock_id)
        .join(format!("{change_id}.lock"));
    let path = rad_home.join(&lock_relative);
    let Some(lock_directory) = path.parent() else {
        return Err(ChangeImportError::ProfileIo(io::Error::new(
            io::ErrorKind::InvalidInput,
            "import lock parent is missing",
        )));
    };
    fs::create_dir_all(lock_directory)?;
    match OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(_) => Ok(ImportLock { path }),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            Err(ChangeImportError::ConcurrentImport {
                change_id: change_id.to_string(),
                repository_id: repository_id.to_string(),
                lock_relative: lock_relative.display().to_string(),
            })
        }
        Err(error) => Err(ChangeImportError::ProfileIo(error)),
    }
}

fn require_import_authority(
    inspection: &ChangeImportInspection,
    change_id: &str,
) -> Result<(), ChangeImportError> {
    if !inspection.authority.is_delegate {
        return Err(ChangeImportError::CurrentDelegateRequired {
            actor: inspection.authority.actor.clone(),
            identity_oid: inspection.authority.identity_oid.clone(),
        });
    }
    if let Some(object_id) = &inspection.existing_object_id {
        return Err(ChangeImportError::ExistingChangeObject {
            change_id: change_id.to_string(),
            object_id: object_id.clone(),
        });
    }
    Ok(())
}

fn change_object_ids(repo: &Repository) -> Result<BTreeSet<ObjectId>, ChangeImportError> {
    const NAMESPACED_REF_PREFIX: &str = "refs/namespaces/";
    let type_marker = format!("/refs/cobs/{}/", &*TYPENAME);
    let mut object_ids = BTreeSet::new();
    for reference in repo.backend.references()? {
        let reference = reference?;
        let Some(reference_text) = reference.name() else {
            continue;
        };
        if !reference_text.starts_with(NAMESPACED_REF_PREFIX) {
            continue;
        }
        let Some((_, object_id_text)) = reference_text.split_once(&type_marker) else {
            continue;
        };
        if object_id_text.contains('/') {
            return Err(ChangeImportError::InvalidCobObjectRef {
                reference: reference_text.to_string(),
                reason: "object ID must be the final reference component".to_string(),
            });
        }
        let object_id = ObjectId::from_str(object_id_text).map_err(|error| {
            ChangeImportError::InvalidCobObjectRef {
                reference: reference_text.to_string(),
                reason: error.to_string(),
            }
        })?;
        object_ids.insert(object_id);
    }
    Ok(object_ids)
}

fn validate_execution_context(
    current: &ChangeImportInspection,
    expected: &ChangeImportInspection,
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
            "base_git_oid",
            current.base_git_oid.as_str(),
            expected.base_git_oid.as_str(),
        ),
    ];
    for (field, current, expected) in fields {
        if current != expected {
            return Err(ChangeImportError::ContextDrift {
                field: field.to_string(),
            });
        }
    }
    if current.authority.is_delegate != expected.authority.is_delegate {
        return Err(ChangeImportError::ContextDrift {
            field: "delegate_authority".to_string(),
        });
    }
    if current.existing_object_id != expected.existing_object_id {
        return Err(ChangeImportError::ContextDrift {
            field: "existing_object_id".to_string(),
        });
    }
    Ok(())
}

pub(crate) fn load_profile_at(rad_home: &Path) -> Result<Profile, ChangeImportError> {
    let home = Home::load(rad_home)?;
    let keystore = Keystore::new(&home.keys());
    let public_key =
        keystore
            .public_key()?
            .ok_or_else(|| ChangeImportError::ProfileKeyMissing {
                path: rad_home.display().to_string(),
            })?;
    let config = Config::load(home.config().as_path())?;
    let storage = Storage::open(
        home.storage(),
        git::UserInfo {
            alias: config.alias().clone(),
            key: public_key,
        },
    )?;
    Ok(Profile {
        home,
        storage,
        keystore,
        public_key,
        config,
    })
}

pub(crate) fn parse_repository_id(value: &str) -> Result<RepoId, ChangeImportError> {
    RepoId::from_str(value).map_err(|error| ChangeImportError::InvalidRepositoryId {
        repository_id: value.to_string(),
        reason: error.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs};

    use super::*;

    const TEST_DIRECTORY_PREFIX: &str = "cairn-radicle-import-lock";
    const TEST_REPOSITORY_ID: &str = "rad:z3example";
    const TEST_CHANGE_ID: &str = "radicle-import-cli";

    fn test_directory() -> PathBuf {
        std::env::temp_dir().join(format!("{TEST_DIRECTORY_PREFIX}-{}", std::process::id()))
    }

    fn remove_test_directory(path: &Path) -> Result<(), Box<dyn Error>> {
        match fs::remove_dir_all(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    // r[verify cairn.radicle_import_cli.signed_creation.concurrent]
    // r[verify cairn.radicle_import_cli.receipt.blocked]
    #[test]
    fn import_lock_serializes_same_change_and_releases_explicitly() -> Result<(), Box<dyn Error>> {
        let home = test_directory();
        remove_test_directory(&home)?;
        fs::create_dir_all(&home)?;

        let first = acquire_import_lock(&home, TEST_REPOSITORY_ID, TEST_CHANGE_ID)?;
        assert!(matches!(
            acquire_import_lock(&home, TEST_REPOSITORY_ID, TEST_CHANGE_ID),
            Err(ChangeImportError::ConcurrentImport { .. })
        ));
        first.release()?;
        let second = acquire_import_lock(&home, TEST_REPOSITORY_ID, TEST_CHANGE_ID)?;
        second.release()?;

        fs::remove_dir_all(home)?;
        Ok(())
    }
}
