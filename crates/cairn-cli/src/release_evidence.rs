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

use cairn_core::canonical_json_hash;
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

pub(crate) fn run_release_evidence(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("release-evidence requires a subcommand: index".into());
    };
    match subcommand.as_str() {
        "index" => run_release_evidence_index(iter.collect()),
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
