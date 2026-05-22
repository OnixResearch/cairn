// r[impl release_evidence.cli_source_split.publish_module]
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use cairn_core::{blake3_hex, canonical_json_hash};
use serde_json::{json, Value};

use crate::agent_receipt::{read_json_artifact, AGENT_RECEIPT_MAX_BYTES};
use crate::evidence::{evidence_index_verify_receipt, EvidenceIndexVerifyOptions};
use crate::support::{
    path_string, print_json, read_option, read_string_option, with_portable_receipt_hash,
};

use super::common::{
    collect_invalid_receipt, git_head_state, read_file_hash, release_evidence_issue,
    write_json_file,
};

pub(super) fn run_release_evidence_publish(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = release_evidence_publish_options(&args)?;
    let receipt = release_evidence_publish_receipt(&options);
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

struct ReleaseEvidencePublishOptions {
    root: PathBuf,
    index: PathBuf,
    artifact_dir: PathBuf,
    publish_dir: PathBuf,
    channel: Option<String>,
    release: Option<String>,
    retention: Option<String>,
}

fn release_evidence_publish_options(
    args: &[String],
) -> Result<ReleaseEvidencePublishOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let Some(index) = read_string_option(args, "--index").map(PathBuf::from) else {
        return Err("release-evidence publish requires --index <file>".into());
    };
    let Some(artifact_dir) = read_string_option(args, "--artifact-dir").map(PathBuf::from) else {
        return Err("release-evidence publish requires --artifact-dir <dir>".into());
    };
    let Some(publish_dir) = read_string_option(args, "--publish-dir").map(PathBuf::from) else {
        return Err("release-evidence publish requires --publish-dir <dir>".into());
    };
    Ok(ReleaseEvidencePublishOptions {
        root,
        index,
        artifact_dir,
        publish_dir,
        channel: read_string_option(args, "--channel"),
        release: read_string_option(args, "--release"),
        retention: read_string_option(args, "--retention"),
    })
}

fn release_evidence_publish_receipt(options: &ReleaseEvidencePublishOptions) -> Value {
    // r[impl release_evidence.publish_materialization.cli_contract]
    // r[impl release_evidence.publish_materialization.cli_contract.inputs_outputs]
    // r[impl release_evidence.publish_materialization.composes_existing_validators]
    // r[impl release_evidence.publish_materialization.composes_existing_validators.source_evidence]
    // r[impl release_evidence.publish_materialization.manifest_receipt]
    // r[impl release_evidence.publish_materialization.manifest_receipt.artifact_binding]
    // r[impl release_evidence.publish_materialization.manifest_receipt.metadata]
    // r[impl release_evidence.publish_materialization.fail_closed]
    // r[impl release_evidence.publish_materialization.fail_closed.path_or_hash]
    // r[impl release_evidence.publish_materialization.operator_docs]
    // r[impl release_evidence.publish_materialization.operator_docs.non_claims]
    let mut issues = Vec::new();
    let source_index_hash = read_file_hash(&options.index, &mut issues, "index", "source_index");
    let index_receipt_path = options.artifact_dir.join("release-evidence-index.json");
    let source_index_receipt =
        read_json_artifact(&index_receipt_path, AGENT_RECEIPT_MAX_BYTES).ok();
    if source_index_receipt.is_none() {
        issues.push(release_evidence_issue(
            "release_evidence_publish.source_index_receipt_unreadable",
            "artifact_dir/release-evidence-index.json",
            "readable evidence index receipt",
            "unreadable",
        ));
    }
    let index_verify = evidence_index_verify_receipt(&EvidenceIndexVerifyOptions {
        root: options.root.clone(),
        index: Some(index_receipt_path.clone()),
    });
    collect_invalid_receipt("publish_source_index_verify", &index_verify, &mut issues);
    validate_publish_label("channel", options.channel.as_deref(), &mut issues);
    validate_publish_label("release", options.release.as_deref(), &mut issues);
    validate_publish_label("retention", options.retention.as_deref(), &mut issues);

    let artifact_entries = source_index_receipt
        .as_ref()
        .and_then(|receipt| receipt.get("artifacts"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut materialized = Vec::new();
    let publish_index_path = options.publish_dir.join("index.md");
    if issues.is_empty() {
        copy_publish_file(
            &options.index,
            &publish_index_path,
            "release_evidence_index",
            source_index_hash.as_deref(),
            &options.publish_dir,
            &mut materialized,
            &mut issues,
        );
    }
    let source_bundle_dir =
        evidence_publish_source_bundle_dir(&options.artifact_dir, source_index_receipt.as_ref());
    if issues.is_empty() {
        for artifact in &artifact_entries {
            copy_publish_artifact(
                artifact,
                &source_bundle_dir,
                &options.publish_dir,
                &mut materialized,
                &mut issues,
            );
        }
    }
    materialized.sort_by_key(canonical_json_hash);
    let labels = release_evidence_publish_labels(options);
    let payload_commit = git_head_state(&options.root, &mut Vec::new())
        .get("payload_commit")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let manifest = json!({
        "schema": "cairn.release-evidence.publish-manifest.v1",
        "layout_version": "release-evidence-publish-v1",
        "payload_commit": payload_commit,
        "source_index_hash": source_index_hash.clone().unwrap_or_default(),
        "source_index_receipt_hash": source_index_receipt
            .as_ref()
            .and_then(|receipt| receipt.get("receipt_hash"))
            .cloned()
            .unwrap_or(Value::Null),
        "labels": labels,
        "materialized": materialized,
        "non_claims": [
            "local_producer_materialization_only",
            "no_network_transport",
            "no_credential_validation",
            "no_remote_mirror_trust",
            "no_signature_validation",
            "no_tag_movement",
            "no_public_unattended_production_readiness",
        ],
    });
    let mut manifest_hash = None;
    if issues.is_empty() {
        let manifest_path = options.publish_dir.join("manifest.json");
        write_json_file(&manifest_path, &manifest, &mut issues, "manifest");
        manifest_hash = read_file_hash(&manifest_path, &mut issues, "manifest", "manifest");
    }
    let receipt = release_evidence_publish_final_receipt(
        options,
        issues,
        source_index_hash,
        source_index_receipt
            .as_ref()
            .and_then(|receipt| receipt.get("receipt_hash"))
            .cloned()
            .unwrap_or(Value::Null),
        index_verify
            .get("receipt_hash")
            .cloned()
            .unwrap_or(Value::Null),
        manifest_hash,
        materialized,
    );
    write_release_evidence_publish_receipt(options, receipt)
}

fn write_release_evidence_publish_receipt(
    options: &ReleaseEvidencePublishOptions,
    receipt: Value,
) -> Value {
    if receipt.get("valid").and_then(Value::as_bool) != Some(true) {
        return receipt;
    }
    let receipt_path = options.publish_dir.join("receipt.json");
    if let Some(parent) = receipt_path.parent() {
        if fs::create_dir_all(parent).is_err() {
            let mut issues = vec![release_evidence_issue(
                "release_evidence_publish.receipt_parent_create_failed",
                "publish_dir/receipt.json",
                "writable receipt parent",
                "failed",
            )];
            issues.sort_by_key(canonical_json_hash);
            return publish_receipt_with_replaced_issues(options, receipt, issues);
        }
    }
    let bytes = serde_json::to_vec_pretty(&receipt).unwrap_or_default();
    if fs::write(&receipt_path, bytes).is_err() {
        let mut issues = vec![release_evidence_issue(
            "release_evidence_publish.receipt_write_failed",
            "publish_dir/receipt.json",
            "writable publication receipt",
            "failed",
        )];
        issues.sort_by_key(canonical_json_hash);
        return publish_receipt_with_replaced_issues(options, receipt, issues);
    }
    receipt
}

fn publish_receipt_with_replaced_issues(
    options: &ReleaseEvidencePublishOptions,
    mut receipt: Value,
    issues: Vec<Value>,
) -> Value {
    if let Some(object) = receipt.as_object_mut() {
        object.remove("receipt_hash");
        object.insert("valid".to_string(), json!(false));
        object.insert("verdict".to_string(), json!("fail"));
        object.insert("issues".to_string(), Value::Array(issues));
    }
    with_portable_receipt_hash(&options.root, receipt)
}

fn evidence_publish_source_bundle_dir(
    artifact_dir: &Path,
    source_index_receipt: Option<&Value>,
) -> PathBuf {
    let index_path = artifact_dir.join("release-evidence-index.json");
    let parent = index_path.parent().unwrap_or(artifact_dir);
    if let Some(relative) = source_index_receipt
        .and_then(|receipt| receipt.get("bundle_relative"))
        .and_then(Value::as_str)
    {
        if safe_relative_path_text(relative) {
            return parent.join(relative);
        }
    }
    artifact_dir.join("republished")
}

pub(super) fn copy_publish_artifact(
    artifact: &Value,
    source_bundle_dir: &Path,
    publish_dir: &Path,
    materialized: &mut Vec<Value>,
    issues: &mut Vec<Value>,
) {
    let role = artifact
        .get("role")
        .and_then(Value::as_str)
        .unwrap_or("<missing>");
    let relative = artifact.get("path").and_then(Value::as_str).unwrap_or("");
    let expected_hash = artifact.get("hash").and_then(Value::as_str).unwrap_or("");
    if !safe_relative_path_text(relative) {
        issues.push(release_evidence_issue(
            "release_evidence_publish.artifact_path_unsafe",
            role,
            "safe relative artifact path",
            "unsafe",
        ));
        return;
    }
    let source = source_bundle_dir.join(relative);
    let destination = publish_dir.join("artifacts").join(relative);
    copy_publish_file(
        &source,
        &destination,
        role,
        Some(expected_hash),
        publish_dir,
        materialized,
        issues,
    );
}

fn copy_publish_file(
    source: &Path,
    destination: &Path,
    role: &str,
    expected_hash: Option<&str>,
    publish_dir: &Path,
    materialized: &mut Vec<Value>,
    issues: &mut Vec<Value>,
) {
    if source
        .symlink_metadata()
        .map(|meta| meta.file_type().is_symlink())
        .unwrap_or(false)
    {
        issues.push(release_evidence_issue(
            "release_evidence_publish.source_symlink_rejected",
            role,
            "regular source file",
            "symlink",
        ));
        return;
    }
    let Ok(bytes) = fs::read(source) else {
        issues.push(release_evidence_issue(
            "release_evidence_publish.source_unreadable",
            role,
            "readable source file",
            "unreadable",
        ));
        return;
    };
    let observed_hash = blake3_hex(&bytes);
    if let Some(expected) = expected_hash {
        if observed_hash != expected {
            issues.push(release_evidence_issue(
                "release_evidence_publish.source_hash_mismatch",
                role,
                "declared source hash",
                "mismatch",
            ));
            return;
        }
    }
    if let Some(parent) = destination.parent() {
        if fs::create_dir_all(parent).is_err() {
            issues.push(release_evidence_issue(
                "release_evidence_publish.destination_parent_create_failed",
                role,
                "writable destination parent",
                "failed",
            ));
            return;
        }
    }
    if fs::write(destination, &bytes).is_err() {
        issues.push(release_evidence_issue(
            "release_evidence_publish.destination_write_failed",
            role,
            "writable destination file",
            "failed",
        ));
        return;
    }
    let copied_hash = match fs::read(destination) {
        Ok(copied) => blake3_hex(&copied),
        Err(_) => {
            issues.push(release_evidence_issue(
                "release_evidence_publish.destination_unreadable",
                role,
                "readable copied file",
                "unreadable",
            ));
            return;
        }
    };
    if copied_hash != observed_hash {
        issues.push(release_evidence_issue(
            "release_evidence_publish.destination_hash_mismatch",
            role,
            "copied hash equals source hash",
            "mismatch",
        ));
        return;
    }
    materialized.push(json!({
        "role": role,
        "path": publish_relative_path(publish_dir, destination),
        "expected_hash": expected_hash.unwrap_or(&observed_hash),
        "observed_hash": copied_hash,
    }));
}

fn release_evidence_publish_labels(options: &ReleaseEvidencePublishOptions) -> Value {
    let mut object = serde_json::Map::new();
    if let Some(channel) = &options.channel {
        object.insert("channel".to_string(), json!(channel));
    }
    if let Some(release) = &options.release {
        object.insert("release".to_string(), json!(release));
    }
    if let Some(retention) = &options.retention {
        object.insert("retention".to_string(), json!(retention));
    }
    Value::Object(object)
}

pub(super) fn validate_publish_label(field: &str, value: Option<&str>, issues: &mut Vec<Value>) {
    if let Some(text) = value {
        if text.is_empty()
            || text.contains("..")
            || text.contains('/')
            || text.contains('\\')
            || !text
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
        {
            issues.push(release_evidence_issue(
                "release_evidence_publish.label_invalid",
                field,
                "safe policy label text",
                "invalid",
            ));
        }
    }
}

fn safe_relative_path_text(relative: &str) -> bool {
    let path = PathBuf::from(relative);
    !relative.is_empty()
        && !path.is_absolute()
        && !path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
}

fn publish_relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn release_evidence_publish_final_receipt(
    options: &ReleaseEvidencePublishOptions,
    mut issues: Vec<Value>,
    source_index_hash: Option<String>,
    source_index_receipt_hash: Value,
    source_index_verification_receipt_hash: Value,
    manifest_hash: Option<String>,
    materialized: Vec<Value>,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        &options.root,
        json!({
            "command": "release-evidence publish",
            "root": path_string(&options.root),
            "index": path_string(&options.index),
            "artifact_dir": path_string(&options.artifact_dir),
            "publish_dir": path_string(&options.publish_dir),
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "source_index_hash": source_index_hash.unwrap_or_default(),
            "source_index_receipt_hash": source_index_receipt_hash,
            "source_index_verification_receipt_hash": source_index_verification_receipt_hash,
            "manifest_hash": manifest_hash.unwrap_or_default(),
            "labels": release_evidence_publish_labels(options),
            "materialized_count": materialized.len(),
            "materialized": materialized,
            "issues": issues,
        }),
    )
}
