use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use cairn_core::{
    canonical_json_hash, ExceptionRequest, PolicyExceptionState, PolicyScope, ValidityObservation,
};
use serde_json::{json, Value};

const OBLIGATION_REQUIREMENT: &str = "cairn.supersede.agent_run_receipts.verification_contract";
const PASS_ONLY_REQUIREMENT: &str = "cairn.supersede.agent_run_receipts.pass_only";
const FRESHNESS_REQUIREMENT: &str = "cairn.supersede.agent_run_receipts.freshness";
const AUTHORITY_REQUIREMENT: &str = "cairn.supersede.agent_run_receipts.completion_authority";
const CLAIM_REQUIREMENT: &str = "cairn.verification_obligation_cob.claim_revision";
const CONTRACT_REVISION: &str =
    "cab928f04ff3d61d2ea9e2a2640262e2022dc3ba820d1949fad73d8d6dc977af";
const PASS_ONLY_REVISION: &str =
    "34abbfe64cae739b75554f48d89d0e0bc19ca8e55d54cf51a9363ffe84a6f58d";
const FRESHNESS_REVISION: &str =
    "653914140db582ef1145f36f87c233a9ab57a310f715cadf8a23efa5f5333e5e";
const AUTHORITY_REVISION: &str =
    "5427af464b1ebc00db599a9ec90cdee48d0ce7b4a0b2652c6dc1d134bd89451b";
const CLAIM_REVISION: &str =
    "1de40130cc0a88bb88288b16daa621de599b1508511648332e9e23ef11a3b666";
const SUBJECT: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const CONTEXT: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const EXCEPTION_POLICY: &str =
    "9f463020b564b252f6322a6f4ecfa8094a28a78ff5bc32e6b9ad1b2e01541542";
const CONTROL_REVISION: &str = FRESHNESS_REVISION;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repository root")
        .to_path_buf()
}

fn policy_path() -> PathBuf {
    repo_root().join("cairn-policy/generated/cairn-policy.json")
}

fn policy_hash() -> String {
    let value: Value = serde_json::from_slice(&fs::read(policy_path()).unwrap()).unwrap();
    canonical_json_hash(&value)
}

fn test_root(name: &str) -> PathBuf {
    let root = env::temp_dir().join(format!(
        "cairn-workflow-obligations-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    root
}

fn evidence(
    predicate: &str,
    requirement: &str,
    revision: &str,
    state: &str,
    policy_hash: &str,
) -> Value {
    json!({
        "predicate": predicate,
        "requirement_id": requirement,
        "revision_blake3": revision,
        "subject_blake3": SUBJECT,
        "context_blake3": CONTEXT,
        "policy_blake3": policy_hash,
        "observation_id": format!("observation:{predicate}"),
        "state": state,
    })
}

fn base_case(definition: &str, selection: &[(&str, &str)]) -> Value {
    let policy_hash = policy_hash();
    json!({
        "definition": definition,
        "subject_blake3": SUBJECT,
        "context_blake3": CONTEXT,
        "selected_alternatives": selection
            .iter()
            .map(|(node, alternative)| ((*node).to_string(), (*alternative).to_string()))
            .collect::<BTreeMap<_, _>>(),
        "evidence": [
            evidence("agent-receipt.verification-contract", OBLIGATION_REQUIREMENT, CONTRACT_REVISION, "satisfied", &policy_hash),
            evidence("agent-receipt.pass-only", PASS_ONLY_REQUIREMENT, PASS_ONLY_REVISION, "satisfied", &policy_hash),
            evidence("agent-receipt.freshness", FRESHNESS_REQUIREMENT, FRESHNESS_REVISION, "satisfied", &policy_hash),
            evidence("agent-receipt.completion-authority", AUTHORITY_REQUIREMENT, AUTHORITY_REVISION, "satisfied", &policy_hash),
            evidence("verification-scope.applicable", OBLIGATION_REQUIREMENT, CONTRACT_REVISION, "unsatisfied", &policy_hash),
            evidence("verification-obligation.claim-revision", CLAIM_REQUIREMENT, CLAIM_REVISION, "satisfied", &policy_hash),
        ],
    })
}

fn run_case(root: &Path, command: &str, case: &Value) -> (Output, Option<Value>) {
    let case_path = root.join(format!("{command}.json"));
    fs::write(&case_path, serde_json::to_vec_pretty(case).unwrap()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(repo_root())
        .env_remove("CAIRN_GLOBAL_REGISTRY")
        .args([
            "obligation",
            command,
            "--case",
            case_path.to_str().unwrap(),
            "--policy",
            policy_path().to_str().unwrap(),
        ])
        .output()
        .unwrap();
    let value = if output.stdout.is_empty() {
        None
    } else {
        Some(serde_json::from_slice(&output.stdout).unwrap())
    };
    (output, value)
}

fn exception_state() -> PolicyExceptionState {
    let mut requesters = BTreeSet::new();
    requesters.insert("requester".to_string());
    let mut reviewers = BTreeSet::new();
    reviewers.insert("reviewer".to_string());
    let mut approvers = BTreeSet::new();
    approvers.insert("approver".to_string());
    let mut revokers = BTreeSet::new();
    revokers.insert("revoker".to_string());
    let mut observers = BTreeSet::new();
    observers.insert("observer-a".to_string());
    let mut observations = std::collections::BTreeMap::new();
    observations.insert(
        "observer-a".to_string(),
        ValidityObservation {
            id: "validity:1".to_string(),
            observed_epoch: 10,
            finite_valid_until: 100,
            source: "observer-a".to_string(),
        },
    );
    PolicyExceptionState {
        schema: "cairn.policy-exception.v1".to_string(),
        kind: "policy-exception".to_string(),
        exception_id: "policy-exception:verify-freshness".to_string(),
        workspace_id: "workspace".to_string(),
        request: ExceptionRequest {
            policy_identity: EXCEPTION_POLICY.to_string(),
            scope: PolicyScope {
                rule_ids: vec!["allow-fs-write".to_string()],
                targets: vec!["crates/cairn-core".to_string()],
            },
            rationale: "preserve the original unsatisfied condition".to_string(),
            compensating_controls: vec!["sandboxed-revalidation".to_string()],
            request_root: "request-root".to_string(),
        },
        request_revision: 3,
        requesters,
        reviewers,
        approvers,
        revokers,
        observers,
        findings: BTreeMap::new(),
        approvals: Vec::new(),
        observations,
        lifecycle: "approved".to_string(),
        predecessor: None,
        operations: BTreeSet::from(["init".to_string()]),
    }
}

// r[verify cairn.workflow_obligations.export]
// r[verify cairn.workflow_obligations.inspection]
#[test]
fn export_reports_the_bound_predicate_and_obligation_ids_for_consumers() {
    let root = test_root("export");
    let case = base_case("verify", &[("verify.traceability", "claim-revision")]);
    let (output, value) = run_case(&root, "export", &case);
    assert!(output.status.success(), "{output:?}");
    let value = value.unwrap();
    assert_eq!(value["outcome"], "exported");
    let export = &value["export"];
    assert_eq!(export["workflow_id"], "verify");
    assert_eq!(export["binding"]["workflow_id"], "verify");
    assert_eq!(
        export["predicate"]["allowed_command_classes"],
        json!(["gate", "receipt-inspect", "traceability", "workflow-inspect"])
    );
    assert!(export["obligations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|obligation| obligation["id"] == "verify.traceability"));
    assert_eq!(
        export["predicate"]["selected_alternatives"]["verify.traceability"],
        "claim-revision"
    );
    assert!(export["predicate"]["evidence_requirements"]
        .as_array()
        .unwrap()
        .iter()
        .any(|requirement| requirement["requirement_id"] == OBLIGATION_REQUIREMENT
            && requirement["accepted_revision_blake3"] == CONTRACT_REVISION));
}

// r[verify cairn.workflow_obligations.evaluation]
#[test]
fn explain_reports_satisfied_and_remaining_obligations_and_ignores_agent_claims() {
    let root = test_root("explain");
    let (output, value) = run_case(
        &root,
        "explain",
        &base_case("verify", &[("verify.traceability", "claim-revision")]),
    );
    assert!(output.status.success(), "{output:?}");
    let report = &value.unwrap()["report"];
    assert_eq!(report["outcome"], "proved");
    assert_eq!(report["proved"], true);
    assert_eq!(report["successful_delivery"], true);
    assert!(report["remaining"].as_array().unwrap().is_empty());
    assert!(report["satisfied"]
        .as_array()
        .unwrap()
        .iter()
        .any(|id| id == "verify.traceability"));

    let mut claimed = base_case("verify", &[("verify.traceability", "claim-revision")]);
    claimed["evidence"] = json!([]);
    claimed["agent_claims"] = json!({
        "success_statement": "everything passed",
        "checked_task_boxes": 24,
        "activity_count": 120,
    });
    let (output, value) = run_case(&root, "explain", &claimed);
    assert!(output.status.success(), "{output:?}");
    let report = &value.unwrap()["report"];
    assert_eq!(report["proved"], false);
    assert_eq!(report["successful_delivery"], false);
    assert!(!report["remaining"].as_array().unwrap().is_empty());
    assert_eq!(report["ignored_claims"].as_array().unwrap().len(), 3);
}

#[test]
fn explain_keeps_incomplete_alternatives_unsatisfied() {
    let root = test_root("alternative");
    let mut case = base_case("verify", &[("verify.traceability", "claim-revision")]);
    let evidence = case["evidence"].as_array_mut().unwrap();
    evidence.retain(|observation| observation["predicate"] != "verification-obligation.claim-revision");
    let (output, value) = run_case(&root, "explain", &case);
    assert!(output.status.success(), "{output:?}");
    let report = &value.unwrap()["report"];
    assert_eq!(report["outcome"], "unknown");
    assert!(report["remaining"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["id"] == "verify.traceability.claim"));
}

// r[verify cairn.workflow_obligations.exception]
#[test]
fn explain_accepts_only_a_current_exception_with_control_evidence() {
    let root = test_root("exception");
    let mut case = base_case(
        "verify.bounded-exception",
        &[("verify.traceability", "claim-revision")],
    );
    let policy_hash = policy_hash();
    for observation in case["evidence"].as_array_mut().unwrap() {
        if observation["predicate"] == "agent-receipt.freshness" {
            observation["state"] = json!("unsatisfied");
        }
    }
    case["temporal"] = json!({
        "trace_id": "verify:selected-change",
        "observer": "cairn-lifecycle",
        "clock": "supplied-u64-epoch.v1",
        "window_start_epoch": 0,
        "horizon_epoch": 86400,
        "completeness": "complete",
        "events": [
            {"event": "receipt-recorded", "epoch": 10, "observer": "cairn-lifecycle"},
            {"event": "candidate-frozen", "epoch": 20, "observer": "cairn-lifecycle"}
        ],
        "observation_id": "trace:1",
    });
    case["exceptions"] = json!([{
        "path": "verify.exception.freshness",
        "state": exception_state(),
        "validity_epoch": 50,
        "control_evidence": [evidence(
            "exception.sandbox-revalidation",
            FRESHNESS_REQUIREMENT,
            CONTROL_REVISION,
            "satisfied",
            &policy_hash,
        )],
    }]);
    let (output, value) = run_case(&root, "explain", &case);
    assert!(output.status.success(), "{output:?}");
    let report = &value.unwrap()["report"];
    assert_eq!(report["outcome"], "exception_accepted");
    assert_eq!(report["proved"], false);
    assert_eq!(report["successful_delivery"], true);
    let freshness = report["completion"]
        .as_array()
        .unwrap()
        .iter()
        .find(|node| node["id"] == "verify.evidence.freshness")
        .unwrap();
    assert_eq!(freshness["state"], "exception_accepted");
    assert_eq!(freshness["original"]["state"], "unsatisfied");

    let mut revoked = case.clone();
    revoked["exceptions"][0]["state"]["lifecycle"] = json!("revoked");
    let (output, value) = run_case(&root, "explain", &revoked);
    assert!(output.status.success(), "{output:?}");
    let report = &value.unwrap()["report"];
    assert_eq!(report["outcome"], "unsatisfied");
    assert_eq!(report["successful_delivery"], false);

    let mut unpaid = case;
    unpaid["exceptions"][0]["control_evidence"] = json!([]);
    let (output, value) = run_case(&root, "explain", &unpaid);
    assert!(output.status.success(), "{output:?}");
    let report = &value.unwrap()["report"];
    assert_eq!(report["outcome"], "unsatisfied");
    assert_eq!(report["exceptions"][0]["control_state"], "unknown");
}

#[test]
fn unknown_definition_is_rejected_explicitly() {
    let root = test_root("unknown");
    let case = base_case("verify.missing", &[]);
    let (output, value) = run_case(&root, "explain", &case);
    assert!(!output.status.success());
    assert!(value.is_none());
    assert!(String::from_utf8_lossy(&output.stderr).contains("not admitted"));
}

#[test]
fn list_reports_admitted_definitions_without_a_case() {
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(repo_root())
        .env_remove("CAIRN_GLOBAL_REGISTRY")
        .args([
            "obligation",
            "list",
            "--policy",
            policy_path().to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["outcome"], "listed");
    let definitions = value["definitions"].as_array().unwrap();
    assert!(definitions.iter().any(|definition| definition["definition"] == "verify"));
    assert!(definitions
        .iter()
        .any(|definition| definition["definition"] == "verify.bounded-exception"
            && definition["temporal"] == true));
}
