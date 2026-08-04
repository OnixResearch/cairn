use serde_json::{json, Value};

use super::constants::AGENT_RUN_RECEIPT_SCHEMA_VERSION;
use super::hash::agent_run_receipt_hash;

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
            "replay_receipt_hash": "9e9f804ad430a560972f1b3eac0b277418871d564b5afa27e6c2b128eed07d51",
            "replay_policy_hash": "aea6d3c8021fbd3db19b5e241118b60bcc0ac8400de7051344a2946f0f3ec210",
            "replay_selected_cases": ["agent_receipt_validate", "mcp_describe", "status", "validate"],
            "replay_selected_groups": ["agent"],
            "replay_verdict": "pass",
            "replay_environment_attestation_hash": "3a82ef84b1771b4ab182e40ec81a5165c98c002057f91576bdc925b6442f39a0",
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
