use std::path::Path;

use cairn_core::{
    parse_content_bound_requirement_registry_value, parse_content_bound_trace_index_value,
    validate_content_bound_requirement_registry, validate_content_bound_trace_index,
};
use serde_json::Value;

use crate::agent_receipt::{
    agent_evidence_bundle_verification_receipt, agent_receipt_validation_receipt,
};
use crate::determinism::determinism_verify_replay_receipt;
use crate::evidence_registry::{evidence_registry_verify_receipt, EvidenceRegistryVerifyOptions};
use crate::runtime_evidence::{
    runtime_evidence_verification_receipt, RuntimeEvidenceVerifyOptions,
};
use crate::support::{default_policy_path, load_policy_file, portable_json_hash};

use super::issues::evidence_manifest_issue;

pub(super) fn validate_manifest_artifact_role(
    root: &Path,
    path: &Path,
    role: &str,
    value: &Value,
    issues: &mut Vec<Value>,
) -> bool {
    // r[impl cairn.supersede.registry_manifest_binding.roles]
    // r[impl cairn.supersede.registry_manifest_binding.verify]
    match role {
        "agent_evidence_bundle" => {
            let receipt = agent_evidence_bundle_verification_receipt(root, path);
            if receipt["valid"].as_bool() == Some(true) {
                true
            } else {
                issues.push(evidence_manifest_issue(
                    "artifact.bundle_invalid",
                    role,
                    "valid agent evidence bundle",
                    "invalid",
                ));
                false
            }
        }
        "replay_receipt" => {
            let receipt = determinism_verify_replay_receipt(root, path);
            if receipt["valid"].as_bool() == Some(true) {
                true
            } else {
                issues.push(evidence_manifest_issue(
                    "artifact.replay_invalid",
                    role,
                    "valid replay receipt",
                    "invalid",
                ));
                false
            }
        }
        "replay_verification" => receipt_valid_or_issue(
            value,
            issues,
            role,
            "artifact.replay_verification_invalid",
            "valid replay verification receipt",
        ),
        "agent_receipt" => match load_policy_file(default_policy_path().into()) {
            Ok(policy) => {
                let receipt = agent_receipt_validation_receipt(
                    "agent-receipt validate",
                    None,
                    value,
                    false,
                    &policy,
                );
                if receipt["valid"].as_bool() == Some(true) {
                    true
                } else {
                    issues.push(evidence_manifest_issue(
                        "artifact.agent_receipt_invalid",
                        role,
                        "valid agent receipt",
                        "invalid",
                    ));
                    false
                }
            }
            Err(_) => {
                issues.push(evidence_manifest_issue(
                    "artifact.policy_load_failed",
                    role,
                    "generated policy loads",
                    "failed",
                ));
                false
            }
        },
        "release_readiness" => receipt_valid_or_issue(
            value,
            issues,
            role,
            "artifact.release_readiness_invalid",
            "valid release-readiness receipt",
        ),
        "coverage_audit" => receipt_valid_or_issue(
            value,
            issues,
            role,
            "artifact.coverage_audit_invalid",
            "valid coverage-audit receipt",
        ),
        "requirement_registry" => match parse_content_bound_requirement_registry_value(value) {
            Ok(registry) if validate_content_bound_requirement_registry(&registry).is_empty() => {
                true
            }
            Ok(_) | Err(_) => {
                issues.push(evidence_manifest_issue(
                    "artifact.requirement_registry_invalid",
                    role,
                    "valid self-bound content requirement registry",
                    "invalid",
                ));
                false
            }
        },
        "trace_index" => match parse_content_bound_trace_index_value(value) {
            Ok(index) if validate_content_bound_trace_index(&index).is_empty() => true,
            Ok(_) | Err(_) => {
                issues.push(evidence_manifest_issue(
                    "artifact.trace_index_invalid",
                    role,
                    "valid self-bound content trace index",
                    "invalid",
                ));
                false
            }
        },
        "trace_index_verification" => {
            let valid = value.get("valid").and_then(Value::as_bool) == Some(true);
            let verdict = value.get("verdict").and_then(Value::as_str) == Some("pass");
            let command =
                value.get("command").and_then(Value::as_str) == Some("traceability verify");
            let schema = value.get("schema").and_then(Value::as_str)
                == Some(crate::trace_index::TRACE_INDEX_VERIFY_RECEIPT_SCHEMA_V1);
            let issues_empty = value
                .get("issues")
                .and_then(Value::as_array)
                .is_some_and(Vec::is_empty);
            let registry_issues_empty = value
                .get("registry_issues")
                .and_then(Value::as_array)
                .is_some_and(Vec::is_empty);
            let stated_hash = value.get("receipt_hash").and_then(Value::as_str);
            let mut material = value.clone();
            if let Some(object) = material.as_object_mut() {
                object.remove("receipt_hash");
            }
            let computed_hash = portable_json_hash(root, &material);
            let receipt_hash_valid = stated_hash == Some(computed_hash.as_str());
            if valid
                && verdict
                && command
                && schema
                && issues_empty
                && registry_issues_empty
                && receipt_hash_valid
            {
                true
            } else {
                issues.push(evidence_manifest_issue(
                    "artifact.trace_index_verification_invalid",
                    role,
                    "passing trace-index verification receipt",
                    "invalid",
                ));
                false
            }
        }
        "evidence_registry" => {
            let receipt = evidence_registry_verify_receipt(&EvidenceRegistryVerifyOptions {
                root: root.to_path_buf(),
                registry: Some(path.to_path_buf()),
            });
            if receipt["valid"].as_bool() == Some(true) {
                true
            } else {
                issues.push(evidence_manifest_issue(
                    "artifact.registry_invalid",
                    role,
                    "valid evidence registry",
                    "invalid",
                ));
                false
            }
        }
        "evidence_registry_verification" => receipt_valid_or_issue(
            value,
            issues,
            role,
            "artifact.registry_verification_invalid",
            "valid evidence registry verification receipt",
        ),
        "evidence_registry_selection" => receipt_valid_or_issue(
            value,
            issues,
            role,
            "artifact.registry_selection_invalid",
            "valid evidence registry selection receipt",
        ),
        "evidence_registry_fetch_plan" => receipt_valid_or_issue(
            value,
            issues,
            role,
            "artifact.registry_fetch_plan_invalid",
            "valid evidence registry fetch-plan receipt",
        ),
        "runtime_evidence" => receipt_valid_or_issue(
            value,
            issues,
            role,
            "artifact.runtime_evidence_invalid",
            "valid runtime-evidence receipt",
        ),
        "runtime_chain" => {
            let receipt = runtime_evidence_verification_receipt(&RuntimeEvidenceVerifyOptions {
                root: root.to_path_buf(),
                policy: default_policy_path().into(),
                chain: Some(path.to_path_buf()),
                checkpoint: None,
            });
            if receipt["valid"].as_bool() == Some(true) {
                true
            } else {
                issues.push(evidence_manifest_issue(
                    "artifact.runtime_chain_invalid",
                    role,
                    "valid runtime chain",
                    "invalid",
                ));
                false
            }
        }
        "witness_checkpoint" => {
            if value
                .get("checkpoint_hash")
                .and_then(Value::as_str)
                .is_some()
            {
                true
            } else {
                issues.push(evidence_manifest_issue(
                    "artifact.witness_checkpoint_invalid",
                    role,
                    "witness checkpoint with checkpoint_hash",
                    "invalid",
                ));
                false
            }
        }
        _ => {
            issues.push(evidence_manifest_issue(
                "artifact.unknown_role",
                role,
                "known evidence role",
                "unknown",
            ));
            false
        }
    }
}

fn receipt_valid_or_issue(
    value: &Value,
    issues: &mut Vec<Value>,
    role: &str,
    code: &str,
    expected: &str,
) -> bool {
    if value.get("valid").and_then(Value::as_bool) == Some(true) {
        true
    } else {
        issues.push(evidence_manifest_issue(code, role, expected, "invalid"));
        false
    }
}
