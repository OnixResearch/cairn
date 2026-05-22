use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path};

use super::issue::evidence_republish_issue;

pub(super) fn detect_output_conflicts(out_dir: &Path, issues: &mut Vec<Value>) {
    if out_dir.exists() {
        match bundle_regular_files_with_prefix(out_dir, "evidence_republish") {
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
}

pub(super) fn reject_undeclared_imported_files(
    imported_dir: &Path,
    artifacts: &[Value],
    issues: &mut Vec<Value>,
) {
    let mut declared_files = BTreeSet::new();
    declared_files.insert("evidence-import-receipt.json".to_string());
    match bundle_regular_files_with_prefix(imported_dir, "evidence_republish") {
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
}

pub(super) fn bundle_regular_files_with_prefix(
    bundle_dir: &Path,
    prefix: &'static str,
) -> Result<BTreeSet<String>, &'static str> {
    let mut out = BTreeSet::new();
    collect_bundle_regular_files_with_prefix(bundle_dir, bundle_dir, &mut out, prefix)?;
    Ok(out)
}

fn collect_bundle_regular_files_with_prefix(
    root: &Path,
    dir: &Path,
    out: &mut BTreeSet<String>,
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
            if out.len() >= super::super::import::EVIDENCE_IMPORT_MAX_FILES {
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
