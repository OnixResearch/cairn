use radicle::{
    crypto::PublicKey,
    identity::{Did, DocError},
    prelude::ReadRepository,
    storage::{git::Repository, RepositoryError},
};
use thiserror::Error;

// r[impl cairn.radicle_import_cli.current_authority]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentDelegateAuthority {
    pub actor: String,
    pub identity_oid: String,
    pub is_delegate: bool,
}

#[derive(Debug, Error)]
pub enum CurrentAuthorityError {
    #[error("failed to resolve the current canonical repository identity: {0}")]
    Repository(#[from] RepositoryError),
    #[error("failed to load the current canonical identity document: {0}")]
    Identity(#[from] DocError),
}

pub fn inspect_current_delegate_authority(
    repo: &Repository,
    actor: &PublicKey,
) -> Result<CurrentDelegateAuthority, CurrentAuthorityError> {
    let identity_oid = repo.canonical_identity_head()?;
    let identity = repo.identity_doc_at(identity_oid)?;
    let actor_did = Did::from(*actor);
    Ok(CurrentDelegateAuthority {
        actor: actor.to_string(),
        identity_oid: identity_oid.to_string(),
        is_delegate: identity.is_delegate(&actor_did),
    })
}
