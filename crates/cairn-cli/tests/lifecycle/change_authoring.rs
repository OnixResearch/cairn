use std::{error::Error, fs};

use super::support::{cairn, run_json, temp_fixture, temp_project};

#[test]
fn init_creates_native_layout_and_is_idempotent() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.init] r[verify cairn.supersede.init.empty] r[verify cairn.supersede.init.idempotent]
    // r[verify cairn.supersede.native_layout] r[verify cairn.supersede.native_layout.init]
    // r[verify cairn.supersede.native_layout.init_cairn]
    let root = temp_project("init")?;
    let root_text = root.to_str().ok_or("utf8 root")?;

    let first = run_json(&["init", "--root", root_text])?;
    assert_eq!(first["command"], "init");
    assert_eq!(first["mutated"], true);
    assert_eq!(first["layout"], "cairn");
    assert!(root.join("cairn/specs").is_dir());
    assert!(root.join("cairn/archive").is_dir());
    assert_eq!(first["created"].as_array().ok_or("created")?.len(), 4);

    let second = run_json(&["init", "--root", root_text])?;
    assert_eq!(second["mutated"], false);
    assert_eq!(second["created"].as_array().ok_or("created")?.len(), 0);
    assert_eq!(second["existing"].as_array().ok_or("existing")?.len(), 4);
    Ok(())
}

#[test]
fn change_create_creates_compatible_skeleton() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.change_create] r[verify cairn.supersede.change_create.skeleton]
    let root = temp_project("change-create")?;
    let root_text = root.to_str().ok_or("utf8 root")?;
    run_json(&["init", "--root", root_text])?;

    let receipt = run_json(&["change", "create", "replace-openspec", "--root", root_text])?;
    assert_eq!(receipt["command"], "change_create");
    assert_eq!(receipt["change"], "replace-openspec");
    assert_eq!(receipt["mutated"], true);
    assert_eq!(receipt["layout"], "cairn");
    assert!(root
        .join("cairn/changes/replace-openspec/proposal.md")
        .is_file());
    assert!(root
        .join("cairn/changes/replace-openspec/design.md")
        .is_file());
    assert!(root
        .join("cairn/changes/replace-openspec/tasks.md")
        .is_file());
    assert!(root.join("cairn/changes/replace-openspec/specs").is_dir());

    let status = run_json(&["status", "--root", root_text])?;
    assert_eq!(status["changes"][0]["name"], "replace-openspec");
    assert_eq!(status["changes"][0]["has_proposal"], true);
    assert_eq!(status["changes"][0]["task_counts"]["total"], 2);
    Ok(())
}

#[test]
fn change_create_rejects_invalid_name_without_mutation() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.change_create.invalid_name]
    let root = temp_project("change-invalid")?;
    let output = cairn()
        .args([
            "change",
            "create",
            "bad/name",
            "--root",
            root.to_str().ok_or("utf8 root")?,
        ])
        .output()?;
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid change name"), "{stderr}");
    assert!(!root.join("cairn/changes").exists());
    Ok(())
}

#[test]
fn change_metadata_is_validated_and_filterable() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.change_metadata.schema.valid_metadata]
    // r[verify cairn.supersede.change_metadata.schema.optional_metadata]
    // r[verify cairn.supersede.change_metadata.validation.invalid_metadata]
    // r[verify cairn.supersede.change_metadata.read_receipts.list_show]
    // r[verify cairn.supersede.change_metadata.filters.group]
    // r[verify cairn.supersede.change_metadata.filters.target_date]
    // r[verify cairn.supersede.change_metadata.verification]
    // r[verify cairn.supersede.change_metadata.verification.gates]
    let fixture = temp_fixture("change-metadata", "fixtures/basic")?;
    let root = fixture.to_str().ok_or("utf8 fixture path")?;
    fs::write(
        fixture.join("cairn/changes/demo/metadata.json"),
        "{\"groups\":[\"feature:registry\",\"current\"],\"target_date\":\"2026-06-01\",\"status\":\"planned\"}\n",
    )?;

    let filtered = run_json(&[
        "change",
        "list",
        "--root",
        root,
        "--group",
        "feature:registry",
        "--target-after",
        "2026-05-01",
        "--target-before",
        "2026-07-01",
    ])?;
    assert_eq!(filtered["changes"].as_array().ok_or("changes")?.len(), 1);
    assert_eq!(filtered["changes"][0]["metadata"]["groups"][0], "current");
    assert_eq!(
        filtered["changes"][0]["metadata"]["groups"][1],
        "feature:registry"
    );
    assert_eq!(
        filtered["changes"][0]["metadata"]["target_date"],
        "2026-06-01"
    );

    let excluded = run_json(&["change", "list", "--root", root, "--group", "future"])?;
    assert!(excluded["changes"].as_array().ok_or("changes")?.is_empty());

    fs::write(
        fixture.join("cairn/changes/demo/metadata.json"),
        "{\"groups\":[\"unknown\"],\"target_date\":\"not-a-date\",\"status\":\"unknown\"}\n",
    )?;
    let invalid = run_json(&["validate", "--root", root])?;
    assert_eq!(invalid["valid"], false);
    let issues = invalid["change_issues"].as_array().ok_or("change_issues")?;
    assert_eq!(issues.len(), 3);
    let issue_text = serde_json::to_string(issues)?;
    assert!(
        issue_text.contains("metadata group unknown"),
        "{issue_text}"
    );
    assert!(issue_text.contains("metadata target_date"), "{issue_text}");
    assert!(
        issue_text.contains("metadata status unknown"),
        "{issue_text}"
    );
    fs::remove_dir_all(fixture)?;
    Ok(())
}
