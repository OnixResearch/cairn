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
            stderr.contains("safe path component") || stderr.contains("capability"),
            "{args:?}: {stderr}"
        );
    }
    Ok(())
}
