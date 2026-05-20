use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use cairn_core::canonical_json_hash;
use serde_json::{json, Value};

use crate::determinism::determinism_verify_replay_receipt;
use crate::evidence_registry::{evidence_registry_verify_receipt, EvidenceRegistryVerifyOptions};
use crate::{
    agent_evidence_bundle_verification_receipt, agent_receipt_validation_receipt,
    default_policy_path, load_policy_file, path_string, portable_json_hash, print_json,
    read_json_artifact, read_option, receipt_stated_or_computed_hash, with_portable_receipt_hash,
    AGENT_RECEIPT_MAX_BYTES,
};

pub(crate) fn run_evidence_manifest(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("evidence-manifest requires a subcommand: verify".into());
    };
    match subcommand.as_str() {
        "generate" => run_evidence_manifest_generate(iter.collect()),
        "verify" => run_evidence_manifest_verify(iter.collect()),
        unknown => Err(format!("unknown evidence-manifest subcommand: {unknown}").into()),
    }
}

#[derive(Default)]
pub(crate) struct EvidenceManifestGenerateOptions {
    pub(crate) root: PathBuf,
    pub(crate) bundle: Option<PathBuf>,
    pub(crate) agent_receipt: Option<PathBuf>,
    pub(crate) replay: Option<PathBuf>,
    pub(crate) replay_verification: Option<PathBuf>,
    pub(crate) release_readiness: Option<PathBuf>,
    pub(crate) coverage_audit: Option<PathBuf>,
    pub(crate) registry: Option<PathBuf>,
    pub(crate) registry_verification: Option<PathBuf>,
    pub(crate) registry_selection: Option<PathBuf>,
    pub(crate) registry_fetch_plan: Option<PathBuf>,
    pub(crate) signer_metadata: Option<PathBuf>,
    pub(crate) anchor_metadata: Option<PathBuf>,
    pub(crate) out: Option<String>,
}

struct EvidenceManifestOptions {
    root: PathBuf,
    path: PathBuf,
}

fn run_evidence_manifest_generate(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.evidence_manifest_generate.cli]
    // r[impl cairn.supersede.evidence_manifest_generate.canonical]
    // r[impl cairn.supersede.evidence_manifest_generate.metadata]
    // r[impl cairn.supersede.evidence_manifest_generate.failures]
    // r[impl cairn.supersede.registry_manifest_binding.generate]
    // r[impl cairn.supersede.registry_manifest_binding.failure]
    // r[impl cairn.supersede.registry_manifest_binding.policy]
    let options = evidence_manifest_generate_options(&args);
    let (manifest, issues) = evidence_manifest_generate_manifest(&options);
    let valid = issues.is_empty();
    if !valid {
        print_json(evidence_manifest_generate_receipt(
            &options.root,
            Value::Null,
            false,
            Value::Null,
            issues,
        ))?;
        std::process::exit(1);
    }
    let manifest = manifest.expect("manifest exists when generation issues are empty");
    let manifest_hash = Value::String(portable_json_hash(&options.root, &manifest));
    match options.out.as_deref() {
        Some("-") | None => print_json(manifest)?,
        Some(out) => {
            let path = Path::new(out);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(
                path,
                format!("{}\n", serde_json::to_string_pretty(&manifest)?),
            )?;
            print_json(evidence_manifest_generate_receipt(
                &options.root,
                manifest_hash,
                true,
                Value::String("written".to_string()),
                Vec::new(),
            ))?;
        }
    }
    Ok(())
}

fn evidence_manifest_generate_options(args: &[String]) -> EvidenceManifestGenerateOptions {
    let mut options = EvidenceManifestGenerateOptions {
        root: read_option(args.to_vec(), "--root", "."),
        ..EvidenceManifestGenerateOptions::default()
    };
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        let value = match arg.as_str() {
            "--bundle"
            | "--agent-receipt"
            | "--replay"
            | "--replay-verification"
            | "--release-readiness"
            | "--coverage-audit"
            | "--registry"
            | "--registry-verification"
            | "--registry-selection"
            | "--registry-fetch-plan"
            | "--signer-metadata"
            | "--anchor-metadata"
            | "--out"
            | "--root" => iter.next(),
            _ => None,
        };
        match (arg.as_str(), value) {
            ("--bundle", Some(value)) => options.bundle = Some(PathBuf::from(value)),
            ("--agent-receipt", Some(value)) => options.agent_receipt = Some(PathBuf::from(value)),
            ("--replay", Some(value)) => options.replay = Some(PathBuf::from(value)),
            ("--replay-verification", Some(value)) => {
                options.replay_verification = Some(PathBuf::from(value));
            }
            ("--release-readiness", Some(value)) => {
                options.release_readiness = Some(PathBuf::from(value));
            }
            ("--coverage-audit", Some(value)) => {
                options.coverage_audit = Some(PathBuf::from(value));
            }
            ("--registry", Some(value)) => {
                options.registry = Some(PathBuf::from(value));
            }
            ("--registry-verification", Some(value)) => {
                options.registry_verification = Some(PathBuf::from(value));
            }
            ("--registry-selection", Some(value)) => {
                options.registry_selection = Some(PathBuf::from(value));
            }
            ("--registry-fetch-plan", Some(value)) => {
                options.registry_fetch_plan = Some(PathBuf::from(value));
            }
            ("--signer-metadata", Some(value)) => {
                options.signer_metadata = Some(PathBuf::from(value));
            }
            ("--anchor-metadata", Some(value)) => {
                options.anchor_metadata = Some(PathBuf::from(value));
            }
            ("--out", Some(value)) => options.out = Some(value.to_string()),
            ("--root", Some(_)) => {}
            _ => {}
        }
    }
    options
}

pub(crate) fn evidence_manifest_generate_manifest(
    options: &EvidenceManifestGenerateOptions,
) -> (Option<Value>, Vec<Value>) {
    let mut issues = Vec::new();
    let mut artifacts = Vec::new();
    for (role, path) in [
        ("agent_evidence_bundle", options.bundle.as_deref()),
        ("agent_receipt", options.agent_receipt.as_deref()),
        ("replay_receipt", options.replay.as_deref()),
        (
            "replay_verification",
            options.replay_verification.as_deref(),
        ),
        ("release_readiness", options.release_readiness.as_deref()),
        ("coverage_audit", options.coverage_audit.as_deref()),
        ("evidence_registry", options.registry.as_deref()),
        (
            "evidence_registry_verification",
            options.registry_verification.as_deref(),
        ),
        (
            "evidence_registry_selection",
            options.registry_selection.as_deref(),
        ),
        (
            "evidence_registry_fetch_plan",
            options.registry_fetch_plan.as_deref(),
        ),
    ] {
        if let Some(path) = path {
            match read_json_artifact(path, AGENT_RECEIPT_MAX_BYTES) {
                Ok(value) => artifacts.push(json!({
                    "role": role,
                    "path": path_string(path),
                    "hash": receipt_stated_or_computed_hash(&options.root, &value),
                })),
                Err(_) => issues.push(evidence_manifest_issue(
                    "generate.artifact_read_failed",
                    role,
                    "readable JSON artifact",
                    "unreadable",
                )),
            }
        } else if ![
            "release_readiness",
            "coverage_audit",
            "evidence_registry",
            "evidence_registry_verification",
            "evidence_registry_selection",
            "evidence_registry_fetch_plan",
        ]
        .contains(&role)
        {
            issues.push(evidence_manifest_issue(
                "generate.required_artifact_missing",
                role,
                "artifact path flag",
                "missing",
            ));
        }
    }
    let signer =
        read_optional_manifest_metadata(options.signer_metadata.as_deref(), "signer", &mut issues);
    let anchor =
        read_optional_manifest_metadata(options.anchor_metadata.as_deref(), "anchor", &mut issues);
    issues.sort_by_key(canonical_json_hash);
    if !issues.is_empty() {
        return (None, issues);
    }
    let mut object = serde_json::Map::new();
    object.insert("artifacts".to_string(), Value::Array(artifacts));
    if let Some(value) = signer {
        object.insert("signer".to_string(), value);
    }
    if let Some(value) = anchor {
        object.insert("anchor".to_string(), value);
    }
    (Some(Value::Object(object)), issues)
}

fn read_optional_manifest_metadata(
    path: Option<&Path>,
    field: &str,
    issues: &mut Vec<Value>,
) -> Option<Value> {
    let path = path?;
    match read_json_artifact(path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) if value.is_object() => Some(value),
        Ok(_) => {
            issues.push(evidence_manifest_issue(
                "generate.metadata_not_object",
                field,
                "JSON object",
                "non-object",
            ));
            None
        }
        Err(_) => {
            issues.push(evidence_manifest_issue(
                "generate.metadata_read_failed",
                field,
                "readable JSON object",
                "unreadable",
            ));
            None
        }
    }
}

fn evidence_manifest_generate_receipt(
    root: &Path,
    manifest_hash: Value,
    valid: bool,
    output: Value,
    issues: Vec<Value>,
) -> Value {
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence-manifest generate",
            "root": path_string(root),
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "manifest_hash": manifest_hash,
            "output": output,
        }),
    )
}

fn run_evidence_manifest_verify(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = evidence_manifest_options(&args)?;
    let receipt = evidence_manifest_verification_receipt(&options.root, &options.path);
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

fn evidence_manifest_options(args: &[String]) -> Result<EvidenceManifestOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let path = args
        .iter()
        .find(|arg| !arg.starts_with("--") && *arg != &root)
        .ok_or("evidence-manifest verify requires <manifest.json>")?;
    Ok(EvidenceManifestOptions {
        root,
        path: PathBuf::from(path),
    })
}

pub(crate) fn evidence_manifest_verification_receipt(root: &Path, manifest_path: &Path) -> Value {
    // r[impl cairn.supersede.signed_evidence_manifest.verify]
    // r[impl cairn.supersede.signed_evidence_manifest.chain]
    // r[impl cairn.supersede.signed_evidence_manifest.metadata]
    let manifest = match read_json_artifact(manifest_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            return evidence_manifest_receipt(
                root,
                Value::Null,
                Vec::new(),
                Value::Null,
                Value::Null,
                Value::Null,
                vec![evidence_manifest_issue(
                    "manifest.read_failed",
                    "manifest",
                    "readable JSON object",
                    "unreadable",
                )],
            )
        }
    };
    let manifest_hash = Value::String(portable_json_hash(root, &manifest));
    let mut issues = Vec::new();
    if !manifest.is_object() {
        issues.push(evidence_manifest_issue(
            "manifest.not_object",
            "manifest",
            "object",
            "non-object",
        ));
    }
    let signer_metadata_hash = optional_metadata_hash(&manifest, "signer", &mut issues);
    let anchor_metadata_hash = optional_metadata_hash(&manifest, "anchor", &mut issues);
    let base = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    let mut artifacts = Vec::new();
    match manifest.get("artifacts").and_then(Value::as_array) {
        Some(entries) if !entries.is_empty() => {
            for entry in entries {
                artifacts.push(verify_evidence_manifest_artifact(
                    root,
                    base,
                    entry,
                    &mut issues,
                ));
            }
        }
        _ => issues.push(evidence_manifest_issue(
            "manifest.artifacts_missing",
            "artifacts",
            "non-empty array",
            "missing",
        )),
    }
    let coverage_audit_receipt_hash = artifacts
        .iter()
        .find(|artifact| artifact.get("role").and_then(Value::as_str) == Some("coverage_audit"))
        .and_then(|artifact| artifact.get("hash"))
        .cloned()
        .unwrap_or(Value::Null);
    let has_bundle = artifacts.iter().any(|artifact| {
        artifact.get("role").and_then(Value::as_str) == Some("agent_evidence_bundle")
    });
    if !has_bundle {
        issues.push(evidence_manifest_issue(
            "manifest.bundle_missing",
            "artifacts.role",
            "agent_evidence_bundle",
            "missing",
        ));
    }
    issues.sort_by_key(canonical_json_hash);
    evidence_manifest_receipt(
        root,
        manifest_hash,
        artifacts,
        signer_metadata_hash,
        anchor_metadata_hash,
        coverage_audit_receipt_hash,
        issues,
    )
}

fn verify_evidence_manifest_artifact(
    root: &Path,
    base: &Path,
    entry: &Value,
    issues: &mut Vec<Value>,
) -> Value {
    let role = entry
        .get("role")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let schema = entry.get("schema").cloned().unwrap_or(Value::Null);
    let mut artifact_valid = true;
    if !entry.is_object() {
        artifact_valid = false;
        issues.push(evidence_manifest_issue(
            "artifact.not_object",
            "artifacts[]",
            "object",
            "non-object",
        ));
    }
    let path = match entry.get("path").and_then(Value::as_str) {
        Some(text) if !text.is_empty() => {
            let path = PathBuf::from(text);
            Some(if path.is_absolute() {
                path
            } else {
                base.join(path)
            })
        }
        _ => {
            artifact_valid = false;
            issues.push(evidence_manifest_issue(
                "artifact.path_missing",
                role,
                "non-empty string path",
                "missing",
            ));
            None
        }
    };
    let expected_hash = entry.get("hash").and_then(Value::as_str);
    if expected_hash.is_none() {
        artifact_valid = false;
        issues.push(evidence_manifest_issue(
            "artifact.hash_missing",
            role,
            "declared artifact hash",
            "missing",
        ));
    }
    let mut actual_hash = Value::Null;
    if let Some(path) = path.as_deref() {
        match read_json_artifact(path, AGENT_RECEIPT_MAX_BYTES) {
            Ok(value) => {
                actual_hash = receipt_stated_or_computed_hash(root, &value);
                if let Some(expected) = expected_hash {
                    if actual_hash.as_str() != Some(expected) {
                        artifact_valid = false;
                        issues.push(evidence_manifest_issue(
                            "artifact.hash_mismatch",
                            role,
                            "declared hash matches artifact",
                            "mismatch",
                        ));
                    }
                }
                if !validate_manifest_artifact_role(root, path, role, &value, issues) {
                    artifact_valid = false;
                }
            }
            Err(_) => {
                artifact_valid = false;
                issues.push(evidence_manifest_issue(
                    "artifact.read_failed",
                    role,
                    "readable JSON artifact",
                    "unreadable",
                ));
            }
        }
    }
    json!({
        "role": role,
        "hash": actual_hash,
        "schema": schema,
        "valid": artifact_valid,
    })
}

fn validate_manifest_artifact_role(
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

fn optional_metadata_hash(manifest: &Value, field: &str, issues: &mut Vec<Value>) -> Value {
    match manifest.get(field) {
        None | Some(Value::Null) => Value::Null,
        Some(value) if value.is_object() => Value::String(canonical_json_hash(value)),
        Some(_) => {
            issues.push(evidence_manifest_issue(
                "manifest.metadata_invalid",
                field,
                "object",
                "invalid",
            ));
            Value::Null
        }
    }
}

fn evidence_manifest_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

fn evidence_manifest_receipt(
    root: &Path,
    manifest_hash: Value,
    artifacts: Vec<Value>,
    signer_metadata_hash: Value,
    anchor_metadata_hash: Value,
    coverage_audit_receipt_hash: Value,
    issues: Vec<Value>,
) -> Value {
    // r[impl cairn.supersede.signed_evidence_manifest.schema]
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence-manifest verify",
            "root": path_string(root),
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "manifest_hash": manifest_hash,
            "artifacts": artifacts,
            "signer_metadata_hash": signer_metadata_hash,
            "anchor_metadata_hash": anchor_metadata_hash,
            "coverage_audit_receipt_hash": coverage_audit_receipt_hash,
        }),
    )
}
