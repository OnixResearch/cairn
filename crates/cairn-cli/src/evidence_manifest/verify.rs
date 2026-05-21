use std::{
    error::Error,
    path::{Path, PathBuf},
};

use cairn_core::canonical_json_hash;
use serde_json::{json, Value};

use crate::agent_receipt::{
    read_json_artifact, receipt_stated_or_computed_hash, AGENT_RECEIPT_MAX_BYTES,
};
use crate::{path_string, portable_json_hash, print_json, read_option, with_portable_receipt_hash};

use super::issues::evidence_manifest_issue;
use super::metadata::optional_metadata_hash;
use super::roles::validate_manifest_artifact_role;

struct EvidenceManifestOptions {
    root: PathBuf,
    path: PathBuf,
}

pub(super) fn run_evidence_manifest_verify(args: Vec<String>) -> Result<(), Box<dyn Error>> {
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
