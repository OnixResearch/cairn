use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use cairn_core::{blake3_hex, canonical_json_hash, Policy};
use serde_json::{json, Value};

use crate::content_addressed_evidence::{
    run_evidence_downstream_pilot, run_evidence_export_bundle_receipt,
    run_evidence_function_object, run_evidence_name_pointer,
};
use crate::evidence_manifest::{
    evidence_manifest_generate_manifest, evidence_manifest_verification_receipt,
    EvidenceManifestGenerateOptions,
};
use crate::evidence_registry::run_evidence_registry;
use crate::{
    agent_evidence_bundle_verification_receipt, agent_receipt_validation_receipt,
    default_policy_path, determinism_coverage_audit_receipt, determinism_replay_receipt,
    determinism_verify_replay_receipt, load_policy_file, path_string, portable_json_hash,
    print_json, read_json_artifact, read_option, read_string_option,
    receipt_stated_or_computed_hash, release_readiness_receipt, with_portable_receipt_hash,
    AGENT_RECEIPT_MAX_BYTES, REQUIRED_AGENT_REPLAY_GROUP,
};

#[derive(Default)]
struct EvidenceExportOptions {
    root: PathBuf,
    out: Option<PathBuf>,
    agent_receipt: Option<PathBuf>,
}

#[derive(Default)]
struct EvidenceVerifyOptions {
    root: PathBuf,
    bundle: Option<PathBuf>,
}

#[derive(Default)]
struct EvidenceImportOptions {
    root: PathBuf,
    bundle: Option<PathBuf>,
    into: Option<PathBuf>,
}

#[derive(Default)]
struct EvidenceRepublishOptions {
    root: PathBuf,
    imported: Option<PathBuf>,
    out: Option<PathBuf>,
}

#[derive(Default)]
struct EvidenceIndexOptions {
    root: PathBuf,
    bundle: Option<PathBuf>,
    out: Option<PathBuf>,
}

#[derive(Default)]
pub(crate) struct EvidenceIndexVerifyOptions {
    pub(crate) root: PathBuf,
    pub(crate) index: Option<PathBuf>,
}

pub(crate) fn run_evidence(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("evidence requires a subcommand: export".into());
    };
    match subcommand.as_str() {
        "export" => run_evidence_export(iter.collect()),
        "verify" => run_evidence_verify(iter.collect()),
        "import" => run_evidence_import(iter.collect()),
        "republish" => run_evidence_republish(iter.collect()),
        "index" => run_evidence_index(iter.collect()),
        "registry" => run_evidence_registry(iter.collect()),
        "function-object" => run_evidence_function_object(iter.collect()),
        "name-pointer" => run_evidence_name_pointer(iter.collect()),
        "export-bundle" => run_evidence_export_bundle_receipt(iter.collect()),
        "downstream-pilot" => run_evidence_downstream_pilot(iter.collect()),
        unknown => Err(format!("unknown evidence subcommand: {unknown}").into()),
    }
}

fn run_evidence_export(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.evidence_export_bundle.command]
    // r[impl cairn.supersede.evidence_export_bundle.artifacts]
    // r[impl cairn.supersede.evidence_export_bundle.failures]
    let options = evidence_export_options(&args)?;
    let receipt = evidence_export_receipt(&options);
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

fn run_evidence_verify(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.evidence_bundle_verify_command.command]
    // r[impl cairn.supersede.evidence_bundle_verify_command.artifacts]
    // r[impl cairn.supersede.evidence_bundle_verify_command.failures]
    let options = evidence_verify_options(&args)?;
    let receipt = evidence_verify_receipt(&options);
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

fn run_evidence_import(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.evidence_import_consume_rail.command]
    // r[impl cairn.supersede.evidence_import_consume_rail.artifacts]
    // r[impl cairn.supersede.evidence_import_consume_rail.failures]
    let options = evidence_import_options(&args)?;
    let receipt = evidence_import_receipt(&options);
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

fn run_evidence_index(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    if let Some(first) = iter.next() {
        if first == "verify" {
            return run_evidence_index_verify(iter.collect());
        }
        let mut rest = vec![first];
        rest.extend(iter);
        let options = evidence_index_options(&rest)?;
        let receipt = evidence_index_receipt(&options);
        let valid = receipt
            .get("valid")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        print_json(receipt)?;
        if !valid {
            std::process::exit(1);
        }
        return Ok(());
    }
    Err("evidence index requires --bundle <dir> --out <file> or verify".into())
}

fn run_evidence_index_verify(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = evidence_index_verify_options(&args)?;
    let receipt = evidence_index_verify_receipt(&options);
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

fn evidence_index_options(args: &[String]) -> Result<EvidenceIndexOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let bundle = read_string_option(args, "--bundle").map(PathBuf::from);
    let out = read_string_option(args, "--out").map(PathBuf::from);
    if bundle.is_none() {
        return Err("evidence index requires --bundle <dir>".into());
    }
    if out.is_none() {
        return Err("evidence index requires --out <file>".into());
    }
    Ok(EvidenceIndexOptions { root, bundle, out })
}

fn evidence_index_verify_options(
    args: &[String],
) -> Result<EvidenceIndexVerifyOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let index = read_string_option(args, "--index").map(PathBuf::from);
    if index.is_none() {
        return Err("evidence index verify requires --index <file>".into());
    }
    Ok(EvidenceIndexVerifyOptions { root, index })
}

fn evidence_index_receipt(options: &EvidenceIndexOptions) -> Value {
    // r[impl cairn.supersede.evidence_publish_index_rail.command]
    // r[impl cairn.supersede.evidence_publish_index_rail.failures]
    let root = options.root.clone();
    let bundle = options.bundle.clone().unwrap_or_else(|| PathBuf::from("."));
    let out = options.out.clone().unwrap_or_else(|| PathBuf::from("."));
    let mut issues = Vec::new();
    let bundle_summary =
        evidence_index_bundle_summary(&root, &bundle, "evidence_index", &mut issues);
    let bundle_relative = evidence_index_bundle_relative(&bundle, &out);
    let mut receipt = evidence_index_final_receipt(
        &root,
        "evidence index",
        issues,
        bundle_summary,
        bundle_relative,
        json!({}),
    );
    if receipt.get("valid").and_then(Value::as_bool) == Some(true) {
        if let Some(parent) = out.parent() {
            if fs::create_dir_all(parent).is_err() {
                receipt = evidence_index_final_receipt(
                    &root,
                    "evidence index",
                    vec![evidence_index_issue(
                        "evidence_index.output_create_failed",
                        "out",
                        "writable output parent",
                        "failed",
                    )],
                    None,
                    Value::Null,
                    json!({}),
                );
            }
        }
        if receipt.get("valid").and_then(Value::as_bool) == Some(true)
            && fs::write(
                &out,
                serde_json::to_vec_pretty(&receipt).unwrap_or_default(),
            )
            .is_err()
        {
            receipt = evidence_index_final_receipt(
                &root,
                "evidence index",
                vec![evidence_index_issue(
                    "evidence_index.receipt_write_failed",
                    "out",
                    "writable index receipt",
                    "failed",
                )],
                None,
                Value::Null,
                json!({}),
            );
        }
    }
    receipt
}

pub(crate) fn evidence_index_verify_receipt(options: &EvidenceIndexVerifyOptions) -> Value {
    // r[impl cairn.supersede.evidence_publish_index_rail.verify]
    // r[impl cairn.supersede.evidence_publish_index_rail.failures]
    let root = options.root.clone();
    let index_path = options.index.clone().unwrap_or_else(|| PathBuf::from("."));
    let mut issues = Vec::new();
    let index = match read_json_artifact(&index_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            return evidence_index_verify_final_receipt(
                &root,
                issues_with_index_missing(),
                Value::Null,
                None,
            );
        }
    };
    let index_receipt_hash = index.get("receipt_hash").cloned().unwrap_or(Value::Null);
    if let Some(stated_hash) = index.get("receipt_hash").and_then(Value::as_str) {
        let mut hash_input = index.clone();
        if let Some(object) = hash_input.as_object_mut() {
            object.remove("receipt_hash");
        }
        if portable_json_hash(&root, &hash_input) != stated_hash {
            issues.push(evidence_index_issue(
                "evidence_index_verify.index_hash_mismatch",
                "receipt_hash",
                "portable receipt hash",
                "mismatch",
            ));
        }
    } else {
        issues.push(evidence_index_issue(
            "evidence_index_verify.index_hash_missing",
            "receipt_hash",
            "BLAKE3 hex",
            "missing",
        ));
    }
    let bundle = evidence_index_verify_bundle_path(&index_path, &index);
    let current_summary =
        evidence_index_bundle_summary(&root, &bundle, "evidence_index_verify", &mut issues);
    if let Some(summary) = current_summary.as_ref() {
        evidence_index_compare_summary(&mut issues, &index, summary);
    }
    evidence_index_verify_final_receipt(&root, issues, index_receipt_hash, current_summary)
}

fn issues_with_index_missing() -> Vec<Value> {
    vec![evidence_index_issue(
        "evidence_index_verify.index_unreadable",
        "index",
        "readable index receipt",
        "unreadable",
    )]
}

fn evidence_index_bundle_relative(bundle: &Path, out: &Path) -> Value {
    let Some(out_parent) = out.parent() else {
        return Value::Null;
    };
    if out_parent == bundle {
        return Value::Null;
    }
    if let (Some(bundle_parent), Some(bundle_name)) = (bundle.parent(), bundle.file_name()) {
        if bundle_parent == out_parent {
            return json!(bundle_name.to_string_lossy().to_string());
        }
    }
    Value::Null
}

fn evidence_index_verify_bundle_path(index_path: &Path, index: &Value) -> PathBuf {
    let parent = index_path.parent().unwrap_or_else(|| Path::new("."));
    if parent.join("evidence-republish-receipt.json").exists()
        || parent.join("evidence-import-receipt.json").exists()
        || parent.join("evidence-export-receipt.json").exists()
    {
        return parent.to_path_buf();
    }
    if let Some(relative) = index.get("bundle_relative").and_then(Value::as_str) {
        if !relative.is_empty() && !relative.starts_with('/') && !relative.contains("..") {
            return parent.join(relative);
        }
    }
    parent.to_path_buf()
}

fn evidence_index_bundle_summary(
    root: &Path,
    bundle: &Path,
    prefix: &'static str,
    issues: &mut Vec<Value>,
) -> Option<Value> {
    let receipt_files = [
        ("republished", "evidence-republish-receipt.json"),
        ("imported", "evidence-import-receipt.json"),
        ("exported", "evidence-export-receipt.json"),
    ];
    let mut present = Vec::new();
    for (kind, file) in receipt_files {
        if bundle.join(file).exists() {
            present.push((kind, file));
        }
    }
    if present.is_empty() {
        issues.push(evidence_index_issue(
            &format!("{prefix}.bundle_receipt_missing"),
            "bundle",
            "one canonical evidence receipt",
            "missing",
        ));
        return None;
    }
    let (kind, receipt_file) = present[0];
    let receipt_path = bundle.join(receipt_file);
    let receipt = match read_json_artifact(&receipt_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            issues.push(evidence_index_issue(
                &format!("{prefix}.bundle_receipt_unreadable"),
                receipt_file,
                "readable bundle receipt",
                "unreadable",
            ));
            return None;
        }
    };
    if receipt.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_index_issue(
            &format!("{prefix}.bundle_receipt_invalid"),
            receipt_file,
            "valid bundle receipt",
            "invalid",
        ));
    }
    let mut artifacts = receipt
        .get("artifacts")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    artifacts.sort_by_key(canonical_json_hash);
    for artifact in artifacts.iter() {
        let role = artifact
            .get("role")
            .and_then(Value::as_str)
            .unwrap_or("<missing>");
        let relative = artifact.get("path").and_then(Value::as_str).unwrap_or("");
        let declared_hash = artifact.get("hash").and_then(Value::as_str).unwrap_or("");
        let Some(path) = safe_bundle_artifact_path(bundle, relative) else {
            issues.push(evidence_index_issue(
                &format!("{prefix}.artifact_path_unsafe"),
                role,
                "safe relative artifact path",
                "unsafe",
            ));
            continue;
        };
        match fs::read(&path) {
            Ok(bytes) => {
                if blake3_hex(&bytes) != declared_hash {
                    issues.push(evidence_index_issue(
                        &format!("{prefix}.artifact_hash_mismatch"),
                        role,
                        "declared artifact hash",
                        "mismatch",
                    ));
                }
            }
            Err(_) => issues.push(evidence_index_issue(
                &format!("{prefix}.artifact_missing"),
                role,
                "declared artifact file",
                "missing",
            )),
        }
    }
    Some(json!({
        "bundle_kind": kind,
        "bundle_receipt": receipt_file,
        "bundle_receipt_hash": receipt.get("receipt_hash").cloned().unwrap_or(Value::Null),
        "receipt_chain": evidence_index_receipt_chain(&receipt),
        "artifacts": artifacts,
        "policy_hash": evidence_index_policy_hash(&receipt),
        "root_hash": portable_json_hash(root, &receipt),
    }))
}

fn evidence_index_receipt_chain(receipt: &Value) -> Value {
    let keys = [
        "receipt_hash",
        "export_receipt_hash",
        "verification_receipt_hash",
        "import_receipt_hash",
        "coverage_audit_receipt_hash",
        "release_readiness_receipt_hash",
    ];
    let mut object = serde_json::Map::new();
    for key in keys {
        if let Some(value) = receipt.get(key) {
            object.insert(key.to_string(), value.clone());
        }
    }
    Value::Object(object)
}

fn evidence_index_policy_hash(receipt: &Value) -> Value {
    receipt
        .get("current_replay_policy_hash")
        .or_else(|| receipt.get("replay_policy_hash"))
        .cloned()
        .unwrap_or(Value::Null)
}

fn evidence_index_compare_summary(issues: &mut Vec<Value>, index: &Value, summary: &Value) {
    for key in [
        "bundle_kind",
        "bundle_receipt_hash",
        "receipt_chain",
        "artifacts",
    ] {
        if index.get(key) != summary.get(key) {
            issues.push(evidence_index_issue(
                "evidence_index_verify.index_bundle_mismatch",
                key,
                "indexed bundle value",
                "mismatch",
            ));
        }
    }
}

fn evidence_index_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

fn evidence_index_final_receipt(
    root: &Path,
    command: &str,
    mut issues: Vec<Value>,
    bundle_summary: Option<Value>,
    bundle_relative: Value,
    extra: Value,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    let empty = json!({});
    let summary = bundle_summary.as_ref().unwrap_or(&empty);
    let mut receipt = json!({
        "command": command,
        "root": path_string(root),
        "bundle": "<bundle>",
        "bundle_relative": bundle_relative,
        "out": "<out>",
        "valid": valid,
        "verdict": if valid { "pass" } else { "fail" },
        "issues": issues,
        "bundle_kind": summary.get("bundle_kind").cloned().unwrap_or(Value::Null),
        "bundle_receipt": summary.get("bundle_receipt").cloned().unwrap_or(Value::Null),
        "bundle_receipt_hash": summary.get("bundle_receipt_hash").cloned().unwrap_or(Value::Null),
        "receipt_chain": summary.get("receipt_chain").cloned().unwrap_or_else(|| json!({})),
        "policy_hash": summary.get("policy_hash").cloned().unwrap_or(Value::Null),
        "artifacts": summary.get("artifacts").cloned().unwrap_or_else(|| json!([])),
    });
    if let Some(object) = receipt.as_object_mut() {
        if let Some(extra_object) = extra.as_object() {
            for (key, value) in extra_object {
                object.insert(key.clone(), value.clone());
            }
        }
    }
    with_portable_receipt_hash(root, receipt)
}

fn evidence_index_verify_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    index_receipt_hash: Value,
    current_summary: Option<Value>,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    let empty = json!({});
    let summary = current_summary.as_ref().unwrap_or(&empty);
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence index verify",
            "root": path_string(root),
            "index": "<index>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "index_receipt_hash": index_receipt_hash,
            "bundle_kind": summary.get("bundle_kind").cloned().unwrap_or(Value::Null),
            "bundle_receipt_hash": summary.get("bundle_receipt_hash").cloned().unwrap_or(Value::Null),
            "receipt_chain": summary.get("receipt_chain").cloned().unwrap_or_else(|| json!({})),
            "artifacts": summary.get("artifacts").cloned().unwrap_or_else(|| json!([])),
        }),
    )
}

fn run_evidence_republish(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.evidence_import_republish_rail.command]
    // r[impl cairn.supersede.evidence_import_republish_rail.artifacts]
    // r[impl cairn.supersede.evidence_import_republish_rail.failures]
    let options = evidence_republish_options(&args)?;
    let receipt = evidence_republish_receipt(&options);
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

fn evidence_republish_options(args: &[String]) -> Result<EvidenceRepublishOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let imported = read_string_option(args, "--imported").map(PathBuf::from);
    let out = read_string_option(args, "--out").map(PathBuf::from);
    if imported.is_none() {
        return Err("evidence republish requires --imported <dir>".into());
    }
    if out.is_none() {
        return Err("evidence republish requires --out <dir>".into());
    }
    Ok(EvidenceRepublishOptions {
        root,
        imported,
        out,
    })
}

fn evidence_republish_receipt(options: &EvidenceRepublishOptions) -> Value {
    let root = options.root.clone();
    let imported_dir = options
        .imported
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));
    let out_dir = options.out.clone().unwrap_or_else(|| PathBuf::from("."));
    let mut issues = Vec::new();
    let import_receipt_path = imported_dir.join("evidence-import-receipt.json");
    let import_receipt = match read_json_artifact(&import_receipt_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(receipt) => receipt,
        Err(_) => {
            issues.push(evidence_republish_issue(
                "evidence_republish.import_receipt_missing",
                "evidence-import-receipt.json",
                "readable import receipt",
                "missing",
            ));
            return evidence_republish_final_receipt(&root, issues, Vec::new(), Value::Null);
        }
    };

    let import_receipt_hash = import_receipt
        .get("receipt_hash")
        .cloned()
        .unwrap_or(Value::Null);
    if import_receipt.get("command").and_then(Value::as_str) != Some("evidence import") {
        issues.push(evidence_republish_issue(
            "evidence_republish.import_receipt_command_invalid",
            "command",
            "evidence import",
            import_receipt
                .get("command")
                .and_then(Value::as_str)
                .unwrap_or("<missing>"),
        ));
    }
    if import_receipt.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_republish_issue(
            "evidence_republish.import_receipt_invalid",
            "valid",
            "true",
            "false",
        ));
    }
    if let Some(stated_hash) = import_receipt.get("receipt_hash").and_then(Value::as_str) {
        let mut receipt_hash_input = import_receipt.clone();
        if let Some(object) = receipt_hash_input.as_object_mut() {
            object.remove("receipt_hash");
        }
        let expected_hash = portable_json_hash(&root, &receipt_hash_input);
        if stated_hash != expected_hash {
            issues.push(evidence_republish_issue(
                "evidence_republish.import_receipt_hash_mismatch",
                "receipt_hash",
                "portable receipt hash",
                "mismatch",
            ));
        }
    } else {
        issues.push(evidence_republish_issue(
            "evidence_republish.import_receipt_hash_missing",
            "receipt_hash",
            "BLAKE3 hex",
            "missing",
        ));
    }

    let Some(artifacts) = import_receipt.get("artifacts").and_then(Value::as_array) else {
        issues.push(evidence_republish_issue(
            "evidence_republish.artifacts_missing",
            "artifacts",
            "artifact array",
            "missing",
        ));
        return evidence_republish_final_receipt(&root, issues, Vec::new(), import_receipt_hash);
    };

    if out_dir.exists() {
        match bundle_regular_files_with_prefix(&out_dir, "evidence_republish") {
            Ok(files) if !files.is_empty() => issues.push(evidence_republish_issue(
                "evidence_republish.output_not_empty",
                "out",
                "empty output directory",
                "non_empty",
            )),
            Err(code) => issues.push(evidence_republish_issue(
                code,
                "out",
                "bounded readable output files",
                "failed",
            )),
            _ => {}
        }
    }

    let mut declared_files = std::collections::BTreeSet::new();
    declared_files.insert("evidence-import-receipt.json".to_string());
    let mut copied = Vec::new();
    let mut seen_paths = std::collections::BTreeSet::new();
    match bundle_regular_files_with_prefix(&imported_dir, "evidence_republish") {
        Ok(files) => {
            for relative in files {
                if !declared_files.contains(&relative)
                    && !artifacts.iter().any(|artifact| {
                        artifact.get("path").and_then(Value::as_str) == Some(relative.as_str())
                    })
                {
                    issues.push(evidence_republish_issue(
                        "evidence_republish.undeclared_imported_file",
                        &relative,
                        "declared imported artifact",
                        "extra",
                    ));
                }
            }
        }
        Err(code) => issues.push(evidence_republish_issue(
            code,
            "imported",
            "bounded readable imported files",
            "failed",
        )),
    }
    for artifact in artifacts {
        let role = artifact
            .get("role")
            .and_then(Value::as_str)
            .unwrap_or("<missing>");
        let relative = artifact.get("path").and_then(Value::as_str).unwrap_or("");
        if !seen_paths.insert(relative.to_string()) {
            issues.push(evidence_republish_issue(
                "evidence_republish.duplicate_artifact_path",
                role,
                "unique artifact path",
                "duplicate",
            ));
            continue;
        }
        let Some(source_path) = safe_bundle_artifact_path(&imported_dir, relative) else {
            issues.push(evidence_republish_issue(
                "evidence_republish.artifact_path_unsafe",
                role,
                "safe relative artifact path",
                "unsafe",
            ));
            continue;
        };
        let Some(destination_path) = safe_bundle_artifact_path(&out_dir, relative) else {
            issues.push(evidence_republish_issue(
                "evidence_republish.destination_path_unsafe",
                role,
                "safe relative destination path",
                "unsafe",
            ));
            continue;
        };
        declared_files.insert(relative.to_string());
        let declared_hash = artifact.get("hash").and_then(Value::as_str).unwrap_or("");
        let source_bytes = match fs::read(&source_path) {
            Ok(bytes) => bytes,
            Err(_) => {
                issues.push(evidence_republish_issue(
                    "evidence_republish.artifact_missing",
                    role,
                    "declared artifact file",
                    "missing",
                ));
                continue;
            }
        };
        let source_hash = blake3_hex(&source_bytes);
        if source_hash != declared_hash {
            issues.push(evidence_republish_issue(
                "evidence_republish.artifact_hash_mismatch",
                role,
                "declared artifact hash",
                "mismatch",
            ));
            continue;
        }
        if issues.is_empty() {
            if let Some(parent) = destination_path.parent() {
                if fs::create_dir_all(parent).is_err() {
                    issues.push(evidence_republish_issue(
                        "evidence_republish.destination_create_failed",
                        role,
                        "writable destination parent",
                        "failed",
                    ));
                    continue;
                }
            }
            if destination_path.exists() {
                issues.push(evidence_republish_issue(
                    "evidence_republish.destination_collision",
                    role,
                    "absent destination artifact",
                    "exists",
                ));
                continue;
            }
            if fs::write(&destination_path, &source_bytes).is_err() {
                issues.push(evidence_republish_issue(
                    "evidence_republish.artifact_copy_failed",
                    role,
                    "copied artifact file",
                    "failed",
                ));
                continue;
            }
        }
        copied.push(json!({
            "role": role,
            "path": relative,
            "hash": source_hash,
        }));
    }

    let mut receipt = evidence_republish_final_receipt(&root, issues, copied, import_receipt_hash);
    if receipt.get("valid").and_then(Value::as_bool) == Some(true) {
        if fs::create_dir_all(&out_dir).is_err() {
            receipt = evidence_republish_final_receipt(
                &root,
                vec![evidence_republish_issue(
                    "evidence_republish.output_create_failed",
                    "out",
                    "writable output directory",
                    "failed",
                )],
                Vec::new(),
                receipt
                    .get("import_receipt_hash")
                    .cloned()
                    .unwrap_or(Value::Null),
            );
        } else {
            let import_receipt_bytes =
                serde_json::to_vec_pretty(&import_receipt).unwrap_or_default();
            if fs::write(
                out_dir.join("evidence-import-receipt.json"),
                import_receipt_bytes,
            )
            .is_err()
            {
                receipt = evidence_republish_final_receipt(
                    &root,
                    vec![evidence_republish_issue(
                        "evidence_republish.import_receipt_copy_failed",
                        "evidence-import-receipt.json",
                        "copied import receipt",
                        "failed",
                    )],
                    Vec::new(),
                    receipt
                        .get("import_receipt_hash")
                        .cloned()
                        .unwrap_or(Value::Null),
                );
            } else if fs::write(
                out_dir.join("evidence-republish-receipt.json"),
                serde_json::to_vec_pretty(&receipt).unwrap_or_default(),
            )
            .is_err()
            {
                receipt = evidence_republish_final_receipt(
                    &root,
                    vec![evidence_republish_issue(
                        "evidence_republish.receipt_write_failed",
                        "evidence-republish-receipt.json",
                        "writable republish receipt",
                        "failed",
                    )],
                    Vec::new(),
                    receipt
                        .get("import_receipt_hash")
                        .cloned()
                        .unwrap_or(Value::Null),
                );
            }
        }
    }
    receipt
}

fn bundle_regular_files_with_prefix(
    bundle_dir: &Path,
    prefix: &'static str,
) -> Result<std::collections::BTreeSet<String>, &'static str> {
    let mut out = std::collections::BTreeSet::new();
    collect_bundle_regular_files_with_prefix(bundle_dir, bundle_dir, &mut out, prefix)?;
    Ok(out)
}

fn collect_bundle_regular_files_with_prefix(
    root: &Path,
    dir: &Path,
    out: &mut std::collections::BTreeSet<String>,
    prefix: &'static str,
) -> Result<(), &'static str> {
    let entries = fs::read_dir(dir).map_err(|_| match prefix {
        "evidence_republish" => "evidence_republish.bundle_read_failed",
        _ => "evidence_import.bundle_read_failed",
    })?;
    let mut paths = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|_| match prefix {
            "evidence_republish" => "evidence_republish.bundle_read_failed",
            _ => "evidence_import.bundle_read_failed",
        })?;
        paths.push(entry.path());
    }
    paths.sort();
    for path in paths {
        let metadata = fs::metadata(&path).map_err(|_| match prefix {
            "evidence_republish" => "evidence_republish.bundle_read_failed",
            _ => "evidence_import.bundle_read_failed",
        })?;
        if metadata.is_dir() {
            collect_bundle_regular_files_with_prefix(root, &path, out, prefix)?;
        } else if metadata.is_file() {
            if out.len() >= EVIDENCE_IMPORT_MAX_FILES {
                return Err(match prefix {
                    "evidence_republish" => "evidence_republish.bundle_file_limit_exceeded",
                    _ => "evidence_import.bundle_file_limit_exceeded",
                });
            }
            let relative = path
                .strip_prefix(root)
                .map_err(|_| match prefix {
                    "evidence_republish" => "evidence_republish.bundle_read_failed",
                    _ => "evidence_import.bundle_read_failed",
                })?
                .to_string_lossy()
                .replace('\\', "/");
            out.insert(relative);
        }
    }
    Ok(())
}

fn evidence_republish_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

fn evidence_republish_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    mut artifacts: Vec<Value>,
    import_receipt_hash: Value,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    artifacts.sort_by_key(|artifact| {
        format!(
            "{}:{}",
            artifact.get("role").and_then(Value::as_str).unwrap_or(""),
            artifact.get("path").and_then(Value::as_str).unwrap_or("")
        )
    });
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence republish",
            "root": path_string(root),
            "imported": "<imported>",
            "out": "<out>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "artifacts": artifacts,
            "import_receipt_hash": import_receipt_hash,
        }),
    )
}

fn evidence_import_options(args: &[String]) -> Result<EvidenceImportOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let bundle = read_string_option(args, "--bundle").map(PathBuf::from);
    let into = read_string_option(args, "--into").map(PathBuf::from);
    if bundle.is_none() {
        return Err("evidence import requires --bundle <dir>".into());
    }
    if into.is_none() {
        return Err("evidence import requires --into <dir>".into());
    }
    Ok(EvidenceImportOptions { root, bundle, into })
}

fn evidence_import_receipt(options: &EvidenceImportOptions) -> Value {
    let root = options.root.clone();
    let bundle_dir = options.bundle.clone().unwrap_or_else(|| PathBuf::from("."));
    let into_dir = options.into.clone().unwrap_or_else(|| PathBuf::from("."));
    let mut issues = Vec::new();
    let verify_options = EvidenceVerifyOptions {
        root: root.clone(),
        bundle: Some(bundle_dir.clone()),
    };
    let verify_receipt = evidence_verify_receipt(&verify_options);
    let verification_receipt_hash = verify_receipt
        .get("receipt_hash")
        .cloned()
        .unwrap_or(Value::Null);
    if verify_receipt.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_import_issue(
            "evidence_import.bundle_verification_failed",
            "bundle",
            "valid evidence verify receipt",
            "invalid",
        ));
        return evidence_import_final_receipt(&root, issues, Vec::new(), verification_receipt_hash);
    }

    let export_receipt_path = bundle_dir.join("evidence-export-receipt.json");
    let export_receipt = match read_json_artifact(&export_receipt_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(receipt) => receipt,
        Err(_) => {
            issues.push(evidence_import_issue(
                "evidence_import.export_receipt_unreadable",
                "evidence-export-receipt.json",
                "readable export receipt",
                "unreadable",
            ));
            return evidence_import_final_receipt(
                &root,
                issues,
                Vec::new(),
                verification_receipt_hash,
            );
        }
    };

    let Some(artifacts) = export_receipt.get("artifacts").and_then(Value::as_array) else {
        issues.push(evidence_import_issue(
            "evidence_import.artifacts_missing",
            "artifacts",
            "artifact array",
            "missing",
        ));
        return evidence_import_final_receipt(&root, issues, Vec::new(), verification_receipt_hash);
    };

    let mut declared_files = std::collections::BTreeSet::new();
    declared_files.insert("evidence-export-receipt.json".to_string());
    let mut copied = Vec::new();
    for artifact in artifacts {
        let role = artifact
            .get("role")
            .and_then(Value::as_str)
            .unwrap_or("<missing>");
        let relative = artifact.get("path").and_then(Value::as_str).unwrap_or("");
        let Some(source_path) = safe_bundle_artifact_path(&bundle_dir, relative) else {
            issues.push(evidence_import_issue(
                "evidence_import.artifact_path_unsafe",
                role,
                "safe relative artifact path",
                "unsafe",
            ));
            continue;
        };
        let Some(destination_path) = safe_bundle_artifact_path(&into_dir, relative) else {
            issues.push(evidence_import_issue(
                "evidence_import.destination_path_unsafe",
                role,
                "safe relative destination path",
                "unsafe",
            ));
            continue;
        };
        declared_files.insert(relative.to_string());
        let declared_hash = artifact.get("hash").and_then(Value::as_str).unwrap_or("");
        let source_bytes = match fs::read(&source_path) {
            Ok(bytes) => bytes,
            Err(_) => {
                issues.push(evidence_import_issue(
                    "evidence_import.artifact_missing",
                    role,
                    "declared artifact file",
                    "missing",
                ));
                continue;
            }
        };
        let source_hash = blake3_hex(&source_bytes);
        if source_hash != declared_hash {
            issues.push(evidence_import_issue(
                "evidence_import.artifact_hash_mismatch",
                role,
                "declared artifact hash",
                "mismatch",
            ));
            continue;
        }
        if let Some(parent) = destination_path.parent() {
            if fs::create_dir_all(parent).is_err() {
                issues.push(evidence_import_issue(
                    "evidence_import.destination_create_failed",
                    role,
                    "writable destination parent",
                    "failed",
                ));
                continue;
            }
        }
        if fs::write(&destination_path, &source_bytes).is_err() {
            issues.push(evidence_import_issue(
                "evidence_import.artifact_copy_failed",
                role,
                "copied artifact file",
                "failed",
            ));
            continue;
        }
        let destination_hash = match fs::read(&destination_path) {
            Ok(bytes) => blake3_hex(&bytes),
            Err(_) => String::new(),
        };
        if destination_hash != source_hash {
            issues.push(evidence_import_issue(
                "evidence_import.destination_hash_mismatch",
                role,
                "copied artifact hash",
                "mismatch",
            ));
            continue;
        }
        copied.push(json!({
            "role": role,
            "path": relative,
            "hash": destination_hash,
        }));
    }

    match bundle_regular_files(&bundle_dir) {
        Ok(files) => {
            for relative in files {
                if !declared_files.contains(&relative) {
                    issues.push(evidence_import_issue(
                        "evidence_import.undeclared_bundle_file",
                        &relative,
                        "declared export artifact",
                        "extra",
                    ));
                }
            }
        }
        Err(code) => issues.push(evidence_import_issue(
            code,
            "bundle",
            "bounded readable bundle files",
            "failed",
        )),
    }

    let mut receipt =
        evidence_import_final_receipt(&root, issues, copied, verification_receipt_hash);
    if receipt.get("valid").and_then(Value::as_bool) == Some(true) {
        if fs::create_dir_all(&into_dir).is_err() {
            let mut issues = vec![evidence_import_issue(
                "evidence_import.destination_create_failed",
                "into",
                "writable import directory",
                "failed",
            )];
            receipt = evidence_import_final_receipt(
                &root,
                std::mem::take(&mut issues),
                Vec::new(),
                receipt
                    .get("verification_receipt_hash")
                    .cloned()
                    .unwrap_or(Value::Null),
            );
        } else if fs::write(
            into_dir.join("evidence-import-receipt.json"),
            serde_json::to_vec_pretty(&receipt).unwrap_or_default(),
        )
        .is_err()
        {
            receipt = evidence_import_final_receipt(
                &root,
                vec![evidence_import_issue(
                    "evidence_import.receipt_write_failed",
                    "evidence-import-receipt.json",
                    "writable import receipt",
                    "failed",
                )],
                Vec::new(),
                receipt
                    .get("verification_receipt_hash")
                    .cloned()
                    .unwrap_or(Value::Null),
            );
        }
    }
    receipt
}

const EVIDENCE_IMPORT_MAX_FILES: usize = 64;

fn bundle_regular_files(
    bundle_dir: &Path,
) -> Result<std::collections::BTreeSet<String>, &'static str> {
    let mut out = std::collections::BTreeSet::new();
    collect_bundle_regular_files(bundle_dir, bundle_dir, &mut out)?;
    Ok(out)
}

fn collect_bundle_regular_files(
    root: &Path,
    dir: &Path,
    out: &mut std::collections::BTreeSet<String>,
) -> Result<(), &'static str> {
    let entries = fs::read_dir(dir).map_err(|_| "evidence_import.bundle_read_failed")?;
    let mut paths = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|_| "evidence_import.bundle_read_failed")?;
        paths.push(entry.path());
    }
    paths.sort();
    for path in paths {
        let metadata = fs::metadata(&path).map_err(|_| "evidence_import.bundle_read_failed")?;
        if metadata.is_dir() {
            collect_bundle_regular_files(root, &path, out)?;
        } else if metadata.is_file() {
            if out.len() >= EVIDENCE_IMPORT_MAX_FILES {
                return Err("evidence_import.bundle_file_limit_exceeded");
            }
            let relative = path
                .strip_prefix(root)
                .map_err(|_| "evidence_import.bundle_read_failed")?
                .to_string_lossy()
                .replace('\\', "/");
            out.insert(relative);
        }
    }
    Ok(())
}

fn evidence_import_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

fn evidence_import_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    mut artifacts: Vec<Value>,
    verification_receipt_hash: Value,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    artifacts.sort_by_key(|artifact| {
        format!(
            "{}:{}",
            artifact.get("role").and_then(Value::as_str).unwrap_or(""),
            artifact.get("path").and_then(Value::as_str).unwrap_or("")
        )
    });
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence import",
            "root": path_string(root),
            "bundle": "<bundle>",
            "into": "<into>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "artifacts": artifacts,
            "verification_receipt_hash": verification_receipt_hash,
        }),
    )
}

fn evidence_verify_options(args: &[String]) -> Result<EvidenceVerifyOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let bundle = read_string_option(args, "--bundle").map(PathBuf::from);
    if bundle.is_none() {
        return Err("evidence verify requires --bundle <dir>".into());
    }
    Ok(EvidenceVerifyOptions { root, bundle })
}

fn evidence_verify_receipt(options: &EvidenceVerifyOptions) -> Value {
    let root = options.root.clone();
    let bundle_dir = options.bundle.clone().unwrap_or_else(|| PathBuf::from("."));
    let mut issues = Vec::new();
    let export_receipt_path = bundle_dir.join("evidence-export-receipt.json");
    let export_receipt = match read_json_artifact(&export_receipt_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(receipt) => receipt,
        Err(_) => {
            issues.push(evidence_verify_issue(
                "evidence_verify.export_receipt_missing",
                "evidence-export-receipt.json",
                "readable export receipt",
                "missing",
            ));
            return evidence_verify_final_receipt(
                &root,
                issues,
                Vec::new(),
                Value::Null,
                Value::Null,
                Value::Null,
            );
        }
    };

    if export_receipt.get("command").and_then(Value::as_str) != Some("evidence export") {
        issues.push(evidence_verify_issue(
            "evidence_verify.export_receipt_command_invalid",
            "evidence-export-receipt.command",
            "evidence export",
            "invalid",
        ));
    }
    let stated_export_hash = export_receipt
        .get("receipt_hash")
        .and_then(Value::as_str)
        .unwrap_or("");
    let computed_export_hash = receipt_stated_or_computed_hash(&root, &export_receipt);
    if computed_export_hash.as_str() != Some(stated_export_hash) {
        issues.push(evidence_verify_issue(
            "evidence_verify.export_receipt_hash_mismatch",
            "evidence-export-receipt.receipt_hash",
            "self-consistent receipt hash",
            "mismatch",
        ));
    }

    let mut verified_artifacts = Vec::new();
    let mut seen_roles = std::collections::BTreeSet::new();
    if let Some(artifacts) = export_receipt.get("artifacts").and_then(Value::as_array) {
        for artifact in artifacts {
            let role = artifact
                .get("role")
                .and_then(Value::as_str)
                .unwrap_or("<missing>");
            if !seen_roles.insert(role.to_string()) {
                issues.push(evidence_verify_issue(
                    "evidence_verify.role_duplicate",
                    role,
                    "unique artifact role",
                    "duplicate",
                ));
            }
            let relative = artifact.get("path").and_then(Value::as_str).unwrap_or("");
            let Some(path) = safe_bundle_artifact_path(&bundle_dir, relative) else {
                issues.push(evidence_verify_issue(
                    "evidence_verify.artifact_path_unsafe",
                    role,
                    "safe relative artifact path",
                    "unsafe",
                ));
                continue;
            };
            let declared_hash = artifact.get("hash").and_then(Value::as_str).unwrap_or("");
            let actual_hash = match fs::read(&path) {
                Ok(bytes) => blake3_hex(&bytes),
                Err(_) => {
                    issues.push(evidence_verify_issue(
                        "evidence_verify.artifact_missing",
                        role,
                        "declared artifact file",
                        "missing",
                    ));
                    String::new()
                }
            };
            if !actual_hash.is_empty() && actual_hash != declared_hash {
                issues.push(evidence_verify_issue(
                    "evidence_verify.artifact_hash_mismatch",
                    role,
                    "declared artifact hash",
                    "mismatch",
                ));
            }
            verified_artifacts.push(json!({
                "role": role,
                "path": relative,
                "hash": if actual_hash.is_empty() { Value::Null } else { json!(actual_hash) },
                "required": artifact.get("required").and_then(Value::as_bool).unwrap_or(true),
            }));
        }
    } else {
        issues.push(evidence_verify_issue(
            "evidence_verify.artifacts_missing",
            "artifacts",
            "artifact array",
            "missing",
        ));
    }

    for role in EVIDENCE_EXPORT_REQUIRED_ROLES {
        if !seen_roles.contains(*role) {
            issues.push(evidence_verify_issue(
                "evidence_verify.required_role_missing",
                role,
                "required artifact role",
                "missing",
            ));
        }
    }

    verify_saved_receipt_valid(
        &bundle_dir,
        "coverage_audit",
        "coverage-audit.json",
        "evidence_verify.coverage_audit_invalid",
        &mut issues,
    );
    verify_saved_receipt_valid(
        &bundle_dir,
        "replay_verification",
        "replay-verification.json",
        "evidence_verify.replay_verification_invalid",
        &mut issues,
    );
    verify_saved_receipt_valid(
        &bundle_dir,
        "release_readiness",
        "release-readiness.json",
        "evidence_verify.release_readiness_invalid",
        &mut issues,
    );
    verify_saved_receipt_valid(
        &bundle_dir,
        "agent_evidence_bundle_verification",
        "agent-evidence-bundle-verification.json",
        "evidence_verify.bundle_verification_invalid",
        &mut issues,
    );
    verify_saved_receipt_valid(
        &bundle_dir,
        "evidence_manifest_verification",
        "evidence-manifest-verification.json",
        "evidence_verify.manifest_verification_invalid",
        &mut issues,
    );

    let rerun_bundle = agent_evidence_bundle_verification_receipt(
        &root,
        &bundle_dir.join("agent-evidence-bundle.json"),
    );
    if rerun_bundle.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_verify_issue(
            "evidence_verify.bundle_rerun_invalid",
            "agent-evidence-bundle.json",
            "valid rerun bundle verification",
            "invalid",
        ));
    }
    let rerun_manifest =
        evidence_manifest_verification_receipt(&root, &bundle_dir.join("evidence-manifest.json"));
    if rerun_manifest.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_verify_issue(
            "evidence_verify.manifest_rerun_invalid",
            "evidence-manifest.json",
            "valid rerun manifest verification",
            "invalid",
        ));
    }

    let manifest_hash = export_receipt
        .get("manifest_hash")
        .cloned()
        .unwrap_or(Value::Null);
    let manifest_verification_receipt_hash = export_receipt
        .get("manifest_verification_receipt_hash")
        .cloned()
        .unwrap_or(Value::Null);
    evidence_verify_final_receipt(
        &root,
        issues,
        verified_artifacts,
        computed_export_hash,
        manifest_hash,
        manifest_verification_receipt_hash,
    )
}

const EVIDENCE_EXPORT_REQUIRED_ROLES: &[&str] = &[
    "agent_evidence_bundle",
    "agent_evidence_bundle_verification",
    "agent_receipt",
    "coverage_audit",
    "evidence_manifest",
    "evidence_manifest_verification",
    "release_readiness",
    "replay_receipt",
    "replay_verification",
];

fn safe_bundle_artifact_path(bundle_dir: &Path, relative: &str) -> Option<PathBuf> {
    let path = PathBuf::from(relative);
    if relative.is_empty() || path.is_absolute() {
        return None;
    }
    if path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return None;
    }
    Some(bundle_dir.join(path))
}

fn verify_saved_receipt_valid(
    bundle_dir: &Path,
    role: &str,
    relative: &str,
    code: &str,
    issues: &mut Vec<Value>,
) {
    let path = bundle_dir.join(relative);
    match read_json_artifact(&path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(receipt) => {
            if receipt.get("valid").and_then(Value::as_bool) != Some(true) {
                issues.push(evidence_verify_issue(
                    code,
                    role,
                    "persisted valid receipt",
                    "invalid",
                ));
            }
        }
        Err(_) => issues.push(evidence_verify_issue(
            "evidence_verify.receipt_unreadable",
            role,
            "readable JSON receipt",
            "unreadable",
        )),
    }
}

fn evidence_verify_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

fn evidence_verify_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    mut artifacts: Vec<Value>,
    export_receipt_hash: Value,
    manifest_hash: Value,
    manifest_verification_receipt_hash: Value,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    artifacts.sort_by_key(|artifact| {
        artifact
            .get("role")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string()
    });
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence verify",
            "root": path_string(root),
            "bundle": "<bundle>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "artifacts": artifacts,
            "export_receipt_hash": export_receipt_hash,
            "manifest_hash": manifest_hash,
            "manifest_verification_receipt_hash": manifest_verification_receipt_hash,
        }),
    )
}

fn evidence_export_options(args: &[String]) -> Result<EvidenceExportOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let out = read_string_option(args, "--out").map(PathBuf::from);
    let agent_receipt = read_string_option(args, "--agent-receipt").map(PathBuf::from);
    if out.is_none() {
        return Err("evidence export requires --out <dir>".into());
    }
    Ok(EvidenceExportOptions {
        root,
        out,
        agent_receipt,
    })
}

fn evidence_export_receipt(options: &EvidenceExportOptions) -> Value {
    let root = options.root.clone();
    let out_dir = options.out.clone().unwrap_or_else(|| PathBuf::from("."));
    let mut issues = Vec::new();
    if fs::create_dir_all(&out_dir).is_err() {
        issues.push(evidence_export_issue(
            "export.out_dir_create_failed",
            "out_dir",
            "writable output directory",
            "failed",
        ));
        return evidence_export_final_receipt(
            &root,
            &out_dir,
            issues,
            Vec::new(),
            Value::Null,
            Value::Null,
        );
    }

    let policy = match load_policy_file(default_policy_path().into()) {
        Ok(policy) => policy,
        Err(_) => {
            issues.push(evidence_export_issue(
                "export.policy_load_failed",
                "policy",
                "generated policy loads",
                "failed",
            ));
            return evidence_export_final_receipt(
                &root,
                &out_dir,
                issues,
                Vec::new(),
                Value::Null,
                Value::Null,
            );
        }
    };

    let agent_path = options
        .agent_receipt
        .clone()
        .unwrap_or_else(|| root.join("fixtures/agent-receipts/positive.json"));
    let coverage_path = out_dir.join("coverage-audit.json");
    let replay_path = out_dir.join("replay-agent.json");
    let replay_verification_path = out_dir.join("replay-verification.json");
    let release_path = out_dir.join("release-readiness.json");
    let exported_agent_path = out_dir.join("agent-receipt.json");
    let bundle_path = out_dir.join("agent-evidence-bundle.json");
    let bundle_verification_path = out_dir.join("agent-evidence-bundle-verification.json");
    let manifest_path = out_dir.join("evidence-manifest.json");
    let manifest_verification_path = out_dir.join("evidence-manifest-verification.json");

    let coverage = determinism_coverage_audit_receipt(&root, &policy);
    write_evidence_json(&coverage_path, &coverage, &mut issues, "coverage_audit");

    let replay =
        match determinism_replay_receipt(root.clone(), None, Some(REQUIRED_AGENT_REPLAY_GROUP)) {
            Ok(receipt) => receipt,
            Err(_) => {
                issues.push(evidence_export_issue(
                    "export.replay_failed",
                    "replay_receipt",
                    "valid replay receipt",
                    "failed",
                ));
                Value::Null
            }
        };
    if replay.is_object() {
        write_evidence_json(&replay_path, &replay, &mut issues, "replay_receipt");
    }

    let replay_verification = if replay.is_object() {
        write_evidence_json(&replay_path, &replay, &mut issues, "replay_receipt");
        determinism_verify_replay_receipt(&root, &replay_path)
    } else {
        Value::Null
    };
    if replay_verification.is_object() {
        write_evidence_json(
            &replay_verification_path,
            &replay_verification,
            &mut issues,
            "replay_verification",
        );
    }

    let release = release_readiness_receipt(root.clone(), default_policy_path().into());
    write_evidence_json(&release_path, &release, &mut issues, "release_readiness");

    let exported_agent_receipt = if options.agent_receipt.is_some() {
        match fs::copy(&agent_path, &exported_agent_path) {
            Ok(_) => read_json_artifact(&exported_agent_path, AGENT_RECEIPT_MAX_BYTES)
                .unwrap_or(Value::Null),
            Err(_) => {
                issues.push(evidence_export_issue(
                    "export.agent_receipt_copy_failed",
                    "agent_receipt",
                    "readable agent receipt fixture",
                    "failed",
                ));
                Value::Null
            }
        }
    } else {
        match export_agent_receipt_bound_to_replay(
            &agent_path,
            &exported_agent_path,
            &replay_verification,
            &policy,
        ) {
            Ok(receipt) => receipt,
            Err(_) => {
                issues.push(evidence_export_issue(
                    "export.agent_receipt_generate_failed",
                    "agent_receipt",
                    "agent receipt bound to exported replay evidence",
                    "failed",
                ));
                Value::Null
            }
        }
    };

    let bundle = json!({
        "agent_receipt_path": "agent-receipt.json",
        "replay_receipt_path": "replay-agent.json",
        "replay_verification_path": "replay-verification.json",
        "release_readiness_path": "release-readiness.json",
        "coverage_audit_path": "coverage-audit.json",
        "agent_receipt_hash": receipt_stated_or_computed_hash(&root, &exported_agent_receipt),
        "replay_receipt_hash": receipt_stated_or_computed_hash(&root, &replay),
        "replay_verification_receipt_hash": receipt_stated_or_computed_hash(&root, &replay_verification),
        "release_readiness_receipt_hash": receipt_stated_or_computed_hash(&root, &release),
        "coverage_audit_receipt_hash": receipt_stated_or_computed_hash(&root, &coverage),
    });
    write_evidence_json(&bundle_path, &bundle, &mut issues, "agent_evidence_bundle");

    let bundle_verification = agent_evidence_bundle_verification_receipt(&root, &bundle_path);
    write_evidence_json(
        &bundle_verification_path,
        &bundle_verification,
        &mut issues,
        "agent_evidence_bundle_verification",
    );

    let manifest_options = EvidenceManifestGenerateOptions {
        root: root.clone(),
        bundle: Some(bundle_path.clone()),
        agent_receipt: Some(exported_agent_path.clone()),
        replay: Some(replay_path.clone()),
        replay_verification: Some(replay_verification_path.clone()),
        release_readiness: Some(release_path.clone()),
        coverage_audit: Some(coverage_path.clone()),
        registry: None,
        registry_verification: None,
        registry_selection: None,
        registry_fetch_plan: None,
        signer_metadata: None,
        anchor_metadata: None,
        out: None,
    };
    let (manifest, manifest_issues) = evidence_manifest_generate_manifest(&manifest_options);
    issues.extend(
        manifest_issues
            .into_iter()
            .map(|issue| evidence_export_nested_issue("export.manifest_generate_failed", issue)),
    );
    let manifest = manifest.unwrap_or(Value::Null);
    if manifest.is_object() {
        let relative_manifest = evidence_manifest_with_relative_paths(&manifest, &out_dir);
        write_evidence_json(
            &manifest_path,
            &relative_manifest,
            &mut issues,
            "evidence_manifest",
        );
    }

    let manifest_verification = evidence_manifest_verification_receipt(&root, &manifest_path);
    write_evidence_json(
        &manifest_verification_path,
        &manifest_verification,
        &mut issues,
        "evidence_manifest_verification",
    );

    if bundle_verification.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_export_issue(
            "export.bundle_verification_failed",
            "agent_evidence_bundle_verification",
            "valid bundle verification receipt",
            "invalid",
        ));
    }
    if manifest_verification.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_export_issue(
            "export.manifest_verification_failed",
            "evidence_manifest_verification",
            "valid manifest verification receipt",
            "invalid",
        ));
    }

    let mut artifacts = evidence_export_artifacts(&out_dir, &mut issues);
    artifacts.sort_by_key(|artifact| {
        artifact
            .get("role")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string()
    });
    let manifest_hash = manifest_verification
        .get("manifest_hash")
        .cloned()
        .unwrap_or(Value::Null);
    let manifest_verification_hash = receipt_stated_or_computed_hash(&root, &manifest_verification);
    let receipt = evidence_export_final_receipt(
        &root,
        &out_dir,
        issues,
        artifacts,
        manifest_hash,
        manifest_verification_hash,
    );
    write_evidence_json(
        &out_dir.join("evidence-export-receipt.json"),
        &receipt,
        &mut Vec::new(),
        "evidence_export_receipt",
    );
    receipt
}

fn export_agent_receipt_bound_to_replay(
    template_path: &Path,
    output_path: &Path,
    replay_verification: &Value,
    policy: &Policy,
) -> Result<Value, Box<dyn Error>> {
    let mut receipt = read_json_artifact(template_path, AGENT_RECEIPT_MAX_BYTES)?;
    let evidence = json!({
        "replay_artifact_path": "replay-agent.json",
        "replay_receipt_hash": replay_verification
            .get("replay_receipt_hash")
            .cloned()
            .unwrap_or(Value::Null),
        "replay_policy_hash": replay_verification
            .get("replay_policy_hash")
            .cloned()
            .unwrap_or(Value::Null),
        "replay_environment_attestation_hash": replay_verification
            .get("environment_attestation_hash")
            .cloned()
            .unwrap_or(Value::Null),
        "replay_selected_cases": replay_verification
            .get("selected_cases")
            .cloned()
            .unwrap_or_else(|| json!([])),
        "replay_selected_groups": replay_verification
            .get("selected_groups")
            .cloned()
            .unwrap_or_else(|| json!([])),
        "replay_verdict": if replay_verification.get("valid").and_then(Value::as_bool) == Some(true) { "pass" } else { "fail" },
    });
    receipt["replay_evidence"] = evidence;
    if let Some(object) = receipt.as_object_mut() {
        object.remove("receipt_hash");
    }
    let validation = agent_receipt_validation_receipt(
        "agent-receipt validate",
        Some(output_path.to_path_buf()),
        &receipt,
        false,
        policy,
    );
    receipt["receipt_hash"] = validation
        .get("agent_receipt_hash")
        .cloned()
        .unwrap_or(Value::Null);
    let final_validation = agent_receipt_validation_receipt(
        "agent-receipt validate",
        Some(output_path.to_path_buf()),
        &receipt,
        false,
        policy,
    );
    if final_validation.get("valid").and_then(Value::as_bool) != Some(true) {
        return Err("generated agent receipt failed validation".into());
    }
    write_evidence_json(output_path, &receipt, &mut Vec::new(), "agent_receipt");
    Ok(receipt)
}

fn evidence_manifest_with_relative_paths(manifest: &Value, out_dir: &Path) -> Value {
    let mut value = manifest.clone();
    if let Some(entries) = value.get_mut("artifacts").and_then(Value::as_array_mut) {
        for entry in entries {
            if let Some(path_text) = entry.get("path").and_then(Value::as_str) {
                let path = PathBuf::from(path_text);
                if let Ok(relative) = path.strip_prefix(out_dir) {
                    entry["path"] = json!(path_string(relative));
                }
            }
        }
    }
    value
}

fn evidence_export_artifacts(out_dir: &Path, issues: &mut Vec<Value>) -> Vec<Value> {
    let roles = [
        ("agent_evidence_bundle", "agent-evidence-bundle.json", true),
        (
            "agent_evidence_bundle_verification",
            "agent-evidence-bundle-verification.json",
            true,
        ),
        ("agent_receipt", "agent-receipt.json", true),
        ("coverage_audit", "coverage-audit.json", true),
        ("evidence_manifest", "evidence-manifest.json", true),
        (
            "evidence_manifest_verification",
            "evidence-manifest-verification.json",
            true,
        ),
        ("release_readiness", "release-readiness.json", true),
        ("replay_receipt", "replay-agent.json", true),
        ("replay_verification", "replay-verification.json", true),
    ];
    roles
        .iter()
        .map(|(role, relative, required)| {
            let path = out_dir.join(relative);
            let hash = match fs::read(&path) {
                Ok(bytes) => Value::String(blake3_hex(&bytes)),
                Err(_) => {
                    issues.push(evidence_export_issue(
                        "export.artifact_missing",
                        role,
                        "generated artifact file",
                        "missing",
                    ));
                    Value::Null
                }
            };
            json!({
                "role": role,
                "path": relative,
                "hash": hash,
                "required": required,
            })
        })
        .collect()
}

fn write_evidence_json(path: &Path, value: &Value, issues: &mut Vec<Value>, role: &str) {
    if let Some(parent) = path.parent() {
        if fs::create_dir_all(parent).is_err() {
            issues.push(evidence_export_issue(
                "export.artifact_parent_create_failed",
                role,
                "writable parent directory",
                "failed",
            ));
            return;
        }
    }
    if fs::write(
        path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(value).unwrap_or_default()
        ),
    )
    .is_err()
    {
        issues.push(evidence_export_issue(
            "export.artifact_write_failed",
            role,
            "writable JSON artifact",
            "failed",
        ));
    }
}

fn evidence_export_nested_issue(code: &str, issue: Value) -> Value {
    json!({
        "code": code,
        "field_path": issue.get("field_path").and_then(Value::as_str).unwrap_or("nested"),
        "expected": issue.get("expected").and_then(Value::as_str).unwrap_or("valid nested receipt"),
        "actual": issue.get("actual").and_then(Value::as_str).unwrap_or("invalid"),
    })
}

fn evidence_export_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

fn evidence_export_final_receipt(
    root: &Path,
    _out_dir: &Path,
    mut issues: Vec<Value>,
    artifacts: Vec<Value>,
    manifest_hash: Value,
    manifest_verification_receipt_hash: Value,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence export",
            "root": path_string(root),
            "output": "<out>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "artifacts": artifacts,
            "manifest_hash": manifest_hash,
            "manifest_verification_receipt_hash": manifest_verification_receipt_hash,
        }),
    )
}
