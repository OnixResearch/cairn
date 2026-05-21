use std::path::Path;

use serde_json::Value;

use crate::agent_receipt::{
    agent_evidence_bundle_verification_receipt, agent_receipt_validation_receipt,
};
use crate::determinism::determinism_verify_replay_receipt;
use crate::evidence_registry::{evidence_registry_verify_receipt, EvidenceRegistryVerifyOptions};
use crate::{default_policy_path, load_policy_file};

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
