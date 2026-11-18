use serde_json::{json, Value};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn cairn() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cairn"));
    command.current_dir(repo_root());
    command
}

fn run_json(args: &[&str]) -> Result<Value, Box<dyn Error>> {
    let output = cairn().args(args).output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(serde_json::from_slice(&output.stdout)?)
}

fn run_error(args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = cairn().args(args).output()?;
    assert!(!output.status.success(), "expected failure for {args:?}");
    Ok(String::from_utf8_lossy(&output.stderr).to_string())
}

fn assert_receipt_hash(value: &Value) -> Result<(), Box<dyn Error>> {
    let hash = value["receipt_hash"].as_str().ok_or("receipt_hash")?;
    assert_eq!(hash.len(), 64);
    assert!(hash.chars().all(|character| character.is_ascii_hexdigit()));
    Ok(())
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn temp_root(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("cairn-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    root
}

// r[verify cairn.agent_workflow_readiness.compatibility]
#[test]
fn legacy_drain_receipt_is_byte_compatible_and_the_direct_route_aliases_it(
) -> Result<(), Box<dyn Error>> {
    let default = run_json(&[
        "mcp",
        "--call",
        "cairn_drain_plan",
        r#"{"root":"fixtures/basic"}"#,
    ])?;
    let explicit = run_json(&[
        "mcp",
        "--call",
        "cairn_drain_plan",
        r#"{"root":"fixtures/basic","view":"legacy"}"#,
    ])?;
    assert_eq!(default, explicit);
    let receipt = &default["structuredContent"];
    assert_eq!(receipt["command"], "mcp cairn_drain_plan");
    assert!(receipt.get("schema").is_none());
    assert!(receipt.get("view").is_none());
    assert_eq!(receipt["active_change_count"], 1);
    assert_eq!(receipt["changes"][0]["drain_ready"], false);
    assert_receipt_hash(receipt)?;

    let cli = run_json(&["drain-plan", "--root", "fixtures/basic"])?;
    assert_eq!(cli["command"], "cairn drain-plan");
    assert_receipt_hash(&cli)?;
    let mut cli_compare = cli.clone();
    let mut mcp_compare = receipt.clone();
    cli_compare.as_object_mut().ok_or("cli object")?.remove("command");
    cli_compare.as_object_mut().ok_or("cli object")?.remove("receipt_hash");
    mcp_compare
        .as_object_mut()
        .ok_or("mcp object")?
        .remove("command");
    mcp_compare
        .as_object_mut()
        .ok_or("mcp object")?
        .remove("receipt_hash");
    assert_eq!(cli_compare, mcp_compare);
    Ok(())
}

// r[verify cairn.agent_workflow_readiness.shared]
#[test]
fn readiness_view_shares_direct_reports_and_keeps_missing_evidence_unavailable(
) -> Result<(), Box<dyn Error>> {
    let direct = run_json(&["readiness", "demo", "--root", "fixtures/basic"])?;
    let view = run_json(&[
        "drain-plan",
        "--root",
        "fixtures/basic",
        "--view",
        "readiness",
    ])?;
    assert_eq!(view["schema"], "cairn.drain-readiness.v1");
    assert_eq!(view["view"], "readiness");
    assert_eq!(view["manifest"]["declared"], false);
    assert_eq!(view["mcp"]["read_only"], true);
    assert_eq!(view["mcp"]["destructive"], false);
    assert_receipt_hash(&view)?;
    let change = &view["changes"][0];
    assert_eq!(change["name"], "demo");
    assert_eq!(change["readiness"], direct);
    assert_eq!(change["readiness"]["valid"], true);
    for stage in 1..4 {
        assert_eq!(change["readiness"]["stages"][stage]["state"], "ready");
    }
    assert_eq!(change["readiness"]["stages"][4]["state"], "unavailable");
    assert_eq!(change["classification"], "unavailable");
    assert_eq!(change["evidence"]["declared"], false);
    assert_eq!(change["evidence"]["completion_inputs"], false);
    assert!(change["next_task"].is_null());
    assert_eq!(view["ready_change_count"], 0);
    assert_eq!(view["unavailable_change_count"], 1);
    let actions = view["next_actions"].as_array().ok_or("next actions")?;
    assert!(!actions.is_empty());
    assert!(actions.iter().all(|action| action["authority"] == "advisory"));
    assert!(actions.iter().any(|action| action["kind"] == "unresolved"));
    assert!(actions
        .iter()
        .all(|action| action["kind"] != "sync_preview" && action["kind"] != "archive_preview"));
    assert!(actions.iter().all(|action| {
        action["argv"]
            .as_array()
            .into_iter()
            .flatten()
            .all(|part| part != "--execute")
    }));
    Ok(())
}

// r[verify cairn.agent_workflow_readiness.order]
#[test]
fn readiness_view_orders_dependencies_and_withholds_blocked_work() -> Result<(), Box<dyn Error>> {
    let root = temp_root("readiness-drain-dependencies");
    copy_dir_recursive(&repo_root().join("fixtures/basic"), &root)?;
    let demo = root.join(".cairn/changes/demo");
    let prereq = root.join(".cairn/changes/z-prereq");
    let dependent = root.join(".cairn/changes/a-dependent");
    copy_dir_recursive(&demo, &prereq)?;
    copy_dir_recursive(&demo, &dependent)?;
    fs::write(
        dependent.join("metadata.json"),
        "{\"depends_on\":[\"z-prereq\"]}\n",
    )?;
    let marked_tasks = "## Phase 1\n\n- [ ] [serial] [task:build] Build r[demo.behavior]\n- [ ] [serial] [task:test] [after:build] Test r[demo.behavior.scenario]\n";
    fs::write(prereq.join("tasks.md"), marked_tasks)?;
    fs::write(dependent.join("tasks.md"), marked_tasks)?;
    fs::remove_dir_all(&demo)?;

    let request = json!({"root": root, "view": "readiness"}).to_string();
    let view = run_json(&["mcp", "--call", "cairn_drain_plan", &request])?;
    let projected = &view["structuredContent"];
    assert_eq!(projected["view"], "readiness");
    let dependent_projection = projected["changes"]
        .as_array()
        .ok_or("changes")?
        .iter()
        .find(|change| change["name"] == "a-dependent")
        .ok_or("dependent projection")?;
    assert_eq!(dependent_projection["blocked_by"], json!(["z-prereq"]));
    assert_eq!(dependent_projection["ordering_blocked"], true);
    assert_eq!(dependent_projection["task_ordering"]["valid"], true);
    assert_eq!(
        dependent_projection["task_ordering"]["ready_tasks"],
        json!(["build"])
    );
    assert!(dependent_projection["next_task"].is_null());

    let readiness = run_json(&[
        "drain-plan",
        "--root",
        root.to_str().ok_or("root")?,
        "--view",
        "readiness",
    ])?;
    assert_eq!(readiness["drain_order"], json!(["z-prereq", "a-dependent"]));
    let order = readiness["changes"]
        .as_array()
        .ok_or("changes")?
        .iter()
        .map(|change| change["name"].as_str().ok_or("name"))
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(order, vec!["z-prereq", "a-dependent"]);

    let actions = readiness["next_actions"].as_array().ok_or("actions")?;
    let declared_order = readiness["drain_order"].as_array().ok_or("order")?;
    let mut positions = Vec::with_capacity(actions.len());
    for action in actions {
        let name = action["change"].as_str().ok_or("action change")?;
        let position = declared_order
            .iter()
            .position(|candidate| candidate.as_str() == Some(name))
            .ok_or("action change order")?;
        positions.push(position);
    }
    assert!(positions.windows(2).all(|pair| pair[0] <= pair[1]));
    let blocked_actions = actions
        .iter()
        .filter(|action| action["change"] == "a-dependent")
        .collect::<Vec<_>>();
    assert!(blocked_actions
        .iter()
        .any(|action| action["kind"] == "dependency_wait"));
    assert!(blocked_actions
        .iter()
        .all(|action| action["kind"] != "sync_preview"
            && action["kind"] != "archive_preview"
            && action["kind"] != "task"));

    for name in ["z-prereq", "a-dependent"] {
        let direct = run_json(&["readiness", name, "--root", root.to_str().ok_or("root")?])?;
        let projected = readiness["changes"]
            .as_array()
            .ok_or("changes")?
            .iter()
            .find(|change| change["name"] == name)
            .ok_or("projected change")?;
        assert_eq!(projected["readiness"], direct, "{name}");
    }
    fs::remove_dir_all(root)?;
    Ok(())
}

// r[verify cairn.agent_workflow_readiness.order]
#[test]
fn readiness_view_retains_cycles_without_selecting_executable_work(
) -> Result<(), Box<dyn Error>> {
    let root = temp_root("readiness-drain-cycle");
    copy_dir_recursive(&repo_root().join("fixtures/basic/.cairn"), &root.join(".cairn"))?;
    fs::create_dir_all(root.join(".cairn/archive"))?;
    let demo = root.join(".cairn/changes/demo");
    let first = root.join(".cairn/changes/cycle-a");
    let second = root.join(".cairn/changes/cycle-b");
    copy_dir_recursive(&demo, &first)?;
    copy_dir_recursive(&demo, &second)?;
    fs::write(first.join("metadata.json"), "{\"depends_on\":[\"cycle-b\"]}\n")?;
    fs::write(second.join("metadata.json"), "{\"depends_on\":[\"cycle-a\"]}\n")?;
    let marked_tasks = "## Phase 1\n\n- [ ] [serial] [task:build] Build r[demo.behavior]\n";
    fs::write(first.join("tasks.md"), marked_tasks)?;
    fs::write(second.join("tasks.md"), marked_tasks)?;
    fs::remove_dir_all(&demo)?;

    let view = run_json(&[
        "drain-plan",
        "--root",
        root.to_str().ok_or("root")?,
        "--view",
        "readiness",
    ])?;
    assert_eq!(view["dependency_graph"]["valid"], false);
    let changes = view["changes"].as_array().ok_or("changes")?;
    assert_eq!(changes.len(), 2);
    for change in changes {
        assert_eq!(change["ordering_blocked"], true);
        assert!(change["next_task"].is_null());
        assert!(!change["dependency_issues"]
            .as_array()
            .ok_or("dependency issues")?
            .is_empty());
    }
    let actions = view["next_actions"].as_array().ok_or("actions")?;
    assert!(actions.iter().all(|action| action["kind"] != "task"
        && action["kind"] != "sync_preview"
        && action["kind"] != "archive_preview"));
    assert!(actions.iter().any(|action| action["authority"] == "advisory"));
    fs::remove_dir_all(root)?;
    Ok(())
}

// r[verify cairn.agent_workflow_readiness.order]
#[test]
fn readiness_view_selects_the_declared_ready_task_route() -> Result<(), Box<dyn Error>> {
    let root = temp_root("readiness-drain-task");
    copy_dir_recursive(&repo_root().join("fixtures/basic/.cairn"), &root.join(".cairn"))?;
    fs::create_dir_all(root.join(".cairn/archive"))?;
    fs::write(
        root.join(".cairn/changes/demo/tasks.md"),
        "## Phase 1\n\n- [ ] [serial] [task:build] Build r[demo.behavior]\n- [ ] [serial] [task:test] [after:build] Test r[demo.behavior.scenario]\n",
    )?;
    let view = run_json(&[
        "drain-plan",
        "--root",
        root.to_str().ok_or("root")?,
        "--view",
        "readiness",
    ])?;
    let change = &view["changes"][0];
    assert_eq!(change["task_ordering"]["valid"], true);
    assert_eq!(change["task_ordering"]["ready_tasks"], json!(["build"]));
    let task = &change["next_task"];
    assert_eq!(task["task"], "build");
    assert_eq!(task["instruction"]["tool"], "cairn_workflow_instructions");
    assert_eq!(
        task["instruction"]["arguments"],
        json!({"change": "demo", "artifact": "tasks"})
    );
    assert_eq!(
        task["instruction"]["argv"],
        json!(["cairn", "workflow", "instructions", "tasks", "--change", "demo"])
    );
    let first = &view["next_actions"][0];
    assert_eq!(first["change"], "demo");
    assert_eq!(first["kind"], "task");
    assert_eq!(first["target"], "build");
    assert_eq!(first["authority"], "advisory");
    assert_eq!(first["instruction"], task["instruction"]);
    assert!(first["argv"].as_array().is_some_and(Vec::is_empty));
    fs::remove_dir_all(root)?;
    Ok(())
}

// r[verify cairn.agent_workflow_readiness.compatibility]
#[test]
fn readiness_view_selects_explicit_evidence_and_rejects_stale_or_malformed_manifests(
) -> Result<(), Box<dyn Error>> {
    let root = temp_root("readiness-drain-manifest");
    copy_dir_recursive(&repo_root().join("fixtures/basic/.cairn"), &root.join(".cairn"))?;
    fs::create_dir_all(root.join(".cairn/archive"))?;
    for file in ["contract.json", "results.json"] {
        fs::copy(
            repo_root().join("fixtures/agent-completion").join(file),
            root.join(file),
        )?;
    }
    let subject = "c".repeat(64);
    let manifest = json!({
        "schema": "cairn.workflow-readiness-manifest.v1",
        "changes": [{
            "change": "demo",
            "contract": "contract.json",
            "results": "results.json",
            "subject_hash": subject,
        }],
    });
    fs::write(root.join("manifest.json"), serde_json::to_vec(&manifest)?)?;
    let view = run_json(&[
        "drain-plan",
        "--root",
        root.to_str().ok_or("root")?,
        "--view",
        "readiness",
        "--evidence-manifest",
        "manifest.json",
    ])?;
    let change = &view["changes"][0];
    assert_eq!(change["evidence"]["declared"], true);
    assert_eq!(change["evidence"]["completion_inputs"], true);
    assert_eq!(change["readiness"]["stages"][4]["state"], "ready");
    let direct = run_json(&[
        "readiness",
        "demo",
        "--root",
        root.to_str().ok_or("root")?,
        "--contract",
        "contract.json",
        "--results",
        "results.json",
        "--subject-hash",
        &subject,
    ])?;
    assert_eq!(change["readiness"], direct);

    let stale_subject = "d".repeat(64);
    fs::write(
        root.join("stale.json"),
        serde_json::to_vec(&json!({
            "schema": "cairn.workflow-readiness-manifest.v1",
            "changes": [{
                "change": "demo",
                "contract": "contract.json",
                "results": "results.json",
                "subject_hash": stale_subject,
            }],
        }))?,
    )?;
    let stale = run_json(&[
        "drain-plan",
        "--root",
        root.to_str().ok_or("root")?,
        "--view",
        "readiness",
        "--evidence-manifest",
        "stale.json",
    ])?;
    let stale_change = &stale["changes"][0];
    assert_eq!(stale_change["readiness"]["stages"][4]["state"], "blocked");
    assert!(stale["next_actions"]
        .as_array()
        .ok_or("actions")?
        .iter()
        .any(|action| action["kind"] == "evidence_refresh"));
    assert!(stale["next_actions"]
        .as_array()
        .ok_or("actions")?
        .iter()
        .all(|action| action["kind"] != "sync_preview" && action["kind"] != "archive_preview"));

    fs::write(root.join("empty-results.json"), b"[]")?;
    fs::write(
        root.join("missing.json"),
        serde_json::to_vec(&json!({
            "schema": "cairn.workflow-readiness-manifest.v1",
            "changes": [{
                "change": "demo",
                "contract": "contract.json",
                "results": "empty-results.json",
                "subject_hash": subject,
            }],
        }))?,
    )?;
    let missing = run_json(&[
        "drain-plan",
        "--root",
        root.to_str().ok_or("root")?,
        "--view",
        "readiness",
        "--evidence-manifest",
        "missing.json",
    ])?;
    let missing_change = &missing["changes"][0];
    assert_eq!(missing_change["readiness"]["stages"][4]["state"], "blocked");
    assert!(missing["next_actions"]
        .as_array()
        .ok_or("actions")?
        .iter()
        .any(|action| action["kind"] == "evidence_refresh"));
    assert!(missing["next_actions"]
        .as_array()
        .ok_or("actions")?
        .iter()
        .all(|action| action["kind"] != "sync_preview" && action["kind"] != "archive_preview"));

    let cases = [
        (
            "unknown",
            json!({"schema": "cairn.workflow-readiness-manifest.v1",
                "changes": [{"change": "absent"}]}),
            "workflow_readiness.manifest.unknown_change",
        ),
        (
            "duplicate",
            json!({"schema": "cairn.workflow-readiness-manifest.v1",
                "changes": [{"change": "demo"}, {"change": "demo"}]}),
            "workflow_readiness.manifest.duplicate_change",
        ),
        (
            "partial",
            json!({"schema": "cairn.workflow-readiness-manifest.v1",
                "changes": [{"change": "demo", "contract": "contract.json"}]}),
            "workflow_readiness.manifest.partial_completion",
        ),
        (
            "unsafe",
            json!({"schema": "cairn.workflow-readiness-manifest.v1",
                "changes": [{"change": "demo", "contract": "../outside.json",
                    "results": "results.json", "subject_hash": "c"}]}),
            "workflow_readiness.manifest.unsafe_path",
        ),
        (
            "schema",
            json!({"schema": "cairn.workflow-readiness-manifest.v0",
                "changes": [{"change": "demo"}]}),
            "workflow_readiness.manifest.schema",
        ),
        (
            "unknown-field",
            json!({"schema": "cairn.workflow-readiness-manifest.v1",
                "changes": [{"change": "demo", "unexpected": true}]}),
            "workflow_readiness.manifest.invalid",
        ),
    ];
    for (name, manifest, code) in cases {
        let path = root.join(format!("{name}.json"));
        fs::write(&path, serde_json::to_vec(&manifest)?)?;
        let error = run_error(&[
            "drain-plan",
            "--root",
            root.to_str().ok_or("root")?,
            "--view",
            "readiness",
            "--evidence-manifest",
            path.file_name().and_then(|name| name.to_str()).ok_or("path")?,
        ])?;
        assert!(error.contains(code), "{name}: {error}");
        assert!(!error.contains("--execute"));
    }
    let error = run_error(&[
        "drain-plan",
        "--root",
        root.to_str().ok_or("root")?,
        "--evidence-manifest",
        "manifest.json",
    ])?;
    assert!(error.contains("--evidence-manifest requires the readiness view"));
    let error = run_error(&[
        "drain-plan",
        "--root",
        root.to_str().ok_or("root")?,
        "--view",
        "readiness",
        "--cross-repo-evidence",
        "upstream.json",
    ])?;
    assert!(error.contains("--evidence-manifest"));
    fs::remove_dir_all(root)?;
    Ok(())
}
