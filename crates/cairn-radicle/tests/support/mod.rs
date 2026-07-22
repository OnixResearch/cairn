use std::{collections::BTreeMap, error::Error};

use cairn_core::{
    change_cob_content_root, change_cob_import_content_root, change_cob_state_hash,
    inspect_change_cob_conflicts, plan_change_cob_update, ChangeArtifactHeadSelection,
    ChangeArtifactId, ChangeCobInit, ChangeCollaborationUpdateSnapshot,
    RADICLE_CHANGE_COB_SCHEMA_V1,
};
use onix_cairn_radicle::{
    create_change_import, create_change_update, inspect_change_import, inspect_change_status,
    ChangeStatusInspection,
};
use radicle::{
    cob::{self, identity::Identity},
    crypto::{self, PublicKey},
    storage::{git::Repository, ReadRepository, RemoteRepository, SignRepository},
};

pub const CHANGE_ID: &str = "two-peer-convergence";
pub const POLICY_HASH: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
pub const INITIAL_PROPOSAL: &str = "# Initial proposal\n";
pub const ALICE_PROPOSAL: &str = "# Alice proposal\n";
pub const BOB_PROPOSAL: &str = "# Bob proposal\n";
pub const EXPECTED_CONFLICT_HEADS: usize = 2;
// r[impl cairn.radicle_two_peer_convergence.non_claims]
pub const TWO_PEER_NON_CLAIMS: &[&str] = &[
    "public-network publication",
    "peer discovery",
    "arbitrary-peer trust",
    "availability",
    "eventual convergence",
    "partition recovery",
    "release eligibility",
    "automatic peer discovery or background synchronization",
];
const IDENTITY_UPDATE_TITLE: &str = "Add second convergence delegate";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerDescriptor {
    pub home: String,
    pub storage: String,
    pub public_key: String,
    pub repository_id: String,
}

pub fn validate_distinct_peers(
    alice: &PeerDescriptor,
    bob: &PeerDescriptor,
) -> Result<(), &'static str> {
    if alice.home == bob.home {
        return Err("peer homes must be distinct");
    }
    if alice.storage == bob.storage {
        return Err("peer storage paths must be distinct");
    }
    if alice.public_key == bob.public_key {
        return Err("peer public keys must be distinct");
    }
    if alice.repository_id != bob.repository_id {
        return Err("peers must inspect the same repository ID");
    }
    Ok(())
}

pub fn assert_signed_change_ref(
    repo: &Repository,
    remote_key: PublicKey,
) -> Result<(), Box<dyn Error>> {
    let remote = repo.remote(&remote_key)?;
    assert_eq!(remote.refs.id(), remote_key);
    assert!(remote
        .refs
        .refs()
        .iter()
        .any(|(name, _)| name.as_str().contains("refs/cobs/")));
    Ok(())
}

pub fn add_delegate<Signer>(
    repo: &Repository,
    signer: &Signer,
    delegate: PublicKey,
) -> Result<(), Box<dyn Error>>
where
    Signer: crypto::signature::Keypair<VerifyingKey = PublicKey>,
    Signer: crypto::signature::Signer<crypto::Signature>,
    Signer: crypto::signature::Signer<crypto::ssh::ExtendedSignature>,
    Signer: crypto::signature::Verifier<crypto::Signature>,
{
    let mut identity = Identity::load_mut(repo, signer)?;
    let mut document = identity.doc().clone().edit();
    document.delegate(delegate.into());
    let document = document.verified()?;
    identity.update(cob::Title::new(IDENTITY_UPDATE_TITLE)?, "", &document)?;
    repo.sign_refs(signer)?;
    Ok(())
}

pub fn import_change<Signer>(
    repo: &Repository,
    signer: &Signer,
) -> Result<ChangeStatusInspection, Box<dyn Error>>
where
    Signer: crypto::signature::Keypair<VerifyingKey = PublicKey>,
    Signer: crypto::signature::Signer<crypto::Signature>,
    Signer: crypto::signature::Signer<crypto::ssh::ExtendedSignature>,
    Signer: crypto::signature::Verifier<crypto::Signature>,
{
    let signer_key = signer.verifying_key();
    let init = ChangeCobInit {
        schema: RADICLE_CHANGE_COB_SCHEMA_V1.to_string(),
        change_id: CHANGE_ID.to_string(),
        repository_id: repo.id().to_string(),
        base_git_oid: repo.head()?.1.to_string(),
        policy_hash: POLICY_HASH.to_string(),
        artifacts: BTreeMap::from([(ChangeArtifactId::proposal(), INITIAL_PROPOSAL.to_string())]),
    };
    let inspection = inspect_change_import(repo, &signer_key, CHANGE_ID)?;
    assert!(inspection.authority.is_delegate);
    assert!(inspection.existing_object_id.is_none());
    let content_root = change_cob_import_content_root(&init, &inspection.authority.actor)?;
    create_change_import(repo, signer, init, &inspection, &content_root)?;
    Ok(inspect_change_status(repo, &signer_key, CHANGE_ID)?)
}

pub fn update_proposal<Signer>(
    repo: &Repository,
    signer: &Signer,
    proposal: &str,
) -> Result<ChangeStatusInspection, Box<dyn Error>>
where
    Signer: crypto::signature::Keypair<VerifyingKey = PublicKey>,
    Signer: crypto::signature::Signer<crypto::Signature>,
    Signer: crypto::signature::Signer<crypto::ssh::ExtendedSignature>,
    Signer: crypto::signature::Verifier<crypto::Signature>,
{
    let signer_key = signer.verifying_key();
    let inspection = inspect_change_status(repo, &signer_key, CHANGE_ID)?;
    let plan = plan_change_cob_update(
        &inspection.state,
        ChangeCollaborationUpdateSnapshot {
            change_id: CHANGE_ID.to_string(),
            policy_hash: POLICY_HASH.to_string(),
            actor: inspection.authority.actor.clone(),
            is_delegate: inspection.authority.is_delegate,
            proposal: Some(proposal.to_string()),
            design: None,
            tasks: None,
            metadata: None,
            delta_specs: BTreeMap::new(),
        },
    )?;
    assert!(!plan.is_noop());
    create_change_update(repo, signer, &plan, &inspection)?;
    Ok(inspect_change_status(repo, &signer_key, CHANGE_ID)?)
}

pub fn assert_same_typed_state(
    left: &ChangeStatusInspection,
    right: &ChangeStatusInspection,
) -> Result<(), Box<dyn Error>> {
    assert_eq!(left.object_id, right.object_id);
    assert_eq!(left.state, right.state);
    assert_eq!(
        change_cob_state_hash(&left.state)?,
        change_cob_state_hash(&right.state)?
    );
    assert_eq!(
        change_cob_content_root(&left.state),
        change_cob_content_root(&right.state)
    );
    Ok(())
}

pub fn resolution_selection(
    inspection: &ChangeStatusInspection,
    selected_content: &str,
) -> Result<ChangeArtifactHeadSelection, Box<dyn Error>> {
    let conflicts = inspect_change_cob_conflicts(&inspection.state)?;
    let proposal = conflicts
        .artifact_conflicts
        .iter()
        .find(|conflict| conflict.artifact == ChangeArtifactId::proposal())
        .ok_or("proposal conflict is missing")?;
    let selected_hash = cairn_core::blake3_hex(selected_content.as_bytes());
    let selected = proposal
        .heads
        .iter()
        .find(|head| head.content_blake3.as_deref() == Some(selected_hash.as_str()))
        .ok_or("selected proposal head is missing")?;
    Ok(ChangeArtifactHeadSelection {
        artifact: ChangeArtifactId::proposal(),
        revision_id: selected.revision_id.clone(),
    })
}
