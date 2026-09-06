use std::{error::Error, fs, path::PathBuf, process::Command};

use cairn_core::verified::check_evidence::{profile_command, Applicability, CheckProfile};
use cairn_core::{
    authoritative_command_hash, authoritative_selection_hash, canonical_json_hash,
    AgentRequiredCheck, AgentVerificationContract,
};
use serde_json::{json, Value};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn command() -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_cairn"));
    c.current_dir(root());
    c
}

#[test]
fn explicit_execution_and_import_never_use_a_fallback() -> Result<(), Box<dyn Error>> {
    for operation in ["run", "import"] {
        let output = command()
            .args([
                "check-evidence",
                operation,
                "--execute",
                "--plan-hash",
                "anything",
            ])
            .output()?;
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("unavailable"));
    }
    Ok(())
}

#[test]
fn plan_is_current_policy_bound_read_only_and_strict() -> Result<(), Box<dyn Error>> {
    let profiles: Vec<CheckProfile> = serde_json::from_slice(&fs::read(
        root().join("fixtures/check-evidence/profiles.json"),
    )?)?;
    let argv = profile_command(&profiles[0]);
    let checks = vec![AgentRequiredCheck {
        check_id: "tests".into(),
        command_hash: authoritative_command_hash(&argv),
        command: argv,
        prerequisites: vec![],
    }];
    let policy: Value = serde_json::from_slice(&fs::read(
        root().join("cairn-policy/generated/cairn-policy.json"),
    )?)?;
    let contract = AgentVerificationContract {
        schema: "cairn.agent-completion.v2".into(),
        repository_id: "fixture".into(),
        registry_id: "checks".into(),
        registry_hash: "d".repeat(64),
        selection_hash: authoritative_selection_hash(&checks),
        policy_hash: canonical_json_hash(&policy),
        final_subject_hash: "f".repeat(64),
        required_checks: checks,
    };
    let applicability = Applicability {
        subject_hash: contract.final_subject_hash.clone(),
        policy_hash: contract.policy_hash.clone(),
        dependency_hashes: vec![],
        source_projection_hash: "a".repeat(64),
    };
    let fixture = root()
        .join("target")
        .join(format!("check-evidence-cli-{}", std::process::id()));
    fs::create_dir_all(&fixture)?;
    fs::write(
        fixture.join("contract.json"),
        serde_json::to_vec(&contract)?,
    )?;
    fs::write(
        fixture.join("profiles.json"),
        serde_json::to_vec(&profiles)?,
    )?;
    fs::write(
        fixture.join("applicability.json"),
        serde_json::to_vec(&applicability)?,
    )?;
    let args = [
        "check-evidence",
        "plan",
        "--root",
        fixture.to_str().ok_or("path")?,
        "--contract",
        "contract.json",
        "--profiles",
        "profiles.json",
        "--applicability",
        "applicability.json",
    ];
    let first = command().args(args).output()?;
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let receipt: Value = serde_json::from_slice(&first.stdout)?;
    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["checks_executed"], false);
    assert_eq!(receipt["subject_observed"], false);
    assert_eq!(receipt["execution_available"], false);
    assert_eq!(fs::read_dir(&fixture)?.count(), 3);
    assert_eq!(first.stdout, command().args(args).output()?.stdout);
    assert!(!command()
        .args(args)
        .arg("--execute")
        .output()?
        .status
        .success());
    assert!(!command()
        .args(args)
        .args(["--profiles", "profiles.json"])
        .output()?
        .status
        .success());
    let mut stale = applicability;
    stale.policy_hash = "b".repeat(64);
    fs::write(
        fixture.join("applicability.json"),
        serde_json::to_vec(&stale)?,
    )?;
    assert!(!command().args(args).output()?.status.success());
    fs::write(
        fixture.join("profiles.json"),
        serde_json::to_vec(&json!({"raw": "secret-do-not-echo"}))?,
    )?;
    let malformed = command().args(args).output()?;
    assert!(!malformed.status.success());
    assert!(!String::from_utf8_lossy(&malformed.stderr).contains("secret-do-not-echo"));
    fs::remove_dir_all(fixture)?;
    Ok(())
}
