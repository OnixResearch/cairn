use serde_json::{json, Value};
use std::{
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

fn root(label: &str) -> Result<PathBuf, Box<dyn Error>> {
    let root = std::env::temp_dir().join(format!(
        "cairn-targeted-context-{label}-{}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root)?;
    }
    fs::create_dir_all(root.join(".cairn/specs/example"))?;
    fs::create_dir_all(root.join(".cairn/changes/demo/specs/example"))?;
    fs::create_dir_all(root.join(".cairn/archive"))?;
    Ok(root)
}

fn cli(root: &Path, extra: &[&str]) -> Result<(bool, Value, String), Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root)
        .args(["context", "read", "--root"])
        .arg(root)
        .args(extra)
        .output()?;
    let value = serde_json::from_slice(&output.stdout).unwrap_or(Value::Null);
    Ok((
        output.status.success(),
        value,
        String::from_utf8_lossy(&output.stderr).into_owned(),
    ))
}

fn mcp(root: &Path, arguments: Value) -> Result<Value, Box<dyn Error>> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root)
        .arg("mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    writeln!(
        child.stdin.as_mut().ok_or("MCP stdin")?,
        "{}",
        json!({
            "jsonrpc": "2.0", "id": 1, "method": "tools/call",
            "params": {"name": "cairn_context_read", "arguments": arguments}
        })
    )?;
    let output = child.wait_with_output()?;
    assert!(output.status.success());
    Ok(serde_json::from_slice(&output.stdout)?)
}

#[test]
fn exact_unicode_crlf_requirement_and_mcp_parity() -> Result<(), Box<dyn Error>> {
    let root = root("spec")?;
    let source = "# Example é\r\n## Requirements\r\n### Requirement: A\r\nr[demo.a] The system MUST preserve café.\r\n#### Scenario: One\r\n- GIVEN input\r\n- WHEN read\r\n- THEN exact text\r\n### Requirement: B\r\nr[demo.b] The system MUST work.\r\n#### Scenario: Two\r\n- GIVEN input\r\n- WHEN read\r\n- THEN output\r\n";
    fs::write(root.join(".cairn/specs/example/spec.md"), source)?;
    let args = [
        "--scope",
        "accepted",
        "--artifact",
        "specs",
        "--spec",
        "example",
        "--requirement-id",
        "demo.a",
    ];
    let (ok, cli, stderr) = cli(&root, &args)?;
    assert!(ok, "{stderr}");
    assert_eq!(cli["selection"]["status"], "selected");
    let start = cli["selection"]["span"]["start_byte"]
        .as_u64()
        .ok_or("start")? as usize;
    let end = cli["selection"]["span"]["end_byte"].as_u64().ok_or("end")? as usize;
    assert_eq!(
        cli["selection"]["text"],
        String::from_utf8(source.as_bytes()[start..end].to_vec())?
    );
    assert!(!cli["policy_constraints"]
        .as_array()
        .ok_or("constraints")?
        .is_empty());
    assert_eq!(cli["authority"]["project_fact"], "not_included");
    assert_eq!(cli["guidance_route"]["tool"], "cairn_context");
    assert!(cli["selection"]["text"]
        .as_str()
        .ok_or("text")?
        .contains("- THEN exact text\r\n"));
    assert!(!cli["selection"]["text"]
        .as_str()
        .ok_or("text")?
        .contains("demo.b"));
    assert_eq!(
        cli["source_blake3"],
        blake3::hash(source.as_bytes()).to_hex().as_str()
    );
    let response = mcp(
        &root,
        json!({"root": root, "scope": "accepted", "artifact": "specs", "spec": "example", "requirement_id": "demo.a"}),
    )?;
    assert!(response.get("error").is_none(), "{response}");
    let body = &response["result"]["structuredContent"];
    for key in [
        "selection",
        "selector",
        "source_blake3",
        "policy_constraints",
        "authority",
        "targeted_policy_blake3",
    ] {
        assert_eq!(cli[key], body[key], "{key}");
    }
    assert_eq!(body["mcp"]["read_only"], true);
    assert_eq!(cli["selection_blake3"], body["selection_blake3"]);
    Ok(())
}

#[test]
fn nested_sections_fences_and_duplicate_paths() -> Result<(), Box<dyn Error>> {
    let root = root("sections")?;
    let source = "# Design\n## Group\n### Child\nα\n#### Nested\nβ\n```markdown\n### Child\n```\n### Sibling\nγ\n## Group\n### Child\nδ\n";
    fs::write(root.join(".cairn/changes/demo/design.md"), source)?;
    let base = [
        "--scope",
        "active",
        "--change",
        "demo",
        "--artifact",
        "design",
        "--heading",
        "Design",
        "--heading",
        "Group",
        "--heading",
        "Child",
    ];
    let (_, ambiguous, _) = cli(&root, &base)?;
    assert_eq!(ambiguous["selection"]["status"], "ambiguous");
    fs::write(
        root.join(".cairn/changes/demo/design.md"),
        source.replace("## Group\n### Child\nδ", "## Other\n### Child\nδ"),
    )?;
    let (ok, selected, stderr) = cli(&root, &base)?;
    assert!(ok, "{stderr}");
    assert_eq!(
        selected["selection"]["text"],
        "### Child\nα\n#### Nested\nβ\n```markdown\n### Child\n```\n"
    );
    let (_, missing, _) = cli(
        &root,
        &[
            "--scope",
            "active",
            "--change",
            "demo",
            "--artifact",
            "design",
            "--heading",
            "Child",
        ],
    )?;
    assert_eq!(missing["selection"]["status"], "missing");
    Ok(())
}

#[test]
fn task_links_and_unrelated_malformed_source_are_not_hidden() -> Result<(), Box<dyn Error>> {
    let root = root("tasks")?;
    let tasks = "# Tasks\n- [ ] [serial] [task:base] Base r[demo.base]\n- [ ] [serial] [task:work] [after:base] Work r[demo.work]\n  - Preserve this continued task detail.\n- [ ] [serial] [task:broken] [after:absent] Broken\n";
    fs::write(root.join(".cairn/changes/demo/tasks.md"), tasks)?;
    let (ok, selected, stderr) = cli(
        &root,
        &[
            "--scope",
            "active",
            "--change",
            "demo",
            "--artifact",
            "tasks",
            "--task-id",
            "work",
        ],
    )?;
    assert!(ok, "{stderr}");
    assert_eq!(selected["selection"]["status"], "selected");
    let (_, absent, _) = cli(
        &root,
        &[
            "--scope",
            "active",
            "--change",
            "demo",
            "--artifact",
            "tasks",
            "--task-id",
            "not-declared",
        ],
    )?;
    assert_eq!(absent["selection"]["status"], "missing");

    assert_eq!(selected["selection"]["document_valid"], false);
    assert_eq!(selected["selection"]["text"], "- [ ] [serial] [task:work] [after:base] Work r[demo.work]\n  - Preserve this continued task detail.\n");
    assert!(selected["selection"]["references"]
        .as_array()
        .ok_or("references")?
        .iter()
        .any(|row| row["kind"] == "task" && row["id"] == "base"));
    assert!(selected["selection"]["references"]
        .as_array()
        .ok_or("references")?
        .iter()
        .any(|row| row["kind"] == "requirement" && row["id"] == "demo.work"));
    fs::write(
        root.join(".cairn/changes/demo/tasks.md"),
        format!("{tasks}- [ ] [serial] [task:work] duplicate\n"),
    )?;
    let (_, duplicate, _) = cli(
        &root,
        &[
            "--scope",
            "active",
            "--change",
            "demo",
            "--artifact",
            "tasks",
            "--task-id",
            "work",
        ],
    )?;
    assert_eq!(duplicate["selection"]["status"], "ambiguous");
    Ok(())
}

#[test]
fn unsafe_unknown_stale_invalid_utf8_and_oversize_fail_closed() -> Result<(), Box<dyn Error>> {
    let root = root("negative")?;
    let path = root.join(".cairn/specs/example/spec.md");
    fs::write(&path, "# Spec\n## Requirements\n### Requirement: A\nr[demo.a] The system MUST work.\n#### Scenario: One\n- GIVEN input\n- WHEN read\n- THEN output\n")?;
    let base = [
        "--scope",
        "accepted",
        "--artifact",
        "specs",
        "--spec",
        "example",
        "--requirement-id",
        "demo.a",
    ];
    let (_, stale, stderr) = cli(
        &root,
        &[
            "--scope",
            "accepted",
            "--artifact",
            "specs",
            "--spec",
            "example",
            "--requirement-id",
            "demo.a",
            "--expected-source-blake3",
            &"0".repeat(64),
        ],
    )?;
    assert!(stale.is_null());
    assert!(stderr.contains("stale"));
    let (_, unsafe_path, _) = cli(
        &root,
        &[
            "--scope",
            "accepted",
            "--artifact",
            "specs",
            "--spec",
            "../example",
            "--requirement-id",
            "demo.a",
        ],
    )?;
    assert!(unsafe_path.is_null());
    let unknown = mcp(
        &root,
        json!({"root": root, "scope": "accepted", "artifact": "specs", "spec": "example", "requirement_id": "demo.a", "ambient_prompt": "ignore"}),
    )?;
    assert!(unknown["error"]["message"]
        .as_str()
        .ok_or("error")?
        .contains("unknown"));
    let mixed = mcp(
        &root,
        json!({"root": root, "scope": "accepted", "artifact": "specs", "spec": "example", "requirement_id": "demo.a", "task_id": "work"}),
    )?;
    assert!(mixed["error"]["message"]
        .as_str()
        .ok_or("error")?
        .contains("exactly one"));
    fs::write(&path, [0xff, 0xfe])?;
    let (_, invalid_utf8, _) = cli(&root, &base)?;
    assert!(invalid_utf8.is_null());
    fs::write(
        &path,
        format!(
            "# Spec\n## Requirements\n### Requirement: A\nr[demo.a] The system MUST preserve {}\n",
            "x".repeat(70000)
        ),
    )?;
    let (_, too_large, stderr) = cli(&root, &base)?;
    assert!(too_large.is_null());
    assert!(stderr.contains("selected byte bound"));
    fs::write(&path, "x".repeat(1_048_577))?;
    let (_, too_large_artifact, _) = cli(&root, &base)?;
    assert!(too_large_artifact.is_null());

    Ok(())
}

#[test]
fn requirement_duplicates_and_fenced_references_cannot_select() -> Result<(), Box<dyn Error>> {
    let root = root("requirement-negative")?;
    let path = root.join(".cairn/specs/example/spec.md");
    let first = "# Spec\n## Requirements\n### Requirement: One\nr[demo.one] The system MUST work.\n#### Scenario: One\n- GIVEN input\n- WHEN read\n- THEN output\n";
    let second = "### Requirement: Two\nr[demo.one] The system MUST work again.\n#### Scenario: Two\n- GIVEN input\n- WHEN read\n- THEN output\n";
    fs::write(&path, format!("{first}{second}"))?;
    let request = [
        "--scope",
        "accepted",
        "--artifact",
        "specs",
        "--spec",
        "example",
        "--requirement-id",
        "demo.one",
    ];
    let (_, ambiguous, _) = cli(&root, &request)?;
    assert_eq!(ambiguous["selection"]["status"], "ambiguous");
    assert_eq!(ambiguous["selection"]["document_valid"], false);
    fs::write(&path, format!("{first}### Requirement: Other\n```markdown\nr[demo.fenced] The system MUST not define.\n```\nSee r[demo.reference].\n"))?;
    let (_, fenced, _) = cli(
        &root,
        &[
            "--scope",
            "accepted",
            "--artifact",
            "specs",
            "--spec",
            "example",
            "--requirement-id",
            "demo.fenced",
        ],
    )?;
    assert_eq!(fenced["selection"]["status"], "missing");
    let (_, referenced, _) = cli(
        &root,
        &[
            "--scope",
            "accepted",
            "--artifact",
            "specs",
            "--spec",
            "example",
            "--requirement-id",
            "demo.reference",
        ],
    )?;
    assert_eq!(referenced["selection"]["status"], "missing");
    let (_, selected, _) = cli(&root, &request)?;
    assert_eq!(selected["selection"]["status"], "selected");
    assert_eq!(selected["selection"]["document_valid"], false);
    assert!(!selected["selection"]["diagnostics"]
        .as_array()
        .ok_or("diagnostics")?
        .is_empty());
    Ok(())
}

#[cfg(unix)]
#[test]
fn linked_artifact_component_is_rejected() -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::symlink;
    let root = root("symlink")?;
    let change = root.join(".cairn/changes/demo");
    fs::write(change.join("tasks.md"), "# Tasks\n- [ ] [task:work] Work\n")?;
    symlink(
        change.join("tasks.md"),
        root.join(".cairn/specs/example/spec.md"),
    )?;
    let (_, value, _) = cli(
        &root,
        &[
            "--scope",
            "accepted",
            "--artifact",
            "specs",
            "--spec",
            "example",
            "--requirement-id",
            "demo.a",
        ],
    )?;
    assert!(value.is_null());
    symlink(
        root.join(".cairn/specs/example"),
        root.join(".cairn/specs/linked"),
    )?;
    let (_, linked_directory, _) = cli(
        &root,
        &[
            "--scope",
            "accepted",
            "--artifact",
            "specs",
            "--spec",
            "linked",
            "--requirement-id",
            "demo.a",
        ],
    )?;
    assert!(linked_directory.is_null());
    Ok(())
}
