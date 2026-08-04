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
            "replay_receipt_hash": "496217fba7948f1b2d3450c51b482855a216ef2b6b26310a69f637107cef0154",
            "replay_policy_hash": "0d5cb1655724e88eba5e81e493cdc7b90b210233965dbb785028848b6d0d48fe",
            "replay_selected_cases": ["agent_completion_validate", "agent_receipt_validate", "mcp_describe", "status", "validate"],
            "replay_selected_groups": ["agent"],
            "replay_verdict": "pass",
            "replay_environment_attestation_hash": "8afc1ff8622ca0511b1f43b42d2c014f03cb2e7cb652edc23d11c23d29318b95",
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
