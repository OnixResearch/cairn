mod support;

use std::{collections::BTreeMap, error::Error};

use cairn_core::{
    change_cob_content_root, change_cob_state_hash, inspect_change_cob_conflicts,
    plan_change_cob_resolution, plan_change_cob_update, ChangeArtifactId, ChangeCobAction,
    ChangeCobError, ChangeCollaborationResolutionSnapshot, ChangeCollaborationUpdateSnapshot,
};
use onix_cairn_radicle::{create_change_resolution, inspect_change_status, ChangeImportError};
use radicle::{
    crypto::PublicKey,
    test::setup::{Network, NodeWithRepo},
};

use support::{
    add_delegate, assert_same_typed_state, assert_signed_change_ref, import_change,
    resolution_selection, update_proposal, validate_distinct_peers, PeerDescriptor, ALICE_PROPOSAL,
    BOB_PROPOSAL, CHANGE_ID, EXPECTED_CONFLICT_HEADS, POLICY_HASH,
};

fn peer_descriptor(peer: &NodeWithRepo) -> PeerDescriptor {
    PeerDescriptor {
        home: peer.node.root.display().to_string(),
        storage: peer.node.storage.path().display().to_string(),
        public_key: peer.node.signer.public_key().to_string(),
        repository_id: peer.repo.id.to_string(),
    }
}

// r[impl cairn.radicle_two_peer_convergence.transport]
// r[verify cairn.radicle_two_peer_convergence.transport.independent]
#[test]
fn shared_peer_identity_is_rejected_before_transport() {
    let repository_id = "rad:z3-shared-fixture".to_string();
    let alice = PeerDescriptor {
        home: "alice-home".to_string(),
        storage: "alice-storage".to_string(),
        public_key: "alice-key".to_string(),
        repository_id: repository_id.clone(),
    };
    let mut invalid = alice.clone();
    assert_eq!(
        validate_distinct_peers(&alice, &invalid),
        Err("peer homes must be distinct")
    );
    invalid.home = "bob-home".to_string();
    assert_eq!(
        validate_distinct_peers(&alice, &invalid),
        Err("peer storage paths must be distinct")
    );
    invalid.storage = "bob-storage".to_string();
    assert_eq!(
        validate_distinct_peers(&alice, &invalid),
        Err("peer public keys must be distinct")
    );
    invalid.public_key = "bob-key".to_string();
    invalid.repository_id = "rad:z3-other-fixture".to_string();
    assert_eq!(
        validate_distinct_peers(&alice, &invalid),
        Err("peers must inspect the same repository ID")
    );
    invalid.repository_id = repository_id;
    assert_eq!(validate_distinct_peers(&alice, &invalid), Ok(()));
}

// r[verify cairn.radicle_two_peer_convergence.transport.success]
// r[verify cairn.radicle_two_peer_convergence.conflict.success]
// r[verify cairn.radicle_two_peer_convergence.conflict.stale]
// r[verify cairn.radicle_two_peer_convergence.resolution.success]
// r[verify cairn.radicle_two_peer_convergence.resolution.blocked]
// r[verify cairn.radicle_two_peer_convergence.blocked.authority]
#[test]
fn signed_peer_namespaces_converge_through_conflict_and_resolution() -> Result<(), Box<dyn Error>> {
    let network = Network::default();
    let alice = &network.alice;
    let bob = &network.bob;
    let eve = &network.eve;
    validate_distinct_peers(&peer_descriptor(alice), &peer_descriptor(bob))?;

    assert!(matches!(
        inspect_change_status(&eve.repo, eve.node.signer.public_key(), CHANGE_ID),
        Err(ChangeImportError::ChangeObjectMissing { .. })
    ));

    add_delegate(
        &alice.repo,
        &alice.node.signer,
        *bob.node.signer.public_key(),
    )?;
    let identity_updates = bob.repo.fetch(&alice.node);
    assert!(!identity_updates.is_empty());
    let bob_authority = onix_cairn_radicle::inspect_current_delegate_authority(
        &bob.repo,
        bob.node.signer.public_key(),
    )?;
    assert!(bob_authority.is_delegate);

    let imported = import_change(&alice.repo, &alice.node.signer)?;
    let import_updates = bob.repo.fetch(&alice.node);
    assert!(!import_updates.is_empty());
    assert!(import_updates
        .iter()
        .any(|update| update.name().as_str().contains("refs/cobs/")));
    assert_signed_change_ref(&bob.repo, *alice.node.signer.public_key())?;
    let bob_imported = inspect_change_status(&bob.repo, bob.node.signer.public_key(), CHANGE_ID)?;
    assert_same_typed_state(&imported, &bob_imported)?;
    assert_eq!(
        change_cob_content_root(&bob_imported.state)?,
        change_cob_content_root(&imported.state)?
    );

    let alice_updated = update_proposal(&alice.repo, &alice.node.signer, ALICE_PROPOSAL)?;
    let bob_updated = update_proposal(&bob.repo, &bob.node.signer, BOB_PROPOSAL)?;
    assert_ne!(
        change_cob_state_hash(&alice_updated.state)?,
        change_cob_state_hash(&bob_updated.state)?
    );
    assert!(!inspect_change_cob_conflicts(&alice_updated.state)?.has_conflicts());
    assert!(!inspect_change_cob_conflicts(&bob_updated.state)?.has_conflicts());

    let eve_inspection = inspect_change_status(&eve.repo, eve.node.signer.public_key(), CHANGE_ID);
    assert!(matches!(
        eve_inspection,
        Err(ChangeImportError::ChangeObjectMissing { .. })
    ));

    let alice_fetch = alice.repo.fetch(&bob.node);
    let bob_fetch = bob.repo.fetch(&alice.node);
    assert!(!alice_fetch.is_empty());
    assert!(!bob_fetch.is_empty());
    assert_signed_change_ref(&alice.repo, *bob.node.signer.public_key())?;
    assert_signed_change_ref(&bob.repo, *alice.node.signer.public_key())?;
    let alice_conflicted =
        inspect_change_status(&alice.repo, alice.node.signer.public_key(), CHANGE_ID)?;
    let bob_conflicted = inspect_change_status(&bob.repo, bob.node.signer.public_key(), CHANGE_ID)?;
    assert_same_typed_state(&alice_conflicted, &bob_conflicted)?;
    let conflicts = inspect_change_cob_conflicts(&alice_conflicted.state)?;
    assert!(conflicts.has_conflicts());
    assert_eq!(conflicts.artifact_conflicts.len(), 1);
    let proposal_conflict = &conflicts.artifact_conflicts[0];
    assert_eq!(proposal_conflict.artifact, ChangeArtifactId::proposal());
    assert_eq!(proposal_conflict.heads.len(), EXPECTED_CONFLICT_HEADS);
    let actors = proposal_conflict
        .heads
        .iter()
        .map(|head| head.actor.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(actors.len(), EXPECTED_CONFLICT_HEADS);
    let alice_key: PublicKey = *alice.node.signer.public_key();
    let bob_key: PublicKey = *bob.node.signer.public_key();
    assert!(actors.contains(alice_key.to_string().as_str()));
    assert!(actors.contains(bob_key.to_string().as_str()));

    let stale_snapshot = ChangeCollaborationUpdateSnapshot {
        change_id: CHANGE_ID.to_string(),
        policy_hash: POLICY_HASH.to_string(),
        actor: eve.node.signer.public_key().to_string(),
        is_delegate: false,
        proposal: Some("# Unauthorized proposal\n".to_string()),
        design: None,
        tasks: None,
        metadata: None,
        delta_specs: BTreeMap::new(),
    };
    assert!(matches!(
        plan_change_cob_update(&bob_imported.state, stale_snapshot),
        Err(ChangeCobError::Unauthorized { .. })
    ));

    let selection = resolution_selection(&alice_conflicted, ALICE_PROPOSAL)?;
    let resolution = plan_change_cob_resolution(
        &alice_conflicted.state,
        ChangeCollaborationResolutionSnapshot {
            change_id: CHANGE_ID.to_string(),
            policy_hash: POLICY_HASH.to_string(),
            actor: alice_conflicted.authority.actor.clone(),
            is_delegate: alice_conflicted.authority.is_delegate,
            artifact_selections: vec![selection],
            lifecycle_status: None,
        },
    )?;
    let expected_bases = proposal_conflict
        .heads
        .iter()
        .map(|head| head.revision_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let resolution_bases = resolution
        .actions
        .iter()
        .find_map(|action| match action {
            ChangeCobAction::SetArtifact {
                artifact, basis, ..
            } if *artifact == ChangeArtifactId::proposal() => Some(basis),
            ChangeCobAction::Init { .. }
            | ChangeCobAction::SetArtifact { .. }
            | ChangeCobAction::AttachEvidence { .. }
            | ChangeCobAction::SetLifecycle { .. } => None,
        })
        .ok_or("resolution proposal action is missing")?;
    assert_eq!(resolution_bases, &expected_bases);

    create_change_resolution(
        &alice.repo,
        &alice.node.signer,
        &resolution,
        &alice_conflicted,
    )?;
    let resolution_updates = bob.repo.fetch(&alice.node);
    assert!(!resolution_updates.is_empty());
    assert_signed_change_ref(&bob.repo, alice_key)?;
    let alice_resolved =
        inspect_change_status(&alice.repo, alice.node.signer.public_key(), CHANGE_ID)?;
    let bob_resolved = inspect_change_status(&bob.repo, bob.node.signer.public_key(), CHANGE_ID)?;
    assert_same_typed_state(&alice_resolved, &bob_resolved)?;
    assert!(!inspect_change_cob_conflicts(&alice_resolved.state)?.has_conflicts());
    assert_eq!(
        change_cob_content_root(&alice_resolved.state)?,
        resolution.expected_content_root
    );
    assert_eq!(
        alice_resolved
            .state
            .artifacts
            .get(&ChangeArtifactId::proposal())
            .and_then(|revisions| revisions.revisions.values().find(|revision| {
                revision.value.as_deref() == Some(ALICE_PROPOSAL)
                    && revision.basis == expected_bases
            }))
            .and_then(|revision| revision.value.as_deref()),
        Some(ALICE_PROPOSAL)
    );
    Ok(())
}
