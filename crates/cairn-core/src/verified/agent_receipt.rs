use serde_json::{json, Map, Value};

use crate::verified::evidence::canonical_json_hash;

pub const AGENT_RUN_RECEIPT_SCHEMA_VERSION: &str = "cairn.agent_run_receipt.v1";
pub const NO_MUTATION_PLAN_HASH: &str = "cairn.no_mutation_plan.v1";

const REQUIRED_RECEIPT_FIELDS: &[&str] = &[
    "schema_version",
    "task_id",
    "agent_identity",
    "input_hash",
    "plan_hash",
    "tool_manifest",
    "mutation_manifest",
    "verification_manifest",
    "nondeterminism_manifest",
    "replay_evidence",
    "verdict",
    "receipt_hash",
];
const KNOWN_ENTROPY_CLASSES: &[&str] = &[
    "clock",
    "network",
    "model_provider",
    "environment",
    "random",
    "remote_api",
    "external_artifact",
];
const SIDE_EFFECT_FIELDS: &[&str] = &[
    "files",
    "lifecycle",
    "network_writes",
    "secret_accesses",
    "deploys",
    "other_side_effects",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentRunGateIssue {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentRunGateReport {
    pub verdict: String,
    pub valid: bool,
    pub issues: Vec<AgentRunGateIssue>,
    pub receipt_hash: String,
}

// r[impl cairn.supersede.agent_run_receipts.schema]
// r[impl cairn.supersede.agent_run_receipts.schema.required_fields]
// r[impl cairn.supersede.agent_run_receipts.schema.bounded_content]
pub fn agent_run_receipt_hash(receipt: &Value) -> String {
    let mut canonical = receipt.clone();
    if let Some(object) = canonical.as_object_mut() {
        object.remove("receipt_hash");
    }
    canonical_json_hash(&canonical)
}

// r[impl cairn.supersede.agent_run_receipts.completion_gate]
pub fn evaluate_agent_run_completion(receipt: &Value) -> AgentRunGateReport {
    let issues = agent_run_receipt_issues(receipt);
    let is_valid = issues.is_empty();
    let verdict = if is_valid { "PASS" } else { "FAIL" }.to_string();
    AgentRunGateReport {
        verdict,
        valid: is_valid,
        issues,
        receipt_hash: agent_run_receipt_hash(receipt),
    }
}

pub fn agent_run_receipt_issues(receipt: &Value) -> Vec<AgentRunGateIssue> {
    let mut issues = Vec::with_capacity(8);
    let Some(object) = receipt.as_object() else {
        return vec![issue(
            "receipt.not_object",
            "agent-run receipt must be a JSON object",
        )];
    };
    issues.extend(required_field_issues(object));
    issues.extend(receipt_hash_issues(receipt, object));
    issues.extend(plan_linkage_issues(object));
    issues.extend(verification_issues(object));
    issues.extend(nondeterminism_issues(object));
    issues.extend(replay_evidence_issues(object));
    issues
}

fn required_field_issues(object: &Map<String, Value>) -> Vec<AgentRunGateIssue> {
    REQUIRED_RECEIPT_FIELDS
        .iter()
        .filter(|field| !object.contains_key(**field))
        .map(|field| {
            issue(
                "receipt.missing_field",
                format!("missing required field: {field}"),
            )
        })
        .collect()
}

fn receipt_hash_issues(receipt: &Value, object: &Map<String, Value>) -> Vec<AgentRunGateIssue> {
    let Some(stated_hash) = object.get("receipt_hash").and_then(Value::as_str) else {
        return Vec::new();
    };
    let expected_hash = agent_run_receipt_hash(receipt);
    if stated_hash == expected_hash && is_blake3_hex(stated_hash) {
        Vec::new()
    } else {
        vec![issue(
            "receipt.hash_mismatch",
            "receipt_hash must equal the BLAKE3 hash of canonical receipt content with receipt_hash excluded",
        )]
    }
}

fn plan_linkage_issues(object: &Map<String, Value>) -> Vec<AgentRunGateIssue> {
    // r[impl cairn.supersede.agent_run_receipts.plan_link]
    // r[impl cairn.supersede.agent_run_receipts.plan_link.side_effects]
    // r[impl cairn.supersede.agent_run_receipts.plan_link.read_only]
    let has_side_effects = object
        .get("mutation_manifest")
        .and_then(Value::as_object)
        .map(mutation_manifest_has_side_effects)
        .unwrap_or(false);
    let plan_hash = object
        .get("plan_hash")
        .and_then(Value::as_str)
        .unwrap_or("");
    if has_side_effects && plan_hash.is_empty() {
        return vec![issue(
            "plan.missing_hash",
            "side-effecting agent-run receipts require a non-empty plan_hash",
        )];
    }
    if has_side_effects && plan_hash == NO_MUTATION_PLAN_HASH {
        return vec![issue(
            "plan.no_mutation_plan_with_side_effects",
            "side-effecting agent-run receipts cannot cite the no-mutation plan hash",
        )];
    }
    Vec::new()
}

fn mutation_manifest_has_side_effects(manifest: &Map<String, Value>) -> bool {
    SIDE_EFFECT_FIELDS
        .iter()
        .any(|field| manifest.get(*field).map(value_has_entries).unwrap_or(false))
}

fn value_has_entries(value: &Value) -> bool {
    match value {
        Value::Array(entries) => !entries.is_empty(),
        Value::Object(entries) => !entries.is_empty(),
        Value::String(text) => !text.is_empty(),
        Value::Bool(flag) => *flag,
        Value::Number(_) => true,
        Value::Null => false,
    }
}

fn verification_issues(object: &Map<String, Value>) -> Vec<AgentRunGateIssue> {
    // r[impl cairn.supersede.agent_run_receipts.completion_gate.verified]
    // r[impl cairn.supersede.agent_run_receipts.completion_gate.missing_verification]
    let verdict = object.get("verdict").and_then(Value::as_str).unwrap_or("");
    if verdict != "success" && verdict != "PASS" {
        return Vec::new();
    }
    let checks = object
        .get("verification_manifest")
        .and_then(Value::as_object)
        .and_then(|manifest| manifest.get("checks"))
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0);
    if checks == 0 {
        return vec![issue(
            "verification.missing_evidence",
            "successful agent-run receipts require at least one verification_manifest.checks entry",
        )];
    }
    Vec::new()
}

fn nondeterminism_issues(object: &Map<String, Value>) -> Vec<AgentRunGateIssue> {
    // r[impl cairn.supersede.agent_run_receipts.nondeterminism]
    // r[impl cairn.supersede.agent_run_receipts.nondeterminism.captured]
    // r[impl cairn.supersede.agent_run_receipts.nondeterminism.hidden]
    let Some(manifest) = object
        .get("nondeterminism_manifest")
        .and_then(Value::as_object)
    else {
        return Vec::new();
    };
    let captured = manifest
        .get("captured")
        .and_then(Value::as_array)
        .map(|entries| entries.as_slice())
        .unwrap_or(&[]);
    let forbidden = manifest
        .get("forbidden_uncaptured")
        .and_then(Value::as_array)
        .map(|entries| entries.as_slice())
        .unwrap_or(&[]);
    let mut issues = Vec::with_capacity(captured.len().saturating_add(forbidden.len()));
    for entry in captured {
        if !captured_entropy_is_valid(entry) {
            issues.push(issue(
                "nondeterminism.invalid_capture",
                "captured entropy entries require a known class and bounded metadata or content_hash",
            ));
        }
    }
    for entry in forbidden {
        let class = entry
            .get("class")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        issues.push(issue(
            "nondeterminism.uncaptured",
            format!("forbidden or uncaptured entropy source: {class}"),
        ));
    }
    issues
}

fn captured_entropy_is_valid(entry: &Value) -> bool {
    let Some(class) = entry.get("class").and_then(Value::as_str) else {
        return false;
    };
    KNOWN_ENTROPY_CLASSES.contains(&class)
        && (entry.get("metadata").is_some() || entry.get("content_hash").is_some())
}

fn replay_evidence_issues(object: &Map<String, Value>) -> Vec<AgentRunGateIssue> {
    // r[impl cairn.supersede.agent_receipt_replay_evidence.schema]
    let Some(evidence) = object.get("replay_evidence").and_then(Value::as_object) else {
        return Vec::new();
    };
    let mut issues = Vec::new();
    let replay_hash = evidence
        .get("replay_receipt_hash")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if !is_blake3_hex(replay_hash) {
        issues.push(issue(
            "replay_evidence.invalid_hash",
            "replay_evidence.replay_receipt_hash must be a BLAKE3 hex hash",
        ));
    }
    let policy_hash = evidence
        .get("replay_policy_hash")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if !is_blake3_hex(policy_hash) {
        issues.push(issue(
            "replay_evidence.invalid_policy_hash",
            "replay_evidence.replay_policy_hash must be a BLAKE3 hex hash",
        ));
    }
    if evidence
        .get("replay_verdict")
        .and_then(Value::as_str)
        .unwrap_or_default()
        != "pass"
    {
        issues.push(issue(
            "replay_evidence.verdict_not_pass",
            "replay_evidence.replay_verdict must be pass",
        ));
    }
    let has_cases = evidence
        .get("replay_selected_cases")
        .and_then(Value::as_array)
        .map(|cases| !cases.is_empty() && cases.iter().all(Value::is_string))
        .unwrap_or(false);
    if !has_cases {
        issues.push(issue(
            "replay_evidence.missing_cases",
            "replay_evidence.replay_selected_cases must list replay case ids",
        ));
    }
    let has_groups = evidence
        .get("replay_selected_groups")
        .and_then(Value::as_array)
        .map(|groups| !groups.is_empty() && groups.iter().all(Value::is_string))
        .unwrap_or(false);
    if !has_groups {
        issues.push(issue(
            "replay_evidence.missing_groups",
            "replay_evidence.replay_selected_groups must list replay group ids",
        ));
    }
    issues
}

fn is_blake3_hex(candidate: &str) -> bool {
    candidate.len() == 64 && candidate.as_bytes().iter().all(u8::is_ascii_hexdigit)
}

fn issue(code: impl Into<String>, message: impl Into<String>) -> AgentRunGateIssue {
    AgentRunGateIssue {
        code: code.into(),
        message: message.into(),
    }
}

pub fn positive_agent_run_fixture() -> Value {
    // r[verify cairn.supersede.agent_run_receipts.replay]
    // r[verify cairn.supersede.agent_run_receipts.replay.positive]
    let mut receipt = json!({
        "schema_version": AGENT_RUN_RECEIPT_SCHEMA_VERSION,
        "task_id": "fixture-agent-run-success",
        "agent_identity": {"wrapper": "cairn-fixture", "model": "captured-model"},
        "input_hash": "1111111111111111111111111111111111111111111111111111111111111111",
        "plan_hash": "2222222222222222222222222222222222222222222222222222222222222222",
        "tool_manifest": {"tools": [{"name": "terminal", "mode": "read"}]},
        "mutation_manifest": {"files": ["docs/agent-run-receipts.md"]},
        "verification_manifest": {"checks": [{"name": "cargo test", "verdict": "PASS", "evidence_hash": "3333333333333333333333333333333333333333333333333333333333333333"}]},
        "nondeterminism_manifest": {"captured": [{"class": "model_provider", "metadata": {"provider": "fixture"}}], "forbidden_uncaptured": []},
        "replay_evidence": {
            "replay_receipt_hash": "4444444444444444444444444444444444444444444444444444444444444444",
            "replay_policy_hash": "c6b0062a99e0295ff8a6a3725490adc7322f5d2dd1fc703c02e090d0c6eb93e3",
            "replay_selected_cases": ["agent_receipt_validate", "mcp_describe", "status", "validate"],
            "replay_selected_groups": ["agent"],
            "replay_verdict": "pass",
            "replay_artifact_path": "/tmp/cairn-replay.json"
        },
        "verdict": "success",
        "receipt_hash": ""
    });
    let hash = agent_run_receipt_hash(&receipt);
    if let Some(object) = receipt.as_object_mut() {
        object.insert("receipt_hash".to_string(), Value::String(hash));
    }
    receipt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_agent_run_receipt_hash_is_byte_stable() {
        let first = positive_agent_run_fixture();
        let second = positive_agent_run_fixture();
        assert_eq!(first, second);
        assert_eq!(
            agent_run_receipt_hash(&first),
            agent_run_receipt_hash(&second)
        );
        let hash = first.get("receipt_hash").and_then(Value::as_str).unwrap();
        assert!(is_blake3_hex(hash));
    }

    #[test]
    fn completion_gate_passes_verified_fixture() {
        let report = evaluate_agent_run_completion(&positive_agent_run_fixture());
        assert!(report.valid);
        assert_eq!(report.verdict, "PASS");
        assert!(report.issues.is_empty());
    }

    #[test]
    fn completion_gate_fails_missing_verification() {
        // r[verify cairn.supersede.agent_run_receipts.replay.negative]
        let mut receipt = positive_agent_run_fixture();
        receipt["verification_manifest"] = json!({"checks": []});
        let hash = agent_run_receipt_hash(&receipt);
        receipt["receipt_hash"] = Value::String(hash);
        let report = evaluate_agent_run_completion(&receipt);
        assert!(!report.valid);
        assert_eq!(report.issues[0].code, "verification.missing_evidence");
    }

    #[test]
    fn completion_gate_fails_missing_side_effect_plan() {
        let mut receipt = positive_agent_run_fixture();
        receipt["plan_hash"] = Value::String(String::new());
        let hash = agent_run_receipt_hash(&receipt);
        receipt["receipt_hash"] = Value::String(hash);
        let report = evaluate_agent_run_completion(&receipt);
        assert!(!report.valid);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "plan.missing_hash"));
    }

    #[test]
    fn completion_gate_fails_uncaptured_nondeterminism() {
        let mut receipt = positive_agent_run_fixture();
        receipt["nondeterminism_manifest"] = json!({
            "captured": [],
            "forbidden_uncaptured": [{"class": "network"}]
        });
        let hash = agent_run_receipt_hash(&receipt);
        receipt["receipt_hash"] = Value::String(hash);
        let report = evaluate_agent_run_completion(&receipt);
        assert!(!report.valid);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "nondeterminism.uncaptured"));
    }

    #[test]
    fn completion_gate_fails_mutation_outside_no_mutation_plan() {
        let mut receipt = positive_agent_run_fixture();
        receipt["plan_hash"] = Value::String(NO_MUTATION_PLAN_HASH.to_string());
        let hash = agent_run_receipt_hash(&receipt);
        receipt["receipt_hash"] = Value::String(hash);
        let report = evaluate_agent_run_completion(&receipt);
        assert!(!report.valid);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "plan.no_mutation_plan_with_side_effects"));
    }
}
