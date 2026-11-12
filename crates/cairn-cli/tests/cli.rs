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

fn copy_dir_recursive(source: &PathBuf, dest: &PathBuf) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = dest.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
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

fn steel_fixture(name: &str) -> PathBuf {
    repo_root().join("fixtures/steel-orchestration").join(name)
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

fn write_default_steel_policy(dir: &std::path::Path) -> Result<PathBuf, Box<dyn Error>> {
    let policy_path = repo_root().join("cairn-policy/generated/cairn-policy.json");
    let policy_text = fs::read_to_string(policy_path)?;
    let mut policy: Value = serde_json::from_str(&policy_text)?;
    policy["steel_orchestration_policy"] = serde_json::json!({
        "enabled": true,
        "profiles": [{
            "id": "selected.lifecycle.closeout",
            "mode": "selected_default",
            "allowed_host_functions": ["cairn.validate"],
            "deterministic_budgets": ["no-fs-mutation"],
            "redaction_classes": ["safe_metadata"],
            "fallback_modes": ["rust-nickel-canonical"],
            "receipt_schema_version": "cairn.steel-orchestration-receipt.v1"
        }]
    });
    let output_path = dir.join("default-policy.json");
    fs::write(&output_path, serde_json::to_string_pretty(&policy)?)?;
    Ok(output_path)
}

fn write_malformed_steel_policy(dir: &std::path::Path) -> Result<PathBuf, Box<dyn Error>> {
    let policy_path = repo_root().join("cairn-policy/generated/cairn-policy.json");
    let policy_text = fs::read_to_string(policy_path)?;
    let mut policy: Value = serde_json::from_str(&policy_text)?;
    policy["steel_orchestration_policy"] = serde_json::json!({
        "enabled": true,
        "profiles": [{
            "id": "comparison.lifecycle.closeout",
            "mode": "comparison"
        }]
    });
    let output_path = dir.join("malformed-policy.json");
    fs::write(&output_path, serde_json::to_string_pretty(&policy)?)?;
    Ok(output_path)
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
    // r[verify cairn.lifecycle_orchestration.steel_lifecycle_orchestration.boundary_split.dispatch]
    // r[verify cairn.lifecycle_orchestration.steel_lifecycle_orchestration.boundary_split.receipts]
    // r[verify cairn.lifecycle_orchestration.steel_lifecycle_orchestration.boundary_split.tests]
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
    assert_eq!(receipt["effects_executed"], false);
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
    // r[verify cairn.lifecycle_orchestration.steel_lifecycle_orchestration.boundary_split.evaluation]
    let dir = temp_test_dir("steel-allowed")?;
    let policy_path = write_enabled_steel_policy(&dir)?;
    let script_path = steel_fixture("allowed-plan.json");

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
    assert_eq!(receipt["effects_executed"], false);
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
    let script_path = steel_fixture("denied-host-effect.json");

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

#[test]
fn steel_orchestration_receipt_falls_back_when_script_missing() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("steel-fallback")?;
    let policy_path = write_enabled_steel_policy(&dir)?;

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
        ])
        .output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    let receipt: Value = serde_json::from_str(&stdout)?;

    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["outcome"], "fallback");
    assert_eq!(receipt["fallback_reason"], "steel_runtime_unavailable");
    assert_eq!(receipt["steel_runtime_available"], false);
    assert_eq!(receipt["effects_executed"], false);
    assert_eq!(receipt["issues"][0]["code"], "steel_runtime_unavailable");

    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn steel_orchestration_receipt_denies_malformed_envelope() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("steel-malformed-envelope")?;
    let policy_path = write_enabled_steel_policy(&dir)?;
    let script_path = steel_fixture("malformed-envelope.json");

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
    let issue_codes = receipt["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(receipt["valid"], false);
    assert_eq!(receipt["outcome"], "denied");
    assert!(issue_codes.contains(&"invalid_string"));
    assert!(issue_codes.contains(&"empty_actions"));
    assert_eq!(receipt["authorized_actions"].as_array().unwrap().len(), 0);
    assert_eq!(receipt["effects_executed"], false);

    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn steel_orchestration_receipt_rejects_malformed_policy_surface() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("steel-malformed-policy")?;
    let policy_path = write_malformed_steel_policy(&dir)?;

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
        ])
        .output()?;
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("allowed_host_functions"), "{stderr}");

    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn steel_orchestration_receipt_is_stable_and_redacts_action_args() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("steel-stability-redaction")?;
    let policy_path = write_enabled_steel_policy(&dir)?;
    let script_path = steel_fixture("secret-redaction-plan.json");
    let mut stdout_values = Vec::new();

    for _ in 0..2 {
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
        assert!(!stdout.contains("STEEL_SECRET_DO_NOT_LEAK"));
        stdout_values.push(stdout);
    }

    assert_eq!(stdout_values[0], stdout_values[1]);
    let receipt: Value = serde_json::from_str(&stdout_values[0])?;
    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["mode"], "comparison");
    assert_eq!(receipt["outcome"], "planned");
    assert_eq!(receipt["effects_executed"], false);
    assert!(receipt["authorized_actions"][0].get("args").is_none());
    assert_eq!(
        receipt["authorized_actions"][0]["host_function"],
        "cairn.validate"
    );
    assert_eq!(
        receipt["authorized_actions"][0]["request_hash"]
            .as_str()
            .unwrap()
            .len(),
        64
    );

    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn steel_orchestration_default_gate_denies_default_without_evidence() -> Result<(), Box<dyn Error>>
{
    let dir = temp_test_dir("steel-default-denied")?;
    let policy_path = write_default_steel_policy(&dir)?;

    let output = cairn()
        .args([
            "steel-orchestration",
            "default-gate",
            "--root",
            ".",
            "--policy",
            policy_path.to_str().ok_or("utf8 policy path")?,
            "--profile",
            "selected.lifecycle.closeout",
        ])
        .output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    let receipt: Value = serde_json::from_str(&stdout)?;
    let issue_codes = receipt["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(receipt["valid"], false);
    assert_eq!(receipt["verdict"], "FAIL");
    assert_eq!(receipt["default_candidate"], true);
    assert_eq!(receipt["operator_diagnostics_visible"], false);
    assert!(issue_codes.contains(&"missing_promotion_evidence"));

    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn steel_orchestration_default_gate_accepts_complete_promotion_evidence(
) -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("steel-default-accepted")?;
    let policy_path = write_default_steel_policy(&dir)?;
    let evidence_hash = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    let output = cairn()
        .args([
            "steel-orchestration",
            "default-gate",
            "--root",
            ".",
            "--policy",
            policy_path.to_str().ok_or("utf8 policy path")?,
            "--profile",
            "selected.lifecycle.closeout",
            "--equivalence-receipt-hash",
            evidence_hash,
            "--fallback-receipt-hash",
            evidence_hash,
            "--ci-reproducibility-receipt-hash",
            evidence_hash,
            "--diagnostics-receipt-hash",
            evidence_hash,
        ])
        .output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    let receipt: Value = serde_json::from_str(&stdout)?;

    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["verdict"], "PASS");
    assert_eq!(receipt["mode"], "selected_default");
    assert_eq!(receipt["default_candidate"], true);
    assert_eq!(receipt["operator_diagnostics_visible"], true);
    assert_eq!(
        receipt["evidence"]["equivalence_receipt_hash"],
        evidence_hash
    );
    assert_eq!(receipt["fallback_modes"][0], "rust-nickel-canonical");
    assert_eq!(receipt["receipt_hash"].as_str().unwrap().len(), 64);

    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn read_commands_reject_unknown_and_malformed_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &["status", "--root", "fixtures/basic", "--bogus"][..],
        &["change", "list", "--root", "fixtures/basic", "--group"][..],
        &[
            "change",
            "list",
            "--root",
            "fixtures/basic",
            "--group",
            "--bogus",
        ][..],
        &[
            "change",
            "list",
            "--root",
            "fixtures/basic",
            "--group",
            "a",
            "--group",
            "b",
        ][..],
        &["change", "graph", "--root", "fixtures/basic", "--format"][..],
        &[
            "change",
            "show",
            "demo",
            "--root",
            "fixtures/basic",
            "--bogus",
        ][..],
        &["spec", "list", "--root", "fixtures/basic", "--bogus"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option")
                || stderr.contains("requires a value")
                || stderr.contains("duplicate option")
                || stderr.contains("before options")
                || stderr.contains("non-negative integer"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn remaining_commands_reject_unknown_and_value_less_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &["traceability", "coverage", "--root", ".", "--bogus"][..],
        &["tracey", "coverage", "--root", ".", "--bogus"][..],
        &["determinism", "coverage-audit", "--root", ".", "--bogus"][..],
        &["preserves-corpus", "explain", "--bogus"][..],
        &["preserves-corpus", "update", "--bogus"][..],
        &["policy", "--bogus"][..],
        &["policy", "export", "--bogus"][..],
        &["rename-classification", "--bogus"][..],
        &["test-harness", "--bogus"][..],
        &["test-harness", "--case"][..],
        &["test-harness", "--max-reruns", "not-a-number"][..],
        &["policy", "export", "--source"][..],
        &["preserves-corpus", "update", "--path"][..],
        &[
            "traceability",
            "coverage",
            "--root",
            ".",
            "--json",
            "--json",
        ][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option")
                || stderr.contains("requires a value")
                || stderr.contains("duplicate option")
                || stderr.contains("before options")
                || stderr.contains("non-negative integer"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn developer_and_archive_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &["template", "list", "--bogus"][..],
        &["template", "apply", "generic-cli", "--bogus"][..],
        &["template", "apply", "--bogus"][..],
        &["scaffold", "change", "demo", "--bogus"][..],
        &["scaffold", "change", "--bogus"][..],
        &["discover-specs", "--bogus"][..],
        &["steel-orchestration", "receipt", "--bogus"][..],
        &["compact", "verify", "--bogus"][..],
        &["discover-specs", "--include"][..],
        &["steel-orchestration", "receipt", "--profile"][..],
        &["compact", "verify", "--policy"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option")
                || stderr.contains("requires a value")
                || stderr.contains("duplicate option")
                || stderr.contains("before options"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn lifecycle_mutation_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &["init", "--root", "fixtures/basic", "--bogus"][..],
        &[
            "change",
            "create",
            "demo-next",
            "--root",
            "fixtures/basic",
            "--bogus",
        ][..],
        &[
            "gate",
            "proposal",
            "demo",
            "--root",
            "fixtures/basic",
            "--bogus",
        ][..],
        &[
            "gate-context",
            "demo",
            "--root",
            "fixtures/basic",
            "--bogus",
        ][..],
        &["sync", "demo", "--root", "fixtures/basic", "--bogus"][..],
        &["archive", "demo", "--root", "fixtures/basic", "--bogus"][..],
        &["gate", "proposal", "demo", "--root"][..],
        &["sync", "demo", "--root"][..],
        &["archive", "demo", "--root"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option")
                || stderr.contains("requires a value")
                || stderr.contains("duplicate option"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn release_evidence_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &[
            "release-evidence",
            "index",
            "--root",
            ".",
            "--out",
            "target/i.md",
            "--bogus",
        ][..],
        &[
            "release-evidence",
            "check-index",
            "--index",
            "index.md",
            "--bogus",
        ][..],
        &[
            "release-evidence",
            "publish",
            "--index",
            "index.md",
            "--artifact-dir",
            "artifacts",
            "--publish-dir",
            "published",
            "--bogus",
        ][..],
        &[
            "release-evidence",
            "retention-plan",
            "--publish-root",
            "published",
            "--out",
            "plan.json",
            "--bogus",
        ][..],
        &[
            "release-evidence",
            "rotate",
            "--plan",
            "plan.json",
            "--bogus",
        ][..],
        &["release-evidence", "index", "--out"][..],
        &["release-evidence", "check-index", "--mode"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option") || stderr.contains("requires a value"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn agent_receipt_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    let receipt = "fixtures/agent-receipts/positive.json";
    for args in [
        &["agent-receipt", "validate", receipt, "--bogus"][..],
        &["agent-receipt", "validate", receipt, "extra.json"][..],
        &["agent-receipt", "preserves-hash", receipt, "--bogus"][..],
        &[
            "agent-receipt",
            "complete",
            "--contract",
            "c.json",
            "--results",
            "r.json",
            "--subject-hash",
            &"c".repeat(64),
            "--bogus",
        ][..],
        &["agent-receipt", "verify-bundle", receipt, "--bogus"][..],
        &["agent-receipt", "verify-bundle", receipt, "extra.json"][..],
        &["agent-receipt", "smoke-mcp", "--bogus"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option")
                || stderr.contains("requires a value")
                || stderr.contains("duplicate option")
                || stderr.contains("unexpected")
                || stderr.contains("exactly one"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn validator_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &[
            "hash-policy",
            "verify",
            "--input",
            "fixtures/hash-algorithm-policy/positive/blake3-default.json",
            "--bogus",
        ][..],
        &[
            "toolchain-profile",
            "verify",
            "--input",
            "fixtures/toolchain-profile-matrix/positive/profile-classes.json",
            "--bogus",
        ][..],
        &[
            "workspace-contract",
            "verify",
            "--input",
            "fixtures/workspace-contract-matrix/positive/representative-edges.json",
            "--bogus",
        ][..],
        &[
            "requirement-registry",
            "verify",
            "--input",
            "fixtures/requirement-id-registry/positive/owned-id.json",
            "--bogus",
        ][..],
        &[
            "requirement-registry",
            "verify",
            "--root",
            ".",
            "--repository-id",
            "demo",
            "--registry",
            "fixtures/requirement-id-registry/positive/owned-id.json",
            "--bogus",
        ][..],
        &[
            "runtime-evidence",
            "verify",
            "--chain",
            "fixtures/runtime-evidence/positive/chain.json",
            "--bogus",
        ][..],
        &["hash-policy", "verify", "--input"][..],
        &["runtime-evidence", "verify", "--chain"][..],
        &["requirement-registry", "export", "--root", "."][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option")
                || stderr.contains("requires")
                || stderr.contains("requires a value"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn test_harness_rejects_escaping_case_seed_and_out_paths() -> Result<(), Box<dyn Error>> {
    for args in [
        &["test-harness", "--seed", "../escape"][..],
        &["test-harness", "--case", "../escape"][..],
        &[
            "test-harness",
            "--out",
            "/tmp/cairn-test-harness-outside.json",
        ][..],
        &[
            "test-harness",
            "--out",
            "../cairn-test-harness-outside.json",
        ][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("safe path component")
                || stderr.contains("repository-relative")
                || stderr.contains("cannot be combined")
                || stderr.contains("capability"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn implementation_ready_rejects_legacy_lifecycle_roots() -> Result<(), Box<dyn Error>> {
    let root = std::env::temp_dir().join(format!(
        "cairn-implementation-ready-legacy-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("cairn/specs"))?;
    let output = cairn()
        .args(["implementation-ready", "--root"])
        .arg(&root)
        .output()?;
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("migration_required"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn registry_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &[
            "registry",
            "verify",
            "--registry",
            "registry.json",
            "--bogus",
        ][..],
        &[
            "registry",
            "query",
            "--scope",
            "s",
            "--adapter",
            "a",
            "--registry",
            "r",
            "--tag",
            "k=v",
            "--bogus",
        ][..],
        &[
            "registry",
            "import",
            "--scope",
            "s",
            "--adapter",
            "a",
            "--store",
            "b",
            "--ingest",
            "c",
            "--input",
            "d",
            "--bogus",
        ][..],
        &["registry", "verify", "--registry"][..],
        &[
            "registry",
            "graph",
            "--scope",
            "s",
            "--adapter",
            "a",
            "--registry",
            "r",
            "--profile",
        ][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option") || stderr.contains("requires a value"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn evidence_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &[
            "evidence",
            "export",
            "--root",
            ".",
            "--out",
            "target/ev",
            "--bogus",
        ][..],
        &[
            "evidence", "verify", "--root", ".", "--bundle", "b", "--bogus",
        ][..],
        &[
            "evidence", "import", "--root", ".", "--bundle", "b", "--into", "i", "--bogus",
        ][..],
        &[
            "evidence",
            "republish",
            "--root",
            ".",
            "--imported",
            "i",
            "--out",
            "o",
            "--bogus",
        ][..],
        &[
            "evidence", "index", "--root", ".", "--bundle", "b", "--out", "o", "--bogus",
        ][..],
        &[
            "evidence", "index", "verify", "--root", ".", "--index", "i", "--bogus",
        ][..],
        &["evidence", "export", "--out"][..],
        &["evidence", "import", "--bundle"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option") || stderr.contains("requires a value"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn review_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &["review", "validate", "receipt.json", "--bogus"][..],
        &["review", "ledger", "--include", "receipts", "--bogus"][..],
        &["review", "revision", "validate", "receipt.json", "--bogus"][..],
        &["review", "completeness", "--bogus"][..],
        &[
            "review",
            "metrics-regression",
            "--registry",
            "r",
            "--fixtures",
            "f",
            "--guidance",
            "g",
            "--evidence",
            "e",
            "--bogus",
        ][..],
        &["review", "ledger", "--include"][..],
        &["review", "metrics-regression", "--registry"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option") || stderr.contains("requires a value"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn evidence_registry_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &[
            "evidence", "registry", "build", "--index", "i", "--out", "o", "--bogus",
        ][..],
        &[
            "evidence",
            "registry",
            "verify",
            "--registry",
            "r",
            "--bogus",
        ][..],
        &[
            "evidence",
            "registry",
            "select",
            "--registry",
            "r",
            "--out",
            "o",
            "--bogus",
        ][..],
        &[
            "evidence",
            "registry",
            "fetch-plan",
            "--registry",
            "r",
            "--out",
            "o",
            "--bogus",
        ][..],
        &[
            "evidence",
            "registry",
            "fetch-execute",
            "--plan",
            "p",
            "--source",
            "s",
            "--out",
            "o",
            "--bogus",
        ][..],
        &["evidence", "registry", "build", "--index"][..],
        &["evidence", "registry", "verify", "--registry"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option") || stderr.contains("requires a value"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn implementation_ready_rejects_invalid_limits() -> Result<(), Box<dyn Error>> {
    for limit in ["0", "1001", "not-a-number"] {
        let output = cairn()
            .args([
                "implementation-ready",
                "--root",
                "fixtures/basic",
                "--limit",
                limit,
            ])
            .output()?;
        assert!(!output.status.success(), "{limit} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("limit"),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn evidence_manifest_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &["evidence-manifest", "generate", "--bundle", "b", "--bogus"][..],
        &["evidence-manifest", "generate", "--bundle"][..],
        &["evidence-manifest", "verify", "manifest.json", "--bogus"][..],
        &["evidence-manifest", "verify", "--root"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option") || stderr.contains("requires a value"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn evidence_manifest_verify_rejects_multiple_paths() -> Result<(), Box<dyn Error>> {
    let output = cairn()
        .args(["evidence-manifest", "verify", "one.json", "two.json"])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("exactly one manifest path"));
    Ok(())
}

#[test]
fn option_only_commands_reject_stray_positionals() -> Result<(), Box<dyn Error>> {
    for args in [
        &["policy", "stray"][..],
        &["rename-classification", "stray"][..],
        &["test-harness", "stray"][..],
        &["determinism", "coverage-audit", "--root", ".", "stray"][..],
        &["traceability", "coverage", "--root", ".", "stray"][..],
        &["evidence-manifest", "generate", "stray"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unexpected argument"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn evidence_registry_and_release_commands_reject_stray_positionals() -> Result<(), Box<dyn Error>> {
    for args in [
        &["evidence", "export", "--out", "x", "stray"][..],
        &["evidence", "registry", "verify", "--registry", "r", "stray"][..],
        &["registry", "verify", "--registry", "r", "stray"][..],
        &["release-evidence", "index", "--out", "x", "stray"][..],
        &["release-evidence", "rotate", "--plan", "p", "stray"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unexpected argument"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn implementation_ready_rejects_stray_positionals() -> Result<(), Box<dyn Error>> {
    let output = cairn()
        .args(["implementation-ready", "--root", "fixtures/basic", "stray"])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unexpected argument"));
    Ok(())
}

#[test]
fn review_receipt_commands_reject_multiple_paths() -> Result<(), Box<dyn Error>> {
    for args in [
        &["review", "validate", "one.json", "two.json"][..],
        &["review", "revision", "validate", "one.json", "two.json"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("exactly one receipt path"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn lifecycle_data_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &["lifecycle", "check", "--source", "x", "--bogus"][..],
        &[
            "lifecycle",
            "export",
            "--source",
            "x",
            "--output",
            "y",
            "--bogus",
        ][..],
        &["lifecycle", "render-markdown", "--source", "x", "--bogus"][..],
        &[
            "lifecycle",
            "import-markdown",
            "--root",
            ".",
            "--out",
            "y",
            "--bogus",
        ][..],
        &["lifecycle-patterns", "validate", "--input", "x", "--bogus"][..],
        &["lifecycle", "check", "--source"][..],
        &["lifecycle", "import-markdown", "--out"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option") || stderr.contains("requires a value"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn review_option_only_subcommands_reject_stray_positionals() -> Result<(), Box<dyn Error>> {
    for args in [
        &["review", "ledger", "--include", "x", "stray"][..],
        &["review", "completeness", "stray"][..],
        &[
            "review",
            "metrics-regression",
            "--registry",
            "r",
            "--fixtures",
            "f",
            "--guidance",
            "g",
            "--evidence",
            "e",
            "stray",
        ][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unexpected argument"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn compact_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &[
            "compact",
            "archive",
            "--root",
            "fixtures/archive-ready",
            "--bogus",
        ][..],
        &[
            "compact",
            "unpack",
            "--root",
            "fixtures/archive-ready",
            "--archive",
            "x",
            "--bogus",
        ][..],
        &["compact", "archive", "--plan"][..],
        &["compact", "unpack", "--archive"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option") || stderr.contains("requires a value"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn compact_verify_rejects_stray_positionals() -> Result<(), Box<dyn Error>> {
    let output = cairn().args(["compact", "verify", "stray"]).output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unexpected argument"));
    Ok(())
}

#[test]
fn probe_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &[
            "probe",
            "run",
            "--root",
            ".",
            "--probe-file",
            "p",
            "--bogus",
        ][..],
        &[
            "probe",
            "validate",
            "--root",
            ".",
            "--probe-file",
            "p",
            "--receipt",
            "r",
            "--bogus",
        ][..],
        &["probe", "run", "--probe-file"][..],
        &["probe", "validate", "--receipt"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option") || stderr.contains("requires a value"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn validator_commands_reject_stray_positionals() -> Result<(), Box<dyn Error>> {
    for args in [
        &["hash-policy", "verify", "--input", "x", "stray"][..],
        &["toolchain-profile", "verify", "--input", "x", "stray"][..],
        &["workspace-contract", "verify", "--input", "x", "stray"][..],
        &["requirement-registry", "verify", "--input", "x", "stray"][..],
        &["runtime-evidence", "verify", "--chain", "x", "stray"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unexpected argument"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn rfd_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &[
            "rfd",
            "status",
            "--store",
            "x",
            "--document-id",
            "y",
            "--bogus",
        ][..],
        &[
            "rfd",
            "conflict",
            "--store",
            "x",
            "--document-id",
            "y",
            "--bogus",
        ][..],
        &[
            "rfd",
            "checkout",
            "--store",
            "x",
            "--document-id",
            "y",
            "--out",
            "o",
            "--bogus",
        ][..],
        &[
            "rfd",
            "number",
            "--store",
            "x",
            "--document-id",
            "y",
            "--number",
            "1",
            "--bogus",
        ][..],
        &[
            "rfd",
            "publish",
            "--store",
            "x",
            "--document-id",
            "y",
            "--bogus",
        ][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unknown option"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn rfd_mutations_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &[
            "rfd",
            "import",
            "--store",
            "x",
            "--document-id",
            "y",
            "--base-revision",
            "a:b",
            "--spec",
            "s",
            "--bogus",
        ][..],
        &[
            "rfd",
            "update",
            "--store",
            "x",
            "--document-id",
            "y",
            "--actor",
            "a",
            "--kind",
            "discussion",
            "--text",
            "t",
            "--bogus",
        ][..],
        &[
            "rfd",
            "resolve",
            "--store",
            "x",
            "--document-id",
            "y",
            "--actor",
            "a",
            "--text",
            "t",
            "--bogus",
        ][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unknown option"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn adr_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &[
            "adr",
            "status",
            "--store",
            "x",
            "--document-id",
            "y",
            "--bogus",
        ][..],
        &[
            "adr",
            "conflict",
            "--store",
            "x",
            "--document-id",
            "y",
            "--bogus",
        ][..],
        &[
            "adr",
            "update",
            "--store",
            "x",
            "--document-id",
            "y",
            "--actor",
            "a",
            "--text",
            "t",
            "--bogus",
        ][..],
        &[
            "adr",
            "accept",
            "--store",
            "x",
            "--document-id",
            "y",
            "--actor",
            "a",
            "--bogus",
        ][..],
        &[
            "adr",
            "deprecate",
            "--store",
            "x",
            "--document-id",
            "y",
            "--actor",
            "a",
            "--bogus",
        ][..],
        &[
            "adr",
            "supersede",
            "--store",
            "x",
            "--document-id",
            "y",
            "--actor",
            "a",
            "--replaces",
            "z",
            "--bogus",
        ][..],
        &[
            "adr",
            "resolve",
            "--store",
            "x",
            "--document-id",
            "y",
            "--actor",
            "a",
            "--text",
            "t",
            "--bogus",
        ][..],
        &[
            "adr",
            "number",
            "--store",
            "x",
            "--document-id",
            "y",
            "--number",
            "1",
            "--bogus",
        ][..],
        &[
            "adr",
            "publish",
            "--store",
            "x",
            "--document-id",
            "y",
            "--bogus",
        ][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unknown option"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn vo_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &[
            "verification-obligation",
            "status",
            "--store",
            "x",
            "--obligation-id",
            "y",
            "--bogus",
        ][..],
        &[
            "verification-obligation",
            "conflict",
            "--store",
            "x",
            "--obligation-id",
            "y",
            "--bogus",
        ][..],
        &[
            "verification-obligation",
            "export",
            "--store",
            "x",
            "--obligation-id",
            "y",
            "--bogus",
        ][..],
        &[
            "verification-obligation",
            "update",
            "--store",
            "x",
            "--obligation-id",
            "y",
            "--actor",
            "a",
            "--text",
            "t",
            "--bogus",
        ][..],
        &[
            "verification-obligation",
            "evidence",
            "--store",
            "x",
            "--obligation-id",
            "y",
            "--bogus",
        ][..],
        &[
            "verification-obligation",
            "finding",
            "--store",
            "x",
            "--obligation-id",
            "y",
            "--bogus",
        ][..],
        &[
            "verification-obligation",
            "response",
            "--store",
            "x",
            "--obligation-id",
            "y",
            "--bogus",
        ][..],
        &[
            "verification-obligation",
            "discharge",
            "--store",
            "x",
            "--obligation-id",
            "y",
            "--bogus",
        ][..],
        &[
            "verification-obligation",
            "reject",
            "--store",
            "x",
            "--obligation-id",
            "y",
            "--bogus",
        ][..],
        &[
            "verification-obligation",
            "invalidate",
            "--store",
            "x",
            "--obligation-id",
            "y",
            "--bogus",
        ][..],
        &[
            "verification-obligation",
            "resolve",
            "--store",
            "x",
            "--obligation-id",
            "y",
            "--bogus",
        ][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unknown option"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn release_decision_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &[
            "release-decision",
            "status",
            "--store",
            "x",
            "--decision-id",
            "y",
            "--bogus",
        ][..],
        &[
            "release-decision",
            "conflict",
            "--store",
            "x",
            "--decision-id",
            "y",
            "--bogus",
        ][..],
        &[
            "release-decision",
            "export",
            "--store",
            "x",
            "--decision-id",
            "y",
            "--bogus",
        ][..],
        &[
            "release-decision",
            "evidence",
            "--store",
            "x",
            "--decision-id",
            "y",
            "--bogus",
        ][..],
        &[
            "release-decision",
            "finding",
            "--store",
            "x",
            "--decision-id",
            "y",
            "--bogus",
        ][..],
        &[
            "release-decision",
            "vote",
            "--store",
            "x",
            "--decision-id",
            "y",
            "--bogus",
        ][..],
        &[
            "release-decision",
            "withdraw",
            "--store",
            "x",
            "--decision-id",
            "y",
            "--bogus",
        ][..],
        &[
            "release-decision",
            "invalidate",
            "--store",
            "x",
            "--decision-id",
            "y",
            "--bogus",
        ][..],
        &[
            "release-decision",
            "supersede",
            "--store",
            "x",
            "--decision-id",
            "y",
            "--bogus",
        ][..],
        &[
            "release-decision",
            "create",
            "--store",
            "x",
            "--decision-id",
            "y",
            "--bogus",
        ][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unknown option"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn policy_exception_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &[
            "policy-exception",
            "status",
            "--store",
            "x",
            "--exception-id",
            "y",
            "--bogus",
        ][..],
        &[
            "policy-exception",
            "conflict",
            "--store",
            "x",
            "--exception-id",
            "y",
            "--bogus",
        ][..],
        &[
            "policy-exception",
            "export",
            "--store",
            "x",
            "--exception-id",
            "y",
            "--bogus",
        ][..],
        &[
            "policy-exception",
            "approve",
            "--store",
            "x",
            "--exception-id",
            "y",
            "--bogus",
        ][..],
        &[
            "policy-exception",
            "reject",
            "--store",
            "x",
            "--exception-id",
            "y",
            "--bogus",
        ][..],
        &[
            "policy-exception",
            "revoke",
            "--store",
            "x",
            "--exception-id",
            "y",
            "--bogus",
        ][..],
        &[
            "policy-exception",
            "expire",
            "--store",
            "x",
            "--exception-id",
            "y",
            "--bogus",
        ][..],
        &[
            "policy-exception",
            "close",
            "--store",
            "x",
            "--exception-id",
            "y",
            "--bogus",
        ][..],
        &[
            "policy-exception",
            "supersede",
            "--store",
            "x",
            "--exception-id",
            "y",
            "--bogus",
        ][..],
        &[
            "policy-exception",
            "observe-validity",
            "--store",
            "x",
            "--exception-id",
            "y",
            "--bogus",
        ][..],
        &[
            "policy-exception",
            "request",
            "--store",
            "x",
            "--exception-id",
            "y",
            "--bogus",
        ][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unknown option"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn release_readiness_rejects_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &["release-readiness", "--bogus"][..],
        &["release-readiness", "--review-ledger"][..],
        &["release-readiness", "stray"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option")
                || stderr.contains("requires a value")
                || stderr.contains("unexpected argument"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn release_run_rejects_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &["release-run", "status", "--bogus"][..],
        &["release-run", "status", "--input"][..],
        &["release-run", "explain", "stray"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option")
                || stderr.contains("requires a value")
                || stderr.contains("unexpected argument"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn migrate_rejects_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &["migrate", "cairn", "--bogus"][..],
        &["migrate", "cairn", "--plan-hash"][..],
        &["migrate", "cairn", "stray"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option")
                || stderr.contains("requires a value")
                || stderr.contains("unexpected argument"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn determinism_verify_receipt_rejects_bad_arguments() -> Result<(), Box<dyn Error>> {
    for args in [
        &["determinism", "verify-receipt", "--bogus", "receipt.json"][..],
        &["determinism", "verify-receipt", "--root"][..],
        &["determinism", "verify-receipt", "one.json", "two.json"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option")
                || stderr.contains("requires a value")
                || stderr.contains("exactly one"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn developer_loop_plan_accepts_policy_before_profile() -> Result<(), Box<dyn Error>> {
    let output = cairn()
        .args([
            "developer-loop",
            "plan",
            "--policy",
            "cairn-policy/generated/cairn-policy.json",
            "cairn-local",
        ])
        .output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(value["profile"], "cairn-local");
    Ok(())
}

#[test]
fn developer_loop_rejects_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &["developer-loop", "list", "--bogus"][..],
        &["developer-loop", "list", "stray"][..],
        &["developer-loop", "plan", "cairn-local", "extra"][..],
        &["developer-loop", "verify", "cairn-local", "--bogus"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option")
                || stderr.contains("unexpected argument")
                || stderr.contains("exactly one")
                || stderr.contains("does not accept a profile ID"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn preserves_corpus_rejects_stray_positionals() -> Result<(), Box<dyn Error>> {
    for args in [
        &["preserves-corpus", "update", "--path", "x", "stray"][..],
        &["preserves-corpus", "explain", "--path", "x", "stray"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(String::from_utf8_lossy(&output.stderr).contains("unexpected argument"));
    }
    Ok(())
}

#[test]
fn authoring_and_steel_reject_stray_positionals() -> Result<(), Box<dyn Error>> {
    for args in [
        &["init", "stray"][..],
        &["change", "depend", "list", "demo", "stray"][..],
        &["change", "depend", "add", "demo", "dep", "stray"][..],
        &["steel-orchestration", "receipt", "stray"][..],
        &[
            "steel-orchestration",
            "default-gate",
            "--root",
            ".",
            "stray",
        ][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unexpected argument"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn lifecycle_commands_reject_stray_positionals() -> Result<(), Box<dyn Error>> {
    for args in [
        &["gate", "tasks", "demo", "stray"][..],
        &["gate-context", "demo", "stray"][..],
        &["sync", "demo", "stray"][..],
        &["archive", "demo", "stray"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unexpected argument"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn receipt_commands_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &["receipt-preserves-diff", "--baseline", "a", "--bogus"][..],
        &[
            "receipt-preserves-diff",
            "--baseline",
            "a",
            "--candidate",
            "b",
            "stray",
        ][..],
        &["receipt-verify-content-hash", "--bogus"][..],
        &["receipt-verify-content-hash", "--path", "a", "stray"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option") || stderr.contains("unexpected argument"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn state_machine_and_trace_index_reject_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &[
            "state-machine-receipt",
            "validate",
            "--input",
            "x",
            "--bogus",
        ][..],
        &["state-machine-receipt", "validate", "--input", "x", "stray"][..],
        &["traceability", "export", "--root", ".", "--bogus"][..],
        &[
            "traceability",
            "export",
            "--registry",
            "r",
            "--out",
            "o",
            "stray",
        ][..],
        &["traceability", "verify", "--root", ".", "--bogus"][..],
        &["traceability", "show", "req", "--index", "i", "stray"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option") || stderr.contains("unexpected argument"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn validate_rejects_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &["validate", "--bogus"][..],
        &["validate", "--cross-repo-evidence"][..],
        &["validate", "stray"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option")
                || stderr.contains("requires a value")
                || stderr.contains("unexpected argument"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn adoption_report_rejects_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &["adoption", "report", "--snapshot", "--root", "."][..],
        &["adoption", "report", "--snapshot", "a", "--snapshot", "b"][..],
        &["adoption", "report", "--snapshot", "a", "stray"][..],
        &["adoption", "report", "--bogus"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option")
                || stderr.contains("requires a value")
                || stderr.contains("duplicate option")
                || stderr.contains("unexpected argument"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn content_addressed_evidence_rejects_extra_arguments() -> Result<(), Box<dyn Error>> {
    for command in [
        "circuit-primitive",
        "downstream-pilot",
        "export-bundle",
        "name-pointer",
        "function-object",
    ] {
        let output = cairn()
            .args(["evidence", command, "validate", "receipt.json", "stray"])
            .output()?;
        assert!(!output.status.success(), "{command} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unexpected evidence argument"),
            "{command}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn status_and_doctor_reject_stray_positionals() -> Result<(), Box<dyn Error>> {
    for args in [&["status", "stray"][..], &["doctor", "stray"][..]] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unexpected argument"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn developer_experience_rejects_stray_positionals() -> Result<(), Box<dyn Error>> {
    for args in [
        &["template", "list", "stray"][..],
        &["template", "apply", "demo", "stray"][..],
        &["scaffold", "change", "demo", "stray"][..],
        &["discover-specs", "stray"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unexpected argument"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn agent_receipt_rejects_stray_positionals() -> Result<(), Box<dyn Error>> {
    for args in [
        &[
            "agent-receipt",
            "complete",
            "--contract",
            "a",
            "--results",
            "b",
            "--subject-hash",
            "c",
            "stray",
        ][..],
        &["agent-receipt", "smoke-mcp", "stray"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unexpected argument"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn change_create_rejects_stray_positionals() -> Result<(), Box<dyn Error>> {
    let output = cairn()
        .args(["change", "create", "demo", "--root", ".", "stray"])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unexpected argument"));
    Ok(())
}

#[test]
fn determinism_replay_rejects_bad_options() -> Result<(), Box<dyn Error>> {
    for args in [
        &["determinism", "replay", "--bogus"][..],
        &["determinism", "replay", "--format"][..],
        &["determinism", "replay", "stray"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("unknown option")
                || stderr.contains("requires a value")
                || stderr.contains("unexpected argument"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn readiness_rejects_option_like_values() -> Result<(), Box<dyn Error>> {
    let output = cairn()
        .args(["readiness", "ready", "--results", "--root", "."])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("require values"));
    Ok(())
}

#[test]
fn config_export_rejects_bad_values() -> Result<(), Box<dyn Error>> {
    for args in [
        &["config", "export", "--kind", "--root", "."][..],
        &["config", "export", "--kind", "store", "--root"][..],
        &[
            "config", "export", "--kind", "store", "--root", ".", "--check", "--check",
        ][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("requires a value")
                || stderr.contains("requires a path")
                || stderr.contains("more than once"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn worksets_reject_option_like_values() -> Result<(), Box<dyn Error>> {
    for args in [
        &["workset", "create", "demo", "--member", "--opener", "x"][..],
        &["workset", "list", "--policy"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(String::from_utf8_lossy(&output.stderr).contains("requires a value"));
    }
    Ok(())
}

#[test]
fn store_rejects_empty_option_values() -> Result<(), Box<dyn Error>> {
    let output = cairn()
        .args(["store", "setup", "demo", "--path", ""])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("option requires a value"));
    Ok(())
}

#[test]
fn collaboration_and_source_review_reject_empty_values() -> Result<(), Box<dyn Error>> {
    for args in [
        &[
            "collaboration",
            "status",
            "demo",
            "--backend",
            "portable-json.v1",
            "--root",
            "",
        ][..],
        &[
            "source-review",
            "verify",
            "--root",
            ".",
            "--attachment",
            "",
            "--policy",
            "p",
        ][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("requires a value") || stderr.contains("value missing"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn view_rejects_empty_option_values() -> Result<(), Box<dyn Error>> {
    let output = cairn().args(["view", "graph", "--root", ""]).output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("requires a value"));
    Ok(())
}

#[test]
fn collaboration_rejects_empty_backend() -> Result<(), Box<dyn Error>> {
    let output = cairn()
        .args(["collaboration", "status", "demo", "--backend", ""])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("--backend requires a value"));
    Ok(())
}

#[test]
fn mcp_rejects_empty_tool_name() -> Result<(), Box<dyn Error>> {
    let output = cairn().args(["mcp", "--call", ""]).output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("requires a tool name"));
    Ok(())
}

#[test]
fn view_rejects_duplicate_options() -> Result<(), Box<dyn Error>> {
    let output = cairn()
        .args(["view", "graph", "--format", "json", "--format", "json"])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("duplicate option"));
    Ok(())
}

#[test]
fn readiness_rejects_duplicate_format() -> Result<(), Box<dyn Error>> {
    let output = cairn()
        .args([
            "readiness",
            "ready",
            "--format",
            "json",
            "--format",
            "summary",
        ])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("duplicate readiness option"));
    Ok(())
}

#[test]
fn workset_rejects_empty_id() -> Result<(), Box<dyn Error>> {
    let output = cairn()
        .args(["workset", "create", "", "--member", ".", "--opener", "x"])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("requires an ID"));
    Ok(())
}

#[test]
fn workflow_rejects_empty_positionals() -> Result<(), Box<dyn Error>> {
    let output = cairn()
        .args(["workflow", "status", "", "--policy", "p"])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("requires a change name"));
    Ok(())
}

#[test]
fn review_validate_rejects_empty_receipt_path() -> Result<(), Box<dyn Error>> {
    let output = cairn().args(["review", "validate", ""]).output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("requires a receipt path"));
    Ok(())
}

#[test]
fn manifest_and_bundle_verify_reject_empty_paths() -> Result<(), Box<dyn Error>> {
    for args in [
        &["agent-receipt", "verify-bundle", ""][..],
        &["evidence-manifest", "verify", ""][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("requires <bundle.json>")
                || stderr.contains("requires <manifest.json>"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn evidence_validators_reject_empty_receipt_paths() -> Result<(), Box<dyn Error>> {
    let mut cases = vec![
        vec!["agent-receipt", "validate", ""],
        vec!["agent-receipt", "preserves-hash", ""],
    ];
    for command in [
        "circuit-primitive",
        "downstream-pilot",
        "export-bundle",
        "name-pointer",
        "function-object",
    ] {
        cases.push(vec!["evidence", command, "validate", ""]);
    }
    for args in cases {
        let output = cairn().args(&args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
    }
    Ok(())
}

#[test]
fn store_rejects_empty_positional_ids() -> Result<(), Box<dyn Error>> {
    for args in [
        &["store", "show", ""][..],
        &["store", "setup", "", "--path", "."][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("requires"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn readiness_rejects_empty_change() -> Result<(), Box<dyn Error>> {
    let output = cairn().args(["readiness", ""]).output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("requires a change"));
    Ok(())
}

#[test]
fn traceability_show_rejects_empty_requirement() -> Result<(), Box<dyn Error>> {
    let output = cairn().args(["traceability", "show", ""]).output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("requires <requirement-id>"));
    Ok(())
}

#[test]
fn agent_workflow_rejects_empty_show_id() -> Result<(), Box<dyn Error>> {
    let output = cairn().args(["agent", "workflow", "show", ""]).output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("requires a workflow ID"));
    Ok(())
}

#[test]
fn change_and_spec_reject_empty_positionals() -> Result<(), Box<dyn Error>> {
    for args in [
        &["change", "create", ""][..],
        &["change", "show", ""][..],
        &["spec", "show", ""][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("requires a"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn lifecycle_dispatch_rejects_empty_change_names() -> Result<(), Box<dyn Error>> {
    for args in [
        &["gate", "tasks", ""][..],
        &["gate-context", ""][..],
        &["sync", ""][..],
        &["archive", ""][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("requires a change name"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn cob_rejects_empty_option_values() -> Result<(), Box<dyn Error>> {
    let output = cairn()
        .args(["cob", "import", "demo", "--git-repository", ""])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("--git-repository requires a value"));
    Ok(())
}

#[test]
fn release_and_policy_honor_explicit_policy_paths() -> Result<(), Box<dyn Error>> {
    let missing =
        std::env::temp_dir().join(format!("cairn-missing-policy-{}.json", std::process::id()));
    let _ = std::fs::remove_file(&missing);
    let missing = missing.to_string_lossy().to_string();
    for args in [
        &["release-decision", "create", "--policy", &missing][..],
        &["policy-exception", "request", "--policy", &missing][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("missing-policy"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn receipt_preserves_diff_honors_explicit_policy_path() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("receipt-policy")?;
    let baseline = dir.join("baseline.json");
    let candidate = dir.join("candidate.json");
    fs::write(&baseline, br#"{"a":1}"#)?;
    fs::write(&candidate, br#"{"a":1}"#)?;
    let missing = dir.join("missing-policy.json");
    let output = cairn()
        .args([
            "receipt-preserves-diff",
            "--baseline",
            &baseline.to_string_lossy(),
            "--candidate",
            &candidate.to_string_lossy(),
            "--policy",
            &missing.to_string_lossy(),
        ])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("missing-policy"));
    let output = cairn()
        .args([
            "receipt-preserves-diff",
            "--baseline",
            &dir.join("missing-baseline.json").to_string_lossy(),
            "--candidate",
            &dir.join("missing-candidate.json").to_string_lossy(),
            "--policy",
            &missing.to_string_lossy(),
        ])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("missing-policy"));
    let policy = repo_root().join("cairn-policy/generated/cairn-policy.json");
    let output = cairn()
        .args([
            "receipt-preserves-diff",
            "--baseline",
            &baseline.to_string_lossy(),
            "--candidate",
            &candidate.to_string_lossy(),
            "--policy",
            &policy.to_string_lossy(),
        ])
        .output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(receipt["valid"], true);
    let output = cairn()
        .args([
            "receipt-preserves-diff",
            "--baseline",
            &baseline.to_string_lossy(),
            "--candidate",
            &candidate.to_string_lossy(),
        ])
        .output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let drifted = dir.join("drifted.json");
    fs::write(&drifted, br#"{"a":2}"#)?;
    let output = cairn()
        .args([
            "receipt-preserves-diff",
            "--baseline",
            &baseline.to_string_lossy(),
            "--candidate",
            &drifted.to_string_lossy(),
            "--policy",
            &policy.to_string_lossy(),
        ])
        .output()?;
    assert!(!output.status.success());
    let receipt: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(receipt["valid"], false);
    assert_eq!(receipt["drift_count"], 1);
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn receipt_verify_content_hash_reports_mismatch() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("receipt-hash")?;
    let path = dir.join("receipt.json");
    fs::write(
        &path,
        br#"{"schema":"cairn.receipt.v1","valid":true,"preserves_content_hash":"0000000000000000000000000000000000000000000000000000000000000000"}"#,
    )?;
    let output = cairn()
        .args([
            "receipt-verify-content-hash",
            "--path",
            &path.to_string_lossy(),
        ])
        .output()?;
    assert!(!output.status.success());
    let receipt: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(receipt["valid"], false);
    assert_eq!(receipt["content_hash_matches"], false);
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn receipt_verify_content_hash_accepts_checked_fixtures() -> Result<(), Box<dyn Error>> {
    for (fixture, present) in [("with-hash.json", true), ("no-hash.json", false)] {
        let path = repo_root()
            .join("fixtures/receipt-verify-content-hash")
            .join(fixture);
        let output = cairn()
            .args([
                "receipt-verify-content-hash",
                "--path",
                &path.to_string_lossy(),
            ])
            .output()?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let receipt: Value = serde_json::from_slice(&output.stdout)?;
        assert_eq!(receipt["valid"], true);
        assert_eq!(receipt["preserves_content_hash_present"], present);
    }
    Ok(())
}

#[test]
fn receipt_preserves_diff_reports_checked_fixture_drift() -> Result<(), Box<dyn Error>> {
    let baseline = repo_root().join("fixtures/receipt-preserves-diff/baseline.json");
    let candidate = repo_root().join("fixtures/receipt-preserves-diff/candidate.json");
    let output = cairn()
        .args([
            "receipt-preserves-diff",
            "--baseline",
            &baseline.to_string_lossy(),
            "--candidate",
            &candidate.to_string_lossy(),
        ])
        .output()?;
    assert!(!output.status.success());
    let receipt: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(receipt["valid"], false);
    assert_eq!(receipt["drift_count"], 4);
    let drift = receipt["drift"].as_array().ok_or("drift")?;
    assert!(drift
        .iter()
        .any(|entry| entry["field_path"] == "/extra_field" && entry["class"] == "added"));
    assert!(drift
        .iter()
        .any(|entry| entry["field_path"] == "/valid" && entry["class"] == "changed"));
    Ok(())
}

#[test]
fn receipt_commands_require_input_paths() -> Result<(), Box<dyn Error>> {
    for args in [
        &["receipt-preserves-diff", "--baseline", "a"][..],
        &["receipt-verify-content-hash"][..],
        &["release-run", "status"][..],
        &["state-machine-receipt", "validate"][..],
    ] {
        let output = cairn().args(args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("requires"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn workspace_contract_resolves_relative_input_under_root() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("workspace-contract-root")?;
    let input = dir.join("matrix.json");
    fs::write(&input, br#"{"matrix":{"edges":[]}}"#)?;
    let output = cairn()
        .args([
            "workspace-contract",
            "verify",
            "--root",
            &dir.to_string_lossy(),
            "--input",
            "matrix.json",
        ])
        .output()?;
    assert!(!output.status.success());
    let receipt: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(receipt["command"], "workspace-contract verify");
    assert_eq!(receipt["valid"], false);
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn workspace_contract_validates_root_relative_fixture() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("workspace-contract-root-positive")?;
    let fixture_root = repo_root().join("fixtures/workspace-contract-matrix");
    copy_dir_recursive(
        &fixture_root,
        &dir.join("fixtures/workspace-contract-matrix"),
    )?;
    let output = cairn()
        .args([
            "workspace-contract",
            "verify",
            "--root",
            &dir.to_string_lossy(),
            "--input",
            "fixtures/workspace-contract-matrix/positive/representative-edges.json",
        ])
        .output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(receipt["valid"], true);
    assert_eq!(
        receipt["input"],
        "fixtures/workspace-contract-matrix/positive/representative-edges.json"
    );
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn workspace_contract_accepts_absolute_input() -> Result<(), Box<dyn Error>> {
    let input =
        repo_root().join("fixtures/workspace-contract-matrix/positive/representative-edges.json");
    let output = cairn()
        .args([
            "workspace-contract",
            "verify",
            "--root",
            ".",
            "--input",
            &input.to_string_lossy(),
        ])
        .output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(receipt["valid"], true);
    Ok(())
}

#[test]
fn lifecycle_patterns_resolve_relative_input_under_root() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("lifecycle-patterns-root")?;
    fs::write(dir.join("bundle.json"), br#"{}"#)?;
    let output = cairn()
        .args([
            "lifecycle-patterns",
            "validate",
            "--root",
            &dir.to_string_lossy(),
            "--input",
            "bundle.json",
        ])
        .output()?;
    assert!(!output.status.success());
    let receipt: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(receipt["command"], "lifecycle-patterns validate");
    assert_eq!(receipt["input"], "bundle.json");
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[cfg(unix)]
#[test]
fn workspace_contract_rejects_symlink_artifacts() -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::symlink;

    let dir = temp_test_dir("workspace-contract-symlink")?;
    let outside = std::env::temp_dir().join(format!(
        "cairn-workspace-artifact-{}.json",
        std::process::id()
    ));
    fs::write(&outside, br#"{"outside":true}"#)?;
    symlink(&outside, dir.join("link.json"))?;
    let input = dir.join("matrix.json");
    fs::write(
        &input,
        br#"{"matrix":{"edges":[{"edge_id":"e","artifact_path":"link.json"}]}}"#,
    )?;
    let output = cairn()
        .args([
            "workspace-contract",
            "verify",
            "--root",
            &dir.to_string_lossy(),
            "--input",
            "matrix.json",
        ])
        .output()?;
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("workspace contract artifact link.json"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    fs::remove_file(outside)?;
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[cfg(unix)]
#[test]
fn workspace_contract_rejects_symlink_inputs() -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::symlink;

    let dir = temp_test_dir("workspace-contract-input-symlink")?;
    let outside =
        std::env::temp_dir().join(format!("cairn-workspace-input-{}.json", std::process::id()));
    fs::write(&outside, br#"{"matrix":{"edges":[]}}"#)?;
    symlink(&outside, dir.join("matrix.json"))?;
    let output = cairn()
        .args([
            "workspace-contract",
            "verify",
            "--root",
            &dir.to_string_lossy(),
            "--input",
            "matrix.json",
        ])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("must not be a symlink"));
    fs::remove_file(outside)?;
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[cfg(unix)]
#[test]
fn receipt_inputs_reject_symlinks() -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::symlink;

    let dir = temp_test_dir("receipt-input-symlink")?;
    let outside =
        std::env::temp_dir().join(format!("cairn-receipt-input-{}.json", std::process::id()));
    fs::write(&outside, br#"{}"#)?;
    symlink(&outside, dir.join("input.json"))?;
    for args in [
        &["release-run", "status"][..],
        &["state-machine-receipt", "validate"][..],
        &["lifecycle-patterns", "validate"][..],
    ] {
        let mut command = cairn();
        command
            .args(args)
            .args(["--root", &dir.to_string_lossy(), "--input", "input.json"]);
        let output = command.output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("must not be a symlink"),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    fs::remove_file(outside)?;
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[cfg(unix)]
#[test]
fn receipt_commands_reject_symlink_inputs() -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::symlink;

    let dir = temp_test_dir("receipt-symlink")?;
    let outside =
        std::env::temp_dir().join(format!("cairn-receipt-symlink-{}.json", std::process::id()));
    fs::write(&outside, br#"{"schema":"cairn.receipt.v1"}"#)?;
    let link = dir.join("link.json");
    symlink(&outside, &link)?;
    let link_text = link.to_string_lossy().to_string();
    for args in [
        vec![
            "receipt-preserves-diff",
            "--baseline",
            link_text.as_str(),
            "--candidate",
            link_text.as_str(),
        ],
        vec!["receipt-verify-content-hash", "--path", link_text.as_str()],
    ] {
        let output = cairn().args(&args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stderr.contains("must not be a symlink") || stdout.contains("read_failed"),
            "{args:?}: {stderr}{stdout}"
        );
    }
    fs::remove_file(outside)?;
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[cfg(unix)]
#[test]
fn artifact_readers_reject_symlinks() -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::symlink;

    let dir = temp_test_dir("artifact-symlink")?;
    let outside = std::env::temp_dir().join(format!(
        "cairn-artifact-symlink-{}.json",
        std::process::id()
    ));
    fs::write(&outside, br#"{"schema":"cairn.receipt.v1"}"#)?;
    let link = dir.join("link.json");
    symlink(&outside, &link)?;
    let link_text = link.to_string_lossy().to_string();
    for args in [
        vec!["agent-receipt", "validate", link_text.as_str()],
        vec!["evidence-manifest", "verify", link_text.as_str()],
        vec![
            "requirement-registry",
            "verify",
            "--input",
            link_text.as_str(),
        ],
    ] {
        let output = cairn().args(&args).output()?;
        assert!(!output.status.success(), "{args:?} unexpectedly succeeded");
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stderr.contains("must not be a symlink") || stdout.contains("manifest.read_failed"),
            "{args:?}: {stderr}{stdout}"
        );
    }
    fs::remove_file(outside)?;
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn workspace_contract_rejects_oversized_inputs() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("workspace-contract-oversized")?;
    let input = dir.join("matrix.json");
    let file = fs::File::create(&input)?;
    file.set_len(2 * 1024 * 1024 + 1)?;
    let output = cairn()
        .args([
            "workspace-contract",
            "verify",
            "--root",
            &dir.to_string_lossy(),
            "--input",
            "matrix.json",
        ])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("exceeds the reviewed byte bound"));
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn legacy_registry_rejects_oversized_inputs() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("legacy-registry-oversized")?;
    let input = dir.join("registry.json");
    let file = fs::File::create(&input)?;
    file.set_len(64 * 1024 * 1024 + 1)?;
    let output = cairn()
        .args([
            "requirement-registry",
            "verify",
            "--input",
            &input.to_string_lossy(),
        ])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("exceeds the reviewed byte bound"));
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn receipt_verify_content_hash_rejects_oversized_input() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("receipt-hash-oversized")?;
    let input = dir.join("receipt.json");
    let file = fs::File::create(&input)?;
    file.set_len(16 * 1024 * 1024 + 1)?;
    let output = cairn()
        .args([
            "receipt-verify-content-hash",
            "--path",
            &input.to_string_lossy(),
        ])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("exceeds the reviewed byte bound"));
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn receipt_preserves_diff_rejects_oversized_input() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("receipt-diff-oversized")?;
    let baseline = dir.join("baseline.json");
    let candidate = dir.join("candidate.json");
    let file = fs::File::create(&baseline)?;
    file.set_len(16 * 1024 * 1024 + 1)?;
    fs::write(&candidate, br#"{}"#)?;
    let output = cairn()
        .args([
            "receipt-preserves-diff",
            "--baseline",
            &baseline.to_string_lossy(),
            "--candidate",
            &candidate.to_string_lossy(),
        ])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("exceeds the reviewed byte bound"));
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[cfg(unix)]
#[test]
fn preserves_corpus_rejects_symlink_input() -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::symlink;

    let dir = temp_test_dir("preserves-corpus-symlink")?;
    let outside = std::env::temp_dir().join(format!(
        "cairn-preserves-corpus-{}.json",
        std::process::id()
    ));
    fs::write(&outside, br#"[]"#)?;
    let link = dir.join("corpus.json");
    symlink(&outside, &link)?;
    let output = cairn()
        .args([
            "preserves-corpus",
            "explain",
            "--path",
            &link.to_string_lossy(),
        ])
        .output()?;
    assert!(!output.status.success());
    let receipt: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(receipt["valid"], false);
    assert!(receipt["issues"].as_array().is_some_and(|issues| issues
        .iter()
        .any(|issue| issue["code"] == "preserves_corpus.read_failed")));
    fs::remove_file(outside)?;
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn preserves_corpus_rejects_oversized_input() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("preserves-corpus-oversized")?;
    let input = dir.join("corpus.json");
    let file = fs::File::create(&input)?;
    file.set_len(16 * 1024 * 1024 + 1)?;
    let output = cairn()
        .args([
            "preserves-corpus",
            "explain",
            "--path",
            &input.to_string_lossy(),
        ])
        .output()?;
    assert!(!output.status.success());
    let receipt: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(receipt["valid"], false);
    assert!(receipt["issues"].as_array().is_some_and(|issues| issues
        .iter()
        .any(|issue| issue["code"] == "preserves_corpus.read_failed")));
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[cfg(unix)]
#[test]
fn readiness_rejects_symlinked_address_and_provenance_inputs() -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::symlink;

    let dir = temp_test_dir("readiness-input-symlink")?;
    let outside = dir.join("outside.json");
    fs::write(&outside, br#"{}"#)?;
    let link = dir.join("link.json");
    symlink(&outside, &link)?;
    let link_text = link.to_string_lossy().to_string();
    for (flag, code) in [
        (
            "--function-address-valence",
            "function_address.input_unreadable",
        ),
        ("--stack-provenance", "stack_provenance.input_unreadable"),
    ] {
        let output = cairn()
            .args([
                "release-readiness",
                "--root",
                "fixtures/basic",
                flag,
                &link_text,
            ])
            .output()?;
        let text = String::from_utf8_lossy(&output.stdout);
        assert!(text.contains(code), "{flag}: {text}");
        assert!(text.contains("must not be a symlink"), "{flag}: {text}");
    }
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn readiness_rejects_oversized_address_input() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("readiness-input-oversized")?;
    let input = dir.join("address.json");
    let file = fs::File::create(&input)?;
    file.set_len(4 * 1024 * 1024 + 1)?;
    let output = cairn()
        .args([
            "release-readiness",
            "--root",
            "fixtures/basic",
            "--function-address-valence",
            &input.to_string_lossy(),
        ])
        .output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("function_address.input_unreadable"), "{text}");
    assert!(text.contains("exceeds the reviewed byte bound"), "{text}");
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[cfg(unix)]
#[test]
fn lifecycle_check_rejects_symlinked_bundle() -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::symlink;

    let dir = temp_test_dir("lifecycle-bundle-symlink")?;
    let outside = dir.join("outside.json");
    fs::write(&outside, br#"{}"#)?;
    let link = dir.join("link.json");
    symlink(&outside, &link)?;
    let output = cairn()
        .args(["lifecycle", "check", "--source", &link.to_string_lossy()])
        .output()?;
    assert!(!output.status.success());
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(text.contains("must not be a symlink"), "{text}");
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn lifecycle_check_rejects_oversized_bundle() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("lifecycle-bundle-oversized")?;
    let input = dir.join("bundle.json");
    let file = fs::File::create(&input)?;
    file.set_len(16 * 1024 * 1024 + 1)?;
    let output = cairn()
        .args(["lifecycle", "check", "--source", &input.to_string_lossy()])
        .output()?;
    assert!(!output.status.success());
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(text.contains("exceeds the reviewed byte bound"), "{text}");
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[cfg(unix)]
#[test]
fn readiness_rejects_symlinked_authentication_manifest() -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::symlink;

    let dir = temp_test_dir("readiness-auth-symlink")?;
    let outside = dir.join("outside.json");
    fs::write(&outside, br#"{}"#)?;
    let link = dir.join("link.json");
    symlink(&outside, &link)?;
    let output = cairn()
        .args([
            "release-readiness",
            "--root",
            "fixtures/basic",
            "--stack-provenance-authentication",
            &link.to_string_lossy(),
        ])
        .output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("authentication"), "{text}");
    assert!(text.contains("must not be a symlink"), "{text}");
    fs::remove_dir_all(dir)?;
    Ok(())
}

#[test]
fn receipt_commands_reject_oversized_inputs() -> Result<(), Box<dyn Error>> {
    let dir = temp_test_dir("receipt-input-oversized")?;
    for (name, size, args) in [
        (
            "ledger.json",
            2 * 1024 * 1024 + 1,
            vec!["release-run", "status"],
        ),
        (
            "receipt.json",
            1024 * 1024 + 1,
            vec!["state-machine-receipt", "validate"],
        ),
    ] {
        let input = dir.join(name);
        let file = fs::File::create(&input)?;
        file.set_len(size)?;
        let mut command = cairn();
        command
            .args(args)
            .args(["--input", &input.to_string_lossy()]);
        let output = command.output()?;
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("exceeds the reviewed byte bound"));
    }
    fs::remove_dir_all(dir)?;
    Ok(())
}
