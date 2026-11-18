use serde_json::Value;
use std::{error::Error, fs, path::PathBuf, process::Command};

#[test]
// r[verify cairn.unified_readiness.decisions]
// r[verify cairn.unified_readiness.boundary]
fn completion_and_stale_missing_and_symlink_controls() -> Result<(), Box<dyn Error>> {
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let temporary =
        std::env::temp_dir().join(format!("cairn-readiness-completion-{}", std::process::id()));
    fs::create_dir_all(temporary.join(".cairn/changes/demo/specs/demo"))?;
    fs::create_dir_all(temporary.join(".cairn/specs"))?;
    fs::create_dir_all(temporary.join(".cairn/archive"))?;
    for path in ["proposal.md", "design.md", "tasks.md", "specs/demo/spec.md"] {
        fs::copy(
            source.join("fixtures/basic/.cairn/changes/demo").join(path),
            temporary.join(".cairn/changes/demo").join(path),
        )?;
    }
    for path in ["contract.json", "results.json"] {
        fs::copy(
            source.join("fixtures/agent-completion").join(path),
            temporary.join(path),
        )?;
    }
    let observe = |subject: &str| -> Result<Value, Box<dyn Error>> {
        let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
            .current_dir(&source)
            .args([
                "readiness",
                "demo",
                "--root",
                temporary.to_str().ok_or("root")?,
                "--contract",
                "contract.json",
                "--results",
                "results.json",
                "--subject-hash",
                subject,
            ])
            .output()?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let report: Value = serde_json::from_slice(&output.stdout)?;
        let mcp_input = serde_json::json!({"change": "demo", "root": temporary,
            "contract": "contract.json", "results": "results.json", "subject_hash": subject});
        let mcp_output = Command::new(env!("CARGO_BIN_EXE_cairn"))
            .current_dir(&source)
            .args(["mcp", "--call", "cairn_readiness", &mcp_input.to_string()])
            .output()?;
        assert!(mcp_output.status.success());
        let mcp: Value = serde_json::from_slice(&mcp_output.stdout)?;
        assert_eq!(mcp["structuredContent"], report);
        let compact_output = Command::new(env!("CARGO_BIN_EXE_cairn"))
            .current_dir(&source)
            .args([
                "readiness",
                "demo",
                "--root",
                temporary.to_str().ok_or("root")?,
                "--contract",
                "contract.json",
                "--results",
                "results.json",
                "--subject-hash",
                subject,
                "--view",
                "agent",
            ])
            .output()?;
        assert!(
            compact_output.status.success(),
            "{}",
            String::from_utf8_lossy(&compact_output.stderr)
        );
        let compact: Value = serde_json::from_slice(&compact_output.stdout)?;
        assert_eq!(compact["full_receipt_hash"], report["receipt_hash"]);
        assert_eq!(compact["valid"], report["valid"]);
        assert_eq!(compact["classification"], report["classification"]);
        assert_eq!(compact["mutation_authorized"], false);
        for (short, full) in compact["stages"]
            .as_array()
            .unwrap()
            .iter()
            .zip(report["stages"].as_array().unwrap())
        {
            assert_eq!(short["state"], full["state"]);
            assert_eq!(short["decision_hash"], full["decision_hash"]);
        }
        for action in compact["actions"].as_array().unwrap() {
            assert!(report["actions"].as_array().unwrap().contains(action));
        }
        let mut compact_input = mcp_input.clone();
        compact_input["view"] = serde_json::json!("agent");
        let compact_mcp_output = Command::new(env!("CARGO_BIN_EXE_cairn"))
            .current_dir(&source)
            .args([
                "mcp",
                "--call",
                "cairn_readiness",
                &compact_input.to_string(),
            ])
            .output()?;
        assert!(
            compact_mcp_output.status.success(),
            "{}",
            String::from_utf8_lossy(&compact_mcp_output.stderr)
        );
        let compact_mcp: Value = serde_json::from_slice(&compact_mcp_output.stdout)?;
        assert_eq!(compact_mcp["structuredContent"], compact);
        if report["stages"][4]["state"] != "unavailable" {
            let original_output = Command::new(env!("CARGO_BIN_EXE_cairn"))
                .current_dir(&source)
                .args([
                    "agent-receipt",
                    "complete",
                    "--contract",
                    temporary.join("contract.json").to_str().ok_or("contract")?,
                    "--results",
                    temporary.join("results.json").to_str().ok_or("results")?,
                    "--subject-hash",
                    subject,
                ])
                .output()?;
            assert!(original_output.status.success());
            let original: Value = serde_json::from_slice(&original_output.stdout)?;
            assert_eq!(report["stages"][4]["decision"], original["completion"]);
        }
        Ok(report)
    };
    let before = fs::read(temporary.join(".cairn/changes/demo/tasks.md"))?;
    let passing = observe(&"c".repeat(64))?;
    assert_eq!(passing["stages"][4]["state"], "ready", "{passing:#}");
    let stale = observe(&"d".repeat(64))?;
    assert_eq!(
        stale["stages"][4]["decision"]["classification"],
        "authoritative"
    );
    assert_eq!(stale["stages"][4]["state"], "blocked");
    assert_eq!(stale["stages"][5]["state"], "blocked");
    assert!(stale["actions"]
        .as_array()
        .unwrap()
        .iter()
        .any(|a| a["kind"] == "evidence_refresh"));
    fs::write(temporary.join("results.json"), "[]")?;
    let missing = observe(&"c".repeat(64))?;
    assert_eq!(missing["stages"][4]["state"], "blocked");
    assert!(missing["actions"]
        .as_array()
        .unwrap()
        .iter()
        .any(|a| a["kind"] == "evidence_refresh"));
    assert_ne!(missing["receipt_hash"], passing["receipt_hash"]);
    let expected = passing["receipt_hash"]
        .as_str()
        .ok_or("passing receipt hash")?;
    let stale_detail = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&source)
        .args([
            "readiness",
            "demo",
            "--root",
            temporary.to_str().ok_or("root")?,
            "--contract",
            "contract.json",
            "--results",
            "results.json",
            "--subject-hash",
            &"c".repeat(64),
            "--detail-kind",
            "stage",
            "--detail-id",
            "completion",
            "--expected-receipt",
            expected,
        ])
        .output()?;
    assert!(!stale_detail.status.success());
    assert!(
        String::from_utf8_lossy(&stale_detail.stderr).contains("stale readiness detail receipt")
    );
    #[cfg(unix)]
    {
        fs::remove_file(temporary.join("results.json"))?;
        std::os::unix::fs::symlink(
            source.join("fixtures/agent-completion/results.json"),
            temporary.join("results.json"),
        )?;
        let unsafe_input = observe(&"c".repeat(64))?;
        assert_eq!(unsafe_input["classification"], "unavailable");
        assert_eq!(unsafe_input["mutation_authorized"], false);
    }
    assert_eq!(
        fs::read(temporary.join(".cairn/changes/demo/tasks.md"))?,
        before
    );
    fs::remove_dir_all(temporary)?;
    Ok(())
}
