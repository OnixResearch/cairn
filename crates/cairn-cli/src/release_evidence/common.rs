use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use cairn_core::{blake3_hex, canonical_json_hash};
use serde_json::{json, Value};

use crate::evidence::write_evidence_json;

pub(super) fn git_head_state(root: &Path, issues: &mut Vec<Value>) -> Value {
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

pub(super) fn collect_invalid_receipt(role: &str, receipt: &Value, issues: &mut Vec<Value>) {
    if receipt.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(release_evidence_issue(
            &format!("release_evidence.{role}_invalid"),
            role,
            "valid existing rail receipt",
            "invalid",
        ));
    }
}

pub(super) fn write_json_artifact(path: &Path, value: &Value, issues: &mut Vec<Value>, role: &str) {
    write_evidence_json(path, value, issues, role);
}

pub(super) fn write_markdown_artifact(path: &Path, markdown: &str, issues: &mut Vec<Value>) {
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

pub(super) fn release_evidence_issue(
    code: &str,
    field_path: &str,
    expected: &str,
    actual: &str,
) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

pub(super) fn read_file_hash(
    path: &Path,
    issues: &mut Vec<Value>,
    role: &str,
    field: &str,
) -> Option<String> {
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

pub(super) fn write_json_file(path: &Path, value: &Value, issues: &mut Vec<Value>, role: &str) {
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

pub(super) fn safe_relative_path_text(relative: &str) -> bool {
    let path = PathBuf::from(relative);
    !relative.is_empty()
        && !path.is_absolute()
        && !path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
}

pub(super) fn publish_relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

pub(super) fn directory_manifest_hash(path: &Path) -> Option<String> {
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
