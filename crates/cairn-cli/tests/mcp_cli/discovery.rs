use std::{
    collections::BTreeSet,
    error::Error,
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, ChildStdout, Stdio},
};

use serde_json::{json, Value};

use super::common::{cairn, run_json};

struct Session {
    child: Child,
    input: ChildStdin,
    output: BufReader<ChildStdout>,
}

impl Session {
    fn start(options: &[&str]) -> Result<Self, Box<dyn Error>> {
        let mut child = cairn()
            .arg("mcp")
            .args(options)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let input = child.stdin.take().ok_or("stdin")?;
        let output = BufReader::new(child.stdout.take().ok_or("stdout")?);
        Ok(Self {
            child,
            input,
            output,
        })
    }

    fn request(&mut self, method: &str, params: Value) -> Result<Value, Box<dyn Error>> {
        writeln!(
            self.input,
            "{}",
            json!({"jsonrpc":"2.0", "id":1, "method":method, "params":params})
        )?;
        self.input.flush()?;
        let mut line = String::new();
        self.output.read_line(&mut line)?;
        Ok(serde_json::from_str(&line)?)
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn profiles_constrain_execution_and_never_grant_mutations() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.mcp_discovery.profiles]
    // r[verify cairn.mcp_discovery.authority]
    for profile in ["full", "inspect", "author", "verify"] {
        for allow in [false, true] {
            let mut args = vec!["--tool-profile", profile];
            if allow {
                args.push("--allow-mutations");
            }
            let mut session = Session::start(&args)?;
            let listed = session.request("tools/list", json!({}))?;
            let tools = listed["result"]["tools"].as_array().ok_or("tools")?;
            assert!(tools
                .iter()
                .any(|tool| tool["name"] == "cairn_tool_describe"));
            assert!(tools.iter().any(|tool| tool["name"] == "cairn_doctor"));
            assert_eq!(
                tools
                    .iter()
                    .any(|tool| tool["name"] == "cairn_change_create"),
                allow && matches!(profile, "full" | "author")
            );
            let lookup = session.request(
                "tools/call",
                json!({"name":"cairn_tool_describe", "arguments":{"name":"cairn_status"}}),
            )?;
            assert_eq!(
                lookup["result"]["structuredContent"]["name"],
                "cairn_status"
            );
            assert_eq!(
                lookup["result"]["structuredContent"]["inputSchema"]["additionalProperties"],
                false
            );
            let mutation = session.request("tools/call", json!({"name":"cairn_change_create", "arguments":{"root":"/not/a/cairn/discovery-root", "unexpected":true}}))?;
            if !allow {
                assert_eq!(
                    mutation["result"]["structuredContent"]["error"],
                    "mutation capability required"
                );
                assert_eq!(mutation["result"]["structuredContent"]["mutated"], false);
            } else if matches!(profile, "inspect" | "verify") {
                assert!(mutation["error"]["message"]
                    .as_str()
                    .ok_or("unavailable error")?
                    .contains("tool_unavailable"));
            } else {
                assert!(mutation["error"]["message"]
                    .as_str()
                    .ok_or("argument error")?
                    .contains("invalid arguments"));
            }
            // A denied mutation is unavailable to discovery even though dispatch retains capability-first denial.
            let lookup = session.request(
                "tools/call",
                json!({"name":"cairn_tool_describe", "arguments":{"name":"cairn_change_create"}}),
            )?;
            assert_eq!(
                lookup.get("error").is_some(),
                !allow || matches!(profile, "inspect" | "verify")
            );
        }
    }
    let mut inspect = Session::start(&["--tool-profile", "inspect", "--allow-mutations"])?;
    let unavailable = inspect.request("tools/call", json!({"name":"cairn_validate", "arguments":{"root":"/not/a/cairn/discovery-root", "extra":true}}))?;
    assert!(unavailable["error"]["message"]
        .as_str()
        .ok_or("profile error")?
        .contains("tool_unavailable"));
    assert_eq!(unavailable["error"]["data"]["code"], "tool_unavailable");
    assert_eq!(unavailable["error"]["data"]["tool"], "cairn_validate");
    assert_eq!(unavailable["error"]["data"]["mutated"], false);
    let status = inspect.request(
        "tools/call",
        json!({"name":"cairn_status", "arguments":{"root":"fixtures/basic"}}),
    )?;
    assert!(status.get("result").is_some());
    Ok(())
}

#[test]
fn strict_lookup_rejects_unknown_unavailable_and_forwarded_arguments() -> Result<(), Box<dyn Error>>
{
    // r[verify cairn.mcp_discovery.lookup]
    let descriptor = run_json(&[
        "mcp",
        "--tool-profile",
        "inspect",
        "--describe-tool",
        "cairn_status",
    ])?;
    assert_eq!(descriptor["name"], "cairn_status");
    for args in [
        vec!["mcp", "--tool-profile", "unknown", "--describe"],
        vec![
            "mcp",
            "--tool-profile",
            "inspect",
            "--describe-tool",
            "cairn_validate",
        ],
        vec!["mcp", "--describe-tool", "unknown"],
        vec!["mcp", "--describe-tool", "cairn_init"],
        vec!["mcp", "--describe", "--describe-tool", "cairn_status"],
    ] {
        assert!(!cairn().args(args).output()?.status.success());
    }
    let mut session = Session::start(&["--tool-profile", "inspect"])?;
    for arguments in [
        json!({}),
        json!({"name":""}),
        json!({"name":4}),
        json!({"name":"cairn_status", "root":"/not/read"}),
        json!({"name":"cairn_status", "arguments":{}}),
    ] {
        let response = session.request(
            "tools/call",
            json!({"name":"cairn_tool_describe", "arguments":arguments}),
        )?;
        assert!(response["error"]["message"]
            .as_str()
            .ok_or("strict error")?
            .contains("invalid arguments"));
    }
    for name in ["unknown", "cairn_validate", "cairn_init"] {
        let response = session.request(
            "tools/call",
            json!({"name":"cairn_tool_describe", "arguments":{"name":name}}),
        )?;
        assert!(response["error"]["message"]
            .as_str()
            .ok_or("lookup error")?
            .contains("tool_unavailable"));
    }
    // Self-description stays one descriptor, not a recursively embedded catalog.
    let response = session.request(
        "tools/call",
        json!({"name":"cairn_tool_describe", "arguments":{"name":"cairn_tool_describe"}}),
    )?;
    assert_eq!(
        response["result"]["structuredContent"]["name"],
        "cairn_tool_describe"
    );
    assert!(response["result"]["structuredContent"]
        .get("tools")
        .is_none());
    Ok(())
}

#[test]
// r[verify cairn.mcp_discovery.conformance]
fn bounded_pages_complete_without_empty_terminal_pages_and_reject_stale_cursors(
) -> Result<(), Box<dyn Error>> {
    // r[verify cairn.mcp_discovery.pages]
    for profile in ["full", "inspect", "author", "verify"] {
        for allow in [false, true] {
            let mut options = vec!["--paged-tools", "--tool-profile", profile];
            if allow {
                options.push("--allow-mutations");
            }
            let mut session = Session::start(&options)?;
            let mut params = json!({});
            let mut names = Vec::new();
            let mut first_cursor = None;
            loop {
                let page = session.request("tools/list", params)?;
                let result = &page["result"];
                assert!(serde_json::to_vec(result)?.len() <= 131072);
                let tools = result["tools"].as_array().ok_or("page tools")?;
                assert!(!tools.is_empty());
                assert!(tools.len() <= 8);
                names.extend(
                    tools
                        .iter()
                        .map(|tool| tool["name"].as_str().unwrap_or_default().to_owned()),
                );
                if let Some(cursor) = result["nextCursor"].as_str() {
                    first_cursor.get_or_insert_with(|| cursor.to_owned());
                    params = json!({"cursor":cursor});
                } else {
                    break;
                }
            }
            assert_eq!(names.len(), names.iter().collect::<BTreeSet<_>>().len());
            assert!(names.iter().any(|name| name == "cairn_tool_describe"));
            assert_eq!(
                names.iter().any(|name| name == "cairn_change_create"),
                allow && matches!(profile, "full" | "author")
            );
            // Policy identity is exposed by the CLI, not a nonstandard list field.
            let mut args = vec!["mcp", "--tool-profile", profile, "--describe"];
            if allow {
                args.push("--allow-mutations");
            }
            let catalog = run_json(&args)?;
            assert_eq!(
                names.len(),
                catalog["tools"].as_array().ok_or("catalog tools")?.len()
            );
            let identity = catalog["catalog_identity"].as_str().ok_or("identity")?;
            for cursor in [
                format!("v1:{identity}:0"),
                format!("v1:{identity}:{}", names.len()),
                format!("v1:{identity}:99999"),
                format!("v1:{identity}:01"),
                "x".repeat(257),
            ] {
                assert_eq!(
                    session.request("tools/list", json!({"cursor":cursor}))?["error"]["code"],
                    -32602
                );
            }
            let cursor = first_cursor.ok_or("expected continuation")?;
            let mut changed_profile = Session::start(&[
                "--paged-tools",
                "--tool-profile",
                if profile == "inspect" {
                    "verify"
                } else {
                    "inspect"
                },
            ])?;
            let stale = changed_profile.request("tools/list", json!({"cursor":cursor}))?;
            assert!(stale["error"]["message"]
                .as_str()
                .ok_or("stale error")?
                .contains("stale"));
            let mut changed_options = vec!["--paged-tools", "--tool-profile", profile];
            if !allow {
                changed_options.push("--allow-mutations");
            }
            let mut changed_capability = Session::start(&changed_options)?;
            let stale = changed_capability.request("tools/list", json!({"cursor":cursor}))?;
            assert!(stale["error"]["message"]
                .as_str()
                .ok_or("stale capability")?
                .contains("stale"));
        }
    }
    for paged in [false, true] {
        let mut session = Session::start(if paged { &["--paged-tools"] } else { &[] })?;
        for params in [
            json!(null),
            json!([]),
            json!({"limit":1}),
            json!({"cursor":null}),
            json!({"cursor":1}),
            json!({"cursor":"bad", "extra":true}),
        ] {
            assert_eq!(
                session.request("tools/list", params)?["error"]["code"],
                -32602
            );
        }
        if !paged {
            let result = session.request("tools/list", json!({}))?;
            assert!(result["result"].get("nextCursor").is_none());
            assert!(result["result"]["tools"].as_array().ok_or("unpaged")?.len() > 8);
            assert_eq!(
                session.request("tools/list", json!({"cursor":"bad"}))?["error"]["code"],
                -32602
            );
        }
    }
    Ok(())
}

#[test]
fn author_profile_retains_sync_and_archive_preflight() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.mcp_discovery.authority]
    use super::common::{copy_dir_recursive, repo_root};
    for profile in ["full", "author"] {
        let fixture = std::env::temp_dir().join(format!(
            "cairn-discovery-preflight-{profile}-{}",
            std::process::id()
        ));
        copy_dir_recursive(&repo_root().join("fixtures/archive-ready"), &fixture)?;
        let change = fixture.join(".cairn/changes/ready");
        std::fs::write(change.join("proposal.md"), "\n")?;
        std::fs::write(change.join("design.md"), "CAIRN_PLACEHOLDER\n")?;
        std::fs::write(
            change.join("tasks.md"),
            "- [x] [depends:] CAIRN_PLACEHOLDER\n",
        )?;
        std::fs::write(change.join("specs/demo/spec.md"), "\n")?;
        let accepted = fixture.join(".cairn/specs/demo/spec.md");
        let before = std::fs::read(&accepted)?;
        let mut session = Session::start(&["--tool-profile", profile, "--allow-mutations"])?;
        for tool in ["cairn_sync_execute", "cairn_archive_execute"] {
            let response = session.request(
                "tools/call",
                json!({"name":tool, "arguments":{"root":fixture, "change":"ready"}}),
            )?;
            let receipt = &response["result"]["structuredContent"];
            assert_eq!(receipt["blocked"], true, "{profile}: {tool}: {response}");
            assert_eq!(receipt["mutated"], false);
            assert!(change.is_dir());
            assert_eq!(std::fs::read(&accepted)?, before);
        }
        std::fs::remove_dir_all(fixture)?;
    }
    Ok(())
}
