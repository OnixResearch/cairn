// r[impl cli_maintainability.main_compat_release_cli.focused_modules]
// r[impl cli_maintainability.main_compat_release_cli.focused_modules.visible_boundaries]
// r[impl cli_maintainability.main_compat_release_cli.focused_modules.narrow_bridges]
// r[impl cli_maintainability.main_compat_release_cli.behavior_preserved]
// r[verify cli_maintainability.main_compat_release_cli.behavior_preserved.focused_checks]
// r[verify cli_maintainability.main_compat_release_cli.behavior_preserved.repository_checks]
// r[impl release_evidence.current_head_evidence_index.focused_modules]
// r[impl release_evidence.current_head_evidence_index.focused_modules.visible_boundaries]
// r[impl release_evidence.current_head_evidence_index.focused_modules.narrow_bridges]
// r[verify release_evidence.current_head_evidence_index.behavior_preserved]
// r[verify release_evidence.current_head_evidence_index.behavior_preserved.focused_checks]
// r[verify release_evidence.current_head_evidence_index.behavior_preserved.repository_checks]
use std::{error::Error, path::PathBuf};

use cairn_core::canonical_json_hash;
use serde_json::{json, Value};

use crate::determinism::determinism_coverage_audit_receipt;
use crate::mcp::mcp_agent_smoke_receipt;
use crate::native::cairn_validation_receipt;
use crate::support::{
    default_policy_path, load_policy_file, path_string, print_json, read_option,
    with_portable_receipt_hash,
};
use crate::tracey::{repository_gate_summary, tracey_coverage_receipt};

pub(crate) fn run_release_readiness(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args.clone(), "--root", ".");
    let policy_path = read_option(args, "--policy", default_policy_path());
    print_json(release_readiness_receipt(root, policy_path))
}

pub(crate) fn release_readiness_receipt(root: PathBuf, policy_path: PathBuf) -> Value {
    // r[impl cairn.supersede.release_readiness_bundle]
    // r[impl cairn.supersede.release_readiness_bundle.schema]
    // r[impl cairn.supersede.release_readiness_bundle.local]
    // r[impl cairn.supersede.release_readiness_bundle.evidence]
    // r[impl cairn.supersede.release_readiness_bundle.failure]
    let root_text = path_string(root.clone());
    let validation = readiness_component("cairn_validate", || {
        cairn_validation_receipt(root.clone(), policy_path.clone())
    });
    let tracey = readiness_component("tracey_coverage", || tracey_coverage_receipt(root.clone()));
    let mcp_smoke =
        readiness_component("mcp_agent_smoke", || mcp_agent_smoke_receipt(root.clone()));
    let coverage_audit = readiness_component("determinism_coverage_audit", || {
        let policy = load_policy_file(policy_path.clone())?;
        Ok(determinism_coverage_audit_receipt(&root, &policy))
    });
    let repository_gate_summary = repository_gate_summary();
    let replay_preflight = json!({
        "command": "determinism replay --root <root>",
        "included": false,
        "required": true,
        "reason": "workspace replay may copy fixtures and is an explicit operator/CI preflight",
    });
    let coverage_audit_valid = coverage_audit.check["passed"].clone();
    let mut checks = vec![
        validation.check,
        tracey.check,
        mcp_smoke.check,
        coverage_audit.check,
    ];
    checks.push(json!({
        "name": "repository_gate_policy_listed",
        "passed": true,
        "source": "maintained local policy",
        "required_external_commands": repository_gate_summary["required_commands"].clone(),
        "evidence_hash": canonical_json_hash(&repository_gate_summary)
    }));
    let is_valid = checks
        .iter()
        .all(|check| check["passed"].as_bool().unwrap_or(false));
    with_portable_receipt_hash(
        &root,
        json!({
            "command": "release-readiness",
            "root": root_text,
            "valid": is_valid,
            "verdict": if is_valid { "pass" } else { "fail" },
            "checks": checks,
            "cairn_validation_hash": validation.receipt_hash,
            "tracey_coverage_hash": tracey.receipt_hash,
            "mcp_smoke_receipt_hash": mcp_smoke.receipt_hash,
            "coverage_audit_valid": coverage_audit_valid,
            "coverage_audit_receipt_hash": coverage_audit.receipt_hash,
            "repository_gate_summary": repository_gate_summary,
            "determinism_replay": replay_preflight,
            "mcp": {"read_only": true, "destructive": false}
        }),
    )
}

struct ReadinessComponent {
    check: Value,
    receipt_hash: String,
}

fn readiness_component<F>(name: &str, operation: F) -> ReadinessComponent
where
    F: FnOnce() -> Result<Value, Box<dyn Error>>,
{
    match operation() {
        Ok(receipt) => {
            let is_passed = receipt["valid"].as_bool().unwrap_or(false)
                || receipt["verdict"].as_str() == Some("pass")
                || receipt["verdict"].as_str() == Some("PASS");
            let receipt_hash = receipt["receipt_hash"]
                .as_str()
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| canonical_json_hash(&receipt));
            ReadinessComponent {
                check: json!({
                    "name": name,
                    "passed": is_passed,
                    "source": component_source(name),
                    "receipt_hash": receipt_hash,
                }),
                receipt_hash,
            }
        }
        Err(error) => ReadinessComponent {
            check: json!({
                "name": name,
                "passed": false,
                "source": component_source(name),
                "issue": error.to_string(),
            }),
            receipt_hash: String::new(),
        },
    }
}

fn component_source(name: &str) -> &'static str {
    match name {
        "cairn_validate" => "cairn validate --root <root>",
        "tracey_coverage" => "cairn tracey coverage --root <root>",
        "mcp_agent_smoke" => "cairn agent-receipt smoke-mcp --root <root>",
        "determinism_coverage_audit" => "cairn determinism coverage-audit --root <root>",
        _ => "unknown",
    }
}
