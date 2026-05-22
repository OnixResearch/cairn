// r[impl release_evidence.automated_current_head_index.cli_contract]
// r[impl release_evidence.automated_current_head_index.cli_contract.inputs_outputs]
// r[impl release_evidence.automated_current_head_index.composes_existing_rails]
// r[impl release_evidence.automated_current_head_index.composes_existing_rails.authoritative_validators]
// r[impl release_evidence.automated_current_head_index.generated_index]
// r[impl release_evidence.automated_current_head_index.generated_index.payload_head]
// r[impl release_evidence.automated_current_head_index.generated_index.required_hashes]
// r[impl release_evidence.automated_current_head_index.fail_closed]
// r[impl release_evidence.automated_current_head_index.fail_closed.dirty_or_mismatched]
// r[impl release_evidence.automated_current_head_index.operator_docs]
// r[impl release_evidence.automated_current_head_index.operator_docs.non_claims]
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use cairn_core::{blake3_hex, canonical_json_hash};
use serde_json::{json, Value};

use crate::agent_receipt::{
    read_json_artifact, receipt_stated_or_computed_hash, AGENT_RECEIPT_MAX_BYTES,
};
use crate::evidence::{
    evidence_export_receipt, evidence_import_receipt, evidence_index_receipt,
    evidence_index_verify_receipt, evidence_republish_receipt, evidence_verify_receipt,
    write_evidence_json, EvidenceExportOptions, EvidenceImportOptions, EvidenceIndexOptions,
    EvidenceIndexVerifyOptions, EvidenceRepublishOptions, EvidenceVerifyOptions,
};
use crate::support::{
    path_string, print_json, read_option, read_string_option, with_portable_receipt_hash,
};

const RETENTION_PUBLICATION_SCAN_MAX_DEPTH: usize = 8;

pub(crate) fn run_release_evidence(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err(
            "release-evidence requires a subcommand: index|publish|retention-plan|rotate".into(),
        );
    };
    match subcommand.as_str() {
        "index" => run_release_evidence_index(iter.collect()),
        "publish" => run_release_evidence_publish(iter.collect()),
        "retention-plan" => run_release_evidence_retention_plan(iter.collect()),
        "rotate" => run_release_evidence_rotate(iter.collect()),
        unknown => Err(format!("unknown release-evidence subcommand: {unknown}").into()),
    }
}

fn run_release_evidence_index(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = release_evidence_index_options(&args)?;
    let receipt = release_evidence_index_receipt(&options);
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

struct ReleaseEvidenceIndexOptions {
    root: PathBuf,
    out: PathBuf,
    artifact_dir: PathBuf,
}

fn release_evidence_index_options(
    args: &[String],
) -> Result<ReleaseEvidenceIndexOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let Some(out) = read_string_option(args, "--out").map(PathBuf::from) else {
        return Err("release-evidence index requires --out <file>".into());
    };
    let artifact_dir = read_string_option(args, "--artifact-dir")
        .or_else(|| read_string_option(args, "--work-dir"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/release-evidence-index"));
    Ok(ReleaseEvidenceIndexOptions {
        root,
        out,
        artifact_dir,
    })
}

fn release_evidence_index_receipt(options: &ReleaseEvidenceIndexOptions) -> Value {
    let root = options.root.clone();
    let mut issues = Vec::new();
    let git_state = git_head_state(&root, &mut issues);
    if !issues.is_empty() {
        return release_evidence_index_final_receipt(
            &root,
            &options.out,
            &options.artifact_dir,
            issues,
            git_state,
            Vec::new(),
            Vec::new(),
        );
    }

    let export_dir = options.artifact_dir.join("exported");
    let imported_dir = options.artifact_dir.join("imported");
    let republished_dir = options.artifact_dir.join("republished");
    let index_path = options.artifact_dir.join("release-evidence-index.json");
    let verify_path = options.artifact_dir.join("evidence-verify-receipt.json");
    let index_verify_path = options
        .artifact_dir
        .join("evidence-index-verification-receipt.json");

    let export = evidence_export_receipt(&EvidenceExportOptions {
        root: root.clone(),
        out: Some(export_dir.clone()),
        agent_receipt: None,
    });
    collect_invalid_receipt("evidence_export", &export, &mut issues);

    let verify = evidence_verify_receipt(&EvidenceVerifyOptions {
        root: root.clone(),
        bundle: Some(export_dir.clone()),
    });
    write_json_artifact(&verify_path, &verify, &mut issues, "evidence_verify");
    collect_invalid_receipt("evidence_verify", &verify, &mut issues);

    let import = evidence_import_receipt(&EvidenceImportOptions {
        root: root.clone(),
        bundle: Some(export_dir.clone()),
        into: Some(imported_dir.clone()),
    });
    collect_invalid_receipt("evidence_import", &import, &mut issues);

    let republish = evidence_republish_receipt(&EvidenceRepublishOptions {
        root: root.clone(),
        imported: Some(imported_dir.clone()),
        out: Some(republished_dir.clone()),
    });
    collect_invalid_receipt("evidence_republish", &republish, &mut issues);

    let index = evidence_index_receipt(&EvidenceIndexOptions {
        root: root.clone(),
        bundle: Some(republished_dir.clone()),
        out: Some(index_path.clone()),
    });
    collect_invalid_receipt("evidence_index", &index, &mut issues);

    let index_verify = evidence_index_verify_receipt(&EvidenceIndexVerifyOptions {
        root: root.clone(),
        index: Some(index_path.clone()),
    });
    write_json_artifact(
        &index_verify_path,
        &index_verify,
        &mut issues,
        "evidence_index_verify",
    );
    collect_invalid_receipt("evidence_index_verify", &index_verify, &mut issues);

    let receipt_table = release_evidence_receipt_table(
        &root,
        &export_dir,
        &[
            ("evidence_export", &export),
            ("evidence_verify", &verify),
            ("evidence_import", &import),
            ("evidence_republish", &republish),
            ("evidence_index", &index),
            ("evidence_index_verify", &index_verify),
        ],
    );
    let artifacts = release_evidence_artifact_table(&export);
    let markdown = render_release_evidence_index(&git_state, &receipt_table, &artifacts);
    if issues.is_empty() {
        write_markdown_artifact(&options.out, &markdown, &mut issues);
    }

    release_evidence_index_final_receipt(
        &root,
        &options.out,
        &options.artifact_dir,
        issues,
        git_state,
        receipt_table,
        artifacts,
    )
}

fn git_head_state(root: &Path, issues: &mut Vec<Value>) -> Value {
    let head = git_output(root, &["rev-parse", "--short", "HEAD"]);
    let branch = git_output(root, &["branch", "--show-current"]);
    let status = git_output(root, &["status", "--porcelain"]);
    let upstream = git_output(root, &["rev-parse", "--abbrev-ref", "@{u}"]).unwrap_or_default();
    let ahead_behind = if upstream.is_empty() {
        String::new()
    } else {
        git_output(
            root,
            &["rev-list", "--left-right", "--count", "@{u}...HEAD"],
        )
        .unwrap_or_default()
    };

    if head.is_none() {
        issues.push(release_evidence_issue(
            "release_evidence.git_head_unavailable",
            "git.head",
            "readable git HEAD",
            "unavailable",
        ));
    }
    if let Some(status_text) = &status {
        if !status_text.is_empty() {
            issues.push(release_evidence_issue(
                "release_evidence.worktree_dirty",
                "git.status",
                "clean worktree",
                "dirty",
            ));
        }
    } else {
        issues.push(release_evidence_issue(
            "release_evidence.git_status_unavailable",
            "git.status",
            "readable git status",
            "unavailable",
        ));
    }

    json!({
        "payload_commit": head.unwrap_or_default(),
        "branch": branch.unwrap_or_default(),
        "upstream": upstream,
        "ahead_behind": ahead_behind,
        "clean": status.as_deref() == Some(""),
    })
}

fn git_output(root: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn collect_invalid_receipt(role: &str, receipt: &Value, issues: &mut Vec<Value>) {
    if receipt.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(release_evidence_issue(
            &format!("release_evidence.{role}_invalid"),
            role,
            "valid existing rail receipt",
            "invalid",
        ));
    }
}

fn write_json_artifact(path: &Path, value: &Value, issues: &mut Vec<Value>, role: &str) {
    write_evidence_json(path, value, issues, role);
}

fn write_markdown_artifact(path: &Path, markdown: &str, issues: &mut Vec<Value>) {
    if let Some(parent) = path.parent() {
        if fs::create_dir_all(parent).is_err() {
            issues.push(release_evidence_issue(
                "release_evidence.index_parent_create_failed",
                "out",
                "writable output parent",
                "failed",
            ));
            return;
        }
    }
    if fs::write(path, markdown).is_err() {
        issues.push(release_evidence_issue(
            "release_evidence.index_write_failed",
            "out",
            "writable Markdown index",
            "failed",
        ));
    }
}

fn release_evidence_receipt_table(
    root: &Path,
    export_dir: &Path,
    receipts: &[(&str, &Value)],
) -> Vec<Value> {
    let mut table = Vec::with_capacity(receipts.len() + 1);
    if let Ok(release) = read_json_artifact(
        &export_dir.join("release-readiness.json"),
        AGENT_RECEIPT_MAX_BYTES,
    ) {
        table.push(receipt_row(root, "release_readiness", &release));
    }
    for (role, receipt) in receipts {
        table.push(receipt_row(root, role, receipt));
    }
    table.sort_by_key(canonical_json_hash);
    table
}

fn receipt_row(root: &Path, role: &str, receipt: &Value) -> Value {
    json!({
        "role": role,
        "command": receipt.get("command").and_then(Value::as_str).unwrap_or("<missing>"),
        "verdict": receipt.get("verdict").and_then(Value::as_str).unwrap_or("<missing>"),
        "receipt_hash": receipt_stated_or_computed_hash(root, receipt),
    })
}

fn release_evidence_artifact_table(export_receipt: &Value) -> Vec<Value> {
    let mut artifacts = export_receipt
        .get("artifacts")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    artifacts.sort_by_key(canonical_json_hash);
    artifacts
}

fn render_release_evidence_index(git: &Value, receipts: &[Value], artifacts: &[Value]) -> String {
    let mut out = String::new();
    out.push_str("# Current-HEAD Release Evidence Index\n\n");
    out.push_str("## Payload\n\n");
    out.push_str(&format!(
        "- Payload commit: `{}`\n- Branch: `{}`\n- Worktree clean: `{}`\n- Upstream: `{}`\n- Ahead/behind: `{}`\n\n",
        git["payload_commit"].as_str().unwrap_or(""),
        git["branch"].as_str().unwrap_or(""),
        git["clean"].as_bool().unwrap_or(false),
        git["upstream"].as_str().unwrap_or(""),
        git["ahead_behind"].as_str().unwrap_or(""),
    ));
    out.push_str("## Commands\n\n");
    for command in [
        "release-readiness --root <root>",
        "evidence export --root <root> --out <artifact-dir>/exported",
        "evidence verify --root <root> --bundle <artifact-dir>/exported",
        "evidence import --root <root> --bundle <artifact-dir>/exported --into <artifact-dir>/imported",
        "evidence republish --root <root> --imported <artifact-dir>/imported --out <artifact-dir>/republished",
        "evidence index --root <root> --bundle <artifact-dir>/republished --out <artifact-dir>/release-evidence-index.json",
        "evidence index verify --root <root> --index <artifact-dir>/release-evidence-index.json",
    ] {
        out.push_str(&format!("- `{command}`\n"));
    }
    out.push_str("\n## Receipt Hashes\n\n| Role | Verdict | Receipt hash |\n| --- | --- | --- |\n");
    for receipt in receipts {
        out.push_str(&format!(
            "| `{}` | `{}` | `{}` |\n",
            receipt["role"].as_str().unwrap_or(""),
            receipt["verdict"].as_str().unwrap_or(""),
            receipt["receipt_hash"].as_str().unwrap_or(""),
        ));
    }
    out.push_str("\n## Bundle Artifacts\n\n| Role | Path | BLAKE3 |\n| --- | --- | --- |\n");
    for artifact in artifacts {
        out.push_str(&format!(
            "| `{}` | `{}` | `{}` |\n",
            artifact["role"].as_str().unwrap_or(""),
            artifact["path"].as_str().unwrap_or(""),
            artifact["hash"].as_str().unwrap_or(""),
        ));
    }
    out.push_str("\n## Non-claims\n\n");
    out.push_str("- Local trusted evidence only; no remote mirror trust claim.\n");
    out.push_str(
        "- No credential, transport, or public unattended production-readiness validation.\n",
    );
    out.push_str("- No tag creation or tag movement.\n");
    out.push_str("- The payload commit is the evaluated clean HEAD, not a later documentation commit carrying this index.\n");
    out
}

fn release_evidence_index_final_receipt(
    root: &Path,
    out: &Path,
    artifact_dir: &Path,
    mut issues: Vec<Value>,
    git_state: Value,
    receipt_hashes: Vec<Value>,
    artifacts: Vec<Value>,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "release-evidence index",
            "root": path_string(root),
            "out": path_string(out),
            "artifact_dir": path_string(artifact_dir),
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "git": git_state,
            "receipt_hashes": receipt_hashes,
            "artifacts": artifacts,
            "issues": issues,
        }),
    )
}

fn release_evidence_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

fn run_release_evidence_publish(args: Vec<String>) -> Result<(), Box<dyn Error>> {
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

fn copy_publish_artifact(
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

fn read_file_hash(path: &Path, issues: &mut Vec<Value>, role: &str, field: &str) -> Option<String> {
    match fs::read(path) {
        Ok(bytes) => Some(blake3_hex(&bytes)),
        Err(_) => {
            issues.push(release_evidence_issue(
                &format!("release_evidence_publish.{field}_unreadable"),
                role,
                "readable file",
                "unreadable",
            ));
            None
        }
    }
}

fn write_json_file(path: &Path, value: &Value, issues: &mut Vec<Value>, role: &str) {
    if let Some(parent) = path.parent() {
        if fs::create_dir_all(parent).is_err() {
            issues.push(release_evidence_issue(
                "release_evidence_publish.json_parent_create_failed",
                role,
                "writable JSON parent",
                "failed",
            ));
            return;
        }
    }
    let Ok(bytes) = serde_json::to_vec_pretty(value) else {
        issues.push(release_evidence_issue(
            "release_evidence_publish.json_serialize_failed",
            role,
            "serializable JSON",
            "failed",
        ));
        return;
    };
    if fs::write(path, bytes).is_err() {
        issues.push(release_evidence_issue(
            "release_evidence_publish.json_write_failed",
            role,
            "writable JSON file",
            "failed",
        ));
    }
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

fn validate_publish_label(field: &str, value: Option<&str>, issues: &mut Vec<Value>) {
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

fn run_release_evidence_retention_plan(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = release_evidence_retention_options(&args)?;
    let receipt = release_evidence_retention_plan_receipt(&options);
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

struct ReleaseEvidenceRetentionOptions {
    root: PathBuf,
    publish_root: PathBuf,
    out: PathBuf,
    channel: Option<String>,
    keep_latest: usize,
    retain: Option<String>,
}

fn release_evidence_retention_options(
    args: &[String],
) -> Result<ReleaseEvidenceRetentionOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let Some(publish_root) = read_string_option(args, "--publish-root").map(PathBuf::from) else {
        return Err("release-evidence retention-plan requires --publish-root <dir>".into());
    };
    let Some(out) = read_string_option(args, "--out").map(PathBuf::from) else {
        return Err("release-evidence retention-plan requires --out <file>".into());
    };
    let keep_latest = read_string_option(args, "--keep-latest")
        .map(|value| value.parse::<usize>())
        .transpose()
        .map_err(|_| "--keep-latest must be a non-negative integer")?
        .unwrap_or(1);
    Ok(ReleaseEvidenceRetentionOptions {
        root,
        publish_root,
        out,
        channel: read_string_option(args, "--channel"),
        keep_latest,
        retain: read_string_option(args, "--retain"),
    })
}

fn release_evidence_retention_plan_receipt(options: &ReleaseEvidenceRetentionOptions) -> Value {
    // r[impl release_evidence.retention_rotation.cli_contract]
    // r[impl release_evidence.retention_rotation.cli_contract.inputs_outputs]
    // r[impl release_evidence.retention_rotation.composes_publication_validators]
    // r[impl release_evidence.retention_rotation.composes_publication_validators.manifest_receipt]
    // r[impl release_evidence.retention_rotation.plan_receipt]
    // r[impl release_evidence.retention_rotation.plan_receipt.decisions]
    // r[impl release_evidence.retention_rotation.plan_receipt.ordering]
    // r[impl release_evidence.retention_rotation.fail_closed]
    // r[impl release_evidence.retention_rotation.fail_closed.unsafe_or_stale]
    // r[impl release_evidence.retention_rotation.operator_docs]
    // r[impl release_evidence.retention_rotation.operator_docs.non_claims]
    let mut issues = Vec::new();
    validate_publish_label("channel", options.channel.as_deref(), &mut issues);
    validate_publish_label("retain", options.retain.as_deref(), &mut issues);
    let mut candidates = retention_candidates(options, &mut issues);
    let mut seen_current = std::collections::BTreeSet::new();
    for candidate in &mut candidates {
        if candidate.get("valid").and_then(Value::as_bool) != Some(true) {
            candidate["action"] = json!("invalid");
            continue;
        }
        let release = candidate["labels"]
            .get("release")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let channel = candidate["labels"]
            .get("channel")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let retention = candidate["labels"]
            .get("retention")
            .and_then(Value::as_str)
            .unwrap_or("");
        if options
            .channel
            .as_deref()
            .is_some_and(|wanted| wanted != channel)
        {
            candidate["action"] = json!("keep");
            candidate["reason"] = json!("outside_selected_channel");
        } else if options
            .retain
            .as_deref()
            .is_some_and(|wanted| wanted == retention)
        {
            candidate["action"] = json!("keep");
            candidate["reason"] = json!("retention_label_matched");
        } else if !seen_current.insert((channel.clone(), release.clone())) {
            candidate["action"] = json!("invalid");
            candidate["reason"] = json!("duplicate_channel_release");
            issues.push(release_evidence_issue(
                "release_evidence_retention.duplicate_channel_release",
                "labels.channel_release",
                "unique channel/release identity",
                "duplicate",
            ));
        }
    }
    let mut sortable: Vec<(String, usize)> = candidates
        .iter()
        .enumerate()
        .map(|(index, candidate)| {
            let channel = candidate["labels"]
                .get("channel")
                .and_then(Value::as_str)
                .unwrap_or("");
            let release = candidate["labels"]
                .get("release")
                .and_then(Value::as_str)
                .unwrap_or("");
            (
                format!(
                    "{channel}\u{0}{release}\u{0}{}",
                    candidate["path"].as_str().unwrap_or("")
                ),
                index,
            )
        })
        .collect();
    sortable.sort_by(|left, right| right.0.cmp(&left.0));
    let mut kept_by_channel = std::collections::BTreeMap::<String, usize>::new();
    for (_, index) in sortable {
        if candidates[index]
            .get("action")
            .and_then(Value::as_str)
            .is_some()
        {
            continue;
        }
        let channel = candidates[index]["labels"]
            .get("channel")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let kept = kept_by_channel.entry(channel).or_default();
        if *kept < options.keep_latest {
            candidates[index]["action"] = json!("keep");
            candidates[index]["reason"] = json!("within_keep_latest");
            *kept += 1;
        } else {
            candidates[index]["action"] = json!("prune_candidate");
            candidates[index]["reason"] = json!("superseded_by_newer_release");
        }
    }
    candidates.sort_by_key(canonical_json_hash);
    let plan = json!({
        "schema": "cairn.release-evidence.retention-plan.v1",
        "publish_root": path_string(&options.publish_root),
        "selectors": {
            "channel": options.channel,
            "keep_latest": options.keep_latest,
            "retain": options.retain,
        },
        "decisions": candidates,
        "non_claims": [
            "local_retention_plan_only",
            "no_remote_deletion",
            "no_credential_validation",
            "no_remote_mirror_trust",
            "no_signature_validation",
            "no_tag_movement",
            "no_public_release_page_mutation",
        ],
    });
    let mut plan_hash = None;
    if issues.is_empty() {
        write_json_file(&options.out, &plan, &mut issues, "retention_plan");
        plan_hash = read_file_hash(
            &options.out,
            &mut issues,
            "retention_plan",
            "retention_plan",
        );
    }
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        &options.root,
        json!({
            "command": "release-evidence retention-plan",
            "root": path_string(&options.root),
            "publish_root": path_string(&options.publish_root),
            "out": path_string(&options.out),
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "plan_hash": plan_hash.unwrap_or_default(),
            "decision_count": plan["decisions"].as_array().map(Vec::len).unwrap_or(0),
            "decisions": plan["decisions"].clone(),
            "issues": issues,
        }),
    )
}

fn retention_candidates(
    options: &ReleaseEvidenceRetentionOptions,
    issues: &mut Vec<Value>,
) -> Vec<Value> {
    if fs::read_dir(&options.publish_root).is_err() {
        issues.push(release_evidence_issue(
            "release_evidence_retention.publish_root_unreadable",
            "publish_root",
            "readable publication root",
            "unreadable",
        ));
        return Vec::new();
    };
    let mut candidates = Vec::new();
    collect_retention_candidates(
        &options.publish_root,
        &options.publish_root,
        0,
        &mut candidates,
        issues,
    );
    candidates
}

fn collect_retention_candidates(
    publish_root: &Path,
    current: &Path,
    depth: usize,
    candidates: &mut Vec<Value>,
    issues: &mut Vec<Value>,
) {
    if depth > RETENTION_PUBLICATION_SCAN_MAX_DEPTH {
        issues.push(release_evidence_issue(
            "release_evidence_retention.scan_depth_exceeded",
            "publish_root",
            "bounded publication tree depth",
            "too_deep",
        ));
        return;
    }
    let Ok(entries) = fs::read_dir(current) else {
        issues.push(release_evidence_issue(
            "release_evidence_retention.directory_unreadable",
            &publish_relative_path(publish_root, current),
            "readable publication directory",
            "unreadable",
        ));
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let relative = publish_relative_path(publish_root, &path);
        if !safe_relative_path_text(&relative) {
            issues.push(release_evidence_issue(
                "release_evidence_retention.path_unsafe",
                "candidate.path",
                "safe relative candidate path",
                "unsafe",
            ));
            continue;
        }
        let Ok(meta) = fs::symlink_metadata(&path) else {
            continue;
        };
        if meta.file_type().is_symlink() {
            issues.push(release_evidence_issue(
                "release_evidence_retention.symlink_rejected",
                &relative,
                "real publication directory",
                "symlink",
            ));
            continue;
        }
        if !meta.is_dir() {
            continue;
        }
        let has_manifest = path.join("manifest.json").exists();
        let has_receipt = path.join("receipt.json").exists();
        if has_manifest || has_receipt {
            candidates.push(read_retention_candidate(&path, &relative, issues));
        } else {
            collect_retention_candidates(publish_root, &path, depth + 1, candidates, issues);
        }
    }
}

fn read_retention_candidate(path: &Path, relative: &str, issues: &mut Vec<Value>) -> Value {
    let manifest_path = path.join("manifest.json");
    let receipt_path = path.join("receipt.json");
    let manifest_hash =
        read_file_hash(&manifest_path, issues, relative, "retention_manifest").unwrap_or_default();
    let receipt_hash =
        read_file_hash(&receipt_path, issues, relative, "retention_receipt").unwrap_or_default();
    let manifest = read_json_artifact(&manifest_path, AGENT_RECEIPT_MAX_BYTES).ok();
    let receipt = read_json_artifact(&receipt_path, AGENT_RECEIPT_MAX_BYTES).ok();
    let mut valid = true;
    if manifest.is_none() || receipt.is_none() {
        valid = false;
    }
    let stated_manifest_hash = receipt
        .as_ref()
        .and_then(|receipt| receipt.get("manifest_hash"))
        .and_then(Value::as_str)
        .unwrap_or("");
    if !stated_manifest_hash.is_empty() && stated_manifest_hash != manifest_hash {
        valid = false;
        issues.push(release_evidence_issue(
            "release_evidence_retention.manifest_hash_mismatch",
            relative,
            "receipt manifest hash matches manifest.json",
            "mismatch",
        ));
    }
    if receipt
        .as_ref()
        .and_then(|receipt| receipt.get("valid"))
        .and_then(Value::as_bool)
        != Some(true)
    {
        valid = false;
        issues.push(release_evidence_issue(
            "release_evidence_retention.publication_receipt_invalid",
            relative,
            "valid publication receipt",
            "invalid",
        ));
    }
    let labels = manifest
        .as_ref()
        .and_then(|manifest| manifest.get("labels"))
        .cloned()
        .unwrap_or_else(|| json!({}));
    for field in ["channel", "release", "retention"] {
        validate_publish_label(field, labels.get(field).and_then(Value::as_str), issues);
    }
    json!({
        "path": relative,
        "manifest_hash": manifest_hash,
        "receipt_hash": receipt_hash,
        "labels": labels,
        "valid": valid,
    })
}

fn run_release_evidence_rotate(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = release_evidence_rotate_options(&args)?;
    let receipt = release_evidence_rotate_receipt(&options);
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

struct ReleaseEvidenceRotateOptions {
    root: PathBuf,
    plan: PathBuf,
    execute: bool,
}

fn release_evidence_rotate_options(
    args: &[String],
) -> Result<ReleaseEvidenceRotateOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let Some(plan) = read_string_option(args, "--plan").map(PathBuf::from) else {
        return Err("release-evidence rotate requires --plan <file>".into());
    };
    Ok(ReleaseEvidenceRotateOptions {
        root,
        plan,
        execute: args.iter().any(|arg| arg == "--execute"),
    })
}

fn release_evidence_rotate_receipt(options: &ReleaseEvidenceRotateOptions) -> Value {
    // r[impl release_evidence.retention_rotation.rotation_execution]
    // r[impl release_evidence.retention_rotation.rotation_execution.verify_before_mutation]
    // r[impl release_evidence.retention_rotation.fail_closed]
    let mut issues = Vec::new();
    let plan_hash =
        read_file_hash(&options.plan, &mut issues, "plan", "rotation_plan").unwrap_or_default();
    let plan = read_json_artifact(&options.plan, AGENT_RECEIPT_MAX_BYTES).ok();
    if plan.is_none() {
        issues.push(release_evidence_issue(
            "release_evidence_rotation.plan_unreadable",
            "plan",
            "readable retention plan JSON",
            "unreadable",
        ));
    }
    let publish_root = plan
        .as_ref()
        .and_then(|plan| plan.get("publish_root"))
        .and_then(Value::as_str)
        .map(PathBuf::from)
        .unwrap_or_default();
    let mut mutations = Vec::new();
    if let Some(plan) = &plan {
        for decision in plan
            .get("decisions")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            if decision.get("action").and_then(Value::as_str) != Some("prune_candidate") {
                continue;
            }
            let relative = decision.get("path").and_then(Value::as_str).unwrap_or("");
            if !safe_relative_path_text(relative) {
                issues.push(release_evidence_issue(
                    "release_evidence_rotation.path_unsafe",
                    "decision.path",
                    "safe relative prune path",
                    "unsafe",
                ));
                continue;
            }
            let target = publish_root.join(relative);
            let before_hash = directory_manifest_hash(&target).unwrap_or_default();
            if options.execute && issues.is_empty() && fs::remove_dir_all(&target).is_err() {
                issues.push(release_evidence_issue(
                    "release_evidence_rotation.remove_failed",
                    relative,
                    "removable local publication tree",
                    "failed",
                ));
            }
            let after_hash = directory_manifest_hash(&target).unwrap_or_default();
            mutations.push(json!({
                "path": relative,
                "action": if options.execute { "removed" } else { "would_remove" },
                "before_hash": before_hash,
                "after_hash": after_hash,
            }));
        }
    }
    mutations.sort_by_key(canonical_json_hash);
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        &options.root,
        json!({
            "command": "release-evidence rotate",
            "root": path_string(&options.root),
            "plan": path_string(&options.plan),
            "execute": options.execute,
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "plan_hash": plan_hash,
            "mutation_count": mutations.len(),
            "mutation_manifest": mutations,
            "issues": issues,
        }),
    )
}

fn directory_manifest_hash(path: &Path) -> Option<String> {
    if !path.exists() {
        return None;
    }
    let mut entries = Vec::new();
    collect_directory_hashes(path, path, &mut entries).ok()?;
    Some(canonical_json_hash(&json!(entries)))
}

fn collect_directory_hashes(
    root: &Path,
    path: &Path,
    entries: &mut Vec<Value>,
) -> std::io::Result<()> {
    let mut children = fs::read_dir(path)?.collect::<Result<Vec<_>, _>>()?;
    children.sort_by_key(|entry| entry.path());
    for child in children {
        let child_path = child.path();
        let meta = fs::symlink_metadata(&child_path)?;
        if meta.file_type().is_symlink() {
            continue;
        }
        if meta.is_dir() {
            collect_directory_hashes(root, &child_path, entries)?;
        } else if meta.is_file() {
            let relative = publish_relative_path(root, &child_path);
            let bytes = fs::read(&child_path)?;
            entries.push(json!({"path": relative, "hash": blake3_hex(&bytes)}));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env, fs, process::Command};

    use serde_json::json;

    use super::*;

    #[test]
    fn rendered_index_binds_payload_head_without_final_commit_claim() {
        // r[verify release_evidence.automated_current_head_index.generated_index]
        // r[verify release_evidence.automated_current_head_index.generated_index.payload_head]
        // r[verify release_evidence.automated_current_head_index.generated_index.required_hashes]
        let markdown = render_release_evidence_index(
            &json!({
                "payload_commit": "abc1234",
                "branch": "main",
                "upstream": "origin/main",
                "ahead_behind": "0\t0",
                "clean": true,
            }),
            &[json!({
                "role": "release_readiness",
                "verdict": "pass",
                "receipt_hash": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            })],
            &[json!({
                "role": "release_readiness",
                "path": "release-readiness.json",
                "hash": "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
            })],
        );

        assert!(markdown.contains("Payload commit: `abc1234`"));
        assert!(markdown.contains("release-readiness --root <root>"));
        assert!(markdown.contains("No tag creation or tag movement"));
        assert!(markdown.contains("not a later documentation commit"));
    }

    #[test]
    fn git_state_reports_dirty_worktree_as_failure_issue() {
        // r[verify release_evidence.automated_current_head_index.fail_closed]
        // r[verify release_evidence.automated_current_head_index.fail_closed.dirty_or_mismatched]
        let root = env::temp_dir().join(format!(
            "cairn-release-evidence-dirty-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("temp repo directory");
        run_git(&root, &["init"]);
        run_git(&root, &["config", "user.email", "cairn@example.invalid"]);
        run_git(&root, &["config", "user.name", "Cairn Test"]);
        fs::write(root.join("tracked.txt"), "clean\n").expect("write tracked file");
        run_git(&root, &["add", "tracked.txt"]);
        run_git(&root, &["commit", "-m", "initial"]);
        fs::write(root.join("tracked.txt"), "dirty\n").expect("dirty tracked file");

        let mut issues = Vec::new();
        let state = git_head_state(&root, &mut issues);

        assert_eq!(state["clean"].as_bool(), Some(false));
        assert!(issues
            .iter()
            .any(|issue| { issue["code"].as_str() == Some("release_evidence.worktree_dirty") }));
        fs::remove_dir_all(&root).expect("remove temp repo");
    }

    #[test]
    fn publish_materialization_copies_safe_artifact_and_binds_hash() {
        // r[verify release_evidence.publish_materialization.manifest_receipt]
        // r[verify release_evidence.publish_materialization.manifest_receipt.artifact_binding]
        let root = env::temp_dir().join(format!(
            "cairn-release-evidence-publish-copy-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        let source_bundle = root.join("bundle");
        let publish_dir = root.join("published");
        fs::create_dir_all(&source_bundle).expect("source bundle");
        let content = b"release evidence\n";
        fs::write(source_bundle.join("receipt.json"), content).expect("source artifact");
        let artifact = json!({
            "role": "release_readiness",
            "path": "receipt.json",
            "hash": blake3_hex(content),
        });
        let mut materialized = Vec::new();
        let mut issues = Vec::new();

        copy_publish_artifact(
            &artifact,
            &source_bundle,
            &publish_dir,
            &mut materialized,
            &mut issues,
        );

        assert!(issues.is_empty(), "unexpected issues: {issues:?}");
        assert_eq!(materialized.len(), 1);
        assert_eq!(
            materialized[0]["path"].as_str(),
            Some("artifacts/receipt.json")
        );
        assert_eq!(
            fs::read(publish_dir.join("artifacts/receipt.json")).expect("copied artifact"),
            content
        );
        fs::remove_dir_all(&root).expect("remove temp tree");
    }

    #[test]
    fn publish_materialization_rejects_unsafe_paths_and_labels() {
        // r[verify release_evidence.publish_materialization.fail_closed]
        // r[verify release_evidence.publish_materialization.fail_closed.path_or_hash]
        // r[verify release_evidence.publish_materialization.manifest_receipt.metadata]
        assert!(!safe_relative_path_text("../escape.json"));
        let mut issues = Vec::new();
        validate_publish_label("channel", Some("../stable"), &mut issues);
        assert!(issues
            .iter()
            .any(|issue| issue["code"].as_str() == Some("release_evidence_publish.label_invalid")));
    }

    #[test]
    fn retention_plan_marks_newest_keep_and_older_prune_candidate() {
        // r[verify release_evidence.retention_rotation.plan_receipt]
        // r[verify release_evidence.retention_rotation.plan_receipt.decisions]
        // r[verify release_evidence.retention_rotation.plan_receipt.ordering]
        let root = env::temp_dir().join(format!(
            "cairn-release-evidence-retention-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        let publish_root = root.join("published");
        write_publish_tree(
            &publish_root.join("stable/2026-05-01"),
            "stable",
            "2026-05-01",
            "short",
        );
        write_publish_tree(
            &publish_root.join("stable/2026-05-02"),
            "stable",
            "2026-05-02",
            "short",
        );
        let out = root.join("plan.json");
        let receipt = release_evidence_retention_plan_receipt(&ReleaseEvidenceRetentionOptions {
            root: root.clone(),
            publish_root: publish_root.clone(),
            out: out.clone(),
            channel: Some("stable".to_string()),
            keep_latest: 1,
            retain: None,
        });
        assert_eq!(receipt["valid"].as_bool(), Some(true));
        let decisions = receipt["decisions"].as_array().expect("decisions");
        assert!(decisions.iter().any(|decision| {
            decision["path"].as_str() == Some("stable/2026-05-02")
                && decision["action"].as_str() == Some("keep")
        }));
        assert!(decisions.iter().any(|decision| {
            decision["path"].as_str() == Some("stable/2026-05-01")
                && decision["action"].as_str() == Some("prune_candidate")
        }));
        assert!(out.exists());
        fs::remove_dir_all(&root).expect("remove temp tree");
    }

    #[test]
    fn rotate_dry_run_does_not_delete_prune_candidate() {
        // r[verify release_evidence.retention_rotation.rotation_execution]
        // r[verify release_evidence.retention_rotation.rotation_execution.verify_before_mutation]
        let root = env::temp_dir().join(format!(
            "cairn-release-evidence-rotate-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        let publish_root = root.join("published");
        write_publish_tree(&publish_root.join("old"), "stable", "2026-05-01", "short");
        let plan = root.join("plan.json");
        fs::write(
            &plan,
            serde_json::to_vec_pretty(&json!({
                "schema": "cairn.release-evidence.retention-plan.v1",
                "publish_root": path_string(&publish_root),
                "decisions": [{
                    "path": "old",
                    "action": "prune_candidate",
                    "manifest_hash": "unused",
                    "receipt_hash": "unused",
                    "labels": {"channel": "stable", "release": "2026-05-01"},
                    "valid": true,
                }]
            }))
            .expect("serialize plan"),
        )
        .expect("write plan");
        let receipt = release_evidence_rotate_receipt(&ReleaseEvidenceRotateOptions {
            root: root.clone(),
            plan,
            execute: false,
        });
        assert_eq!(receipt["valid"].as_bool(), Some(true));
        assert!(publish_root.join("old").exists());
        assert_eq!(receipt["mutation_count"].as_u64(), Some(1));
        fs::remove_dir_all(&root).expect("remove temp tree");
    }

    #[test]
    fn retention_rejects_duplicate_release_identity() {
        // r[verify release_evidence.retention_rotation.fail_closed]
        // r[verify release_evidence.retention_rotation.fail_closed.unsafe_or_stale]
        let root = env::temp_dir().join(format!(
            "cairn-release-evidence-retention-duplicate-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        let publish_root = root.join("published");
        write_publish_tree(&publish_root.join("a"), "stable", "2026-05-01", "short");
        write_publish_tree(&publish_root.join("b"), "stable", "2026-05-01", "short");
        let receipt = release_evidence_retention_plan_receipt(&ReleaseEvidenceRetentionOptions {
            root: root.clone(),
            publish_root,
            out: root.join("plan.json"),
            channel: Some("stable".to_string()),
            keep_latest: 1,
            retain: None,
        });
        assert_eq!(receipt["valid"].as_bool(), Some(false));
        assert!(receipt["issues"]
            .as_array()
            .expect("issues")
            .iter()
            .any(|issue| {
                issue["code"].as_str()
                    == Some("release_evidence_retention.duplicate_channel_release")
            }));
        fs::remove_dir_all(&root).expect("remove temp tree");
    }

    fn write_publish_tree(path: &std::path::Path, channel: &str, release: &str, retention: &str) {
        fs::create_dir_all(path).expect("publish tree");
        let manifest = json!({
            "schema": "cairn.release-evidence.publish-manifest.v1",
            "labels": {"channel": channel, "release": release, "retention": retention},
            "materialized": [],
        });
        let manifest_bytes = serde_json::to_vec_pretty(&manifest).expect("manifest bytes");
        fs::write(path.join("manifest.json"), &manifest_bytes).expect("manifest");
        let manifest_hash = blake3_hex(&manifest_bytes);
        fs::write(
            path.join("receipt.json"),
            serde_json::to_vec_pretty(&json!({
                "command": "release-evidence publish",
                "valid": true,
                "verdict": "pass",
                "manifest_hash": manifest_hash,
            }))
            .expect("receipt bytes"),
        )
        .expect("receipt");
    }

    fn run_git(root: &std::path::Path, args: &[&str]) {
        let output = Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .expect("git command runs");
        assert!(
            output.status.success(),
            "git {:?} failed\nstdout:{}\nstderr:{}",
            args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
