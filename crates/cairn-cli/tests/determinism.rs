// r[verify cli_maintainability.determinism_replay_submodules.behavior_preserved]
// r[verify cli_maintainability.determinism_replay_submodules.behavior_preserved.focused_checks]
// r[verify cli_maintainability.determinism_replay_submodules.behavior_preserved.repository_checks]
use std::{env, error::Error, fs, path::PathBuf, process::Command};

use serde_json::{json, Value};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn cairn() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cairn"));
    command.current_dir(repo_root());
    command
}

fn run_json(args: &[&str]) -> Result<Value, Box<dyn Error>> {
    let mut command = cairn();
    command.args(args);
    run_json_command(&mut command)
}

fn run_json_command(command: &mut Command) -> Result<Value, Box<dyn Error>> {
    let output = command.output()?;
    assert!(
        output.status.success(),
        "status: {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(serde_json::from_slice(&output.stdout)?)
}

fn run_json_failure(args: &[&str]) -> Result<Value, Box<dyn Error>> {
    let mut command = cairn();
    command.args(args);
    run_json_failure_command(&mut command)
}

fn run_json_failure_command(command: &mut Command) -> Result<Value, Box<dyn Error>> {
    let output = command.output()?;
    assert!(
        !output.status.success(),
        "expected failure\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(serde_json::from_slice(&output.stdout)?)
}

fn run_json_strings(args: &[String]) -> Result<Value, Box<dyn Error>> {
    let mut command = cairn();
    command.args(args);
    run_json_command(&mut command)
}

fn run_json_failure_strings(args: &[String]) -> Result<Value, Box<dyn Error>> {
    let mut command = cairn();
    command.args(args);
    run_json_failure_command(&mut command)
}

fn temp_project(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let destination = env::temp_dir().join(format!("cairn-cli-{name}-{}", std::process::id()));
    if destination.exists() {
        fs::remove_dir_all(&destination)?;
    }
    fs::create_dir_all(&destination)?;
    Ok(destination)
}

fn assert_hash_field(value: &Value, field: &str) -> Result<(), Box<dyn Error>> {
    let hash = value[field].as_str().ok_or("hash field missing")?;
    assert_eq!(hash.len(), 64, "{field} should be a BLAKE3 hex hash");
    assert!(
        hash.chars().all(|character| character.is_ascii_hexdigit()),
        "{field} should be hex: {hash}"
    );
    Ok(())
}

fn policy_json() -> Result<Value, Box<dyn Error>> {
    Ok(serde_json::from_slice(&fs::read(
        repo_root().join("cairn-policy/generated/cairn-policy.json"),
    )?)?)
}

fn receipt_schema(command: &str) -> Result<Value, Box<dyn Error>> {
    policy_json()?["receipt_schemas"]
        .as_array()
        .ok_or("receipt_schemas array")?
        .iter()
        .find(|schema| schema["command"] == command)
        .cloned()
        .ok_or_else(|| format!("missing receipt schema for {command}").into())
}

#[test]
fn determinism_replay_oracle_is_stable_and_non_mutating() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.determinism_replay.workspace_oracle]
    // r[verify cairn.supersede.determinism_replay.normalization]
    // r[verify cairn.supersede.determinism_replay.repeated]
    // r[verify cairn.supersede.determinism_replay.cross_root]
    // r[verify cairn.supersede.determinism_replay.receipt]
    // r[verify cairn.supersede.determinism_replay.mcp_cases]
    // r[verify cairn.supersede.determinism_replay.mismatch_diagnostics]
    // r[verify cairn.supersede.determinism_replay.policy_plan]
    // r[verify cairn.supersede.determinism_replay.policy_schema]
    // r[verify cairn.supersede.determinism_replay.expanded_surfaces]
    // r[verify cairn.supersede.determinism_replay.policy_diagnostics]
    // r[verify cairn.supersede.determinism_replay.ci_case_selection]
    // r[verify cairn.supersede.determinism_replay.ci_failure_ergonomics]
    // r[verify cairn.supersede.determinism_replay.groups.policy]
    // r[verify cairn.supersede.determinism_replay.groups.receipt]
    // r[verify cairn.supersede.determinism_entropy_inventory.policy]
    // r[verify cairn.supersede.determinism_entropy_inventory.receipt]
    // r[verify cairn.supersede.determinism_entropy_inventory.tests]
    // r[verify cairn.supersede.replay_environment_attestation.policy]
    // r[verify cairn.supersede.replay_environment_attestation.receipt]
    // r[verify cairn.supersede.replay_environment_attestation.secret_safe]
    let first = run_json(&["determinism", "replay", "--root", "."])?;
    let second = run_json(&["determinism", "replay", "--root", "."])?;
    assert_eq!(first["valid"], true);
    assert_eq!(first["verdict"], "pass");
    assert_eq!(first["mismatches"].as_array().unwrap().len(), 0);
    assert_eq!(
        first["nondeterminism_failures"].as_array().unwrap().len(),
        0
    );
    assert_eq!(first["receipt_hash"], second["receipt_hash"]);
    assert_eq!(first["summary"]["total_cases"], 12);
    assert_eq!(first["summary"]["mismatch_count"], 0);
    assert_eq!(first["available_cases"], first["selected_cases"]);
    assert_eq!(
        first["available_groups"],
        serde_json::json!(["agent", "full", "release", "smoke"])
    );
    assert_eq!(first["selected_groups"], serde_json::json!([]));
    assert_hash_field(&first, "receipt_hash")?;
    assert_hash_field(&first, "entropy_policy_hash")?;
    let policy = policy_json()?;
    let entropy_inventory = first["entropy_inventory"].as_array().unwrap();
    let mut expected_entropy_inventory = policy["determinism_replay_policy"]["entropy_inventory"]
        .as_array()
        .unwrap()
        .clone();
    expected_entropy_inventory.sort_by_key(|entry| entry["id"].as_str().unwrap().to_owned());
    assert_eq!(entropy_inventory, &expected_entropy_inventory);
    assert!(entropy_inventory
        .iter()
        .any(|entry| entry["id"] == "archive_date_env" && entry["handling"] == "reject"));
    assert!(first["entropy_assumptions"]
        .as_array()
        .unwrap()
        .iter()
        .any(|entry| entry["id"] == "cwd_root_paths"));
    assert!(first["entropy_assumptions"]
        .as_array()
        .unwrap()
        .iter()
        .all(|entry| entry["handling"] != "reject"));
    let environment_attestation = &first["environment_attestation"];
    assert_eq!(environment_attestation["cairn_package"], "cairn");
    assert_eq!(
        environment_attestation["cairn_version"],
        env!("CARGO_PKG_VERSION")
    );
    assert_hash_field(environment_attestation, "generated_policy_hash")?;
    assert_hash_field(environment_attestation, "entropy_policy_hash")?;
    assert_hash_field(environment_attestation, "attestation_policy_hash")?;
    assert_hash_field(environment_attestation, "attestation_hash")?;
    assert_eq!(
        environment_attestation["entropy_policy_hash"],
        first["entropy_policy_hash"]
    );
    assert_eq!(
        environment_attestation["selected_cases"],
        first["selected_cases"]
    );
    assert_eq!(
        environment_attestation["selected_groups"],
        first["selected_groups"]
    );
    assert_eq!(
        environment_attestation["replay_scope"]["selector"],
        "default"
    );
    let mut expected_environment_facts = policy["determinism_replay_policy"]
        ["environment_attestation"]
        .as_array()
        .unwrap()
        .clone();
    expected_environment_facts.sort_by_key(|entry| entry["id"].as_str().unwrap().to_owned());
    assert_eq!(
        environment_attestation["policy_facts"].as_array().unwrap(),
        &expected_environment_facts
    );
    let attestation_text = environment_attestation.to_string();
    assert!(!attestation_text.contains(&repo_root().display().to_string()));
    assert!(!attestation_text.contains("SECRET"));
    let case_ids = first["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| case["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        case_ids,
        vec![
            "agent_receipt_validate",
            "archive_dry_run",
            "gate_design",
            "gate_proposal",
            "gate_tasks",
            "mcp_describe",
            "mcp_status",
            "migrate_dry_run",
            "release_readiness",
            "status",
            "sync_dry_run",
            "validate"
        ]
    );
    let policy_case_count = policy["determinism_replay_policy"]["cases"]
        .as_array()
        .unwrap()
        .len();
    assert_eq!(first["cases"].as_array().unwrap().len(), policy_case_count);
    let policy_groups = policy["determinism_replay_policy"]["groups"]
        .as_array()
        .unwrap();
    assert_eq!(policy_groups.len(), 4);
    assert!(policy_groups.iter().any(|group| group["id"] == "agent"));
    assert!(policy_groups.iter().any(|group| group["id"] == "full"));
    assert!(first["cases"].as_array().unwrap().iter().all(|case| {
        case["runs"].as_array().unwrap().len() >= 2
            && case["expected_mutability"] == "read_only_or_dry_run"
            && case["fixture_root"].is_string()
    }));
    assert!(first["mutability_observations"]
        .as_array()
        .unwrap()
        .iter()
        .all(|observation| observation["mutated"] == false));
    Ok(())
}

#[test]
fn determinism_replay_supports_ci_case_selection_and_output_modes() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.determinism_replay.ci_case_selection]
    // r[verify cairn.supersede.determinism_replay.ci_output_modes]
    // r[verify cairn.supersede.determinism_replay.ci_tests_docs]
    // r[verify cairn.supersede.determinism_replay.groups.cli]
    // r[verify cairn.supersede.determinism_replay.groups.receipt]
    // r[verify cairn.supersede.determinism_replay.groups.docs]
    let selected = run_json(&["determinism", "replay", "--root", ".", "--case", "status"])?;
    assert_eq!(selected["valid"], true);
    assert_eq!(selected["selected_cases"], serde_json::json!(["status"]));
    assert_eq!(selected["selected_groups"], serde_json::json!([]));
    assert_eq!(selected["summary"]["total_cases"], 1);
    assert_eq!(selected["cases"].as_array().unwrap().len(), 1);
    assert_eq!(selected["cases"][0]["id"], "status");

    let artifact = temp_project("replay-artifact")?.join("receipt.json");
    let mut summary = cairn();
    summary.args([
        "determinism",
        "replay",
        "--root",
        ".",
        "--case",
        "status",
        "--format",
        "summary",
        "--out",
        artifact.to_str().unwrap(),
    ]);
    let output = summary.output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("determinism replay: verdict=pass total=1"));
    assert!(stdout.contains("artifact="));
    let artifact_receipt: Value = serde_json::from_slice(&fs::read(&artifact)?)?;
    assert_eq!(artifact_receipt["receipt_hash"], selected["receipt_hash"]);

    let mut jsonl = cairn();
    jsonl.args([
        "determinism",
        "replay",
        "--root",
        ".",
        "--case",
        "status",
        "--format",
        "jsonl",
    ]);
    let output = jsonl.output()?;
    assert!(output.status.success());
    let lines = String::from_utf8(output.stdout)?;
    let events = lines
        .lines()
        .map(serde_json::from_str::<Value>)
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(events[0]["event"], "summary");
    assert!(events
        .iter()
        .any(|event| event["event"] == "case" && event["id"] == "status"));

    let group = run_json(&["determinism", "replay", "--root", ".", "--group", "smoke"])?;
    assert_eq!(group["valid"], true);
    assert_eq!(group["selected_groups"], json!(["smoke"]));
    assert_eq!(group["selected_cases"], json!(["status", "validate"]));
    assert_eq!(group["summary"]["total_cases"], 2);
    Ok(())
}

#[test]
fn determinism_coverage_audit_reports_policy_surface_coverage() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.determinism_surface_coverage_audit.cli]
    // r[verify cairn.supersede.determinism_surface_coverage_audit.inventory]
    // r[verify cairn.supersede.determinism_surface_coverage_audit.failures]
    // r[verify cairn.supersede.determinism_surface_coverage_audit.docs]
    // r[verify cairn.supersede.determinism_surface_coverage_audit.gates]
    // r[verify cairn.supersede.determinism_surface_coverage_audit.archive]
    // r[verify cairn.supersede.determinism_surface_coverage_audit.tests]
    let first = run_json(&["determinism", "coverage-audit", "--root", "."])?;
    let second = run_json(&["determinism", "coverage-audit", "--root", "."])?;
    assert_eq!(first["valid"], true);
    assert_eq!(first["verdict"], "pass");
    assert_eq!(first["receipt_hash"], second["receipt_hash"]);
    assert_eq!(first["issues"].as_array().unwrap().len(), 0);
    assert_eq!(first["missing_surfaces"].as_array().unwrap().len(), 0);
    assert_hash_field(&first, "receipt_hash")?;
    assert_hash_field(&first, "current_replay_policy_hash")?;
    assert_hash_field(&first, "surface_inventory_hash")?;

    let policy = policy_json()?;
    let surfaces = policy["determinism_surface_inventory"].as_array().unwrap();
    assert_eq!(first["surface_count"], surfaces.len());
    assert_eq!(
        first["covered_surfaces"].as_array().unwrap().len(),
        surfaces.len()
    );
    let covered_ids = first["covered_surfaces"]
        .as_array()
        .unwrap()
        .iter()
        .map(|surface| surface["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        covered_ids,
        vec![
            "agent_receipt_validate",
            "archive_dry_run",
            "evidence_registry_build",
            "evidence_registry_fetch_execute",
            "evidence_registry_fetch_plan",
            "evidence_registry_mirror_verify",
            "evidence_registry_select",
            "evidence_registry_verify",
            "gate_design",
            "gate_proposal",
            "gate_tasks",
            "mcp_describe",
            "mcp_status",
            "migrate_dry_run",
            "release_readiness",
            "status",
            "sync_dry_run",
            "validate"
        ]
    );
    let schema = receipt_schema("determinism coverage-audit")?;
    assert!(schema["top_level_keys"]
        .as_array()
        .unwrap()
        .contains(&json!("surface_inventory_hash")));
    assert!(schema["issue_keys"]
        .as_array()
        .unwrap()
        .contains(&json!("surface_id")));
    Ok(())
}

#[test]
fn determinism_verify_receipt_validates_saved_replay_evidence() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.replay_receipt_verification.contract]
    // r[verify cairn.supersede.replay_receipt_verification.cli]
    // r[verify cairn.supersede.replay_receipt_verification.validation]
    // r[verify cairn.supersede.replay_receipt_verification.receipt]
    // r[verify cairn.supersede.replay_receipt_verification.failures]
    // r[verify cairn.supersede.replay_receipt_verification.secret_safe]
    let artifact = temp_project("verify-replay-receipt")?.join("receipt.json");
    let mut replay = cairn();
    replay.args([
        "determinism",
        "replay",
        "--root",
        ".",
        "--group",
        "smoke",
        "--format",
        "summary",
        "--out",
        artifact.to_str().unwrap(),
    ]);
    let output = replay.output()?;
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let verified = run_json_strings(&[
        "determinism".into(),
        "verify-receipt".into(),
        artifact.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    let replay_receipt: Value = serde_json::from_slice(&fs::read(&artifact)?)?;
    assert_eq!(verified["valid"], true);
    assert_eq!(verified["verdict"], "pass");
    assert_eq!(verified["issues"].as_array().unwrap().len(), 0);
    assert_eq!(
        verified["replay_receipt_hash"],
        replay_receipt["receipt_hash"]
    );
    assert_eq!(
        verified["replay_policy_hash"],
        replay_receipt["environment_attestation"]["generated_policy_hash"]
    );
    assert_eq!(verified["selected_cases"], json!(["status", "validate"]));
    assert_eq!(verified["selected_groups"], json!(["smoke"]));
    assert_eq!(
        verified["environment_attestation_hash"],
        replay_receipt["environment_attestation"]["attestation_hash"]
    );
    assert_hash_field(&verified, "receipt_hash")?;

    let mut tampered = replay_receipt.clone();
    tampered["selected_cases"] = json!(["status", "unknown_case"]);
    let tampered_path = artifact.with_file_name("tampered.json");
    fs::write(
        &tampered_path,
        serde_json::to_string_pretty(&tampered)? + "\n",
    )?;
    let failure = run_json_failure_strings(&[
        "determinism".into(),
        "verify-receipt".into(),
        tampered_path.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    assert_eq!(failure["valid"], false);
    let issue_codes = failure["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(issue_codes.contains(&"selected_cases.invalid"));
    assert!(issue_codes.contains(&"receipt_hash.mismatch"));
    assert!(!serde_json::to_string(&failure)?.contains("SECRET"));

    let malformed_path = artifact.with_file_name("malformed.json");
    fs::write(&malformed_path, "{not json\n")?;
    let malformed = run_json_failure_strings(&[
        "determinism".into(),
        "verify-receipt".into(),
        malformed_path.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    assert_eq!(malformed["valid"], false);
    assert_eq!(malformed["issues"][0]["code"], "receipt.malformed_json");
    Ok(())
}

#[test]
fn determinism_replay_unknown_case_is_machine_readable_failure() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.determinism_replay.ci_case_selection]
    // r[verify cairn.supersede.determinism_replay.ci_failure_ergonomics]
    // r[verify cairn.supersede.determinism_replay.groups.failures]
    let value = run_json_failure(&[
        "determinism",
        "replay",
        "--root",
        ".",
        "--case",
        "not_a_case",
    ])?;
    assert_eq!(value["valid"], false);
    assert_eq!(value["mismatches"][0]["class"], "unknown_case");
    assert_eq!(value["mismatches"][0]["actual"], "not_a_case");
    assert!(value["mismatches"][0]["reproduce"]
        .as_str()
        .unwrap()
        .contains("--case not_a_case"));
    assert_hash_field(&value, "receipt_hash")?;

    let group = run_json_failure(&[
        "determinism",
        "replay",
        "--root",
        ".",
        "--group",
        "not_a_group",
    ])?;
    assert_eq!(group["valid"], false);
    assert_eq!(group["mismatches"][0]["class"], "unknown_group");
    assert_eq!(group["mismatches"][0]["actual"], "not_a_group");
    assert!(group["mismatches"][0]["reproduce"]
        .as_str()
        .unwrap()
        .contains("--group not_a_group"));

    let conflict = run_json_failure(&[
        "determinism",
        "replay",
        "--root",
        ".",
        "--case",
        "status",
        "--group",
        "smoke",
    ])?;
    assert_eq!(conflict["valid"], false);
    assert_eq!(conflict["mismatches"][0]["class"], "selector_conflict");
    assert_eq!(conflict["mismatches"][0]["field_path"], "--case/--group");
    assert_hash_field(&conflict, "receipt_hash")
}

#[test]
fn determinism_replay_rejects_mutation_flags() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.determinism_replay.no_mutation]
    let value = run_json_failure(&[
        "determinism",
        "replay",
        "--root",
        "fixtures/basic",
        "--execute",
    ])?;
    assert_eq!(value["valid"], false);
    assert_eq!(value["verdict"], "fail");
    assert_eq!(value["mismatches"][0]["code"], "replay.mutation_denied");
    assert_hash_field(&value, "receipt_hash")
}

#[test]
fn determinism_replay_rejects_hidden_environment_entropy() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.determinism_replay.hidden_entropy]
    // r[verify cairn.supersede.determinism_entropy_inventory.failures]
    // r[verify cairn.supersede.determinism_entropy_inventory.tests]
    let mut command = cairn();
    command
        .args(["determinism", "replay", "--root", "fixtures/basic"])
        .env("CAIRN_ARCHIVE_DATE", "2026-05-18");
    let value = run_json_failure_command(&mut command)?;
    assert_eq!(value["valid"], false);
    assert_eq!(value["verdict"], "fail");
    assert_eq!(
        value["nondeterminism_failures"][0]["code"],
        "replay.hidden_entropy.environment"
    );
    assert_eq!(
        value["nondeterminism_failures"][0]["source"],
        "CAIRN_ARCHIVE_DATE"
    );
    assert_eq!(
        value["nondeterminism_failures"][0]["inventory_id"],
        "archive_date_env"
    );
    assert_eq!(value["nondeterminism_failures"][0]["handling"], "reject");
    assert!(
        !serde_json::to_string(&value["nondeterminism_failures"])?.contains("2026-05-18"),
        "replay failure must not disclose rejected environment values"
    );
    assert_hash_field(&value, "entropy_policy_hash")?;
    assert_hash_field(&value, "receipt_hash")
}
