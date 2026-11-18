use std::{
    env,
    error::Error,
    fs,
    io::Write,
    process::Stdio,
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::{json, Value};

use super::common::{cairn, run_json};

fn text(result: &Value) -> &str {
    result["content"][0]["text"].as_str().expect("text content")
}

fn direct(profile: Option<&str>, tool: &str, arguments: &str) -> Result<Value, Box<dyn Error>> {
    let mut command = cairn();
    command.arg("mcp");
    if let Some(profile) = profile {
        command.args(["--response-profile", profile]);
    }
    command.args(["--call", tool, arguments]);
    super::common::run_json_command(&mut command)
}

fn rpc(profile: Option<&str>, calls: &[(String, Value)]) -> Result<Vec<Value>, Box<dyn Error>> {
    let mut command = cairn();
    command.arg("mcp");
    if let Some(profile) = profile {
        command.args(["--response-profile", profile]);
    }
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    {
        let stdin = child.stdin.as_mut().ok_or("stdin")?;
        for (index, (tool, arguments)) in calls.iter().enumerate() {
            writeln!(
                stdin,
                "{}",
                json!({"jsonrpc":"2.0","id":index+1,"method":"tools/call", "params":{"name":tool,"arguments":arguments}})
            )?;
        }
    }
    let output = child.wait_with_output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(String::from_utf8(output.stdout)?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?)
}

fn parity(legacy: &Value, compact: &Value, tool: &str) {
    assert_eq!(
        legacy["structuredContent"], compact["structuredContent"],
        "{tool}"
    );
    let receipt = &legacy["structuredContent"];
    assert_eq!(legacy["isError"], compact["isError"]);
    assert!(text(compact).len() <= 256, "{}", text(compact));
    assert!(text(compact).contains("structuredContent"));
    assert!(!text(compact).contains(&serde_json::to_string(receipt).unwrap()));
}

#[test]
fn direct_profiles_preserve_receipts_and_distinguish_states() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.mcp_projection.single_payload]
    // r[verify cairn.mcp_projection.status]
    // r[verify cairn.mcp_projection.compatibility]
    for (tool, arguments) in [
        ("cairn_validate", r#"{"root":"fixtures/basic"}"#),
        (
            "cairn_gate",
            r#"{"root":"fixtures/basic","change":"demo","stage":"proposal"}"#,
        ),
        ("cairn_implementation_ready", r#"{"root":"fixtures/basic"}"#),
        (
            "cairn_readiness",
            r#"{"root":"fixtures/basic","change":"demo"}"#,
        ),
        (
            "cairn_sync_execute",
            r#"{"root":"fixtures/basic","invalid":true}"#,
        ),
        ("cairn_status", r#"{"root":"fixtures/basic"}"#),
    ] {
        let default = direct(None, tool, arguments)?;
        let legacy = direct(Some("legacy"), tool, arguments)?;
        let compact = direct(Some("structured-summary"), tool, arguments)?;
        assert_eq!(default, legacy, "default compatibility for {tool}");
        parity(&legacy, &compact, tool);
        if tool == "cairn_sync_execute" {
            assert_eq!(compact["structuredContent"]["mutated"], false);
        }
    }
    Ok(())
}

#[test]
fn rpc_profile_is_fixed_and_preserves_error_and_capability_boundaries() -> Result<(), Box<dyn Error>>
{
    // r[verify cairn.mcp_projection.profile]
    // r[verify cairn.mcp_projection.compatibility]
    let calls = vec![
        ("cairn_validate".into(), json!({"root":"fixtures/basic"})),
        ("cairn_sync_execute".into(), json!({"invalid":true})),
        (
            "cairn_gate".into(),
            json!({"root":"fixtures/basic","change":"demo","stage":"archive"}),
        ),
        (
            "cairn_readiness".into(),
            json!({"root":"/no/such/cairn-root","change":"demo"}),
        ),
    ];
    let legacy = rpc(None, &calls)?;
    let compact = rpc(Some("structured-summary"), &calls)?;
    assert_eq!(legacy.len(), calls.len());
    assert_eq!(compact.len(), calls.len());
    for (index, (tool, _)) in calls.iter().enumerate() {
        assert_eq!(legacy[index]["id"], compact[index]["id"]);
        if index == 2 {
            assert_eq!(legacy[index]["error"], compact[index]["error"]);
            assert!(compact[index]["result"].is_null());
        } else {
            parity(&legacy[index]["result"], &compact[index]["result"], tool);
        }
    }
    assert_eq!(
        compact[3]["result"]["structuredContent"]["classification"],
        "unavailable"
    );
    assert_eq!(compact[1]["result"]["structuredContent"]["mutated"], false);
    let override_attempt = rpc(
        Some("structured-summary"),
        &[(
            "cairn_status".into(),
            json!({"root":"fixtures/basic", "response_profile":"legacy"}),
        )],
    )?;
    let default_override_attempt = rpc(
        None,
        &[(
            "cairn_status".into(),
            json!({"root":"fixtures/basic", "response_profile":"legacy"}),
        )],
    )?;
    assert_eq!(
        override_attempt[0]["error"],
        default_override_attempt[0]["error"]
    );
    assert!(override_attempt[0]["error"].is_object());
    assert_eq!(override_attempt[0]["result"], Value::Null);
    Ok(())
}

#[test]
fn long_source_is_retained_only_in_structured_detail() -> Result<(), Box<dyn Error>> {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = env::temp_dir().join(format!(
        "cairn-mcp-long-source-{}-{nonce}",
        std::process::id()
    ));
    let spec = root.join(".cairn/specs/demo/spec.md");
    fs::create_dir_all(spec.parent().ok_or("spec parent")?)?;
    let markdown = format!("# Demo\n\n{}\n", "unique-private-diagnostic-".repeat(1000));
    fs::write(&spec, &markdown)?;
    let input = json!({"root": root.to_string_lossy(), "spec":"demo"});
    let legacy = direct(None, "cairn_read_spec", &input.to_string())?;
    let compact = direct(
        Some("structured-summary"),
        "cairn_read_spec",
        &input.to_string(),
    )?;
    parity(&legacy, &compact, "cairn_read_spec");
    assert!(text(&legacy).contains("unique-private-diagnostic-"));
    assert!(!text(&compact).contains("unique-private-diagnostic-"));
    let rpc = rpc(
        Some("structured-summary"),
        &[("cairn_read_spec".into(), input)],
    )?;
    assert_eq!(rpc[0]["result"], compact);
    assert_eq!(fs::read_to_string(&spec)?, markdown);
    fs::remove_dir_all(&root)?;
    Ok(())
}

#[test]
fn invalid_profiles_and_strict_options_fail_before_dispatch() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.mcp_projection.profile]
    for arguments in [
        vec!["--response-profile"],
        vec!["--response-profile", "--call", "cairn_init"],
        vec![
            "--response-profile",
            "unknown",
            "--call",
            "cairn_init",
            "{}",
        ],
        vec![
            "--response-profile",
            "legacy",
            "--response-profile",
            "structured-summary",
            "--call",
            "cairn_init",
        ],
        vec![
            "--response-profile=structured-summary",
            "--call",
            "cairn_init",
        ],
    ] {
        let output = cairn().arg("mcp").args(&arguments).output()?;
        assert!(!output.status.success(), "{arguments:?}");
        assert!(output.stdout.is_empty(), "{arguments:?}");
    }
    let catalog = run_json(&["mcp", "--describe"])?;
    let compact_catalog = run_json(&[
        "mcp",
        "--response-profile",
        "structured-summary",
        "--describe",
    ])?;
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = env::temp_dir().join(format!(
        "cairn-mcp-profile-denied-{}-{nonce}",
        std::process::id()
    ));
    let input = json!({"root": root.to_string_lossy()});
    let output = cairn()
        .args([
            "mcp",
            "--response-profile",
            "not-admitted",
            "--allow-mutations",
            "--call",
            "cairn_init",
            &input.to_string(),
        ])
        .output()?;
    assert!(!output.status.success());
    assert!(!root.exists(), "invalid profile dispatched a mutation");
    let server = cairn()
        .args([
            "mcp",
            "--response-profile",
            "not-admitted",
            "--allow-mutations",
        ])
        .stdout(Stdio::piped())
        .spawn()?;
    let server_output = server.wait_with_output()?;
    assert!(!server_output.status.success());
    assert!(server_output.stdout.is_empty());
    assert!(
        !root.exists(),
        "invalid server profile dispatched a mutation"
    );
    let denied = direct(Some("structured-summary"), "cairn_init", &input.to_string())?;
    assert_eq!(denied["structuredContent"]["mutated"], false);
    assert!(
        !root.exists(),
        "response profile granted mutation authority"
    );
    assert_eq!(catalog, compact_catalog);
    assert_eq!(
        run_json(&["mcp", "--allow-mutations", "--describe"])?,
        run_json(&[
            "mcp",
            "--allow-mutations",
            "--response-profile",
            "structured-summary",
            "--describe"
        ])?,
    );
    Ok(())
}
