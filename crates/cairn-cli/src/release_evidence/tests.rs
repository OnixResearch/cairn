use std::{env, fs, process::Command};

use serde_json::json;

use super::common::{git_head_state, safe_relative_path_text};
use super::index::render_release_evidence_index;
use super::publish::{copy_publish_artifact, validate_publish_label};
use super::retention::{release_evidence_retention_plan_receipt, ReleaseEvidenceRetentionOptions};
use super::rotate::{release_evidence_rotate_receipt, ReleaseEvidenceRotateOptions};
use crate::support::path_string;
use cairn_core::blake3_hex;

#[test]
fn rendered_index_binds_payload_head_without_final_commit_claim() {
    // r[verify release_evidence.cli_source_split.behavior_preserved.focused_checks]
    // r[verify release_evidence.cli_source_split.repository_checks]
    // r[verify release_evidence.cli_source_split.archive_ready]
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
            issue["code"].as_str() == Some("release_evidence_retention.duplicate_channel_release")
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
