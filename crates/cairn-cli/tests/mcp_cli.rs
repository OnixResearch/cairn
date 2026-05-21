use std::{env, error::Error, fs, io::Write, path::PathBuf, process::Command};

use serde_json::Value;

// r[impl cli_maintainability.cli_integration_tests.focused_modules]
// r[impl cli_maintainability.cli_integration_tests.focused_modules.visible_boundaries]
// r[impl cli_maintainability.cli_integration_tests.focused_modules.narrow_bridges]
// r[verify cli_maintainability.cli_integration_tests.behavior_preserved]
// r[verify cli_maintainability.cli_integration_tests.behavior_preserved.focused_checks]
// r[verify cli_maintainability.cli_integration_tests.behavior_preserved.repository_checks]
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn cairn() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cairn"));
    command.current_dir(repo_root());
    command
}

fn run_json(args: &[&str]) -> Result<Value, Box<dyn Error>> {
    let mut command = cairn();
    command.args(args);
    run_json_command(&mut command)
}

fn run_json_command(command: &mut Command) -> Result<Value, Box<dyn Error>> {
    let output = command.output()?;
    assert!(
        output.status.success(),
        "status: {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(serde_json::from_slice(&output.stdout)?)
}

fn assert_hash_field(value: &Value, field: &str) -> Result<(), Box<dyn Error>> {
    let hash = value[field].as_str().ok_or("hash field missing")?;
    assert_eq!(hash.len(), 64, "{field} should be a BLAKE3 hex hash");
    assert!(
        hash.chars().all(|character| character.is_ascii_hexdigit()),
        "{field} should be hex: {hash}"
    );
    Ok(())
}

#[test]
fn mcp_agent_smoke_receipt_proves_operator_flow() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.mcp_agent_smoke_receipts.flow]
    // r[verify cairn.supersede.mcp_agent_smoke_receipts.evidence]
    // r[verify cairn.supersede.mcp_agent_smoke_receipts.read_only]
    // r[verify cairn.supersede.mcp_agent_smoke_receipts.docs]
    // r[verify cairn.supersede.mcp_agent_smoke_receipts.verification]
    let receipt = run_json(&["agent-receipt", "smoke-mcp", "--root", "."])?;
    assert_eq!(receipt["command"], "agent-receipt smoke-mcp");
    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["verdict"], "pass");
    assert_eq!(receipt["fixture"], "fixtures/agent-receipts/positive.json");
    assert_eq!(receipt["mcp"]["read_only"], true);
    assert_eq!(receipt["mutation_denial"]["allowed"], false);
    assert_eq!(receipt["mutation_denial"]["mutated"], false);
    assert!(receipt["catalog_tool_count"].as_u64().unwrap_or_default() >= 9);
    assert!(receipt["invalid_argument_error"]
        .as_str()
        .unwrap_or_default()
        .contains("invalid arguments for cairn_gate"));
    assert_hash_field(&receipt, "receipt_hash")?;
    assert_hash_field(&receipt, "receipt_tool_schema_hash")?;
    assert_hash_field(&receipt, "agent_receipt_validation_hash")?;
    assert_hash_field(&receipt, "agent_receipt_hash")?;
    let checks = receipt["checks"].as_array().ok_or("checks")?;
    assert_eq!(checks.len(), 5);
    assert!(checks
        .iter()
        .all(|check| check["passed"].as_bool() == Some(true)));
    Ok(())
}

#[test]
fn mcp_agent_receipt_validation_is_read_only_and_bounded() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.agent_receipt_validation_surface.mcp]
    // r[verify cairn.supersede.agent_receipt_validation_surface.mcp.read_only]
    // r[verify cairn.supersede.agent_receipt_validation_surface.verification]
    let catalog = run_json(&["mcp", "--describe"])?;
    let tools = catalog["tools"].as_array().ok_or("tools")?;
    assert!(tools
        .iter()
        .any(|tool| tool["name"] == "cairn_agent_receipt_validate"
            && tool["annotations"]["readOnlyHint"] == true
            && tool["annotations"]["destructiveHint"] == false));

    let by_path = run_json(&[
        "mcp",
        "--call",
        "cairn_agent_receipt_validate",
        r#"{"root":".","path":"fixtures/agent-receipts/positive.json"}"#,
    ])?;
    let receipt = &by_path["structuredContent"];
    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["mcp"]["read_only"], true);
    assert_hash_field(receipt, "agent_receipt_hash")?;
    assert_hash_field(receipt, "receipt_hash")?;

    let inline_fixture =
        fs::read_to_string(repo_root().join("fixtures/agent-receipts/positive.json"))?;
    let inline_request = format!(r#"{{"receipt":{inline_fixture}}}"#);
    let inline = run_json(&[
        "mcp",
        "--call",
        "cairn_agent_receipt_validate",
        &inline_request,
    ])?;
    assert_eq!(inline["structuredContent"]["input_kind"], "inline");
    assert_eq!(inline["structuredContent"]["valid"], true);

    let traversal = cairn()
        .args([
            "mcp",
            "--call",
            "cairn_agent_receipt_validate",
            r#"{"root":".","path":"../secret.json"}"#,
        ])
        .output()?;
    assert!(!traversal.status.success());
    let stderr = String::from_utf8_lossy(&traversal.stderr);
    assert!(
        stderr.contains("selector escapes lifecycle bounds"),
        "{stderr}"
    );
    Ok(())
}

#[test]
fn mcp_describes_read_only_tools_without_mutations() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.mcp_server.verification.registration]
    // r[verify cairn.supersede.mcp_server.read_tools]
    // r[verify cairn.supersede.mcp_reproducibility] r[verify cairn.supersede.mcp_reproducibility.describe]
    let value = run_json(&["mcp", "--describe"])?;
    assert_eq!(value, run_json(&["mcp", "--describe"])?);
    assert_eq!(value["allow_mutations"], false);
    let tools = value["tools"].as_array().ok_or("tools")?;
    let status_tool = tools
        .iter()
        .find(|tool| tool["name"] == "cairn_status")
        .ok_or("status tool")?;
    assert_eq!(status_tool["annotations"]["readOnlyHint"], true);
    assert_eq!(status_tool["annotations"]["destructiveHint"], false);
    assert_eq!(status_tool["inputSchema"]["additionalProperties"], false);
    assert!(status_tool["inputSchema"]["properties"]["root"].is_object());

    let sync_plan_tool = tools
        .iter()
        .find(|tool| tool["name"] == "cairn_sync_plan")
        .ok_or("sync plan tool")?;
    assert_eq!(sync_plan_tool["annotations"]["readOnlyHint"], true);
    assert_eq!(sync_plan_tool["inputSchema"]["required"][0], "change");
    assert_eq!(sync_plan_tool["inputSchema"]["additionalProperties"], false);
    assert!(!tools
        .iter()
        .any(|tool| tool["name"] == "cairn_sync_execute"));
    let receipt_tool = tools
        .iter()
        .find(|tool| tool["name"] == "cairn_agent_receipt_validate")
        .ok_or("receipt tool")?;
    assert!(receipt_tool["inputSchema"]["anyOf"].is_array());

    let mutation_value = run_json(&["mcp", "--allow-mutations", "--describe"])?;
    assert_eq!(mutation_value["allow_mutations"], true);
    let mutation_tool = mutation_value["tools"]
        .as_array()
        .ok_or("mutation tools")?
        .iter()
        .find(|tool| tool["name"] == "cairn_sync_execute")
        .ok_or("sync execute")?;
    assert_eq!(mutation_tool["annotations"]["destructiveHint"], true);
    assert_eq!(mutation_tool["inputSchema"]["required"][0], "change");
    Ok(())
}

#[test]
fn mcp_calls_return_structured_receipts() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.mcp_server.read_tools.status]
    // r[verify cairn.supersede.mcp_server.read_tools.gates]
    // r[verify cairn.supersede.mcp_server.verification]
    // r[verify cairn.supersede.mcp_reproducibility.call]
    // r[verify cairn.supersede.mcp_server.verification.docs]
    // r[verify cairn.supersede.mcp_server.strict_schemas.docs]
    let value = run_json(&[
        "mcp",
        "--call",
        "cairn_status",
        r#"{"root":"fixtures/basic"}"#,
    ])?;
    assert_eq!(
        value,
        run_json(&[
            "mcp",
            "--call",
            "cairn_status",
            r#"{"root":"fixtures/basic"}"#,
        ])?
    );
    let receipt = &value["structuredContent"];
    assert_eq!(receipt["layout"], "cairn");
    assert_eq!(receipt["mcp"]["read_only"], true);
    assert_hash_field(receipt, "receipt_hash")?;
    Ok(())
}

#[test]
fn mcp_rejects_invalid_arguments_before_dispatch() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.mcp_server.strict_schemas.validation]
    // r[verify cairn.supersede.mcp_server.strict_schemas.errors]
    // r[verify cairn.supersede.mcp_server.strict_schemas.verification]
    let missing = cairn()
        .args([
            "mcp",
            "--call",
            "cairn_read_spec",
            r#"{"root":"fixtures/basic"}"#,
        ])
        .output()?;
    assert!(!missing.status.success());
    let stderr = String::from_utf8_lossy(&missing.stderr);
    assert!(
        stderr.contains("invalid arguments for cairn_read_spec: missing required argument spec"),
        "{stderr}"
    );

    let unknown = cairn()
        .args([
            "mcp",
            "--call",
            "cairn_status",
            r#"{"root":"fixtures/basic","surprise":"no"}"#,
        ])
        .output()?;
    assert!(!unknown.status.success());
    let stderr = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        stderr.contains("invalid arguments for cairn_status: unknown argument surprise"),
        "{stderr}"
    );

    let bad_stage = cairn()
        .args([
            "mcp",
            "--call",
            "cairn_gate",
            r#"{"root":"fixtures/basic","change":"demo","stage":"archive"}"#,
        ])
        .output()?;
    assert!(!bad_stage.status.success());
    let stderr = String::from_utf8_lossy(&bad_stage.stderr);
    assert!(
        stderr.contains("invalid arguments for cairn_gate: argument stage must be one of proposal, design, tasks"),
        "{stderr}"
    );

    let bad_source = cairn()
        .args([
            "mcp",
            "--call",
            "cairn_migrate_plan",
            r#"{"root":"fixtures/basic","source":"cairn"}"#,
        ])
        .output()?;
    assert!(!bad_source.status.success());
    let stderr = String::from_utf8_lossy(&bad_source.stderr);
    assert!(
        stderr.contains("invalid arguments for cairn_migrate_plan: argument source must be one of openspec, ironspec"),
        "{stderr}"
    );
    Ok(())
}

#[test]
fn mcp_rejects_traversal_and_default_mutation() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.mcp_server.boundary.shell]
    // r[verify cairn.supersede.mcp_server.mutation_guard.default_deny]
    // r[verify cairn.supersede.mcp_server.verification.negative]
    let traversal = cairn()
        .args([
            "mcp",
            "--call",
            "cairn_read_spec",
            r#"{"spec":"../secret","root":"fixtures/basic"}"#,
        ])
        .output()?;
    assert!(!traversal.status.success());
    let stderr = String::from_utf8_lossy(&traversal.stderr);
    assert!(
        stderr.contains("selector escapes lifecycle bounds"),
        "{stderr}"
    );

    let denied = run_json(&[
        "mcp",
        "--call",
        "cairn_sync_execute",
        r#"{"change":"demo","root":"fixtures/basic"}"#,
    ])?;
    let receipt = &denied["structuredContent"];
    assert_eq!(receipt["allowed"], false);
    assert_eq!(receipt["mutated"], false);
    Ok(())
}

#[test]
fn mcp_json_rpc_lists_and_calls_tools() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.mcp_server.verification.registration]
    let mut child = cairn()
        .arg("mcp")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    {
        let stdin = child.stdin.as_mut().ok_or("stdin")?;
        writeln!(stdin, r#"{{"jsonrpc":"2.0","id":1,"method":"tools/list"}}"#)?;
        writeln!(
            stdin,
            r#"{{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{{"name":"cairn_validate","arguments":{{"root":"fixtures/basic"}}}}}}"#
        )?;
        writeln!(
            stdin,
            r#"{{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{{"name":"cairn_gate","arguments":{{"root":"fixtures/basic","change":"demo","stage":"archive"}}}}}}"#
        )?;
    }
    let output = child.wait_with_output()?;
    assert!(output.status.success());
    let lines = String::from_utf8(output.stdout)?;
    let responses = lines
        .lines()
        .map(serde_json::from_str::<Value>)
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(responses.len(), 3);
    assert!(responses[0]["result"]["tools"]
        .as_array()
        .ok_or("tools")?
        .iter()
        .any(|tool| tool["name"] == "cairn_validate"));
    assert_eq!(responses[1]["result"]["structuredContent"]["valid"], true);
    assert!(responses[2]["error"]["message"]
        .as_str()
        .unwrap_or_default()
        .contains("invalid arguments for cairn_gate"));
    Ok(())
}
