#![deny(clippy::unwrap_used)]

use std::{ops::Deref, str::FromStr, sync::LazyLock};

use cairn_core::{
    apply_change_cob_operation, initialize_change_cob, ChangeCobAction, ChangeCobError,
    ChangeCobOperation, ChangeCobState, RADICLE_CHANGE_COB_TYPE_NAME,
};
use radicle::{
    cob::{
        self,
        op::{Op, OpEncodingError},
        store::{Cob, CobAction, CobWithType},
        EntryId, TypeName,
    },
    git,
    identity::doc::DocError,
    prelude::ReadRepository,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// r[impl cairn.radicle_change_cob.radicle_adapter]

pub static TYPENAME: LazyLock<TypeName> =
    LazyLock::new(|| match TypeName::from_str(RADICLE_CHANGE_COB_TYPE_NAME) {
        Ok(type_name) => type_name,
        Err(error) => panic!("invalid static Cairn Radicle COB type name: {error}"),
    });

pub type ChangeOp = Op<Action>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Action(pub ChangeCobAction);

impl Action {
    pub fn into_core(self) -> ChangeCobAction {
        self.0
    }

    pub fn as_core(&self) -> &ChangeCobAction {
        &self.0
    }
}

impl From<ChangeCobAction> for Action {
    fn from(action: ChangeCobAction) -> Self {
        Self(action)
    }
}

impl CobAction for Action {
    fn parents(&self) -> Vec<git::Oid> {
        action_parent_oids(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadicleChangeCob {
    state: ChangeCobState,
}

impl RadicleChangeCob {
    pub fn state(&self) -> &ChangeCobState {
        &self.state
    }

    pub fn into_state(self) -> ChangeCobState {
        self.state
    }
}

impl Deref for RadicleChangeCob {
    type Target = ChangeCobState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl CobWithType for RadicleChangeCob {
    fn type_name() -> &'static TypeName {
        &TYPENAME
    }
}

impl Cob for RadicleChangeCob {
    type Action = Action;
    type Error = Error;

    fn from_root<R: ReadRepository>(op: ChangeOp, repo: &R) -> Result<Self, Self::Error> {
        validate_manifest(&op)?;
        validate_root_relations(&op)?;
        let operation = operation_from_radicle(&op, repo)?;
        let state = initialize_change_cob(operation)?;
        let repository_id = repo.id().to_string();
        if state.repository_id != repository_id {
            return Err(Error::RepositoryMismatch {
                declared: state.repository_id,
                actual: repository_id,
            });
        }
        Ok(Self { state })
    }

    fn op<'a, R: ReadRepository, I: IntoIterator<Item = &'a cob::Entry>>(
        &mut self,
        op: ChangeOp,
        _concurrent: I,
        repo: &R,
    ) -> Result<(), Self::Error> {
        validate_manifest(&op)?;
        let operation = operation_from_radicle(&op, repo)?;
        apply_change_cob_operation(&mut self.state, operation)?;
        Ok(())
    }
}

impl<R: ReadRepository> cob::Evaluate<R> for RadicleChangeCob {
    type Error = Error;

    fn init(entry: &cob::Entry, repo: &R) -> Result<Self, Self::Error> {
        let op = ChangeOp::try_from(entry)?;
        Self::from_root(op, repo)
    }

    fn apply<'a, I: Iterator<Item = (&'a EntryId, &'a cob::Entry)>>(
        &mut self,
        entry: &cob::Entry,
        concurrent: I,
        repo: &R,
    ) -> Result<(), Self::Error> {
        let op = ChangeOp::try_from(entry)?;
        self.op(op, concurrent.map(|(_, entry)| entry), repo)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("identity document is required for Cairn change operations")]
    MissingIdentity,
    #[error("operation identity document failed to load: {0}")]
    Identity(#[from] DocError),
    #[error("operation decoding failed: {0}")]
    Op(#[from] OpEncodingError),
    #[error("Cairn change reducer rejected operation: {0}")]
    Core(#[from] ChangeCobError),
    #[error("unexpected COB type {actual}; expected {expected}")]
    TypeMismatch { expected: String, actual: String },
    #[error("unsupported COB manifest version {actual}; expected {expected}")]
    VersionMismatch { expected: usize, actual: usize },
    #[error("root action base Git object {base} is missing from related objects")]
    MissingRelatedBase { base: String },
    #[error("root repository identity {declared} does not match Radicle repository {actual}")]
    RepositoryMismatch { declared: String, actual: String },
}

fn operation_from_radicle<R: ReadRepository>(
    op: &ChangeOp,
    repo: &R,
) -> Result<ChangeCobOperation, Error> {
    let doc = op.identity_doc(repo)?.ok_or(Error::MissingIdentity)?;
    let is_delegate = doc.is_delegate(&op.author.into());
    Ok(ChangeCobOperation {
        id: op.id.to_string(),
        actor: op.author.to_string(),
        is_delegate,
        actions: op.actions.iter().cloned().map(Action::into_core).collect(),
    })
}

fn validate_manifest(op: &ChangeOp) -> Result<(), Error> {
    if op.manifest.type_name != *TYPENAME {
        return Err(Error::TypeMismatch {
            expected: TYPENAME.to_string(),
            actual: op.manifest.type_name.to_string(),
        });
    }
    let expected = usize::from(cob::Version::default());
    let actual = usize::from(op.manifest.version);
    if actual != expected {
        return Err(Error::VersionMismatch { expected, actual });
    }
    Ok(())
}

fn validate_root_relations(op: &ChangeOp) -> Result<(), Error> {
    for action in op.actions.iter() {
        let ChangeCobAction::Init { state } = action.as_core() else {
            continue;
        };
        let Ok(base) = git::Oid::from_str(&state.base_git_oid) else {
            continue;
        };
        if !op.related.contains(&base) {
            return Err(Error::MissingRelatedBase {
                base: state.base_git_oid.clone(),
            });
        }
    }
    Ok(())
}

fn action_parent_oids(action: &ChangeCobAction) -> Vec<git::Oid> {
    match action {
        ChangeCobAction::Init { state } => parse_parent_oid(&state.base_git_oid),
        ChangeCobAction::SetArtifact { .. }
        | ChangeCobAction::AttachEvidence { .. }
        | ChangeCobAction::SetLifecycle { .. } => Vec::new(),
    }
}

fn parse_parent_oid(value: &str) -> Vec<git::Oid> {
    git::Oid::from_str(value).map_or_else(|_| Vec::new(), |oid| vec![oid])
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use cairn_core::{
        ChangeArtifactId, ChangeCobInit, ChangeCobOperation, RADICLE_CHANGE_COB_SCHEMA_V1,
    };
    use nonempty::NonEmpty;
    use radicle::{
        cob::{
            store::{access::WriteAs, encoding, Store},
            Manifest, Timestamp, Version,
        },
        crypto::test::signer::MockSigner,
        node::device::Device,
        test::setup::NodeWithRepo,
    };

    use super::*;

    const ROOT_OPERATION: &str = "1111111111111111111111111111111111111111";
    const ACTOR: &str = "actor";
    const BASE_GIT_OID: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const POLICY_HASH: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const EXPECTED_PARENT_COUNT: usize = 1;

    fn init_action() -> Action {
        init_action_for("rad:z3example", BASE_GIT_OID)
    }

    fn init_action_for(repository_id: &str, base_git_oid: &str) -> Action {
        Action(ChangeCobAction::Init {
            state: ChangeCobInit {
                schema: RADICLE_CHANGE_COB_SCHEMA_V1.to_string(),
                change_id: "radicle-change-cob".to_string(),
                repository_id: repository_id.to_string(),
                base_git_oid: base_git_oid.to_string(),
                policy_hash: POLICY_HASH.to_string(),
                artifacts: BTreeMap::from([(
                    ChangeArtifactId::proposal(),
                    "# Proposal\n".to_string(),
                )]),
            },
        })
    }

    #[test]
    fn stable_type_name_and_action_encoding() {
        assert_eq!(
            RadicleChangeCob::type_name().to_string(),
            RADICLE_CHANGE_COB_TYPE_NAME
        );
        let action = init_action();
        let encoded = encoding::encode(action.clone()).unwrap();
        let decoded: Action = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(decoded, action);
        assert_eq!(action.parents().len(), EXPECTED_PARENT_COUNT);
    }

    #[test]
    fn wrapper_operation_reaches_pure_root_validation() {
        let operation = ChangeCobOperation {
            id: ROOT_OPERATION.to_string(),
            actor: ACTOR.to_string(),
            is_delegate: false,
            actions: vec![init_action().into_core()],
        };
        let state = initialize_change_cob(operation).unwrap();
        assert_eq!(state.change_id, "radicle-change-cob");
    }

    #[test]
    fn malformed_action_encoding_and_root_shape_fail() {
        let malformed = br#"{"type":"unknown"}"#;
        assert!(serde_json::from_slice::<Action>(malformed).is_err());
        let invalid = ChangeCobOperation {
            id: ROOT_OPERATION.to_string(),
            actor: ACTOR.to_string(),
            is_delegate: false,
            actions: Vec::new(),
        };
        assert!(matches!(
            initialize_change_cob(invalid),
            Err(ChangeCobError::InvalidRootShape)
        ));
    }

    #[test]
    fn manifest_version_is_stable() {
        let manifest = Manifest::new(TYPENAME.clone(), Version::default());
        assert_eq!(manifest.type_name, *TYPENAME);
        assert_eq!(
            usize::from(manifest.version),
            usize::from(Version::default())
        );
    }

    // r[verify cairn.radicle_change_cob.radicle_adapter]
    // r[verify cairn.radicle_change_cob.radicle_adapter.mapping]
    // r[verify cairn.radicle_change_cob.radicle_adapter.reject]
    #[test]
    fn git_backed_store_creates_updates_and_gets_change_cob() {
        let NodeWithRepo { node, repo } = NodeWithRepo::default();
        let repository_id = repo.id().to_string();
        let base_git_oid = repo.head().unwrap().1;
        let identity = repo.identity_head().unwrap();
        let mut store = Store::<RadicleChangeCob, _, _>::open(&*repo, WriteAs::new(&node.signer))
            .unwrap()
            .identity(identity);
        let root = init_action_for(&repository_id, &base_git_oid.to_string());
        let (object_id, created) = store
            .create("Create Cairn change", NonEmpty::new(root), Vec::new())
            .unwrap();
        assert_eq!(created.repository_id, repository_id);

        let outsider = Device::<MockSigner>::default();
        let mut outsider_store =
            Store::<RadicleChangeCob, _, _>::open(&*repo, WriteAs::new(&outsider))
                .unwrap()
                .identity(identity);
        let unauthorized = Action(ChangeCobAction::SetArtifact {
            artifact: ChangeArtifactId::proposal(),
            content: Some("# Unauthorized proposal\n".to_string()),
            basis: BTreeSet::from([object_id.to_string()]),
        });
        assert!(outsider_store
            .update(
                &TYPENAME,
                object_id,
                "Unauthorized Cairn edit",
                NonEmpty::new(unauthorized),
                Vec::new(),
            )
            .is_err());
        let unchanged = store.get(&object_id).unwrap().unwrap();
        assert_eq!(unchanged.state(), created.state());

        let subject_root = cairn_core::change_cob_content_root(&created).unwrap();
        let lifecycle = Action(ChangeCobAction::SetLifecycle {
            status: cairn_core::ChangeLifecycleStatus::ArchiveRequested,
            subject_root,
            evidence: BTreeSet::new(),
            basis: BTreeSet::from([object_id.to_string()]),
        });
        store
            .update(
                &TYPENAME,
                object_id,
                "Request Cairn archive",
                NonEmpty::new(lifecycle),
                Vec::new(),
            )
            .unwrap();
        let fetched = store.get(&object_id).unwrap().unwrap();
        assert!(matches!(
            cairn_core::change_cob_lifecycle_view(&fetched).unwrap(),
            cairn_core::ChangeLifecycleView::Current(cairn_core::ChangeLifecycleRecord {
                status: cairn_core::ChangeLifecycleStatus::ArchiveRequested,
                ..
            })
        ));
    }

    #[test]
    fn missing_identity_document_is_rejected() {
        let NodeWithRepo { node, repo } = NodeWithRepo::default();
        let action = init_action_for(&repo.id().to_string(), BASE_GIT_OID);
        let operation_id = git::Oid::from_str(ROOT_OPERATION).unwrap();
        let mut op = ChangeOp::new(
            operation_id,
            NonEmpty::new(action.clone()),
            *node.signer.public_key(),
            Timestamp::from_secs(1),
            None,
            Manifest::new(TYPENAME.clone(), Version::default()),
        );
        op.related = action.parents();
        assert!(matches!(
            RadicleChangeCob::from_root(op, &*repo),
            Err(Error::MissingIdentity)
        ));
    }

    #[test]
    fn evidence_without_git_oid_has_no_parent() {
        let evidence = ChangeCobAction::AttachEvidence {
            evidence: cairn_core::ChangeCobEvidence {
                blake3: POLICY_HASH.to_string(),
                schema: "receipt.v1".to_string(),
                role: "gate".to_string(),
                subject_root: POLICY_HASH.to_string(),
                policy_hash: POLICY_HASH.to_string(),
                git_oid: None,
                uri: None,
                non_claims: BTreeSet::from(["does not prove correctness".to_string()]),
            },
        };
        assert!(Action(evidence).parents().is_empty());
    }
}
