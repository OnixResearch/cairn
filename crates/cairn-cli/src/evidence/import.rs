use super::verify::{evidence_verify_receipt, EvidenceVerifyOptions};
use super::*;

struct EvidenceImportOptions {
    root: PathBuf,
    bundle: Option<PathBuf>,
    into: Option<PathBuf>,
}

pub(super) fn run_evidence_import(args: Vec<String>) -> Result<(), Box<dyn Error>> {
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

pub(super) const EVIDENCE_IMPORT_MAX_FILES: usize = 64;

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
