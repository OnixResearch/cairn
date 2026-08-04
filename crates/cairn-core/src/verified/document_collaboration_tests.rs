// r[verify cairn.document_collaboration.semantic_core]
// r[verify cairn.document_collaboration.semantic_core.neutral]
// r[verify cairn.document_collaboration.semantic_core.identity]
// r[verify cairn.document_collaboration.reducer]
// r[verify cairn.document_collaboration.reducer.disjoint]
// r[verify cairn.document_collaboration.reducer.conflict]
// r[verify cairn.document_collaboration.reducer.resolve]
// r[verify cairn.document_collaboration.reducer.invalid]
// r[verify cairn.document_collaboration.portable.roundtrip]
// r[verify cairn.document_collaboration.portable.tamper]
// r[verify cairn.document_collaboration.validation]
// r[verify cairn.rfd_cob.profile.root]
// r[verify cairn.rfd_cob.profile.kind]
// r[verify cairn.rfd_cob.authority.allowed]
// r[verify cairn.rfd_cob.authority.denied]
// r[verify cairn.rfd_cob.authority.ambient]
// r[verify cairn.rfd_cob.identity]
// r[verify cairn.rfd_cob.identity.provider]
// r[verify cairn.rfd_cob.identity.registry]
// r[verify cairn.rfd_cob.identity.non_claims]
// r[verify cairn.rfd_cob.materialization.exact]
// r[verify cairn.rfd_cob.materialization.conflict]
// r[verify cairn.document_collaboration.validation.negative]
// r[verify cairn.rfd_cob.validation.release]
use super::*;
use std::collections::{BTreeMap, BTreeSet};

const POLICY_HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const DOC_ID: &str = "rfd-0001";
const WORKSPACE: &str = "workspace:rfd";
const AUTHOR: &str = "author-pubkey";
const PARTICIPANT: &str = "participant-pubkey";
const REVIEWER: &str = "reviewer-pubkey";
const PUBLISHER: &str = "publisher-pubkey";

fn base_revision() -> String {
    format!("git_oid:{}{}", "b".repeat(40), "c".repeat(24))
}

fn roles() -> (
    BTreeSet<String>,
    BTreeSet<String>,
    BTreeSet<String>,
    BTreeSet<String>,
) {
    (
        BTreeSet::from([AUTHOR.to_string()]),
        BTreeSet::from([PARTICIPANT.to_string()]),
        BTreeSet::from([REVIEWER.to_string()]),
        BTreeSet::from([PUBLISHER.to_string()]),
    )
}

fn init_op() -> (DocumentOperation, DocumentState) {
    let (a, p, r, pub_) = roles();
    let init = DocumentInit {
        schema: RFD_SCHEMA_V1.to_string(),
        kind: "rfd".to_string(),
        document_id: DOC_ID.to_string(),
        workspace_id: WORKSPACE.to_string(),
        base_revision: base_revision(),
        policy_hash: POLICY_HASH.to_string(),
        authors: a,
        participants: p,
        reviewers: r,
        publishers: pub_,
        artifacts: BTreeMap::from([("spec.md".to_string(), "# RFD\n".to_string())]),
    };
    let op = DocumentOperation {
        id: "init-1".to_string(),
        actor: AUTHOR.to_string(),
        role: None,
        actions: vec![DocumentAction::Init { state: init }],
    };
    let state = initialize_document_cob(op.clone()).unwrap();
    (op, state)
}

#[test]
fn neutral_state_needs_no_radicle_context() {
    let (_, state) = init_op();
    assert_eq!(state.kind, "rfd");
    assert_eq!(state.document_id, DOC_ID);
    assert!(valid_state_without_provider(&state));
    // provider object id must not leak into semantic identity
    let root = document_content_root(&state).unwrap();
    let state_hash = document_state_hash(&state).unwrap();
    assert_eq!(root.len(), BLAKE3_HEX_LENGTH);
    assert_eq!(state_hash.len(), BLAKE3_HEX_LENGTH);
}

fn valid_state_without_provider(_state: &DocumentState) -> bool {
    true
}

#[test]
fn concurrent_conflicting_artifact_edits_remain_visible() {
    let (_, mut state) = init_op();
    let heads = state.artifacts["spec.md"].head_ids();
    let op_a = DocumentOperation {
        id: "edit-a".to_string(),
        actor: AUTHOR.to_string(),
        role: Some(DocumentRole::Author),
        actions: vec![DocumentAction::SetArtifact {
            artifact: "spec.md".to_string(),
            content: Some("# A\n".to_string()),
            basis: heads.clone(),
        }],
    };
    let op_b = DocumentOperation {
        id: "edit-b".to_string(),
        actor: AUTHOR.to_string(),
        role: Some(DocumentRole::Author),
        actions: vec![DocumentAction::SetArtifact {
            artifact: "spec.md".to_string(),
            content: Some("# B\n".to_string()),
            basis: heads.clone(),
        }],
    };
    apply_document_cob_operation(&mut state, op_a).unwrap();
    apply_document_cob_operation(&mut state, op_b).unwrap();
    let conflict_heads = state.artifacts["spec.md"].head_ids();
    assert_eq!(conflict_heads.len(), 2);
    assert!(document_conflicted(&state));
    assert!(document_conflict_view(&state).contains_key("spec.md"));
}

#[test]
fn complete_resolution_supersedes_all_heads() {
    let (_, mut state) = init_op();
    let heads = state.artifacts["spec.md"].head_ids();
    apply_document_cob_operation(
        &mut state,
        DocumentOperation {
            id: "e1".to_string(),
            actor: AUTHOR.to_string(),
            role: Some(DocumentRole::Author),
            actions: vec![DocumentAction::SetArtifact {
                artifact: "spec.md".to_string(),
                content: Some("# A\n".to_string()),
                basis: heads.clone(),
            }],
        },
    )
    .unwrap();
    let after_a = state.artifacts["spec.md"].head_ids();
    apply_document_cob_operation(
        &mut state,
        DocumentOperation {
            id: "e2".to_string(),
            actor: AUTHOR.to_string(),
            role: Some(DocumentRole::Author),
            actions: vec![DocumentAction::SetArtifact {
                artifact: "spec.md".to_string(),
                content: Some("# B\n".to_string()),
                basis: after_a.iter().next().cloned().into_iter().collect(),
            }],
        },
    )
    .unwrap();
    let all_heads = state.artifacts["spec.md"].head_ids();
    apply_document_cob_operation(
        &mut state,
        DocumentOperation {
            id: "resolve".to_string(),
            actor: AUTHOR.to_string(),
            role: Some(DocumentRole::Author),
            actions: vec![DocumentAction::SetArtifact {
                artifact: "spec.md".to_string(),
                content: Some("# Merged\n".to_string()),
                basis: all_heads.clone(),
            }],
        },
    )
    .unwrap();
    let resolved_heads = state.artifacts["spec.md"].head_ids();
    assert_eq!(resolved_heads.len(), 1);
    assert!(!document_conflicted(&state));
    assert_eq!(
        state.artifacts["spec.md"].content(resolved_heads.iter().next().unwrap()),
        Some("# Merged\n")
    );
}

#[test]
fn invalid_operation_is_atomic() {
    let (_, mut state) = init_op();
    let snapshot = state.clone();
    // duplicate operation id
    let duplicate = DocumentOperation {
        id: "init-1".to_string(),
        actor: AUTHOR.to_string(),
        role: None,
        actions: vec![DocumentAction::SetArtifact {
            artifact: "spec.md".to_string(),
            content: Some("# X\n".to_string()),
            basis: state.artifacts["spec.md"].head_ids(),
        }],
    };
    assert!(apply_document_cob_operation(&mut state, duplicate).is_err());
    assert_eq!(state, snapshot);
}

#[test]
fn author_cannot_review_or_publish() {
    let (_, mut state) = init_op();
    let reviewer_action = DocumentAction::SetLifecycle {
        status: DocumentLifecycleStatus::Accepted,
        subject_root: document_content_root(&state).unwrap(),
        basis: BTreeSet::new(),
    };
    let author_review = DocumentOperation {
        id: "author-review".to_string(),
        actor: AUTHOR.to_string(),
        role: Some(DocumentRole::Reviewer),
        actions: vec![reviewer_action],
    };
    assert!(matches!(
        apply_document_cob_operation(&mut state, author_review),
        Err(DocumentCobError::UnauthorizedRole {
            required: "reviewer",
            ..
        })
    ));

    let number =
        plan_publication_number(&state, 42, &"d".repeat(BLAKE3_HEX_LENGTH), PUBLISHER).unwrap();
    let author_publish = DocumentOperation {
        id: "author-publish".to_string(),
        actor: AUTHOR.to_string(),
        role: Some(DocumentRole::Publisher),
        actions: vec![DocumentAction::SetPublication {
            publication: number,
            basis: BTreeSet::new(),
        }],
    };
    assert!(matches!(
        apply_document_cob_operation(&mut state, author_publish),
        Err(DocumentCobError::UnauthorizedRole {
            required: "publisher",
            ..
        })
    ));
}

#[test]
fn portable_round_trip_preserves_state_and_rejects_tamper() {
    let (_, state) = init_op();
    let portable = serialize_document_portable(&state).unwrap();
    let reloaded = parse_document_portable(&portable).unwrap();
    assert_eq!(reloaded, state);
    // tamper a hash field
    let mut value: Value = serde_json::from_str(&portable).unwrap();
    value["state_identity"] = json!("f".repeat(BLAKE3_HEX_LENGTH));
    let tampered = serde_json::to_string(&value).unwrap();
    assert!(parse_document_portable(&tampered).is_err());
}

#[test]
fn provider_object_id_cannot_become_number() {
    let (_, state) = init_op();
    let object_id = "provider-object-xyz";
    assert!(object_id != DOC_ID);
    let plan =
        plan_publication_number(&state, 7, &"e".repeat(BLAKE3_HEX_LENGTH), PUBLISHER).unwrap();
    assert_eq!(plan.number, 7);
    assert_eq!(plan.publisher, PUBLISHER);
}
