use super::*;

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

pub(super) fn run_evidence_index(args: Vec<String>) -> Result<(), Box<dyn Error>> {
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

pub(super) fn run_evidence_index_verify(args: Vec<String>) -> Result<(), Box<dyn Error>> {
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
