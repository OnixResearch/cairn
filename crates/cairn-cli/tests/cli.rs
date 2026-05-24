use std::{env, error::Error, fs, path::PathBuf, process::Command};

use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn cairn() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cairn"));
    command.current_dir(repo_root());
    command
}

#[test]
fn missing_spec_returns_error() -> Result<(), Box<dyn Error>> {
    let output = cairn()
        .args(["spec", "show", "missing", "--root", "fixtures/basic"])
        .output()?;
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("spec not found: missing"), "{stderr}");
    Ok(())
}

#[test]
fn invalid_policy_returns_parse_error() -> Result<(), Box<dyn Error>> {
    let dir = env::temp_dir().join(format!("cairn-cli-test-{}", std::process::id()));
    fs::create_dir_all(&dir)?;
    let policy_path = dir.join("bad-policy.json");
    fs::write(
        &policy_path,
        r#"{"name":"bad","version":999,"schemas":[{}],"task_marker_policy":{"enabled":true,"strictness":"strict"}}"#,
    )?;

    let policy_path_text = policy_path.to_str().ok_or("utf8 policy path")?;
    let output = cairn()
        .args([
            "validate",
            "--root",
            "fixtures/basic",
            "--policy",
            policy_path_text,
        ])
        .output()?;
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unsupported policy version 999"),
        "{stderr}"
    );

    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn steel_orchestration_receipt_reports_disabled_default_without_effects(
) -> Result<(), Box<dyn Error>> {
    let output = cairn()
        .args(["steel-orchestration", "receipt", "--root", "."])
        .output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    let receipt: Value = serde_json::from_str(&stdout)?;

    assert_eq!(
        receipt["schema_version"],
        "cairn.steel-orchestration-receipt.v1"
    );
    assert_eq!(receipt["command"], "steel-orchestration receipt");
    assert_eq!(receipt["outcome"], "disabled");
    assert_eq!(receipt["fallback_reason"], "policy_disabled");
    assert_eq!(receipt["steel_enabled"], false);
    assert_eq!(receipt["steel_runtime_available"], false);
    assert_eq!(receipt["authorized_actions"].as_array().unwrap().len(), 0);
    assert_eq!(receipt["denied_actions"].as_array().unwrap().len(), 0);
    assert_eq!(receipt["receipt_hash"].as_str().unwrap().len(), 64);
    Ok(())
}

#[test]
fn steel_orchestration_receipt_denies_unknown_profile() -> Result<(), Box<dyn Error>> {
    let output = cairn()
        .args([
            "steel-orchestration",
            "receipt",
            "--root",
            ".",
            "--profile",
            "missing.profile",
        ])
        .output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    let receipt: Value = serde_json::from_str(&stdout)?;

    assert_eq!(receipt["valid"], false);
    assert_eq!(receipt["issues"][0]["code"], "profile_not_found");
    assert_eq!(receipt["issues"][0]["class"], "denied");
    assert_eq!(receipt["fallback_reason"], "policy_disabled");
    Ok(())
}
