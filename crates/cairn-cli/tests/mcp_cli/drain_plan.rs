use std::{env, error::Error, fs};

use super::common::{assert_hash_field, copy_dir_recursive, repo_root, run_json};

#[test]
fn mcp_drain_plan_reports_blockers_without_mutation() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.mcp_drain_all.plan.read_only]
    // r[verify cairn.mcp_drain_all.plan.readiness]
    // r[verify cairn.mcp_drain_all.plan.next_actions]
    // r[verify cairn.mcp_drain_all.plan.mutation_boundary]
    let value = run_json(&[
        "mcp",
        "--call",
        "cairn_drain_plan",
        r#"{"root":"fixtures/basic"}"#,
    ])?;
    assert_eq!(
        value,
        run_json(&[
            "mcp",
            "--call",
            "cairn_drain_plan",
            r#"{"root":"fixtures/basic"}"#,
        ])?
    );
    let receipt = &value["structuredContent"];
    assert_eq!(receipt["command"], "mcp cairn_drain_plan");
    assert_eq!(receipt["mcp"]["read_only"], true);
    assert_eq!(receipt["mcp"]["destructive"], false);
    assert_eq!(receipt["active_change_count"], 1);
    assert_eq!(receipt["ready_change_count"], 0);
    let change = &receipt["changes"][0];
    assert_eq!(change["name"], "demo");
    assert_eq!(change["drain_ready"], false);
    assert_eq!(change["task_counts"]["todo"], 2);
    assert_eq!(change["gates"]["proposal"]["valid"], true);
    assert_eq!(change["gates"]["design"]["valid"], true);
    assert_eq!(change["gates"]["tasks"]["valid"], true);
    assert!(change["blockers"]
        .as_array()
        .ok_or("blockers")?
        .iter()
        .any(|blocker| blocker["class"] == "unfinished_tasks"));
    assert!(change["next_actions"]
        .as_array()
        .ok_or("next_actions")?
        .iter()
        .any(|action| action["action"]
            == "complete implementation tasks and mark tasks.md checkboxes"));
    assert!(receipt["next_actions"]
        .as_array()
        .ok_or("root next_actions")?
        .iter()
        .all(|action| action["requires_allow_mutations"].as_bool() != Some(true)));
    assert_hash_field(receipt, "receipt_hash")?;
    Ok(())
}

#[test]
fn mcp_drain_plan_recommends_explicit_mutation_tools_for_ready_changes(
) -> Result<(), Box<dyn Error>> {
    // r[verify cairn.mcp_drain_all.plan.mutation_boundary]
    let temp_root = env::temp_dir().join(format!("cairn-mcp-drain-ready-{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_root);
    copy_dir_recursive(&repo_root().join("fixtures/basic"), &temp_root)?;
    let tasks_path = temp_root.join("cairn/changes/demo/tasks.md");
    let tasks = fs::read_to_string(&tasks_path)?.replace("- [ ]", "- [x]");
    fs::write(&tasks_path, tasks)?;

    let request = format!(r#"{{"root":"{}"}}"#, temp_root.display());
    let value = run_json(&["mcp", "--call", "cairn_drain_plan", &request])?;
    let receipt = &value["structuredContent"];
    assert_eq!(receipt["ready_change_count"], 1);
    let change = &receipt["changes"][0];
    assert_eq!(change["drain_ready"], true);
    let actions = change["next_actions"].as_array().ok_or("next_actions")?;
    assert!(actions
        .iter()
        .any(|action| action["tool"] == "cairn_sync_execute"
            && action["requires_allow_mutations"] == true));
    assert!(actions
        .iter()
        .any(|action| action["tool"] == "cairn_archive_execute"
            && action["requires_allow_mutations"] == true));
    fs::remove_dir_all(&temp_root)?;
    Ok(())
}
