use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use cairn_core::{canonical_json_hash, evaluate_agent_run_completion, AgentRunGateIssue, Policy};
use serde_json::{json, Value};

use crate::determinism::{determinism_verify_replay_receipt, replay_groups_from_policy};
use crate::{
    default_policy_path, load_policy_file, path_string, portable_json_hash, print_json,
    read_option, with_portable_receipt_hash, with_receipt_hash,
};

pub(crate) const AGENT_RECEIPT_MAX_BYTES: u64 = 64 * 1024;
pub(crate) const REQUIRED_AGENT_REPLAY_GROUP: &str = "agent";

pub(crate) fn run_agent_receipt(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("agent-receipt requires a subcommand: validate".into());
    };
    match subcommand.as_str() {
        "validate" => {
            let Some(path) = iter.next() else {
                return Err("agent-receipt validate requires a JSON receipt path".into());
            };
            let path = PathBuf::from(path);
            let receipt = read_agent_receipt_file(&path)?;
            let policy = load_policy_file(default_policy_path().into())?;
            print_json(agent_receipt_validation_receipt(
                "agent-receipt validate",
                Some(path),
                &receipt,
                false,
                &policy,
            ))
        }
        "verify-bundle" => run_agent_receipt_verify_bundle(iter.collect()),
        "smoke-mcp" => {
            let rest = iter.collect::<Vec<_>>();
            let root = read_option(rest, "--root", ".");
            print_json(crate::mcp::mcp_agent_smoke_receipt(root)?)
        }
        unknown => Err(format!("unknown agent-receipt subcommand: {unknown}").into()),
    }
}

struct AgentEvidenceBundleOptions {
    root: PathBuf,
    path: PathBuf,
}

fn run_agent_receipt_verify_bundle(args: Vec<String>) -> Result<(), Box<dyn Error>> {
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

pub(crate) fn read_json_artifact(path: &Path, max_bytes: u64) -> Result<Value, Box<dyn Error>> {
    let metadata = fs::metadata(path)?;
    if metadata.len() > max_bytes {
        return Err("artifact exceeds bounded read limit".into());
    }
    Ok(serde_json::from_slice(&fs::read(path)?)?)
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

pub(crate) fn read_agent_receipt_file(path: &Path) -> Result<Value, Box<dyn Error>> {
    // r[impl cairn.supersede.agent_receipt_validation_surface.cli]
    let metadata = fs::metadata(path)?;
    if metadata.len() > AGENT_RECEIPT_MAX_BYTES {
        return Err(format!(
            "agent receipt exceeds bounded read limit: {} bytes > {}",
            metadata.len(),
            AGENT_RECEIPT_MAX_BYTES
        )
        .into());
    }
    let bytes = fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub(crate) fn ensure_inline_receipt_is_bounded(receipt: &Value) -> Result<(), Box<dyn Error>> {
    let bytes = serde_json::to_vec(receipt)?;
    if bytes.len() as u64 > AGENT_RECEIPT_MAX_BYTES {
        return Err(format!(
            "inline agent receipt exceeds bounded read limit: {} bytes > {}",
            bytes.len(),
            AGENT_RECEIPT_MAX_BYTES
        )
        .into());
    }
    Ok(())
}

pub(crate) fn agent_receipt_validation_receipt(
    command: &str,
    path: Option<PathBuf>,
    receipt: &Value,
    is_mcp: bool,
    policy: &Policy,
) -> Value {
    // r[impl cairn.supersede.agent_receipt_validation_surface.cli]
    // r[impl cairn.supersede.agent_receipt_validation_surface.cli.schema]
    // r[impl cairn.supersede.agent_receipt_replay_evidence.validation]
    // r[impl cairn.supersede.agent_receipt_replay_evidence.failures]
    let report = evaluate_agent_run_completion(receipt);
    // r[impl cairn.supersede.agent_receipt_replay_group_coverage.validation]
    let current_replay_policy_hash = canonical_json_hash(&policy.raw);
    let (required_replay_cases, required_group_issue) = agent_required_replay_cases(policy);
    let replay_issues = agent_receipt_replay_evidence_issues(
        receipt,
        &current_replay_policy_hash,
        &required_replay_cases,
        required_group_issue,
    );
    let mut issues = report
        .issues
        .iter()
        .map(agent_gate_issue_json)
        .collect::<Vec<_>>();
    issues.extend(replay_issues.iter().map(agent_gate_issue_json));
    issues.sort_by_key(canonical_json_hash);
    let is_valid = report.valid && replay_issues.is_empty();
    let input_kind = if is_mcp && path.is_none() {
        "inline"
    } else {
        "path"
    };
    with_receipt_hash(json!({
        "command": command,
        "path": path.map(path_string),
        "input_kind": input_kind,
        "valid": is_valid,
        "verdict": if is_valid { "PASS" } else { "FAIL" },
        "issues": issues,
        "agent_receipt_hash": report.receipt_hash,
        "input_hash": canonical_json_hash(receipt),
        "stated_agent_receipt_hash": receipt.get("receipt_hash").and_then(Value::as_str).unwrap_or_default(),
        "current_replay_policy_hash": current_replay_policy_hash,
        "required_replay_group": REQUIRED_AGENT_REPLAY_GROUP,
        "required_replay_cases": required_replay_cases,
        "replay_evidence": agent_replay_evidence_summary(receipt),
        "max_content_bytes": AGENT_RECEIPT_MAX_BYTES,
    }))
}

fn agent_required_replay_cases(policy: &Policy) -> (Vec<String>, Option<AgentRunGateIssue>) {
    match replay_groups_from_policy(policy) {
        Ok(groups) => {
            let Some(group) = groups
                .iter()
                .find(|group| group.id == REQUIRED_AGENT_REPLAY_GROUP)
            else {
                return (
                    Vec::new(),
                    Some(AgentRunGateIssue {
                        code: "replay_evidence.required_group_missing".to_string(),
                        message: format!(
                            "generated replay policy must define required agent replay group: {REQUIRED_AGENT_REPLAY_GROUP}"
                        ),
                    }),
                );
            };
            let mut cases = group.cases.clone();
            cases.sort();
            cases.dedup();
            (cases, None)
        }
        Err(error) => (
            Vec::new(),
            Some(AgentRunGateIssue {
                code: "replay_evidence.required_group_missing".to_string(),
                message: format!(
                    "generated replay policy missing usable agent replay group metadata: {error}"
                ),
            }),
        ),
    }
}

fn agent_receipt_replay_evidence_issues(
    receipt: &Value,
    current_policy_hash: &str,
    required_replay_cases: &[String],
    required_group_issue: Option<AgentRunGateIssue>,
) -> Vec<AgentRunGateIssue> {
    let Some(evidence) = receipt.get("replay_evidence").and_then(Value::as_object) else {
        return vec![AgentRunGateIssue {
            code: "replay_evidence.missing".to_string(),
            message: "agent-run receipts require replay_evidence from a passing determinism replay receipt".to_string(),
        }];
    };
    let mut issues = Vec::new();
    if let Some(issue) = required_group_issue {
        issues.push(issue);
    }
    let replay_hash = evidence
        .get("replay_receipt_hash")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if !is_blake3_hex_text(replay_hash) {
        issues.push(AgentRunGateIssue {
            code: "replay_evidence.invalid_hash".to_string(),
            message: "replay_evidence.replay_receipt_hash must be a BLAKE3 hex hash".to_string(),
        });
    }
    let replay_verdict = evidence
        .get("replay_verdict")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if replay_verdict != "pass" {
        issues.push(AgentRunGateIssue {
            code: "replay_evidence.verdict_not_pass".to_string(),
            message: "replay_evidence.replay_verdict must be pass".to_string(),
        });
    }
    let replay_policy_hash = evidence
        .get("replay_policy_hash")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if replay_policy_hash != current_policy_hash {
        issues.push(AgentRunGateIssue {
            code: "replay_evidence.policy_hash_stale".to_string(),
            message:
                "replay_evidence.replay_policy_hash must match the current generated policy hash"
                    .to_string(),
        });
    }
    let selected_groups = evidence
        .get("replay_selected_groups")
        .and_then(Value::as_array)
        .map(|groups| groups.iter().filter_map(Value::as_str).collect::<Vec<_>>())
        .unwrap_or_default();
    if !selected_groups.contains(&REQUIRED_AGENT_REPLAY_GROUP) {
        issues.push(AgentRunGateIssue {
            code: "replay_evidence.missing_group_coverage".to_string(),
            message: format!(
                "replay evidence must include required selected group: {REQUIRED_AGENT_REPLAY_GROUP}"
            ),
        });
    }
    if let Some(attestation_hash) = evidence.get("replay_environment_attestation_hash") {
        if !attestation_hash.as_str().is_some_and(is_blake3_hex_text) {
            issues.push(AgentRunGateIssue {
                code: "replay_evidence.invalid_environment_attestation_hash".to_string(),
                message:
                    "replay_evidence.replay_environment_attestation_hash must be a BLAKE3 hex hash"
                        .to_string(),
            });
        }
    }
    let selected_cases = evidence
        .get("replay_selected_cases")
        .and_then(Value::as_array)
        .map(|cases| cases.iter().filter_map(Value::as_str).collect::<Vec<_>>())
        .unwrap_or_default();
    for required in required_replay_cases {
        if !selected_cases.contains(&required.as_str()) {
            issues.push(AgentRunGateIssue {
                code: "replay_evidence.insufficient_coverage".to_string(),
                message: format!("replay evidence missing required case: {required}"),
            });
        }
    }
    issues
}

fn agent_replay_evidence_summary(receipt: &Value) -> Value {
    let Some(evidence) = receipt.get("replay_evidence") else {
        return Value::Null;
    };
    json!({
        "replay_receipt_hash": evidence.get("replay_receipt_hash").cloned().unwrap_or(Value::Null),
        "replay_policy_hash": evidence.get("replay_policy_hash").cloned().unwrap_or(Value::Null),
        "replay_selected_cases": evidence.get("replay_selected_cases").cloned().unwrap_or(Value::Null),
        "replay_selected_groups": evidence.get("replay_selected_groups").cloned().unwrap_or(Value::Null),
        "replay_verdict": evidence.get("replay_verdict").cloned().unwrap_or(Value::Null),
        "replay_environment_attestation_hash": evidence.get("replay_environment_attestation_hash").cloned().unwrap_or(Value::Null),
        "replay_artifact_path": evidence.get("replay_artifact_path").cloned().unwrap_or(Value::Null),
    })
}

pub(crate) fn is_blake3_hex_text(candidate: &str) -> bool {
    candidate.len() == 64 && candidate.as_bytes().iter().all(u8::is_ascii_hexdigit)
}

fn agent_gate_issue_json(issue: &AgentRunGateIssue) -> Value {
    json!({"code": issue.code, "message": issue.message})
}
