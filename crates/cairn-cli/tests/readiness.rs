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
