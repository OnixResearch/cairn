use serde_json::{Map, Value};

use super::common::issue;
use super::types::AgentRunGateIssue;

// r[impl cairn.supersede.agent_run_receipts.completion_gate.verified]
// r[impl cairn.supersede.agent_run_receipts.completion_gate.missing_verification]
pub(super) fn verification_issues(object: &Map<String, Value>) -> Vec<AgentRunGateIssue> {
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
