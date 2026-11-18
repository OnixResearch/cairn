use serde_json::{json, Value};
use std::{error::Error, fs, path::PathBuf, process::Command};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn run(args: &[&str]) -> Result<Value, Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root())
        .args(args)
        .output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(serde_json::from_slice(&output.stdout)?)
}

#[test]
// r[verify cairn.unified_readiness.transport]
// r[verify cairn.unified_readiness.conformance]
fn cli_mcp_and_original_gates_agree_without_completion_claims() -> Result<(), Box<dyn Error>> {
    let report = run(&["readiness", "demo", "--root", "fixtures/basic"])?;
    assert_eq!(report["valid"], true, "{report:#}");
    assert_eq!(report["classification"], "unavailable");
    assert_eq!(report["mutation_authorized"], false);
    let mcp = run(&[
        "mcp",
        "--call",
        "cairn_readiness",
        &json!({"root":"fixtures/basic", "change":"demo"}).to_string(),
    ])?;
    assert_eq!(mcp["structuredContent"], report, "{mcp:#}");
    for (index, stage) in [(1, "proposal"), (2, "design"), (3, "tasks")] {
        let original = run(&["gate", stage, "demo", "--root", "fixtures/basic"])?;
        assert_eq!(
            report["stages"][index]["decision"]["verdict"],
            original["verdict"]
        );
        assert_eq!(
            report["stages"][index]["decision"]["issues"]
                .as_array()
                .unwrap()
                .len(),
            original["issues"].as_array().unwrap().len()
        );
    }
    Ok(())
}

#[test]
// r[verify cairn.unified_readiness.boundary]
fn denied_mutation_and_unavailable_evidence_do_not_change_files() -> Result<(), Box<dyn Error>> {
    let path = root().join("fixtures/basic/.cairn/changes/demo/tasks.md");
    let before = fs::read(&path)?;
    let unavailable = run(&[
        "readiness",
        "demo",
        "--root",
        "fixtures/basic",
        "--contract",
        "../escape",
        "--results",
        "absent",
        "--subject-hash",
        &"c".repeat(64),
    ])?;
    assert_eq!(unavailable["classification"], "unavailable");
    assert_eq!(unavailable["stages"][4]["decision"]["evaluated"], false);
    let ordinary = run(&["readiness", "demo", "--root", "fixtures/basic"])?;
    for stage in 0..4 {
        assert_eq!(
            unavailable["stages"][stage]["decision"],
            ordinary["stages"][stage]["decision"]
        );
        assert_eq!(
            unavailable["stages"][stage]["state"],
            ordinary["stages"][stage]["state"]
        );
    }
    assert!(unavailable["blockers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|b| b["code"] == "observation.load_failed"));
    assert_eq!(unavailable["mutation_authorized"], false);
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root())
        .args(["readiness", "demo", "--root", "fixtures/basic", "--execute"])
        .output()?;
    assert!(!output.status.success());
    assert_eq!(fs::read(path)?, before);
    Ok(())
}

#[test]
fn readiness_mcp_rejects_wrong_evidence_types_and_unknown_authority() -> Result<(), Box<dyn Error>>
{
    for fields in [
        json!({"cross_repo_evidence": "upstream.json"}),
        json!({"cross_repo_evidence": [1]}),
        json!({"execute": true}),
    ] {
        let mut input = fields;
        input["root"] = json!("fixtures/basic");
        input["change"] = json!("demo");
        let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
            .current_dir(root())
            .args(["mcp", "--call", "cairn_readiness", &input.to_string()])
            .output()?;
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr)
            .contains("invalid arguments for cairn_readiness"));
    }
    Ok(())
}

#[test]
// r[verify cairn.unified_readiness.transport]
fn successful_machine_transport_preserves_unavailable_lifecycle() -> Result<(), Box<dyn Error>> {
    let report = run(&["--machine", "readiness", "demo", "--root", "fixtures/basic"])?;
    assert_eq!(report["ok"], true);
    assert_eq!(report["exit_class"], "success");
    assert_eq!(report["result"]["classification"], "unavailable");
    assert_eq!(report["result"]["mutation_authorized"], false);
    let catalog = run(&["mcp", "--describe"])?;
    assert!(catalog.to_string().contains("cairn_readiness"));
    Ok(())
}

fn reject(args: &[&str], fragment: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root())
        .args(args)
        .output()?;
    assert!(!output.status.success(), "unexpected success: {args:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains(fragment),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(())
}

#[test]
// r[verify cairn.agent_readiness.view]
// r[verify cairn.agent_readiness.actions]
fn real_cli_mcp_agent_projection_and_details_preserve_full_identity() -> Result<(), Box<dyn Error>>
{
    let path = root().join("fixtures/basic/.cairn/changes/demo/tasks.md");
    let before = fs::read(&path)?;
    let full = run(&["readiness", "demo", "--root", "fixtures/basic"])?;
    let explicit_full = run(&[
        "readiness",
        "demo",
        "--root",
        "fixtures/basic",
        "--view",
        "full",
    ])?;
    assert_eq!(full, explicit_full);
    let agent = run(&[
        "readiness",
        "demo",
        "--root",
        "fixtures/basic",
        "--view",
        "agent",
    ])?;
    assert_eq!(agent["schema"], "cairn.readiness-agent.v1");
    assert_eq!(agent["full_receipt_hash"], full["receipt_hash"]);
    for field in [
        "scope",
        "valid",
        "classification",
        "mutation_authorized",
        "readiness_policy_hash",
        "input_hash",
    ] {
        assert_eq!(agent[field], full[field], "field {field}");
    }
    for (compact, original) in agent["stages"]
        .as_array()
        .unwrap()
        .iter()
        .zip(full["stages"].as_array().unwrap())
    {
        for field in ["stage", "state", "decision_hash"] {
            assert_eq!(compact[field], original[field]);
        }
    }
    for kind in ["blockers", "actions"] {
        for entry in agent[kind].as_array().unwrap() {
            assert!(full[kind].as_array().unwrap().contains(entry));
        }
    }
    assert_eq!(
        agent["omitted_blockers"].as_u64().unwrap()
            + agent["blockers"].as_array().unwrap().len() as u64,
        agent["blocker_count"].as_u64().unwrap()
    );
    assert_eq!(
        agent["omitted_actions"].as_u64().unwrap()
            + agent["actions"].as_array().unwrap().len() as u64,
        agent["action_count"].as_u64().unwrap()
    );
    let request = json!({"root":"fixtures/basic", "change":"demo", "view":"agent"});
    let mcp = run(&["mcp", "--call", "cairn_readiness", &request.to_string()])?;
    assert_eq!(mcp["structuredContent"], agent);
    let machine = run(&[
        "--machine",
        "readiness",
        "demo",
        "--root",
        "fixtures/basic",
        "--view",
        "agent",
    ])?;
    assert_eq!(machine["ok"], true);
    assert_eq!(machine["result"], agent);
    let selected = &agent["detail_selectors"][0];
    let kind = selected["kind"].as_str().unwrap();
    let id = selected["id"].as_str().unwrap();
    let receipt = full["receipt_hash"].as_str().unwrap();
    let detail = run(&[
        "readiness",
        "demo",
        "--root",
        "fixtures/basic",
        "--detail-kind",
        kind,
        "--detail-id",
        id,
        "--expected-receipt",
        receipt,
    ])?;
    assert_eq!(detail["detail"], full["stages"][0]);
    assert_eq!(detail["full_receipt_hash"], receipt);
    let mcp_detail = run(&["mcp", "--call", "cairn_readiness", &json!({"root":"fixtures/basic", "change":"demo", "detail":{"kind":kind, "id":id, "expected_receipt":receipt}}).to_string()])?;
    assert_eq!(mcp_detail["structuredContent"], detail);
    let omitted = run(&[
        "readiness",
        "demo",
        "--root",
        "fixtures/basic",
        "--view",
        "agent",
        "--max-entries",
        "0",
    ])?;
    for (kind, field) in [("blocker", "blockers"), ("action", "actions")] {
        if let Some(selector) = omitted["detail_selectors"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["kind"] == kind)
        {
            let id = selector["id"].as_str().unwrap();
            let exact = full[field]
                .as_array()
                .unwrap()
                .iter()
                .find(|entry| entry["id"] == id)
                .unwrap();
            let selected = run(&[
                "readiness",
                "demo",
                "--root",
                "fixtures/basic",
                "--detail-kind",
                kind,
                "--detail-id",
                id,
                "--expected-receipt",
                receipt,
            ])?;
            assert_eq!(&selected["detail"], exact);
            let mcp_selected = run(&["mcp", "--call", "cairn_readiness", &json!({"root":"fixtures/basic", "change":"demo", "detail":{"kind":kind, "id":id, "expected_receipt":receipt}}).to_string()])?;
            assert_eq!(mcp_selected["structuredContent"], selected);
        }
    }
    assert_eq!(fs::read(&path)?, before);
    Ok(())
}

#[test]
// r[verify cairn.agent_readiness.decisions]
// r[verify cairn.agent_readiness.detail]
fn real_cli_mcp_budget_stale_and_invalid_selector_controls() -> Result<(), Box<dyn Error>> {
    let full = run(&["readiness", "demo", "--root", "fixtures/basic"])?;
    let receipt = full["receipt_hash"].as_str().unwrap();
    let baseline = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root())
        .args([
            "readiness",
            "demo",
            "--root",
            "fixtures/basic",
            "--view",
            "agent",
            "--max-entries",
            "0",
        ])
        .output()?;
    assert!(
        baseline.status.success(),
        "{}",
        String::from_utf8_lossy(&baseline.stderr)
    );
    assert_eq!(baseline.stdout.last(), Some(&b'\n'));
    let bytes = baseline.stdout.len();
    let base: Value = serde_json::from_slice(&baseline.stdout)?;
    let accepted_output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root())
        .args([
            "readiness",
            "demo",
            "--root",
            "fixtures/basic",
            "--view",
            "agent",
            "--max-entries",
            "0",
            "--max-bytes",
            &bytes.to_string(),
        ])
        .output()?;
    assert!(
        accepted_output.status.success(),
        "{}",
        String::from_utf8_lossy(&accepted_output.stderr)
    );
    assert_eq!(accepted_output.stdout.len(), bytes);
    assert_eq!(accepted_output.stdout, baseline.stdout);
    let accepted: Value = serde_json::from_slice(&accepted_output.stdout)?;
    assert_eq!(accepted, base);
    assert!(base["omitted_actions"].as_u64().unwrap() > 0);
    assert_eq!(base["complete"], false);
    reject(
        &[
            "readiness",
            "demo",
            "--root",
            "fixtures/basic",
            "--view",
            "agent",
            "--max-entries",
            "0",
            "--max-bytes",
            &(bytes - 1).to_string(),
        ],
        "insufficient agent readiness budget",
    )?;
    reject(
        &[
            "readiness",
            "demo",
            "--root",
            "fixtures/basic",
            "--view",
            "agent",
            "--format",
            "summary",
        ],
        "require JSON format",
    )?;
    reject(
        &[
            "readiness",
            "demo",
            "--root",
            "fixtures/basic",
            "--view",
            "unknown",
        ],
        "invalid readiness options",
    )?;
    reject(
        &[
            "readiness",
            "demo",
            "--root",
            "fixtures/basic",
            "--max-entries",
            "1",
        ],
        "budgets require agent view",
    )?;
    reject(
        &[
            "readiness",
            "demo",
            "--root",
            "fixtures/basic",
            "--detail-kind",
            "stage",
        ],
        "requires kind, id, and expected_receipt",
    )?;
    reject(
        &[
            "readiness",
            "demo",
            "--root",
            "fixtures/basic",
            "--detail-kind",
            "stage",
            "--detail-id",
            "../escape",
            "--expected-receipt",
            receipt,
        ],
        "unsafe readiness detail id",
    )?;
    reject(
        &[
            "readiness",
            "demo",
            "--root",
            "fixtures/basic",
            "--detail-kind",
            "stage",
            "--detail-id",
            "absent",
            "--expected-receipt",
            receipt,
        ],
        "unknown readiness detail id",
    )?;
    reject(
        &[
            "readiness",
            "demo",
            "--root",
            "fixtures/basic",
            "--detail-kind",
            "stage",
            "--detail-id",
            "artifact",
            "--expected-receipt",
            &"a".repeat(64),
        ],
        "stale readiness detail receipt",
    )?;
    let other_inputs = run(&[
        "readiness",
        "demo",
        "--root",
        "fixtures/basic",
        "--contract",
        "../escape",
        "--results",
        "absent",
        "--subject-hash",
        &"c".repeat(64),
        "--view",
        "agent",
    ])?;
    assert_eq!(other_inputs["mutation_authorized"], false);
    assert_eq!(other_inputs["classification"], "unavailable");
    assert_ne!(other_inputs["full_receipt_hash"], receipt);
    reject(
        &[
            "readiness",
            "demo",
            "--root",
            "fixtures/basic",
            "--contract",
            "../escape",
            "--results",
            "absent",
            "--subject-hash",
            &"c".repeat(64),
            "--detail-kind",
            "stage",
            "--detail-id",
            "artifact",
            "--expected-receipt",
            receipt,
        ],
        "stale readiness detail receipt",
    )?;
    reject(&["mcp", "--call", "cairn_readiness", &json!({"root":"fixtures/basic", "change":"demo", "view":"agent", "max_entries":0, "max_bytes":bytes - 1}).to_string()], "insufficient agent readiness budget")?;
    reject(
        &[
            "mcp",
            "--call",
            "cairn_readiness",
            &json!({"root":"fixtures/basic", "change":"demo", "view":"agent", "max_entries":-1})
                .to_string(),
        ],
        "invalid arguments for cairn_readiness",
    )?;
    reject(&["mcp", "--call", "cairn_readiness", &json!({"root":"fixtures/basic", "change":"demo", "detail":{"kind":"stage","id":"artifact"}}).to_string()], "invalid arguments for cairn_readiness")?;
    reject(
        &[
            "mcp",
            "--call",
            "cairn_readiness",
            &json!({"root":"fixtures/basic", "change":"demo", "view":"unknown"}).to_string(),
        ],
        "unknown readiness view",
    )?;
    Ok(())
}
