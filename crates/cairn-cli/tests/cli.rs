use std::{env, error::Error, fs, path::PathBuf, process::Command, time::SystemTime};

use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn cairn() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cairn"));
    command.current_dir(repo_root());
    command
}

fn temp_test_dir(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_nanos();
    let dir = env::temp_dir().join(format!(
        "cairn-cli-test-{name}-{}-{nanos}",
        std::process::id()
    ));
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn write_enabled_steel_policy(dir: &std::path::Path) -> Result<PathBuf, Box<dyn Error>> {
    let policy_path = repo_root().join("cairn-policy/generated/cairn-policy.json");
    let policy_text = fs::read_to_string(policy_path)?;
    let mut policy: Value = serde_json::from_str(&policy_text)?;
    policy["steel_orchestration_policy"] = serde_json::json!({
        "enabled": true,
        "profiles": [{
            "id": "comparison.lifecycle.closeout",
            "mode": "comparison",
            "allowed_host_functions": ["cairn.validate"],
            "deterministic_budgets": ["no-fs-mutation"],
            "redaction_classes": ["safe_metadata"],
            "fallback_modes": ["rust-nickel-canonical"],
            "receipt_schema_version": "cairn.steel-orchestration-receipt.v1"
        }]
    });
    let output_path = dir.join("policy.json");
    fs::write(&output_path, serde_json::to_string_pretty(&policy)?)?;
    Ok(output_path)
}

fn write_script_envelope(
    dir: &std::path::Path,
    host_function: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let script_path = dir.join("steel-plan.json");
    fs::write(
        &script_path,
        serde_json::to_string_pretty(&serde_json::json!({
            "schema_version": "cairn.steel-action-envelope.v1",
            "profile_id": "comparison.lifecycle.closeout",
            "script_hash": "reviewed-script-fixture",
            "actions": [{
                "id": "validate",
                "host_function": host_function,
                "args": {"root": "."}
            }]
        }))?,
    )?;
    Ok(script_path)
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

#[test]
fn steel_orchestration_receipt_plans_reviewed_allowed_script() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("steel-allowed")?;
    let policy_path = write_enabled_steel_policy(&dir)?;
    let script_path = write_script_envelope(&dir, "cairn.validate")?;

    let output = cairn()
        .args([
            "steel-orchestration",
            "receipt",
            "--root",
            ".",
            "--policy",
            policy_path.to_str().ok_or("utf8 policy path")?,
            "--profile",
            "comparison.lifecycle.closeout",
            "--script",
            script_path.to_str().ok_or("utf8 script path")?,
        ])
        .output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    let receipt: Value = serde_json::from_str(&stdout)?;

    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["outcome"], "planned");
    assert_eq!(receipt["fallback_reason"], "none");
    assert_eq!(receipt["steel_enabled"], true);
    assert_eq!(receipt["steel_runtime_available"], true);
    assert_eq!(receipt["script_hash"].as_str().unwrap().len(), 64);
    assert_eq!(receipt["authorized_actions"].as_array().unwrap().len(), 1);
    assert_eq!(
        receipt["authorized_actions"][0]["host_function"],
        "cairn.validate"
    );
    assert_eq!(receipt["denied_actions"].as_array().unwrap().len(), 0);
    assert_eq!(receipt["receipt_hash"].as_str().unwrap().len(), 64);

    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn steel_orchestration_receipt_denies_unapproved_host_function() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("steel-denied")?;
    let policy_path = write_enabled_steel_policy(&dir)?;
    let script_path = write_script_envelope(&dir, "host.fs.write")?;

    let output = cairn()
        .args([
            "steel-orchestration",
            "receipt",
            "--root",
            ".",
            "--policy",
            policy_path.to_str().ok_or("utf8 policy path")?,
            "--profile",
            "comparison.lifecycle.closeout",
            "--script",
            script_path.to_str().ok_or("utf8 script path")?,
        ])
        .output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    let receipt: Value = serde_json::from_str(&stdout)?;

    assert_eq!(receipt["valid"], false);
    assert_eq!(receipt["outcome"], "denied");
    assert_eq!(receipt["fallback_reason"], "none");
    assert_eq!(receipt["authorized_actions"].as_array().unwrap().len(), 0);
    assert_eq!(
        receipt["denied_actions"][0]["code"],
        "host_function_not_allowed"
    );
    assert_eq!(receipt["issues"][0]["class"], "denied");

    fs::remove_dir_all(dir)?;
    Ok(())
}
