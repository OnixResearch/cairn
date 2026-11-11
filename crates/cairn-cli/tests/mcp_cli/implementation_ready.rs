use std::{env, error::Error, fs, path::Path};

use super::common::{assert_hash_field, cairn, copy_dir_recursive, repo_root, run_json};

fn request(root: &Path) -> String {
    format!(r#"{{"root":"{}"}}"#, root.display())
}

#[test]
fn implementation_ready_selects_unfinished_work_while_archive_is_unready(
) -> Result<(), Box<dyn Error>> {
    // r[verify cairn.implementation_ready.projection] r[verify cairn.implementation_ready.projection.scenario.unfinished]
    let request = r#"{"root":"fixtures/basic"}"#;
    let mcp = run_json(&["mcp", "--call", "cairn_implementation_ready", request])?;
    let receipt = &mcp["structuredContent"];
    assert_eq!(receipt["schema"], "cairn.implementation-ready.v1");
    assert_eq!(receipt["outcome"], "ready");
    assert_eq!(receipt["complete"], true);
    assert_eq!(receipt["ready_change_count"], 1);
    assert_eq!(receipt["active_change_count"], 1);
    assert_hash_field(receipt, "receipt_hash")?;
    let selection = receipt["selection"].as_array().ok_or("selection")?;
    assert_eq!(selection.len(), 1);
    assert_eq!(selection[0]["unit"], "change:demo");
    assert_eq!(selection[0]["archive_ready"], false);
    assert!(selection[0]["requirement_references"]
        .as_array()
        .ok_or("requirements")?
        .iter()
        .any(|value| value == "demo.behavior"));
    let change = &receipt["changes"][0];
    assert_eq!(change["implementation_ready"], true);
    assert_eq!(change["archive_ready"], false);
    assert_eq!(change["completion_only"], false);
    assert_hash_field(change, "snapshot_digest")?;

    let cli = run_json(&["implementation-ready", "--root", "fixtures/basic"])?;
    assert_eq!(cli, receipt.clone());
    Ok(())
}

#[test]
// r[verify cairn.implementation_ready.projection.scenario.tasks]
fn implementation_ready_selects_ready_tasks_and_skips_completed_tasks() -> Result<(), Box<dyn Error>>
{
    let root = env::temp_dir().join(format!("cairn-impl-ready-tasks-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    copy_dir_recursive(&repo_root().join("fixtures/basic"), &root)?;
    let tasks_path = root.join(".cairn/changes/demo/tasks.md");
    fs::write(
        &tasks_path,
        "## Phase 1\n\n- [x] [serial] [task:build] Build r[demo.behavior]\n- [ ] [serial] [task:test] [after:build] Test r[demo.behavior.scenario]\n",
    )?;
    let value = run_json(&[
        "mcp",
        "--call",
        "cairn_implementation_ready",
        &request(&root),
    ])?;
    let receipt = &value["structuredContent"];
    assert_eq!(receipt["outcome"], "ready");
    let change = &receipt["changes"][0];
    assert_eq!(change["implementation_ready"], true);
    assert_eq!(change["archive_ready"], false);
    assert_eq!(change["completed_tasks"], serde_json::json!(["build"]));
    let selection = receipt["selection"].as_array().ok_or("selection")?;
    assert_eq!(selection.len(), 1);
    assert_eq!(selection[0]["unit"], "task:test");
    assert_eq!(
        selection[0]["requirement_references"],
        serde_json::json!(["demo.behavior.scenario"])
    );
    Ok(())
}

#[test]
// r[verify cairn.implementation_ready.projection.scenario.completion_only]
fn implementation_ready_reports_completion_only_without_dispatch() -> Result<(), Box<dyn Error>> {
    let root = env::temp_dir().join(format!("cairn-impl-ready-done-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    copy_dir_recursive(&repo_root().join("fixtures/basic"), &root)?;
    let tasks_path = root.join(".cairn/changes/demo/tasks.md");
    let tasks = fs::read_to_string(&tasks_path)?.replace("- [ ]", "- [x]");
    fs::write(&tasks_path, tasks)?;
    let value = run_json(&[
        "mcp",
        "--call",
        "cairn_implementation_ready",
        &request(&root),
    ])?;
    let receipt = &value["structuredContent"];
    assert_eq!(receipt["outcome"], "completion_only");
    assert_eq!(receipt["selection"], serde_json::json!([]));
    let change = &receipt["changes"][0];
    assert_eq!(change["implementation_ready"], false);
    assert_eq!(change["completion_only"], true);
    assert_eq!(change["archive_ready"], true);
    Ok(())
}

#[test]
// r[verify cairn.implementation_ready.projection.scenario.limit]
fn implementation_ready_bounds_enumeration_explicitly() -> Result<(), Box<dyn Error>> {
    let root = env::temp_dir().join(format!("cairn-impl-ready-limit-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    copy_dir_recursive(&repo_root().join("fixtures/basic"), &root)?;
    copy_dir_recursive(
        &root.join(".cairn/changes/demo"),
        &root.join(".cairn/changes/demo-two"),
    )?;
    let value = run_json(&[
        "mcp",
        "--call",
        "cairn_implementation_ready",
        &format!(r#"{{"root":"{}","limit":1}}"#, root.display()),
    ])?;
    let receipt = &value["structuredContent"];
    assert_eq!(receipt["active_change_count"], 2);
    assert_eq!(receipt["processed_change_count"], 1);
    assert_eq!(receipt["complete"], false);
    assert_eq!(receipt["outcome"], "incomplete_limit");
    assert_eq!(receipt["changes"].as_array().ok_or("changes")?.len(), 1);
    let cli = run_json(&[
        "implementation-ready",
        "--root",
        &root.display().to_string(),
        "--limit",
        "1",
    ])?;
    assert_eq!(cli, receipt.clone());
    Ok(())
}

#[test]
fn implementation_ready_replay_is_deterministic_and_edits_change_the_digest(
) -> Result<(), Box<dyn Error>> {
    // r[verify cairn.implementation_ready.projection]
    let root = env::temp_dir().join(format!("cairn-impl-ready-digest-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    copy_dir_recursive(&repo_root().join("fixtures/basic"), &root)?;
    let first = run_json(&[
        "mcp",
        "--call",
        "cairn_implementation_ready",
        &request(&root),
    ])?;
    let second = run_json(&[
        "mcp",
        "--call",
        "cairn_implementation_ready",
        &request(&root),
    ])?;
    assert_eq!(first, second);
    let digest = first["structuredContent"]["changes"][0]["snapshot_digest"]
        .as_str()
        .ok_or("digest")?
        .to_string();
    let tasks_path = root.join(".cairn/changes/demo/tasks.md");
    let tasks = fs::read_to_string(&tasks_path)?;
    fs::write(&tasks_path, format!("{tasks}\n<!-- reviewed edit -->\n"))?;
    let edited = run_json(&[
        "mcp",
        "--call",
        "cairn_implementation_ready",
        &request(&root),
    ])?;
    assert_ne!(
        edited["structuredContent"]["changes"][0]["snapshot_digest"]
            .as_str()
            .ok_or("edited digest")?,
        digest
    );
    Ok(())
}

#[test]
fn implementation_ready_reports_invalid_task_definitions_as_blockers() -> Result<(), Box<dyn Error>>
{
    // r[verify cairn.implementation_ready.projection]
    let root = env::temp_dir().join(format!("cairn-impl-ready-cycle-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    copy_dir_recursive(&repo_root().join("fixtures/basic"), &root)?;
    let tasks_path = root.join(".cairn/changes/demo/tasks.md");
    fs::write(
        &tasks_path,
        "## Phase 1\n\n- [ ] [serial] [task:a] [after:b] A r[demo.behavior]\n- [ ] [serial] [task:b] [after:a] B r[demo.behavior.scenario]\n",
    )?;
    let value = run_json(&[
        "mcp",
        "--call",
        "cairn_implementation_ready",
        &request(&root),
    ])?;
    let receipt = &value["structuredContent"];
    assert_eq!(receipt["outcome"], "blocked");
    assert_eq!(receipt["selection"], serde_json::json!([]));
    let change = &receipt["changes"][0];
    assert_eq!(change["implementation_ready"], false);
    assert!(change["blockers"]
        .as_array()
        .ok_or("blockers")?
        .iter()
        .any(|blocker| blocker["class"] == "task_definition"));
    Ok(())
}

#[test]
fn implementation_ready_rejects_empty_string_arguments() -> Result<(), Box<dyn Error>> {
    for request in [r#"{"root":""}"#, r#"{"policy":""}"#] {
        let output = cairn()
            .args(["mcp", "--call", "cairn_implementation_ready", request])
            .output()?;
        assert!(!output.status.success(), "{request} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("must not be empty"),
            "{request}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}
