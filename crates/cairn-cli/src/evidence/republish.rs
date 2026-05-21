use super::import::EVIDENCE_IMPORT_MAX_FILES;
use super::*;

struct EvidenceRepublishOptions {
    root: PathBuf,
    imported: Option<PathBuf>,
    out: Option<PathBuf>,
}

pub(super) fn run_evidence_republish(args: Vec<String>) -> Result<(), Box<dyn Error>> {
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
