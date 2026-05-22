use std::{
    error::Error,
    path::{Path, PathBuf},
};

use cairn_core::canonical_json_hash;
use serde_json::{json, Value};

use crate::determinism::determinism_verify_replay_receipt;
use crate::support::{
    default_policy_path, load_policy_file, path_string, portable_json_hash, print_json,
    read_option, with_portable_receipt_hash,
};

use super::io::{read_agent_receipt_file, read_json_artifact};
use super::validation::agent_receipt_validation_receipt;
use super::{AGENT_RECEIPT_MAX_BYTES, REQUIRED_AGENT_REPLAY_GROUP};

struct AgentEvidenceBundleOptions {
    root: PathBuf,
    path: PathBuf,
}

pub(super) fn run_agent_receipt_verify_bundle(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = agent_evidence_bundle_options(&args)?;
    let receipt = agent_evidence_bundle_verification_receipt(&options.root, &options.path);
    let valid = receipt
        .get("valid")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    print_json(receipt)?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

fn agent_evidence_bundle_options(
    args: &[String],
) -> Result<AgentEvidenceBundleOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let path = args
        .iter()
        .find(|arg| !arg.starts_with("--") && *arg != &root)
        .ok_or("agent-receipt verify-bundle requires <bundle.json>")?;
    Ok(AgentEvidenceBundleOptions {
        root,
        path: PathBuf::from(path),
    })
}

pub(crate) fn agent_evidence_bundle_verification_receipt(root: &Path, bundle_path: &Path) -> Value {
    // r[impl cairn.supersede.agent_evidence_bundle_verification.cli]
    // r[impl cairn.supersede.agent_evidence_bundle_verification.validation]
    // r[impl cairn.supersede.agent_evidence_bundle_verification.receipt]
    // r[impl cairn.supersede.agent_evidence_bundle_verification.secret_safe]
    let policy = match load_policy_file(default_policy_path().into()) {
        Ok(policy) => policy,
        Err(_) => {
            return agent_bundle_receipt(
                root,
                vec![agent_bundle_issue(
                    "policy.load_failed",
                    "policy",
                    "generated policy loads",
                    "failed",
                )],
                String::new(),
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
            )
        }
    };
    let current_policy_hash = canonical_json_hash(&policy.raw);
    let bundle = match read_json_artifact(bundle_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            return agent_bundle_receipt(
                root,
                vec![agent_bundle_issue(
                    "bundle.read_failed",
                    "bundle",
                    "readable JSON object",
                    "unreadable",
                )],
                current_policy_hash,
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
            )
        }
    };
    let mut issues = Vec::new();
    if !bundle.is_object() {
        issues.push(agent_bundle_issue(
            "bundle.not_object",
            "bundle",
            "object",
            "non-object",
        ));
    }
    let bundle_base = bundle_path.parent().unwrap_or_else(|| Path::new("."));
    let agent_path = bundle_artifact_path(&bundle, bundle_base, "agent_receipt_path", &mut issues);
    let replay_path =
        bundle_artifact_path(&bundle, bundle_base, "replay_receipt_path", &mut issues);
    let replay_verify_path = bundle_artifact_path(
        &bundle,
        bundle_base,
        "replay_verification_path",
        &mut issues,
    );
    let release_path =
        optional_bundle_artifact_path(&bundle, bundle_base, "release_readiness_path", &mut issues);
    let coverage_audit_path =
        optional_bundle_artifact_path(&bundle, bundle_base, "coverage_audit_path", &mut issues);

    let mut agent_receipt_hash = Value::Null;
    let mut replay_receipt_hash = Value::Null;
    let mut replay_verification_receipt_hash = Value::Null;
    let mut environment_attestation_hash = Value::Null;
    let mut release_readiness_receipt_hash = Value::Null;
    let mut coverage_audit_receipt_hash = Value::Null;

    let mut agent_receipt = Value::Null;
    if let Some(path) = agent_path.as_deref() {
        match read_agent_receipt_file(path) {
            Ok(receipt) => {
                agent_receipt = receipt;
                let validation = agent_receipt_validation_receipt(
                    "agent-receipt validate",
                    None,
                    &agent_receipt,
                    false,
                    &policy,
                );
                agent_receipt_hash = validation["agent_receipt_hash"].clone();
                if validation["valid"].as_bool() != Some(true) {
                    issues.push(agent_bundle_issue(
                        "agent_receipt.invalid",
                        "agent_receipt",
                        "valid agent receipt",
                        "invalid",
                    ));
                }
            }
            Err(_) => issues.push(agent_bundle_issue(
                "agent_receipt.read_failed",
                "agent_receipt_path",
                "readable agent receipt",
                "unreadable",
            )),
        }
    }

    let mut replay_verifier = Value::Null;
    if let Some(path) = replay_path.as_deref() {
        replay_verifier = determinism_verify_replay_receipt(root, path);
        replay_receipt_hash = replay_verifier["replay_receipt_hash"].clone();
        environment_attestation_hash = replay_verifier["environment_attestation_hash"].clone();
        if replay_verifier["valid"].as_bool() != Some(true) {
            issues.push(agent_bundle_issue(
                "replay_receipt.invalid",
                "replay_receipt",
                "valid saved replay receipt",
                "invalid",
            ));
        }
    }

    let mut saved_replay_verification = Value::Null;
    if let Some(path) = replay_verify_path.as_deref() {
        match read_json_artifact(path, AGENT_RECEIPT_MAX_BYTES) {
            Ok(value) => {
                saved_replay_verification = value;
                replay_verification_receipt_hash =
                    receipt_stated_or_computed_hash(root, &saved_replay_verification);
                if saved_replay_verification["valid"].as_bool() != Some(true) {
                    issues.push(agent_bundle_issue(
                        "replay_verification.invalid",
                        "replay_verification",
                        "valid replay verification receipt",
                        "invalid",
                    ));
                }
            }
            Err(_) => issues.push(agent_bundle_issue(
                "replay_verification.read_failed",
                "replay_verification_path",
                "readable replay verification receipt",
                "unreadable",
            )),
        }
    }

    verify_optional_bundle_hash(
        &mut issues,
        &bundle,
        "agent_receipt_hash",
        &agent_receipt_hash,
    );
    verify_optional_bundle_hash(
        &mut issues,
        &bundle,
        "replay_receipt_hash",
        &replay_receipt_hash,
    );
    verify_optional_bundle_hash(
        &mut issues,
        &bundle,
        "replay_verification_receipt_hash",
        &replay_verification_receipt_hash,
    );

    if saved_replay_verification.is_object() && replay_verifier.is_object() {
        compare_bundle_field(
            &mut issues,
            "replay_verification.replay_receipt_hash_mismatch",
            "replay_verification.replay_receipt_hash",
            &replay_verifier["replay_receipt_hash"],
            &saved_replay_verification["replay_receipt_hash"],
        );
        compare_bundle_field(
            &mut issues,
            "replay_verification.policy_hash_mismatch",
            "replay_verification.replay_policy_hash",
            &replay_verifier["replay_policy_hash"],
            &saved_replay_verification["replay_policy_hash"],
        );
        compare_bundle_field(
            &mut issues,
            "replay_verification.attestation_hash_mismatch",
            "replay_verification.environment_attestation_hash",
            &replay_verifier["environment_attestation_hash"],
            &saved_replay_verification["environment_attestation_hash"],
        );
    }

    if agent_receipt.is_object() && replay_verifier.is_object() {
        let evidence = agent_receipt.get("replay_evidence").unwrap_or(&Value::Null);
        compare_bundle_field(
            &mut issues,
            "agent_replay_evidence.replay_hash_mismatch",
            "agent_receipt.replay_evidence.replay_receipt_hash",
            &replay_verifier["replay_receipt_hash"],
            evidence.get("replay_receipt_hash").unwrap_or(&Value::Null),
        );
        compare_bundle_field(
            &mut issues,
            "agent_replay_evidence.policy_hash_mismatch",
            "agent_receipt.replay_evidence.replay_policy_hash",
            &Value::String(current_policy_hash.clone()),
            evidence.get("replay_policy_hash").unwrap_or(&Value::Null),
        );
        compare_bundle_field(
            &mut issues,
            "agent_replay_evidence.attestation_hash_mismatch",
            "agent_receipt.replay_evidence.replay_environment_attestation_hash",
            &replay_verifier["environment_attestation_hash"],
            evidence
                .get("replay_environment_attestation_hash")
                .unwrap_or(&Value::Null),
        );
        let groups = evidence
            .get("replay_selected_groups")
            .and_then(Value::as_array)
            .map(|groups| groups.iter().filter_map(Value::as_str).collect::<Vec<_>>())
            .unwrap_or_default();
        if !groups.contains(&REQUIRED_AGENT_REPLAY_GROUP) {
            issues.push(agent_bundle_issue(
                "agent_replay_evidence.missing_group_coverage",
                "agent_receipt.replay_evidence.replay_selected_groups",
                REQUIRED_AGENT_REPLAY_GROUP,
                "missing",
            ));
        }
    }

    if let Some(path) = release_path.as_deref() {
        match read_json_artifact(path, AGENT_RECEIPT_MAX_BYTES) {
            Ok(value) => {
                release_readiness_receipt_hash = receipt_stated_or_computed_hash(root, &value);
                if value["valid"].as_bool() != Some(true) {
                    issues.push(agent_bundle_issue(
                        "release_readiness.invalid",
                        "release_readiness",
                        "valid release-readiness receipt",
                        "invalid",
                    ));
                }
            }
            Err(_) => issues.push(agent_bundle_issue(
                "release_readiness.read_failed",
                "release_readiness_path",
                "readable release-readiness receipt",
                "unreadable",
            )),
        }
        verify_optional_bundle_hash(
            &mut issues,
            &bundle,
            "release_readiness_receipt_hash",
            &release_readiness_receipt_hash,
        );
    }

    if let Some(path) = coverage_audit_path.as_deref() {
        match read_json_artifact(path, AGENT_RECEIPT_MAX_BYTES) {
            Ok(value) => {
                coverage_audit_receipt_hash = receipt_stated_or_computed_hash(root, &value);
                if value["valid"].as_bool() != Some(true) {
                    issues.push(agent_bundle_issue(
                        "coverage_audit.invalid",
                        "coverage_audit",
                        "valid coverage-audit receipt",
                        "invalid",
                    ));
                }
            }
            Err(_) => issues.push(agent_bundle_issue(
                "coverage_audit.read_failed",
                "coverage_audit_path",
                "readable coverage-audit receipt",
                "unreadable",
            )),
        }
        verify_optional_bundle_hash(
            &mut issues,
            &bundle,
            "coverage_audit_receipt_hash",
            &coverage_audit_receipt_hash,
        );
    }

    issues.sort_by_key(canonical_json_hash);
    agent_bundle_receipt(
        root,
        issues,
        current_policy_hash,
        agent_receipt_hash,
        replay_receipt_hash,
        replay_verification_receipt_hash,
        environment_attestation_hash,
        release_readiness_receipt_hash,
        coverage_audit_receipt_hash,
    )
}

fn bundle_artifact_path(
    bundle: &Value,
    base: &Path,
    field: &str,
    issues: &mut Vec<Value>,
) -> Option<PathBuf> {
    match bundle.get(field).and_then(Value::as_str) {
        Some(text) if !text.is_empty() => {
            let path = PathBuf::from(text);
            Some(if path.is_absolute() {
                path
            } else {
                base.join(path)
            })
        }
        _ => {
            issues.push(agent_bundle_issue(
                "bundle.missing_field",
                field,
                "non-empty string",
                "missing",
            ));
            None
        }
    }
}

fn optional_bundle_artifact_path(
    bundle: &Value,
    base: &Path,
    field: &str,
    issues: &mut Vec<Value>,
) -> Option<PathBuf> {
    match bundle.get(field) {
        None | Some(Value::Null) => None,
        Some(value) => match value.as_str() {
            Some(text) if !text.is_empty() => {
                let path = PathBuf::from(text);
                Some(if path.is_absolute() {
                    path
                } else {
                    base.join(path)
                })
            }
            _ => {
                issues.push(agent_bundle_issue(
                    "bundle.invalid_field",
                    field,
                    "non-empty string",
                    "invalid",
                ));
                None
            }
        },
    }
}

pub(crate) fn receipt_stated_or_computed_hash(root: &Path, receipt: &Value) -> Value {
    if let Some(hash) = receipt.get("receipt_hash").and_then(Value::as_str) {
        return Value::String(hash.to_string());
    }
    let mut input = receipt.clone();
    if let Some(object) = input.as_object_mut() {
        object.remove("receipt_hash");
    }
    Value::String(portable_json_hash(root, &input))
}

fn verify_optional_bundle_hash(
    issues: &mut Vec<Value>,
    bundle: &Value,
    field: &str,
    actual: &Value,
) {
    let Some(expected) = bundle.get(field) else {
        return;
    };
    if expected != actual {
        issues.push(agent_bundle_issue(
            "bundle.expected_hash_mismatch",
            field,
            expected.as_str().unwrap_or("hash"),
            actual.as_str().unwrap_or("missing"),
        ));
    }
}

fn compare_bundle_field(
    issues: &mut Vec<Value>,
    code: &str,
    field_path: &str,
    expected: &Value,
    actual: &Value,
) {
    if expected != actual {
        issues.push(agent_bundle_issue(
            code,
            field_path,
            expected.as_str().unwrap_or("value"),
            actual.as_str().unwrap_or("missing"),
        ));
    }
}

fn agent_bundle_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

#[allow(clippy::too_many_arguments)]
fn agent_bundle_receipt(
    root: &Path,
    issues: Vec<Value>,
    current_replay_policy_hash: String,
    agent_receipt_hash: Value,
    replay_receipt_hash: Value,
    replay_verification_receipt_hash: Value,
    environment_attestation_hash: Value,
    release_readiness_receipt_hash: Value,
    coverage_audit_receipt_hash: Value,
) -> Value {
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "agent-receipt verify-bundle",
            "root": path_string(root),
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "current_replay_policy_hash": current_replay_policy_hash,
            "required_replay_group": REQUIRED_AGENT_REPLAY_GROUP,
            "agent_receipt_hash": agent_receipt_hash,
            "replay_receipt_hash": replay_receipt_hash,
            "replay_verification_receipt_hash": replay_verification_receipt_hash,
            "environment_attestation_hash": environment_attestation_hash,
            "release_readiness_receipt_hash": release_readiness_receipt_hash,
            "coverage_audit_receipt_hash": coverage_audit_receipt_hash,
        }),
    )
}
