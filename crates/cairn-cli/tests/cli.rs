use std::{
    env,
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use serde_json::{json, Value};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn cairn() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cairn"));
    command.current_dir(repo_root());
    command
}

fn ironspec_alias() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ironspec"));
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

fn copy_dir(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    let mut dirs = vec![(source.to_path_buf(), destination.to_path_buf())];
    while let Some((current_source, current_destination)) = dirs.pop() {
        fs::create_dir_all(&current_destination)?;
        for entry in fs::read_dir(current_source)? {
            let entry = entry?;
            let source_path = entry.path();
            let destination_path = current_destination.join(entry.file_name());
            if entry.file_type()?.is_dir() {
                dirs.push((source_path, destination_path));
            } else {
                fs::copy(&source_path, &destination_path)?;
            }
        }
    }
    Ok(())
}

fn temp_project(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let destination = env::temp_dir().join(format!("cairn-cli-{name}-{}", std::process::id()));
    if destination.exists() {
        fs::remove_dir_all(&destination)?;
    }
    fs::create_dir_all(&destination)?;
    Ok(destination)
}

fn temp_fixture(name: &str, fixture: &str) -> Result<PathBuf, Box<dyn Error>> {
    let destination = temp_project(name)?;
    fs::remove_dir_all(&destination)?;
    copy_dir(&repo_root().join(fixture), &destination)?;
    Ok(destination)
}

fn golden(name: &str) -> Result<Value, Box<dyn Error>> {
    let path = repo_root().join("crates/cairn-cli/tests/golden").join(name);
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn assert_matches_golden(args: &[&str], name: &str) -> Result<(), Box<dyn Error>> {
    assert_eq!(run_json(args)?, golden(name)?);
    Ok(())
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

#[test]
fn function_object_evidence_validator_accepts_positive_fixture() -> Result<(), Box<dyn Error>> {
    // Active Cairn refs: content_addressed_rust_evidence.function_object.source_token,
    // content_addressed_rust_evidence.canonical_hash.portable.
    let fixture = repo_root()
        .join("fixtures/content-addressed-rust-evidence/function-object-v1/positive.json");
    let receipt = run_json(&[
        "evidence",
        "function-object",
        "validate",
        fixture.to_str().ok_or("fixture path")?,
    ])?;
    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["verdict"], "PASS");
    assert_eq!(receipt["issues"].as_array().map(Vec::len), Some(0));
    assert_hash_field(&receipt, "computed_object_hash")?;
    assert_hash_field(&receipt, "receipt_hash")?;
    Ok(())
}

#[test]
fn function_object_evidence_validator_rejects_ambient_entropy_fixture() -> Result<(), Box<dyn Error>>
{
    // Active Cairn ref: content_addressed_rust_evidence.canonical_hash.no_entropy.
    let fixture = repo_root()
        .join("fixtures/content-addressed-rust-evidence/function-object-v1/ambient-entropy.json");
    let receipt = run_json_failure(&[
        "evidence",
        "function-object",
        "validate",
        fixture.to_str().ok_or("fixture path")?,
    ])?;
    assert_eq!(receipt["valid"], false);
    assert_eq!(receipt["verdict"], "FAIL");
    assert!(receipt["issues"]
        .as_array()
        .unwrap()
        .iter()
        .any(|issue| { issue["code"] == "function_object.ambient_entropy_in_hash_material" }));
    Ok(())
}

#[test]
fn name_pointer_evidence_validator_accepts_pointer_move_fixture() -> Result<(), Box<dyn Error>> {
    // Active Cairn refs: content_addressed_rust_evidence.name_pointer.move,
    // content_addressed_rust_evidence.export_bundle.mutation_manifest.
    let fixture = repo_root()
        .join("fixtures/content-addressed-rust-evidence/pointers-and-caveats/pointer-move.json");
    let receipt = run_json(&[
        "evidence",
        "name-pointer",
        "validate",
        fixture.to_str().ok_or("fixture path")?,
    ])?;
    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["verdict"], "PASS");
    assert_eq!(receipt["issues"].as_array().map(Vec::len), Some(0));
    assert_hash_field(&receipt, "computed_pointer_hash")?;
    assert_hash_field(&receipt, "receipt_hash")?;
    Ok(())
}

#[test]
fn name_pointer_evidence_validator_rejects_drift_fixture() -> Result<(), Box<dyn Error>> {
    // Active Cairn ref: content_addressed_rust_evidence.name_pointer.drift.
    let fixture = repo_root()
        .join("fixtures/content-addressed-rust-evidence/pointers-and-caveats/pointer-drift.json");
    let receipt = run_json_failure(&[
        "evidence",
        "name-pointer",
        "validate",
        fixture.to_str().ok_or("fixture path")?,
    ])?;
    assert_eq!(receipt["valid"], false);
    assert_eq!(receipt["verdict"], "FAIL");
    assert!(receipt["issues"]
        .as_array()
        .unwrap()
        .iter()
        .any(|issue| { issue["code"] == "name_pointer.drift_detected" }));
    Ok(())
}

#[test]
fn name_pointer_evidence_validator_rejects_unknown_effect_pure_cache_fixture(
) -> Result<(), Box<dyn Error>> {
    // Active Cairn refs: content_addressed_rust_evidence.caveats.dependency_status,
    // content_addressed_rust_evidence.caveats.effects_fail_closed.
    let fixture = repo_root().join(
        "fixtures/content-addressed-rust-evidence/pointers-and-caveats/unknown-effect-pure-cache.json",
    );
    let receipt = run_json_failure(&[
        "evidence",
        "name-pointer",
        "validate",
        fixture.to_str().ok_or("fixture path")?,
    ])?;
    assert_eq!(receipt["valid"], false);
    assert_eq!(receipt["verdict"], "FAIL");
    assert!(receipt["issues"]
        .as_array()
        .unwrap()
        .iter()
        .any(|issue| { issue["code"] == "name_pointer.pure_cache_overclaim" }));
    Ok(())
}

#[test]
fn export_bundle_evidence_validator_accepts_positive_fixture() -> Result<(), Box<dyn Error>> {
    // Active Cairn refs: content_addressed_rust_evidence.export_bundle.replay,
    // content_addressed_rust_evidence.compiler_ir.hir_dependencies.
    let fixture = repo_root().join(
        "fixtures/content-addressed-rust-evidence/ir-and-bundles/export-bundle-positive.json",
    );
    let receipt = run_json(&[
        "evidence",
        "export-bundle",
        "validate",
        fixture.to_str().ok_or("fixture path")?,
    ])?;
    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["verdict"], "PASS");
    assert_eq!(receipt["issues"].as_array().map(Vec::len), Some(0));
    assert_hash_field(&receipt, "computed_bundle_hash")?;
    assert_hash_field(&receipt, "receipt_hash")?;
    Ok(())
}

#[test]
fn export_bundle_evidence_validator_rejects_unsorted_objects() -> Result<(), Box<dyn Error>> {
    // Active Cairn ref: content_addressed_rust_evidence.export_bundle.replay.
    let fixture = repo_root().join(
        "fixtures/content-addressed-rust-evidence/ir-and-bundles/export-bundle-unsorted-objects.json",
    );
    let receipt = run_json_failure(&[
        "evidence",
        "export-bundle",
        "validate",
        fixture.to_str().ok_or("fixture path")?,
    ])?;
    assert_eq!(receipt["valid"], false);
    assert_eq!(receipt["verdict"], "FAIL");
    assert!(receipt["issues"]
        .as_array()
        .unwrap()
        .iter()
        .any(|issue| { issue["code"] == "export_bundle.unsorted_objects" }));
    Ok(())
}

#[test]
fn export_bundle_evidence_validator_rejects_unknown_pointer_object() -> Result<(), Box<dyn Error>> {
    // Active Cairn refs: content_addressed_rust_evidence.export_bundle.replay,
    // content_addressed_rust_evidence.export_bundle.mutation_manifest.
    let fixture = repo_root().join(
        "fixtures/content-addressed-rust-evidence/ir-and-bundles/export-bundle-unknown-pointer.json",
    );
    let receipt = run_json_failure(&[
        "evidence",
        "export-bundle",
        "validate",
        fixture.to_str().ok_or("fixture path")?,
    ])?;
    assert_eq!(receipt["valid"], false);
    assert_eq!(receipt["verdict"], "FAIL");
    assert!(receipt["issues"]
        .as_array()
        .unwrap()
        .iter()
        .any(|issue| { issue["code"] == "export_bundle.unknown_pointer_object" }));
    Ok(())
}

fn assert_object_keys(value: &Value, expected: &[&str]) -> Result<(), Box<dyn Error>> {
    let object = value.as_object().ok_or("json object")?;
    let mut actual = object.keys().map(String::as_str).collect::<Vec<_>>();
    actual.sort_unstable();
    let mut expected = expected.to_vec();
    expected.sort_unstable();
    assert_eq!(actual, expected, "receipt object keys changed");
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

fn policy_replay_group_cases(group_id: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut cases = policy_json()?["determinism_replay_policy"]["groups"]
        .as_array()
        .ok_or("policy replay groups")?
        .iter()
        .find(|group| group["id"] == group_id)
        .ok_or_else(|| format!("missing replay group {group_id}"))?["cases"]
        .as_array()
        .ok_or("group cases")?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .ok_or_else(|| "case id string".into())
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    cases.sort();
    cases.dedup();
    Ok(cases)
}

fn schema_strings(schema: &Value, field: &str) -> Result<Vec<String>, Box<dyn Error>> {
    schema[field]
        .as_array()
        .ok_or_else(|| format!("{field} array"))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .ok_or_else(|| format!("{field} string").into())
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()
}

fn assert_schema_keys(value: &Value, schema: &Value, field: &str) -> Result<(), Box<dyn Error>> {
    let expected = schema_strings(schema, field)?;
    assert_object_keys(
        value,
        &expected.iter().map(String::as_str).collect::<Vec<_>>(),
    )
}

fn assert_first_action_matches_schema(value: &Value, schema: &Value) -> Result<(), Box<dyn Error>> {
    let actions = value["actions"].as_array().ok_or("actions array")?;
    let action = actions.first().ok_or("at least one action")?;
    assert_schema_keys(action, schema, "action_keys")
}

fn assert_reproducible_receipt(
    args: &[&str],
    hash_fields: &[&str],
) -> Result<Value, Box<dyn Error>> {
    let first = run_json(args)?;
    let second = run_json(args)?;
    assert_eq!(
        first, second,
        "receipt output should be reproducible for {args:?}"
    );
    for field in hash_fields {
        assert_hash_field(&first, field)?;
    }
    Ok(first)
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

fn assert_hash_fields_equal(
    first: &Value,
    second: &Value,
    fields: &[&str],
) -> Result<(), Box<dyn Error>> {
    for field in fields {
        assert_hash_field(first, field)?;
        assert_hash_field(second, field)?;
        assert_eq!(
            first[field], second[field],
            "portable hash field {field} changed across roots"
        );
    }
    Ok(())
}

#[test]
fn portable_receipt_hashes_survive_distinct_roots() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.blake3_receipts.lifecycle_plans]
    // r[verify cairn.supersede.portable_receipts] r[verify cairn.supersede.portable_receipts.cross_root]
    // r[verify cairn.supersede.portable_receipts.display_paths] r[verify cairn.supersede.portable_receipts.explicit_absolute_inputs]
    // r[verify cairn.supersede.determinism_replay] r[verify cairn.supersede.determinism_replay.cross_root]
    // r[verify cairn.supersede.determinism_replay.repeated]
    let first_root = temp_fixture("portable-a", "fixtures/basic")?;
    let second_root = temp_fixture("portable-b", "fixtures/basic")?;
    let first = first_root.display().to_string();
    let second = second_root.display().to_string();

    let gate_a = run_json_strings(&[
        "gate".into(),
        "tasks".into(),
        "demo".into(),
        "--root".into(),
        first.clone(),
    ])?;
    let gate_b = run_json_strings(&[
        "gate".into(),
        "tasks".into(),
        "demo".into(),
        "--root".into(),
        second.clone(),
    ])?;
    assert_hash_fields_equal(&gate_a, &gate_b, &["input_hash", "receipt_hash"])?;

    let sync_a = run_json_strings(&["sync".into(), "demo".into(), "--root".into(), first.clone()])?;
    let sync_b = run_json_strings(&[
        "sync".into(),
        "demo".into(),
        "--root".into(),
        second.clone(),
    ])?;
    assert_hash_fields_equal(
        &sync_a,
        &sync_b,
        &["input_hash", "plan_hash", "receipt_hash"],
    )?;

    let migrate_a =
        run_json_strings(&["migrate".into(), "openspec".into(), "--root".into(), first])?;
    let migrate_b =
        run_json_strings(&["migrate".into(), "openspec".into(), "--root".into(), second])?;
    assert_hash_fields_equal(
        &migrate_a,
        &migrate_b,
        &[
            "source_manifest_hash",
            "destination_manifest_hash",
            "manifest_hash",
            "receipt_hash",
        ],
    )
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
fn agent_evidence_bundle_verification_links_agent_replay_and_release_evidence(
) -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.agent_evidence_bundle_verification.cli]
    // r[verify cairn.supersede.agent_evidence_bundle_verification.validation]
    // r[verify cairn.supersede.agent_evidence_bundle_verification.receipt]
    // r[verify cairn.supersede.agent_evidence_bundle_verification.secret_safe]
    // r[verify cairn.supersede.agent_evidence_bundle_verification.tests]
    // r[verify cairn.supersede.agent_evidence_bundle_verification.archive]
    let dir = temp_project("agent-evidence-bundle")?;
    let replay_path = dir.join("agent-replay.json");
    let verify_path = dir.join("agent-replay-verify.json");
    let release_path = dir.join("release-readiness.json");
    let bundle_path = dir.join("bundle.json");

    let mut replay = cairn();
    replay.args([
        "determinism",
        "replay",
        "--root",
        ".",
        "--group",
        "agent",
        "--format",
        "summary",
        "--out",
        replay_path.to_str().unwrap(),
    ]);
    let output = replay.output()?;
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let replay_verify = run_json_strings(&[
        "determinism".into(),
        "verify-receipt".into(),
        replay_path.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    fs::write(
        &verify_path,
        serde_json::to_string_pretty(&replay_verify)? + "\n",
    )?;
    fs::write(
        &release_path,
        serde_json::to_string_pretty(&json!({
            "valid": true,
            "receipt_hash": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        }))? + "\n",
    )?;
    let agent_path = dir.join("agent-receipt.json");
    let mut agent_receipt: Value = serde_json::from_str(&fs::read_to_string(
        repo_root().join("fixtures/agent-receipts/positive.json"),
    )?)?;
    agent_receipt["replay_evidence"]["replay_receipt_hash"] =
        replay_verify["replay_receipt_hash"].clone();
    agent_receipt["replay_evidence"]["replay_policy_hash"] =
        replay_verify["replay_policy_hash"].clone();
    agent_receipt["replay_evidence"]["replay_environment_attestation_hash"] =
        replay_verify["environment_attestation_hash"].clone();
    agent_receipt["receipt_hash"] = Value::Null;
    agent_receipt
        .as_object_mut()
        .unwrap()
        .remove("receipt_hash");
    fs::write(
        &agent_path,
        serde_json::to_string_pretty(&agent_receipt)? + "\n",
    )?;
    let initial_agent_validation = run_json_strings(&[
        "agent-receipt".into(),
        "validate".into(),
        agent_path.display().to_string(),
    ])?;
    agent_receipt["receipt_hash"] = initial_agent_validation["agent_receipt_hash"].clone();
    fs::write(
        &agent_path,
        serde_json::to_string_pretty(&agent_receipt)? + "\n",
    )?;
    let agent_validation = run_json_strings(&[
        "agent-receipt".into(),
        "validate".into(),
        agent_path.display().to_string(),
    ])?;
    assert_eq!(agent_validation["valid"], true);

    let bundle = json!({
        "agent_receipt_path": agent_path,
        "replay_receipt_path": replay_path,
        "replay_verification_path": verify_path,
        "agent_receipt_hash": agent_validation["agent_receipt_hash"],
        "replay_receipt_hash": replay_verify["replay_receipt_hash"],
        "replay_verification_receipt_hash": replay_verify["receipt_hash"],
    });
    fs::write(&bundle_path, serde_json::to_string_pretty(&bundle)? + "\n")?;

    let verified = run_json_strings(&[
        "agent-receipt".into(),
        "verify-bundle".into(),
        bundle_path.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    assert_eq!(verified["valid"], true);
    assert_eq!(verified["verdict"], "pass");
    assert_eq!(verified["required_replay_group"], "agent");
    assert_eq!(verified["issues"].as_array().unwrap().len(), 0);
    assert_eq!(
        verified["agent_receipt_hash"],
        agent_validation["agent_receipt_hash"]
    );
    assert_eq!(
        verified["replay_receipt_hash"],
        replay_verify["replay_receipt_hash"]
    );
    assert_eq!(
        verified["environment_attestation_hash"],
        replay_verify["environment_attestation_hash"]
    );
    assert_hash_field(&verified, "receipt_hash")?;

    let mut tampered_verify = replay_verify.clone();
    tampered_verify["replay_receipt_hash"] =
        json!("0000000000000000000000000000000000000000000000000000000000000000");
    let tampered_verify_path = dir.join("tampered-verify.json");
    fs::write(
        &tampered_verify_path,
        serde_json::to_string_pretty(&tampered_verify)? + "\n",
    )?;
    let tampered_bundle = json!({
        "agent_receipt_path": agent_path,
        "replay_receipt_path": replay_path,
        "replay_verification_path": tampered_verify_path,
        "release_readiness_path": release_path,
        "release_readiness_receipt_hash": "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    });
    let tampered_bundle_path = dir.join("tampered-bundle.json");
    fs::write(
        &tampered_bundle_path,
        serde_json::to_string_pretty(&tampered_bundle)? + "\n",
    )?;
    let failure = run_json_failure_strings(&[
        "agent-receipt".into(),
        "verify-bundle".into(),
        tampered_bundle_path.display().to_string(),
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
    assert!(issue_codes.contains(&"replay_verification.replay_receipt_hash_mismatch"));
    assert!(issue_codes.contains(&"bundle.expected_hash_mismatch"));
    let failure_text = serde_json::to_string(&failure)?;
    assert!(
        !failure_text.contains(&dir.display().to_string()),
        "{failure_text}"
    );
    assert!(!failure_text.contains("SECRET"), "{failure_text}");
    Ok(())
}

#[test]
fn signed_evidence_manifest_verifies_indexed_evidence_artifacts() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.signed_evidence_manifest.verify]
    // r[verify cairn.supersede.signed_evidence_manifest.chain]
    // r[verify cairn.supersede.signed_evidence_manifest.metadata]
    // r[verify cairn.supersede.signed_evidence_manifest.schema]
    // r[verify cairn.supersede.signed_evidence_manifest.tests]
    // r[verify cairn.supersede.signed_evidence_manifest.docs]
    // r[verify cairn.supersede.signed_evidence_manifest.gates]
    // r[verify cairn.supersede.signed_evidence_manifest.archive]
    // r[verify cairn.supersede.evidence_manifest_generate.cli]
    // r[verify cairn.supersede.evidence_manifest_generate.canonical]
    // r[verify cairn.supersede.evidence_manifest_generate.metadata]
    // r[verify cairn.supersede.evidence_manifest_generate.failures]
    // r[verify cairn.supersede.evidence_manifest_generate.docs]
    // r[verify cairn.supersede.evidence_manifest_generate.tests]
    // r[verify cairn.supersede.evidence_manifest_generate.gates]
    // r[verify cairn.supersede.evidence_manifest_generate.archive]
    // r[verify cairn.supersede.determinism_coverage_release_evidence.bundle_manifest]
    // r[verify cairn.supersede.determinism_coverage_release_evidence.failures]
    // r[verify cairn.supersede.determinism_coverage_release_evidence.docs]
    // r[verify cairn.supersede.determinism_coverage_release_evidence.archive]
    let dir = temp_project("signed-evidence-manifest")?;
    let replay_path = dir.join("agent-replay.json");
    let replay_verify_path = dir.join("agent-replay-verify.json");
    let release_path = dir.join("release-readiness.json");
    let coverage_path = dir.join("coverage-audit.json");
    let agent_path = dir.join("agent-receipt.json");
    let bundle_path = dir.join("bundle.json");
    let manifest_path = dir.join("manifest.json");

    let mut replay = cairn();
    replay.args([
        "determinism",
        "replay",
        "--root",
        ".",
        "--group",
        "agent",
        "--format",
        "summary",
        "--out",
        replay_path.to_str().unwrap(),
    ]);
    let output = replay.output()?;
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let replay_verify = run_json_strings(&[
        "determinism".into(),
        "verify-receipt".into(),
        replay_path.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    fs::write(
        &replay_verify_path,
        serde_json::to_string_pretty(&replay_verify)? + "\n",
    )?;
    fs::write(
        &release_path,
        serde_json::to_string_pretty(&json!({
            "valid": true,
            "receipt_hash": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
        }))? + "\n",
    )?;
    let coverage = run_json_strings(&[
        "determinism".into(),
        "coverage-audit".into(),
        "--root".into(),
        ".".into(),
    ])?;
    fs::write(
        &coverage_path,
        serde_json::to_string_pretty(&coverage)? + "\n",
    )?;

    let mut agent_receipt: Value = serde_json::from_str(&fs::read_to_string(
        repo_root().join("fixtures/agent-receipts/positive.json"),
    )?)?;
    agent_receipt["replay_evidence"]["replay_receipt_hash"] =
        replay_verify["replay_receipt_hash"].clone();
    agent_receipt["replay_evidence"]["replay_policy_hash"] =
        replay_verify["replay_policy_hash"].clone();
    agent_receipt["replay_evidence"]["replay_environment_attestation_hash"] =
        replay_verify["environment_attestation_hash"].clone();
    agent_receipt
        .as_object_mut()
        .unwrap()
        .remove("receipt_hash");
    fs::write(
        &agent_path,
        serde_json::to_string_pretty(&agent_receipt)? + "\n",
    )?;
    let initial_agent_validation = run_json_strings(&[
        "agent-receipt".into(),
        "validate".into(),
        agent_path.display().to_string(),
    ])?;
    agent_receipt["receipt_hash"] = initial_agent_validation["agent_receipt_hash"].clone();
    fs::write(
        &agent_path,
        serde_json::to_string_pretty(&agent_receipt)? + "\n",
    )?;
    let agent_validation = run_json_strings(&[
        "agent-receipt".into(),
        "validate".into(),
        agent_path.display().to_string(),
    ])?;
    assert_eq!(agent_validation["valid"], true);

    let bundle = json!({
        "agent_receipt_path": agent_path,
        "replay_receipt_path": replay_path,
        "replay_verification_path": replay_verify_path,
        "release_readiness_path": release_path,
        "coverage_audit_path": coverage_path,
        "agent_receipt_hash": agent_validation["agent_receipt_hash"],
        "replay_receipt_hash": replay_verify["replay_receipt_hash"],
        "replay_verification_receipt_hash": replay_verify["receipt_hash"],
        "release_readiness_receipt_hash": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        "coverage_audit_receipt_hash": coverage["receipt_hash"],
    });
    fs::write(&bundle_path, serde_json::to_string_pretty(&bundle)? + "\n")?;
    let bundle_verify = run_json_strings(&[
        "agent-receipt".into(),
        "verify-bundle".into(),
        bundle_path.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    assert_eq!(bundle_verify["valid"], true);
    assert_eq!(
        bundle_verify["coverage_audit_receipt_hash"],
        coverage["receipt_hash"]
    );
    let mut bundle_with_hash = bundle.clone();
    bundle_with_hash["receipt_hash"] = bundle_verify["receipt_hash"].clone();
    fs::write(
        &bundle_path,
        serde_json::to_string_pretty(&bundle_with_hash)? + "\n",
    )?;

    let manifest = json!({
        "artifacts": [
            {"role": "agent_evidence_bundle", "path": bundle_path, "hash": bundle_verify["receipt_hash"], "schema": "bundle-v1"},
            {"role": "agent_receipt", "path": agent_path, "hash": agent_validation["agent_receipt_hash"], "schema": "agent-receipt-v1"},
            {"role": "replay_receipt", "path": replay_path, "hash": replay_verify["replay_receipt_hash"], "schema": "determinism-replay-v1"},
            {"role": "replay_verification", "path": replay_verify_path, "hash": replay_verify["receipt_hash"], "schema": "determinism-verify-receipt-v1"},
            {"role": "release_readiness", "path": release_path, "hash": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", "schema": "release-readiness-v1"},
            {"role": "coverage_audit", "path": coverage_path, "hash": coverage["receipt_hash"], "schema": "determinism-coverage-audit-v1"}
        ],
        "signer": {"kind": "ci", "id": "onix-ci"},
        "anchor": {"kind": "local-artifact-index", "id": "run-123"}
    });
    fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&manifest)? + "\n",
    )?;
    let verified = run_json_strings(&[
        "evidence-manifest".into(),
        "verify".into(),
        manifest_path.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    assert_eq!(verified["valid"], true);
    assert_eq!(verified["verdict"], "pass");
    assert_eq!(verified["artifacts"].as_array().unwrap().len(), 6);
    assert_eq!(
        verified["coverage_audit_receipt_hash"],
        coverage["receipt_hash"]
    );
    assert_hash_field(&verified, "manifest_hash")?;
    assert_hash_field(&verified, "signer_metadata_hash")?;
    assert_hash_field(&verified, "anchor_metadata_hash")?;
    assert_hash_field(&verified, "receipt_hash")?;

    let generated_manifest_path = dir.join("generated-manifest.json");
    let signer_path = dir.join("signer.json");
    let anchor_path = dir.join("anchor.json");
    fs::write(&signer_path, "{\"id\":\"onix-ci\",\"kind\":\"ci\"}\n")?;
    fs::write(
        &anchor_path,
        "{\"id\":\"run-123\",\"kind\":\"local-artifact-index\"}\n",
    )?;
    let generated = run_json_strings(&[
        "evidence-manifest".into(),
        "generate".into(),
        "--bundle".into(),
        bundle_path.display().to_string(),
        "--agent-receipt".into(),
        agent_path.display().to_string(),
        "--replay".into(),
        replay_path.display().to_string(),
        "--replay-verification".into(),
        replay_verify_path.display().to_string(),
        "--release-readiness".into(),
        release_path.display().to_string(),
        "--coverage-audit".into(),
        coverage_path.display().to_string(),
        "--signer-metadata".into(),
        signer_path.display().to_string(),
        "--anchor-metadata".into(),
        anchor_path.display().to_string(),
        "--out".into(),
        generated_manifest_path.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    assert_eq!(generated["valid"], true);
    assert_hash_field(&generated, "manifest_hash")?;
    let generated_manifest: Value =
        serde_json::from_str(&fs::read_to_string(&generated_manifest_path)?)?;
    let generated_roles = generated_manifest["artifacts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["role"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        generated_roles,
        vec![
            "agent_evidence_bundle",
            "agent_receipt",
            "replay_receipt",
            "replay_verification",
            "release_readiness",
            "coverage_audit"
        ]
    );
    assert_eq!(generated_manifest["signer"]["kind"], "ci");
    let generated_verified = run_json_strings(&[
        "evidence-manifest".into(),
        "verify".into(),
        generated_manifest_path.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    assert_eq!(generated_verified["valid"], true);
    assert_eq!(
        generated_verified["coverage_audit_receipt_hash"],
        coverage["receipt_hash"]
    );
    assert_eq!(
        generated_verified["manifest_hash"],
        generated["manifest_hash"]
    );

    let missing_generation = run_json_failure_strings(&[
        "evidence-manifest".into(),
        "generate".into(),
        "--bundle".into(),
        PathBuf::from("/tmp/SECRET-missing-bundle.json")
            .display()
            .to_string(),
        "--out".into(),
        "-".into(),
        "--root".into(),
        ".".into(),
    ])?;
    assert_eq!(missing_generation["valid"], false);
    let generation_text = serde_json::to_string(&missing_generation)?;
    assert!(
        !generation_text.contains(&dir.display().to_string()),
        "{generation_text}"
    );
    assert!(!generation_text.contains("SECRET"), "{generation_text}");

    fs::write(&signer_path, "[\"not-object\"]\n")?;
    let invalid_metadata = run_json_failure_strings(&[
        "evidence-manifest".into(),
        "generate".into(),
        "--bundle".into(),
        bundle_path.display().to_string(),
        "--agent-receipt".into(),
        agent_path.display().to_string(),
        "--replay".into(),
        replay_path.display().to_string(),
        "--replay-verification".into(),
        replay_verify_path.display().to_string(),
        "--signer-metadata".into(),
        signer_path.display().to_string(),
        "--out".into(),
        "-".into(),
        "--root".into(),
        ".".into(),
    ])?;
    let invalid_codes = invalid_metadata["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(invalid_codes.contains(&"generate.metadata_not_object"));

    let mut tampered = manifest.clone();
    tampered["artifacts"][0]["hash"] =
        json!("0000000000000000000000000000000000000000000000000000000000000000");
    tampered["signer"] = json!("SECRET=/tmp/private-token");
    let tampered_path = dir.join("tampered-manifest.json");
    fs::write(
        &tampered_path,
        serde_json::to_string_pretty(&tampered)? + "\n",
    )?;
    let failure = run_json_failure_strings(&[
        "evidence-manifest".into(),
        "verify".into(),
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
    assert!(issue_codes.contains(&"artifact.hash_mismatch"));
    assert!(issue_codes.contains(&"manifest.metadata_invalid"));
    let failure_text = serde_json::to_string(&failure)?;
    assert!(
        !failure_text.contains(&dir.display().to_string()),
        "{failure_text}"
    );
    assert!(!failure_text.contains("SECRET"), "{failure_text}");
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

#[test]
fn evidence_export_materializes_complete_portable_chain() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.evidence_export_bundle.command]
    // r[verify cairn.supersede.evidence_export_bundle.artifacts]
    // r[verify cairn.supersede.evidence_export_bundle.failures]
    // r[verify cairn.supersede.evidence_export_bundle.gates]
    // r[verify cairn.supersede.evidence_export_bundle.docs]
    // r[verify cairn.supersede.evidence_export_bundle.archive]
    // r[verify cairn.supersede.evidence_bundle_verify_command.command]
    // r[verify cairn.supersede.evidence_bundle_verify_command.artifacts]
    // r[verify cairn.supersede.evidence_bundle_verify_command.failures]
    // r[verify cairn.supersede.evidence_bundle_verify_command.gates]
    // r[verify cairn.supersede.evidence_bundle_verify_command.docs]
    // r[verify cairn.supersede.evidence_bundle_verify_command.archive]
    // r[verify cairn.supersede.evidence_import_consume_rail.command]
    // r[verify cairn.supersede.evidence_import_consume_rail.artifacts]
    // r[verify cairn.supersede.evidence_import_consume_rail.failures]
    // r[verify cairn.supersede.evidence_import_consume_rail.gates]
    // r[verify cairn.supersede.evidence_import_consume_rail.archive]
    // r[verify cairn.supersede.evidence_import_republish_rail.command]
    // r[verify cairn.supersede.evidence_import_republish_rail.artifacts]
    // r[verify cairn.supersede.evidence_import_republish_rail.failures]
    // r[verify cairn.supersede.evidence_import_republish_rail.gates]
    // r[verify cairn.supersede.evidence_import_republish_rail.archive]
    // r[verify cairn.supersede.evidence_publish_index_rail.command]
    // r[verify cairn.supersede.evidence_publish_index_rail.verify]
    // r[verify cairn.supersede.evidence_publish_index_rail.failures]
    // r[verify cairn.supersede.evidence_publish_index_rail.gates]
    // r[verify cairn.supersede.evidence_publish_index_rail.archive]
    // r[verify cairn.supersede.evidence_registry_rail.build]
    // r[verify cairn.supersede.evidence_registry_rail.verify]
    // r[verify cairn.supersede.evidence_registry_rail.failures]
    // r[verify cairn.supersede.evidence_registry_rail.gates]
    // r[verify cairn.supersede.evidence_registry_rail.archive]
    // r[verify cairn.supersede.evidence_registry_consume.select]
    // r[verify cairn.supersede.evidence_registry_consume.fetch_plan]
    // r[verify cairn.supersede.evidence_registry_consume.policy]
    // r[verify cairn.supersede.evidence_registry_consume.docs]
    // r[verify cairn.supersede.evidence_registry_consume.verify]
    // r[verify cairn.supersede.registry_channel_metadata.labels]
    // r[verify cairn.supersede.registry_channel_metadata.policy]
    // r[verify cairn.supersede.registry_channel_metadata.build]
    // r[verify cairn.supersede.registry_channel_metadata.consume]
    // r[verify cairn.supersede.registry_channel_metadata.verify]
    // r[verify cairn.supersede.registry_manifest_binding.roles]
    // r[verify cairn.supersede.registry_manifest_binding.generate]
    // r[verify cairn.supersede.registry_manifest_binding.verify]
    // r[verify cairn.supersede.registry_manifest_binding.failure]
    let dir = temp_project("evidence-export-bundle")?;
    let export_dir = dir.join("export");
    let receipt = run_json_strings(&[
        "evidence".into(),
        "export".into(),
        "--root".into(),
        ".".into(),
        "--out".into(),
        export_dir.display().to_string(),
    ])?;
    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["verdict"], "pass");
    assert_eq!(receipt["issues"].as_array().unwrap().len(), 0);
    assert_hash_field(&receipt, "manifest_hash")?;
    assert_hash_field(&receipt, "manifest_verification_receipt_hash")?;
    assert_hash_field(&receipt, "receipt_hash")?;
    let roles = receipt["artifacts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|artifact| artifact["role"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        roles,
        vec![
            "agent_evidence_bundle",
            "agent_evidence_bundle_verification",
            "agent_receipt",
            "coverage_audit",
            "evidence_manifest",
            "evidence_manifest_verification",
            "release_readiness",
            "replay_receipt",
            "replay_verification",
        ]
    );
    for artifact in receipt["artifacts"].as_array().unwrap() {
        assert_hash_field(artifact, "hash")?;
        assert_eq!(artifact["required"], true);
        let path = artifact["path"].as_str().unwrap();
        assert!(!path.contains(&dir.display().to_string()), "{path}");
        assert!(
            export_dir.join(path).exists(),
            "missing exported artifact: {path}"
        );
    }
    let saved_receipt: Value =
        serde_json::from_slice(&fs::read(export_dir.join("evidence-export-receipt.json"))?)?;
    assert_eq!(saved_receipt["receipt_hash"], receipt["receipt_hash"]);
    let bundle_verification: Value = serde_json::from_slice(&fs::read(
        export_dir.join("agent-evidence-bundle-verification.json"),
    )?)?;
    assert_eq!(bundle_verification["valid"], true);
    let manifest_verification: Value = serde_json::from_slice(&fs::read(
        export_dir.join("evidence-manifest-verification.json"),
    )?)?;
    assert_eq!(manifest_verification["valid"], true);
    assert_eq!(
        manifest_verification["receipt_hash"],
        receipt["manifest_verification_receipt_hash"]
    );
    assert_eq!(
        manifest_verification["manifest_hash"],
        receipt["manifest_hash"]
    );
    let text = serde_json::to_string(&receipt)?;
    assert!(!text.contains(&dir.display().to_string()), "{text}");
    assert!(!text.contains("SECRET"), "{text}");

    let verification = run_json_strings(&[
        "evidence".into(),
        "verify".into(),
        "--root".into(),
        ".".into(),
        "--bundle".into(),
        export_dir.display().to_string(),
    ])?;
    assert_eq!(verification["valid"], true);
    assert_eq!(verification["verdict"], "pass");
    assert_eq!(verification["issues"].as_array().unwrap().len(), 0);
    assert_eq!(verification["bundle"], "<bundle>");
    assert_eq!(verification["export_receipt_hash"], receipt["receipt_hash"]);
    assert_eq!(verification["manifest_hash"], receipt["manifest_hash"]);
    assert_eq!(
        verification["manifest_verification_receipt_hash"],
        receipt["manifest_verification_receipt_hash"]
    );
    assert_hash_field(&verification, "receipt_hash")?;
    let verification_text = serde_json::to_string(&verification)?;
    assert!(
        !verification_text.contains(&dir.display().to_string()),
        "{verification_text}"
    );

    let import_dir = dir.join("imported");
    let import = run_json_strings(&[
        "evidence".into(),
        "import".into(),
        "--root".into(),
        ".".into(),
        "--bundle".into(),
        export_dir.display().to_string(),
        "--into".into(),
        import_dir.display().to_string(),
    ])?;
    assert_eq!(import["valid"], true);
    assert_eq!(import["verdict"], "pass");
    assert_eq!(import["bundle"], "<bundle>");
    assert_eq!(import["into"], "<into>");
    assert_eq!(
        import["verification_receipt_hash"],
        verification["receipt_hash"]
    );
    assert_eq!(import["artifacts"].as_array().unwrap().len(), roles.len());
    assert_hash_field(&import, "receipt_hash")?;
    let saved_import: Value =
        serde_json::from_slice(&fs::read(import_dir.join("evidence-import-receipt.json"))?)?;
    assert_eq!(saved_import["receipt_hash"], import["receipt_hash"]);
    for artifact in import["artifacts"].as_array().unwrap() {
        assert_hash_field(artifact, "hash")?;
        let path = artifact["path"].as_str().unwrap();
        assert!(
            import_dir.join(path).exists(),
            "missing imported artifact: {path}"
        );
        assert_eq!(
            fs::read(import_dir.join(path))?,
            fs::read(export_dir.join(path))?,
            "imported artifact bytes must match export for {path}"
        );
    }
    let import_text = serde_json::to_string(&import)?;
    assert!(
        !import_text.contains(&dir.display().to_string()),
        "{import_text}"
    );

    let republish_dir = dir.join("republished");
    let republish = run_json_strings(&[
        "evidence".into(),
        "republish".into(),
        "--root".into(),
        ".".into(),
        "--imported".into(),
        import_dir.display().to_string(),
        "--out".into(),
        republish_dir.display().to_string(),
    ])?;
    assert_eq!(republish["valid"], true);
    assert_eq!(republish["verdict"], "pass");
    assert_eq!(republish["imported"], "<imported>");
    assert_eq!(republish["out"], "<out>");
    assert_eq!(republish["import_receipt_hash"], import["receipt_hash"]);
    assert_eq!(
        republish["artifacts"].as_array().unwrap().len(),
        roles.len()
    );
    assert_hash_field(&republish, "receipt_hash")?;
    let saved_republish: Value = serde_json::from_slice(&fs::read(
        republish_dir.join("evidence-republish-receipt.json"),
    )?)?;
    assert_eq!(saved_republish["receipt_hash"], republish["receipt_hash"]);
    assert!(republish_dir.join("evidence-import-receipt.json").exists());
    for artifact in republish["artifacts"].as_array().unwrap() {
        assert_hash_field(artifact, "hash")?;
        let path = artifact["path"].as_str().unwrap();
        assert!(
            republish_dir.join(path).exists(),
            "missing republished artifact: {path}"
        );
        assert_eq!(
            fs::read(republish_dir.join(path))?,
            fs::read(import_dir.join(path))?,
            "republished artifact bytes must match import for {path}"
        );
    }
    let republish_text = serde_json::to_string(&republish)?;
    assert!(
        !republish_text.contains(&dir.display().to_string()),
        "{republish_text}"
    );

    let index_path = republish_dir.join("evidence-index-receipt.json");
    let index = run_json_strings(&[
        "evidence".into(),
        "index".into(),
        "--root".into(),
        ".".into(),
        "--bundle".into(),
        republish_dir.display().to_string(),
        "--out".into(),
        index_path.display().to_string(),
    ])?;
    assert_eq!(index["valid"], true);
    assert_eq!(index["verdict"], "pass");
    assert_eq!(index["bundle"], "<bundle>");
    assert_eq!(index["out"], "<out>");
    assert_eq!(index["bundle_kind"], "republished");
    assert_eq!(index["bundle_receipt_hash"], republish["receipt_hash"]);
    assert_eq!(
        index["artifacts"].as_array().unwrap().len(),
        republish["artifacts"].as_array().unwrap().len()
    );
    assert_hash_field(&index, "receipt_hash")?;
    let saved_index: Value = serde_json::from_slice(&fs::read(&index_path)?)?;
    assert_eq!(saved_index["receipt_hash"], index["receipt_hash"]);
    let index_verify = run_json_strings(&[
        "evidence".into(),
        "index".into(),
        "verify".into(),
        "--root".into(),
        ".".into(),
        "--index".into(),
        index_path.display().to_string(),
    ])?;
    assert_eq!(index_verify["valid"], true);
    assert_eq!(index_verify["verdict"], "pass");
    assert_eq!(index_verify["index_receipt_hash"], index["receipt_hash"]);
    assert_eq!(index_verify["bundle_kind"], "republished");
    assert_hash_field(&index_verify, "receipt_hash")?;

    let registry_path = republish_dir.join("evidence-registry-receipt.json");
    let registry = run_json_strings(&[
        "evidence".into(),
        "registry".into(),
        "build".into(),
        "--root".into(),
        ".".into(),
        "--index".into(),
        index_path.display().to_string(),
        "--out".into(),
        registry_path.display().to_string(),
        "--channel".into(),
        "stable".into(),
        "--release".into(),
        "2026-05-19".into(),
        "--platform".into(),
        "x86_64-linux".into(),
        "--compatibility-class".into(),
        "current".into(),
    ])?;
    assert_eq!(registry["valid"], true);
    assert_eq!(registry["verdict"], "pass");
    assert_eq!(registry["out"], "<out>");
    assert_eq!(registry["entry_count"], 1);
    assert_hash_field(&registry, "receipt_hash")?;
    let registry_entry = &registry["entries"][0];
    assert_eq!(registry_entry["index_receipt_hash"], index["receipt_hash"]);
    assert_eq!(registry_entry["bundle_kind"], "republished");
    assert_eq!(registry_entry["labels"]["channel"], "stable");
    assert_eq!(registry_entry["labels"]["release"], "2026-05-19");
    assert_eq!(registry_entry["labels"]["platform"], "x86_64-linux");
    assert_eq!(registry_entry["labels"]["compatibility_class"], "current");
    assert_eq!(
        registry_entry["bundle_receipt_hash"],
        republish["receipt_hash"]
    );
    assert_eq!(
        registry_entry["index_relative"],
        "evidence-index-receipt.json"
    );
    let saved_registry: Value = serde_json::from_slice(&fs::read(&registry_path)?)?;
    assert_eq!(saved_registry["receipt_hash"], registry["receipt_hash"]);
    let registry_verify = run_json_strings(&[
        "evidence".into(),
        "registry".into(),
        "verify".into(),
        "--root".into(),
        ".".into(),
        "--registry".into(),
        registry_path.display().to_string(),
    ])?;
    let registry_verify_path = republish_dir.join("evidence-registry-verify-receipt.json");
    fs::write(
        &registry_verify_path,
        serde_json::to_string_pretty(&registry_verify)? + "\n",
    )?;
    assert_eq!(registry_verify["valid"], true);
    assert_eq!(registry_verify["verdict"], "pass");
    assert_eq!(registry_verify["registry_hash"], registry["receipt_hash"]);
    assert_eq!(registry_verify["entry_count"], 1);
    assert_hash_field(&registry_verify, "receipt_hash")?;

    let mirror_verify = run_json_strings(&[
        "evidence".into(),
        "registry".into(),
        "mirror-verify".into(),
        "--root".into(),
        ".".into(),
        "--registry".into(),
        registry_path.display().to_string(),
        "--mirror".into(),
        republish_dir.display().to_string(),
    ])?;
    assert_eq!(mirror_verify["valid"], true);
    assert_eq!(mirror_verify["verdict"], "pass");
    assert_eq!(mirror_verify["registry"], "<registry>");
    assert_eq!(mirror_verify["mirror"], "<mirror>");
    assert_eq!(mirror_verify["registry_hash"], registry["receipt_hash"]);
    assert_eq!(
        mirror_verify["registry_verification_receipt_hash"],
        registry_verify["receipt_hash"]
    );
    assert_eq!(mirror_verify["entry_count"], 1);
    assert_eq!(
        mirror_verify["entries"][0]["index_receipt_hash"],
        index["receipt_hash"]
    );
    assert_hash_field(&mirror_verify, "receipt_hash")?;
    let mirror_text = serde_json::to_string(&mirror_verify)?;
    assert!(
        !mirror_text.contains(&dir.display().to_string()),
        "{mirror_text}"
    );

    let missing_mirror = dir.join("missing-index-mirror");
    fs::create_dir_all(&missing_mirror)?;
    fs::copy(
        &registry_path,
        missing_mirror.join("evidence-registry-receipt.json"),
    )?;
    let missing_mirror_failure = run_json_failure_strings(&[
        "evidence".into(),
        "registry".into(),
        "mirror-verify".into(),
        "--root".into(),
        ".".into(),
        "--registry".into(),
        registry_path.display().to_string(),
        "--mirror".into(),
        missing_mirror.display().to_string(),
    ])?;
    assert_eq!(missing_mirror_failure["valid"], false);
    let missing_mirror_codes = missing_mirror_failure["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(missing_mirror_codes.contains(&"evidence_registry_mirror_verify.index_missing"));

    let tampered_mirror = dir.join("tampered-index-mirror");
    fs::create_dir_all(&tampered_mirror)?;
    let tampered_registry_path = tampered_mirror.join("evidence-registry-receipt.json");
    let tampered_index_path = tampered_mirror.join("evidence-index-receipt.json");
    fs::copy(&registry_path, &tampered_registry_path)?;
    fs::copy(&index_path, &tampered_index_path)?;
    let mut tampered_index: Value = serde_json::from_slice(&fs::read(&tampered_index_path)?)?;
    tampered_index["receipt_hash"] =
        Value::String("0000000000000000000000000000000000000000000000000000000000000000".into());
    fs::write(
        &tampered_index_path,
        serde_json::to_string_pretty(&tampered_index)? + "\n",
    )?;
    let tampered_mirror_failure = run_json_failure_strings(&[
        "evidence".into(),
        "registry".into(),
        "mirror-verify".into(),
        "--root".into(),
        ".".into(),
        "--registry".into(),
        tampered_registry_path.display().to_string(),
        "--mirror".into(),
        tampered_mirror.display().to_string(),
    ])?;
    assert_eq!(tampered_mirror_failure["valid"], false);
    let tampered_mirror_codes = tampered_mirror_failure["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(tampered_mirror_codes.contains(&"evidence_registry_mirror_verify.index_hash_mismatch"));

    let selected_index_path = dir.join("selected-cairn-evidence-index.json");
    let selection = run_json_strings(&[
        "evidence".into(),
        "registry".into(),
        "select".into(),
        "--root".into(),
        ".".into(),
        "--registry".into(),
        registry_path.display().to_string(),
        "--bundle-kind".into(),
        "republished".into(),
        "--channel".into(),
        "stable".into(),
        "--release".into(),
        "2026-05-19".into(),
        "--platform".into(),
        "x86_64-linux".into(),
        "--compatibility-class".into(),
        "current".into(),
        "--out".into(),
        selected_index_path.display().to_string(),
    ])?;
    assert_eq!(selection["valid"], true);
    assert_eq!(selection["verdict"], "pass");
    assert_eq!(selection["registry"], "<registry>");
    assert_eq!(selection["out"], "<out>");
    assert_eq!(selection["registry_receipt_hash"], registry["receipt_hash"]);
    assert_eq!(
        selection["registry_verification_receipt_hash"],
        registry_verify["receipt_hash"]
    );
    assert_eq!(selection["selectors"]["bundle_kind"], "republished");
    assert_eq!(selection["selectors"]["channel"], "stable");
    assert_eq!(selection["selectors"]["release"], "2026-05-19");
    assert_eq!(selection["selectors"]["platform"], "x86_64-linux");
    assert_eq!(selection["selectors"]["compatibility_class"], "current");
    assert_eq!(
        selection["selected_entry"]["index_receipt_hash"],
        index["receipt_hash"]
    );
    assert_hash_field(&selection, "receipt_hash")?;
    let selection_receipt_path = dir.join("evidence-registry-selection-receipt.json");
    fs::write(
        &selection_receipt_path,
        serde_json::to_string_pretty(&selection)? + "\n",
    )?;
    let selected_index: Value = serde_json::from_slice(&fs::read(&selected_index_path)?)?;
    assert_eq!(selected_index["receipt_hash"], index["receipt_hash"]);
    let selection_text = serde_json::to_string(&selection)?;
    assert!(
        !selection_text.contains(&dir.display().to_string()),
        "{selection_text}"
    );

    let fetch_plan_path = dir.join("cairn-evidence-fetch-plan.json");
    let fetch_plan = run_json_strings(&[
        "evidence".into(),
        "registry".into(),
        "fetch-plan".into(),
        "--root".into(),
        ".".into(),
        "--registry".into(),
        registry_path.display().to_string(),
        "--channel".into(),
        "stable".into(),
        "--platform".into(),
        "x86_64-linux".into(),
        "--out".into(),
        fetch_plan_path.display().to_string(),
    ])?;
    assert_eq!(fetch_plan["valid"], true);
    assert_eq!(fetch_plan["verdict"], "pass");
    assert_eq!(fetch_plan["registry"], "<registry>");
    assert_eq!(fetch_plan["out"], "<out>");
    assert_eq!(
        fetch_plan["registry_receipt_hash"],
        registry["receipt_hash"]
    );
    assert_eq!(fetch_plan["entry_count"], 1);
    assert_eq!(fetch_plan["selectors"]["channel"], "stable");
    assert_eq!(fetch_plan["selectors"]["platform"], "x86_64-linux");
    assert_eq!(
        fetch_plan["entries"][0]["index_receipt_hash"],
        index["receipt_hash"]
    );
    assert_hash_field(&fetch_plan, "receipt_hash")?;
    let saved_fetch_plan: Value = serde_json::from_slice(&fs::read(&fetch_plan_path)?)?;
    assert_eq!(saved_fetch_plan["receipt_hash"], fetch_plan["receipt_hash"]);

    let registry_manifest_path = dir.join("registry-bound-manifest.json");
    let registry_manifest = run_json_strings(&[
        "evidence-manifest".into(),
        "generate".into(),
        "--bundle".into(),
        export_dir
            .join("agent-evidence-bundle.json")
            .display()
            .to_string(),
        "--agent-receipt".into(),
        export_dir.join("agent-receipt.json").display().to_string(),
        "--replay".into(),
        export_dir.join("replay-agent.json").display().to_string(),
        "--replay-verification".into(),
        export_dir
            .join("replay-verification.json")
            .display()
            .to_string(),
        "--registry".into(),
        registry_path.display().to_string(),
        "--registry-verification".into(),
        registry_verify_path.display().to_string(),
        "--registry-selection".into(),
        selection_receipt_path.display().to_string(),
        "--registry-fetch-plan".into(),
        fetch_plan_path.display().to_string(),
        "--out".into(),
        registry_manifest_path.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    assert_eq!(registry_manifest["valid"], true);
    let registry_manifest_value: Value =
        serde_json::from_str(&fs::read_to_string(&registry_manifest_path)?)?;
    let registry_manifest_roles = registry_manifest_value["artifacts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["role"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(registry_manifest_roles.contains(&"evidence_registry"));
    assert!(registry_manifest_roles.contains(&"evidence_registry_verification"));
    assert!(registry_manifest_roles.contains(&"evidence_registry_selection"));
    assert!(registry_manifest_roles.contains(&"evidence_registry_fetch_plan"));
    let registry_manifest_verified = run_json_strings(&[
        "evidence-manifest".into(),
        "verify".into(),
        registry_manifest_path.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    assert_eq!(registry_manifest_verified["valid"], true);
    assert_eq!(
        registry_manifest_verified["manifest_hash"],
        registry_manifest["manifest_hash"]
    );
    let verified_registry_roles = registry_manifest_verified["artifacts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["role"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(verified_registry_roles.contains(&"evidence_registry"));
    assert!(verified_registry_roles.contains(&"evidence_registry_verification"));
    assert!(verified_registry_roles.contains(&"evidence_registry_selection"));
    assert!(verified_registry_roles.contains(&"evidence_registry_fetch_plan"));

    let fetch_plan_text = serde_json::to_string(&fetch_plan)?;
    assert!(
        !fetch_plan_text.contains(&dir.display().to_string()),
        "{fetch_plan_text}"
    );

    let no_match_selection = run_json_failure_strings(&[
        "evidence".into(),
        "registry".into(),
        "select".into(),
        "--root".into(),
        ".".into(),
        "--registry".into(),
        registry_path.display().to_string(),
        "--bundle-kind".into(),
        "exported".into(),
        "--out".into(),
        dir.join("no-match-index.json").display().to_string(),
    ])?;
    assert_eq!(no_match_selection["valid"], false);
    let no_match_codes = no_match_selection["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(no_match_codes.contains(&"evidence_registry_select.no_match"));

    let duplicate_registry_failure = run_json_failure_strings(&[
        "evidence".into(),
        "registry".into(),
        "build".into(),
        "--root".into(),
        ".".into(),
        "--index".into(),
        index_path.display().to_string(),
        "--index".into(),
        index_path.display().to_string(),
        "--out".into(),
        republish_dir
            .join("duplicate-registry.json")
            .display()
            .to_string(),
    ])?;
    assert_eq!(duplicate_registry_failure["valid"], false);
    let duplicate_registry_codes = duplicate_registry_failure["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(duplicate_registry_codes.contains(&"evidence_registry.duplicate_index_hash"));

    let invalid_label_failure = run_json_failure_strings(&[
        "evidence".into(),
        "registry".into(),
        "build".into(),
        "--root".into(),
        ".".into(),
        "--index".into(),
        index_path.display().to_string(),
        "--out".into(),
        republish_dir
            .join("invalid-label-registry.json")
            .display()
            .to_string(),
        "--channel".into(),
        "../../stable".into(),
    ])?;
    assert_eq!(invalid_label_failure["valid"], false);
    let invalid_label_codes = invalid_label_failure["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(invalid_label_codes.contains(&"evidence_registry.label_unsafe"));
    let registry_text = serde_json::to_string(&registry)?;
    assert!(
        !registry_text.contains(&dir.display().to_string()),
        "{registry_text}"
    );
    let index_text = serde_json::to_string(&index)?;
    assert!(
        !index_text.contains(&dir.display().to_string()),
        "{index_text}"
    );

    fs::write(
        republish_dir.join(index["artifacts"][0]["path"].as_str().unwrap()),
        "tampered\n",
    )?;
    let index_verify_failure = run_json_failure_strings(&[
        "evidence".into(),
        "index".into(),
        "verify".into(),
        "--root".into(),
        ".".into(),
        "--index".into(),
        index_path.display().to_string(),
    ])?;
    assert_eq!(index_verify_failure["valid"], false);
    let index_verify_issue_codes = index_verify_failure["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(
        index_verify_issue_codes.contains(&"evidence_index_verify.artifact_hash_mismatch")
            || index_verify_issue_codes.contains(&"evidence_index_verify.index_bundle_mismatch")
    );
    let registry_verify_failure = run_json_failure_strings(&[
        "evidence".into(),
        "registry".into(),
        "verify".into(),
        "--root".into(),
        ".".into(),
        "--registry".into(),
        registry_path.display().to_string(),
    ])?;
    assert_eq!(registry_verify_failure["valid"], false);
    let registry_verify_codes = registry_verify_failure["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(registry_verify_codes.contains(&"evidence_registry_verify.index_verify_failed"));

    fs::write(import_dir.join("undeclared-imported-extra.json"), "{}\n")?;
    let republish_failure = run_json_failure_strings(&[
        "evidence".into(),
        "republish".into(),
        "--root".into(),
        ".".into(),
        "--imported".into(),
        import_dir.display().to_string(),
        "--out".into(),
        dir.join("bad-republish").display().to_string(),
    ])?;
    assert_eq!(republish_failure["valid"], false);
    let republish_issue_codes = republish_failure["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(republish_issue_codes.contains(&"evidence_republish.undeclared_imported_file"));
    let republish_failure_text = serde_json::to_string(&republish_failure)?;
    assert!(
        !republish_failure_text.contains(&dir.display().to_string()),
        "{republish_failure_text}"
    );
    fs::remove_file(import_dir.join("undeclared-imported-extra.json"))?;

    fs::write(export_dir.join("undeclared-extra.json"), "{}\n")?;
    let import_failure = run_json_failure_strings(&[
        "evidence".into(),
        "import".into(),
        "--root".into(),
        ".".into(),
        "--bundle".into(),
        export_dir.display().to_string(),
        "--into".into(),
        dir.join("bad-import").display().to_string(),
    ])?;
    assert_eq!(import_failure["valid"], false);
    let import_issue_codes = import_failure["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(import_issue_codes.contains(&"evidence_import.undeclared_bundle_file"));
    fs::remove_file(export_dir.join("undeclared-extra.json"))?;

    fs::write(export_dir.join("coverage-audit.json"), "{}\n")?;
    let verify_failure = run_json_failure_strings(&[
        "evidence".into(),
        "verify".into(),
        "--root".into(),
        ".".into(),
        "--bundle".into(),
        export_dir.display().to_string(),
    ])?;
    assert_eq!(verify_failure["valid"], false);
    let verify_issue_codes = verify_failure["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(verify_issue_codes.contains(&"evidence_verify.artifact_hash_mismatch"));
    assert!(verify_issue_codes.contains(&"evidence_verify.coverage_audit_invalid"));
    let verify_failure_text = serde_json::to_string(&verify_failure)?;
    assert!(
        !verify_failure_text.contains(&dir.display().to_string()),
        "{verify_failure_text}"
    );

    let bad_agent = dir.join("bad-agent.json");
    let mut stale_agent: Value = serde_json::from_str(&fs::read_to_string(
        repo_root().join("fixtures/agent-receipts/positive.json"),
    )?)?;
    stale_agent["replay_evidence"]["replay_receipt_hash"] =
        json!("0000000000000000000000000000000000000000000000000000000000000000");
    fs::write(
        &bad_agent,
        serde_json::to_string_pretty(&stale_agent)? + "\n",
    )?;
    let bad_export = dir.join("bad-export");
    let failure = run_json_failure_strings(&[
        "evidence".into(),
        "export".into(),
        "--root".into(),
        ".".into(),
        "--out".into(),
        bad_export.display().to_string(),
        "--agent-receipt".into(),
        bad_agent.display().to_string(),
    ])?;
    assert_eq!(failure["valid"], false);
    let issue_codes = failure["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(issue_codes.contains(&"export.bundle_verification_failed"));
    assert!(issue_codes.contains(&"export.manifest_verification_failed"));
    let failure_text = serde_json::to_string(&failure)?;
    assert!(
        !failure_text.contains(&dir.display().to_string()),
        "{failure_text}"
    );
    Ok(())
}

#[test]
fn release_readiness_lists_replay_oracle_preflight() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.release_readiness]
    // r[verify cairn.supersede.release_readiness.determinism_replay]
    // r[verify cairn.supersede.determinism_coverage_release_evidence.release]
    // r[verify cairn.supersede.determinism_coverage_release_evidence.gates]
    let value = run_json(&["release-readiness", "--root", "."])?;
    assert_eq!(value["determinism_replay"]["required"], true);
    assert_eq!(value["coverage_audit_valid"], true);
    assert_hash_field(&value, "coverage_audit_receipt_hash")?;
    assert_eq!(value["determinism_replay"]["included"], false);
    assert_eq!(
        value["determinism_replay"]["command"],
        "determinism replay --root <root>"
    );
    Ok(())
}

#[test]
fn executed_mutations_emit_before_after_manifest() -> Result<(), Box<dyn Error>> {
    // r[verify sync_archive.execute.sync] r[verify sync_archive.execute.archive]
    // r[verify sync_archive.execute.mutation_manifest] r[verify sync_archive.execute.mutation_manifest.sync]
    // r[verify sync_archive.execute.mutation_manifest.archive] r[verify sync_archive.execute.mutation_manifest.migration]
    let sync_root = temp_fixture("mutation-sync", "fixtures/archive-ready")?;
    let sync = run_json_strings(&[
        "sync".into(),
        "ready".into(),
        "--root".into(),
        sync_root.display().to_string(),
        "--execute".into(),
    ])?;
    assert_eq!(sync["mutated"], true);
    assert_eq!(sync["mutation_manifest"]["kind"], "sync");
    assert_hash_field(&sync["mutation_manifest"], "manifest_hash")?;
    assert!(sync["mutation_manifest"]["before"]["entries"].is_array());
    assert!(sync["mutation_manifest"]["after"]["entries"].is_array());

    let archive_root = temp_fixture("mutation-archive", "fixtures/archive-ready")?;
    let archive = run_json_strings(&[
        "archive".into(),
        "ready".into(),
        "--root".into(),
        archive_root.display().to_string(),
        "--execute".into(),
    ])?;
    assert_eq!(archive["mutated"], true);
    assert_eq!(archive["mutation_manifest"]["kind"], "archive");
    assert_hash_field(&archive["mutation_manifest"], "manifest_hash")
}

#[test]
fn status_matches_canonical_golden() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.rename_cairn] r[verify cairn.supersede.rename_cairn.packaging]
    assert_matches_golden(&["status", "--root", "fixtures/basic"], "status-basic.json")
}

#[test]
fn validate_matches_canonical_golden() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.boundary.pure_core]
    // r[verify cairn.supersede.tracey_coverage] r[verify cairn.supersede.tracey_coverage.config]
    // r[verify cairn.supersede.tracey_coverage.check] r[verify cairn.supersede.tracey_coverage.boundary]
    assert_matches_golden(
        &["validate", "--root", "fixtures/basic"],
        "validate-basic.json",
    )
}

#[test]
fn doctor_reports_healthy_fixture() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.doctor]
    let value = run_json(&["doctor", "--root", "fixtures/basic"])?;
    assert_eq!(value["valid"], true);
    assert_eq!(value["layout"], "cairn");
    assert_eq!(value["spec_count"], 1);
    assert_eq!(value["change_count"], 1);
    assert_eq!(value["validation_issue_count"], 0);
    let checks = value["checks"].as_array().ok_or("checks")?;
    assert!(checks
        .iter()
        .any(|check| check["name"] == "policy" && check["status"] == "ok"));
    Ok(())
}

#[test]
fn doctor_reports_missing_policy_without_process_failure() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.doctor]
    let value = run_json(&[
        "doctor",
        "--root",
        "fixtures/basic",
        "--policy",
        "fixtures/basic/missing-policy.json",
    ])?;
    assert_eq!(value["valid"], false);
    let checks = value["checks"].as_array().ok_or("checks")?;
    assert!(checks
        .iter()
        .any(|check| check["name"] == "policy" && check["status"] == "fail"));
    Ok(())
}

#[test]
fn plan_outputs_match_canonical_goldens() -> Result<(), Box<dyn Error>> {
    assert_matches_golden(
        &["sync", "demo", "--root", "fixtures/basic"],
        "sync-basic-demo.json",
    )?;
    assert_matches_golden(
        &["archive", "ready", "--root", "fixtures/archive-ready"],
        "archive-ready.json",
    )?;
    assert_matches_golden(
        &["sync", "no-delta", "--root", "fixtures/no-delta"],
        "sync-no-delta.json",
    )?;
    assert_matches_golden(
        &["sync", "missing", "--root", "fixtures/missing-artifacts"],
        "sync-missing-artifacts.json",
    )?;
    Ok(())
}

#[test]
fn receipt_hashes_are_reproducible() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.blake3_receipts.gate]
    // r[verify cairn.supersede.blake3_receipts.lifecycle_plans]
    // r[verify cairn.supersede.blake3_receipts.migration]
    // r[verify cairn.supersede.blake3_receipts.canonical]
    // r[verify cairn.supersede.deterministic_gates.stage]
    let gate_hashes = ["policy_hash", "input_hash", "receipt_hash"];
    assert_reproducible_receipt(
        &["gate", "proposal", "demo", "--root", "fixtures/basic"],
        &gate_hashes,
    )?;
    assert_reproducible_receipt(
        &["gate", "design", "demo", "--root", "fixtures/basic"],
        &gate_hashes,
    )?;
    assert_reproducible_receipt(
        &["gate", "tasks", "demo", "--root", "fixtures/basic"],
        &gate_hashes,
    )?;

    let lifecycle_hashes = ["policy_hash", "input_hash", "plan_hash", "receipt_hash"];
    assert_reproducible_receipt(
        &["sync", "demo", "--root", "fixtures/basic"],
        &lifecycle_hashes,
    )?;
    assert_reproducible_receipt(
        &["archive", "ready", "--root", "fixtures/archive-ready"],
        &lifecycle_hashes,
    )?;

    let migration = assert_reproducible_receipt(
        &["migrate", "openspec", "--root", "fixtures/basic"],
        &[
            "policy_hash",
            "source_manifest_hash",
            "destination_manifest_hash",
            "manifest_hash",
            "receipt_hash",
        ],
    )?;
    let actions = migration["actions"].as_array().ok_or("migration actions")?;
    assert!(!actions.is_empty());
    for action in actions {
        assert_hash_field(action, "source_hash")?;
        assert_hash_field(action, "destination_hash")?;
    }
    Ok(())
}

fn assert_gate_receipt_schema() -> Result<(), Box<dyn Error>> {
    let gate = run_json(&["gate", "tasks", "demo", "--root", "fixtures/basic"])?;
    let schema = receipt_schema("gate")?;
    assert_schema_keys(&gate, &schema, "top_level_keys")
}

fn assert_sync_receipt_schema() -> Result<(), Box<dyn Error>> {
    let sync = run_json(&["sync", "demo", "--root", "fixtures/basic"])?;
    let schema = receipt_schema("sync")?;
    assert_schema_keys(&sync, &schema, "top_level_keys")?;
    assert_first_action_matches_schema(&sync, &schema)
}

fn assert_archive_receipt_schema() -> Result<(), Box<dyn Error>> {
    let archive = run_json(&["archive", "ready", "--root", "fixtures/archive-ready"])?;
    let schema = receipt_schema("archive")?;
    assert_schema_keys(&archive, &schema, "top_level_keys")?;
    assert_first_action_matches_schema(&archive, &schema)
}

fn assert_migration_receipt_schema() -> Result<(), Box<dyn Error>> {
    let migration = run_json(&["migrate", "openspec", "--root", "fixtures/basic"])?;
    let schema = receipt_schema("migrate")?;
    assert_schema_keys(&migration, &schema, "top_level_keys")?;
    assert_first_action_matches_schema(&migration, &schema)
}

fn assert_agent_receipt_validation_schema() -> Result<(), Box<dyn Error>> {
    let receipt = run_json(&[
        "agent-receipt",
        "validate",
        "fixtures/agent-receipts/positive.json",
    ])?;
    let schema = receipt_schema("agent-receipt validate")?;
    assert_schema_keys(&receipt, &schema, "top_level_keys")?;
    assert_schema_keys(&receipt["replay_evidence"], &schema, "replay_evidence_keys")?;
    assert_hash_field(&receipt, "receipt_hash")
}

fn assert_replay_receipt_schema() -> Result<(), Box<dyn Error>> {
    let replay = run_json(&["determinism", "replay", "--root", "."])?;
    let schema = receipt_schema("determinism replay")?;
    assert_schema_keys(&replay, &schema, "top_level_keys")?;
    assert_schema_keys(&replay["cases"][0], &schema, "case_keys")?;
    assert_hash_field(&replay, "receipt_hash")
}

fn assert_replay_verify_receipt_schema() -> Result<(), Box<dyn Error>> {
    let artifact = temp_project("verify-replay-schema")?.join("receipt.json");
    let mut replay = cairn();
    replay.args([
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
    assert!(replay.output()?.status.success());
    let verify = run_json_strings(&[
        "determinism".into(),
        "verify-receipt".into(),
        artifact.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    let schema = receipt_schema("determinism verify-receipt")?;
    assert_schema_keys(&verify, &schema, "top_level_keys")?;
    assert_hash_field(&verify, "receipt_hash")
}

fn assert_agent_evidence_bundle_schema() -> Result<(), Box<dyn Error>> {
    let dir = temp_project("bundle-schema")?;
    let replay_path = dir.join("replay.json");
    let verify_path = dir.join("verify.json");
    let mut replay = cairn();
    replay.args([
        "determinism",
        "replay",
        "--root",
        ".",
        "--group",
        "agent",
        "--format",
        "summary",
        "--out",
        replay_path.to_str().unwrap(),
    ]);
    assert!(replay.output()?.status.success());
    let verify = run_json_strings(&[
        "determinism".into(),
        "verify-receipt".into(),
        replay_path.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    fs::write(&verify_path, serde_json::to_string_pretty(&verify)? + "\n")?;
    let agent_path = dir.join("agent-receipt.json");
    let mut agent_receipt: Value = serde_json::from_str(&fs::read_to_string(
        repo_root().join("fixtures/agent-receipts/positive.json"),
    )?)?;
    agent_receipt["replay_evidence"]["replay_receipt_hash"] = verify["replay_receipt_hash"].clone();
    agent_receipt["replay_evidence"]["replay_policy_hash"] = verify["replay_policy_hash"].clone();
    agent_receipt["replay_evidence"]["replay_environment_attestation_hash"] =
        verify["environment_attestation_hash"].clone();
    agent_receipt
        .as_object_mut()
        .unwrap()
        .remove("receipt_hash");
    fs::write(
        &agent_path,
        serde_json::to_string_pretty(&agent_receipt)? + "\n",
    )?;
    let initial_agent_validation = run_json_strings(&[
        "agent-receipt".into(),
        "validate".into(),
        agent_path.display().to_string(),
    ])?;
    agent_receipt["receipt_hash"] = initial_agent_validation["agent_receipt_hash"].clone();
    fs::write(
        &agent_path,
        serde_json::to_string_pretty(&agent_receipt)? + "\n",
    )?;
    let bundle_path = dir.join("bundle.json");
    fs::write(
        &bundle_path,
        serde_json::to_string_pretty(&json!({
            "agent_receipt_path": agent_path,
            "replay_receipt_path": replay_path,
            "replay_verification_path": verify_path,
        }))? + "\n",
    )?;
    let receipt = run_json_strings(&[
        "agent-receipt".into(),
        "verify-bundle".into(),
        bundle_path.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    let schema = receipt_schema("agent-receipt verify-bundle")?;
    assert_schema_keys(&receipt, &schema, "top_level_keys")?;
    assert_hash_field(&receipt, "receipt_hash")
}

#[test]
fn public_receipt_schemas_are_stable() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.receipt_schema]
    // r[verify cairn.supersede.receipt_schema.gate]
    // r[verify cairn.supersede.receipt_schema.lifecycle]
    // r[verify cairn.supersede.receipt_schema.migration]
    // r[verify cairn.supersede.agent_receipt_replay_evidence.schema]
    // r[verify cairn.supersede.agent_receipt_replay_group_coverage.schema]
    // r[verify cairn.supersede.determinism_replay.receipt.schema]
    // r[verify cairn.supersede.determinism_replay.groups.receipt]
    // r[verify cairn.supersede.determinism_entropy_inventory.schema]
    // r[verify cairn.supersede.replay_environment_attestation.policy]
    // r[verify cairn.supersede.replay_environment_attestation.receipt]
    // r[verify cairn.supersede.replay_environment_attestation.agent]
    // r[verify cairn.supersede.replay_receipt_verification.schema]
    // r[verify cairn.supersede.agent_evidence_bundle_verification.schema]
    assert_gate_receipt_schema()?;
    assert_sync_receipt_schema()?;
    assert_archive_receipt_schema()?;
    assert_migration_receipt_schema()?;
    assert_agent_receipt_validation_schema()?;
    assert_replay_receipt_schema()?;
    assert_replay_verify_receipt_schema()?;
    assert_agent_evidence_bundle_schema()
}

#[test]
fn public_receipt_schema_docs_list_stable_fields() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.receipt_schema.docs]
    // r[verify cairn.supersede.agent_receipt_replay_evidence.docs]
    // r[verify cairn.supersede.agent_receipt_replay_group_coverage.docs]
    // r[verify cairn.supersede.determinism_replay.groups.docs]
    // r[verify cairn.supersede.determinism_entropy_inventory.docs]
    // r[verify cairn.supersede.replay_environment_attestation.docs]
    // r[verify cairn.supersede.replay_receipt_verification.docs]
    // r[verify cairn.supersede.agent_evidence_bundle_verification.docs]
    let readme = fs::read_to_string(repo_root().join("README.md"))?;
    assert!(readme.contains("Stable public gate fields"), "{readme}");
    assert!(
        readme.contains("`change`, `stage`, `verdict`, `valid`"),
        "{readme}"
    );
    assert!(
        readme.contains("Stable public lifecycle\nreceipt fields"),
        "{readme}"
    );
    assert!(
        readme.contains("sync receipts also include `delta_specs`"),
        "{readme}"
    );
    assert!(
        readme.contains("Stable public migration receipt\nfields"),
        "{readme}"
    );
    assert!(
        readme.contains("`source_hash`, and `destination_hash` fields"),
        "{readme}"
    );
    assert!(
        readme.contains("Stable public determinism replay fields"),
        "{readme}"
    );
    assert!(
        readme
            .contains("Mismatch entries expose stable `case`, `comparison`, `field_path`, `class`"),
        "{readme}"
    );
    assert!(
        readme.contains("Replay groups are policy-owned named suites"),
        "{readme}"
    );
    assert!(
        readme.contains("`available_groups`, `selected_groups`"),
        "{readme}"
    );
    assert!(
        readme.contains("`entropy_inventory`, `entropy_assumptions`, `entropy_policy_hash`"),
        "{readme}"
    );
    assert!(readme.contains("`environment_attestation`"), "{readme}");
    assert!(readme.contains("`attestation_hash`"), "{readme}");
    assert!(
        readme.contains("Stable public replay verification fields"),
        "{readme}"
    );
    assert!(
        readme.contains("`cairn determinism verify-receipt <receipt.json> --root .`"),
        "{readme}"
    );
    assert!(
        readme.contains("raw environment values, usernames"),
        "{readme}"
    );
    assert!(
        readme.contains("Stable public agent evidence bundle verification fields"),
        "{readme}"
    );
    assert!(
        readme.contains("`cairn agent-receipt verify-bundle <bundle.json> --root .`"),
        "{readme}"
    );
    assert!(readme.contains("Rejected\nentropy failures disclose source names and handling but never environment values"), "{readme}");
    assert!(
        readme.contains("Stable public agent receipt validation fields"),
        "{readme}"
    );
    assert!(
        readme.contains("`current_replay_policy_hash`, `required_replay_group`, `required_replay_cases`, `replay_evidence`"),
        "{readme}"
    );
    assert!(readme.contains("`replay_selected_groups`"), "{readme}");
    assert!(
        readme.contains("`cairn determinism replay --group agent`"),
        "{readme}"
    );
    Ok(())
}

#[test]
fn read_outputs_match_canonical_goldens() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.read_parity] r[verify cairn.supersede.read_parity.change_list]
    // r[verify cairn.supersede.read_parity.change_show] r[verify cairn.supersede.read_parity.spec_list]
    // r[verify cairn.supersede.read_parity.spec_show] r[verify cairn.supersede.boundary]
    // r[verify cairn.supersede.boundary.read_models]
    assert_matches_golden(
        &["change", "list", "--root", "fixtures/basic"],
        "change-list-basic.json",
    )?;
    assert_matches_golden(
        &["change", "show", "demo", "--root", "fixtures/basic"],
        "change-show-basic-demo.json",
    )?;
    assert_matches_golden(
        &["spec", "list", "--root", "fixtures/basic"],
        "spec-list-basic.json",
    )?;
    assert_matches_golden(
        &["spec", "show", "demo", "--root", "fixtures/basic"],
        "spec-show-basic-demo.json",
    )?;
    Ok(())
}

#[test]
fn legacy_openspec_is_not_runtime_fallback() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.native_layout.native_only]
    let fixture = temp_fixture("legacy-no-fallback", "fixtures/basic")?;
    fs::remove_dir_all(fixture.join("cairn"))?;
    let root = fixture.to_str().ok_or("utf8 fixture path")?;

    let status = run_json(&["status", "--root", root])?;
    assert_eq!(status["layout"], "cairn");
    assert!(status["specs"].as_array().ok_or("specs")?.is_empty());
    assert!(status["changes"].as_array().ok_or("changes")?.is_empty());

    let doctor = run_json(&["doctor", "--root", root])?;
    assert_eq!(doctor["layout"], "cairn");
    assert_eq!(doctor["valid"], false);
    fs::remove_dir_all(fixture)?;
    Ok(())
}

#[test]
fn migrate_execute_copies_legacy_to_native_and_prefers_native() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.migrate_openspec] r[verify cairn.supersede.migrate_openspec.execute]
    // r[verify cairn.supersede.migrate_openspec.dry_run] r[verify cairn.supersede.native_layout.prefer]
    // r[verify cairn.supersede.native_layout.prefer_cairn]
    // r[verify cairn.supersede.native_layout.native_only] r[verify cairn.supersede.migrate_openspec.boundary]
    let fixture = temp_fixture("migrate-execute", "fixtures/basic")?;
    fs::remove_dir_all(fixture.join("cairn"))?;
    let root = fixture.to_str().ok_or("utf8 fixture path")?;

    let value = run_json(&["migrate", "openspec", "--root", root, "--execute"])?;
    assert_eq!(value["dry_run"], false);
    assert_eq!(value["blocked"], false);
    assert_eq!(value["mutated"], true);
    assert_hash_field(&value, "source_manifest_hash")?;
    assert_hash_field(&value, "destination_manifest_hash")?;
    assert_hash_field(&value, "manifest_hash")?;
    assert_hash_field(&value, "receipt_hash")?;
    assert_hash_field(&value["actions"][0], "source_hash")?;
    assert!(fixture.join("cairn/specs/demo/spec.md").is_file());
    assert!(fixture
        .join("cairn/changes/demo/specs/demo/spec.md")
        .is_file());

    let status = run_json(&["status", "--root", root])?;
    assert_eq!(status["layout"], "cairn");
    assert_eq!(
        status["changes"][0]["path"],
        format!("{root}/cairn/changes/demo")
    );
    fs::remove_dir_all(fixture)?;
    Ok(())
}

#[test]
fn migrate_execute_blocks_existing_native_destination() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.migrate_openspec.conflict]
    let fixture = temp_fixture("migrate-conflict", "fixtures/basic")?;
    fs::remove_dir_all(fixture.join("cairn"))?;
    fs::create_dir_all(fixture.join("cairn/specs/demo"))?;
    fs::write(fixture.join("cairn/specs/demo/spec.md"), "existing")?;
    let root = fixture.to_str().ok_or("utf8 fixture path")?;

    let value = run_json(&["migrate", "openspec", "--root", root, "--execute"])?;
    assert_eq!(value["dry_run"], false);
    assert_eq!(value["blocked"], true);
    assert_eq!(value["mutated"], false);
    assert!(value["actions"]
        .as_array()
        .ok_or("migration actions")?
        .iter()
        .any(|action| action["destination_hash"]
            .as_str()
            .unwrap_or_default()
            .len()
            == 64));
    assert_eq!(
        fs::read_to_string(fixture.join("cairn/specs/demo/spec.md"))?,
        "existing"
    );
    fs::remove_dir_all(fixture)?;
    Ok(())
}

#[test]
fn help_lists_commands() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.hermes_skill] r[verify cairn.supersede.hermes_skill.discover]
    // r[verify cairn.supersede.hermes_skill.contracts] r[verify cairn.supersede.hermes_skill.audit]
    // r[verify cairn.supersede.hermes_skill.canonical_cairn]
    // r[verify cairn.supersede.hermes_skill.canonical_cairn.path]
    // r[verify cairn.supersede.hermes_skill.canonical_cairn.live]
    // r[verify cairn.supersede.hermes_skill.redirect]
    // r[verify cairn.supersede.hermes_skill.redirect.local]
    let output = cairn().arg("--help").output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("usage: cairn <command> [args]"), "{stdout}");
    assert!(stdout.contains("init [--root PATH]"), "{stdout}");
    assert!(
        stdout.contains("change create <name> [--root PATH]"),
        "{stdout}"
    );
    assert!(stdout.contains("change list [--root PATH]"), "{stdout}");
    assert!(
        stdout.contains("change show <name> [--root PATH]"),
        "{stdout}"
    );
    assert!(stdout.contains("spec list [--root PATH]"), "{stdout}");
    assert!(stdout.contains("spec show <id> [--root PATH]"), "{stdout}");
    assert!(
        stdout.contains("migrate <openspec|ironspec> [--root PATH] [--policy PATH] [--execute]"),
        "{stdout}"
    );
    assert!(
        stdout.contains("gate <proposal|design|tasks> <change> [--root PATH] [--policy PATH]"),
        "{stdout}"
    );
    assert!(
        stdout.contains("sync <change> [--root PATH] [--policy PATH] [--execute]"),
        "{stdout}"
    );
    assert!(
        stdout.contains("archive <change> [--root PATH] [--policy PATH] [--execute]"),
        "{stdout}"
    );
    Ok(())
}

#[test]
fn unknown_command_returns_usage_error() -> Result<(), Box<dyn Error>> {
    let output = cairn().arg("bogus").output()?;
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unknown command: bogus"), "{stderr}");
    assert!(stderr.contains("usage: cairn <command> [args]"), "{stderr}");
    Ok(())
}

#[test]
fn init_creates_native_layout_and_is_idempotent() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.init] r[verify cairn.supersede.init.empty] r[verify cairn.supersede.init.idempotent]
    // r[verify cairn.supersede.native_layout] r[verify cairn.supersede.native_layout.init]
    // r[verify cairn.supersede.native_layout.init_cairn]
    let root = temp_project("init")?;
    let root_text = root.to_str().ok_or("utf8 root")?;

    let first = run_json(&["init", "--root", root_text])?;
    assert_eq!(first["command"], "init");
    assert_eq!(first["mutated"], true);
    assert_eq!(first["layout"], "cairn");
    assert!(root.join("cairn/specs").is_dir());
    assert!(root.join("cairn/archive").is_dir());
    assert_eq!(first["created"].as_array().ok_or("created")?.len(), 4);

    let second = run_json(&["init", "--root", root_text])?;
    assert_eq!(second["mutated"], false);
    assert_eq!(second["created"].as_array().ok_or("created")?.len(), 0);
    assert_eq!(second["existing"].as_array().ok_or("existing")?.len(), 4);
    Ok(())
}

#[test]
fn change_create_creates_compatible_skeleton() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.change_create] r[verify cairn.supersede.change_create.skeleton]
    let root = temp_project("change-create")?;
    let root_text = root.to_str().ok_or("utf8 root")?;
    run_json(&["init", "--root", root_text])?;

    let receipt = run_json(&["change", "create", "replace-openspec", "--root", root_text])?;
    assert_eq!(receipt["command"], "change_create");
    assert_eq!(receipt["change"], "replace-openspec");
    assert_eq!(receipt["mutated"], true);
    assert_eq!(receipt["layout"], "cairn");
    assert!(root
        .join("cairn/changes/replace-openspec/proposal.md")
        .is_file());
    assert!(root
        .join("cairn/changes/replace-openspec/design.md")
        .is_file());
    assert!(root
        .join("cairn/changes/replace-openspec/tasks.md")
        .is_file());
    assert!(root.join("cairn/changes/replace-openspec/specs").is_dir());

    let status = run_json(&["status", "--root", root_text])?;
    assert_eq!(status["changes"][0]["name"], "replace-openspec");
    assert_eq!(status["changes"][0]["has_proposal"], true);
    assert_eq!(status["changes"][0]["task_counts"]["total"], 2);
    Ok(())
}

#[test]
fn change_create_rejects_invalid_name_without_mutation() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.change_create.invalid_name]
    let root = temp_project("change-invalid")?;
    let output = cairn()
        .args([
            "change",
            "create",
            "bad/name",
            "--root",
            root.to_str().ok_or("utf8 root")?,
        ])
        .output()?;
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid change name"), "{stderr}");
    assert!(!root.join("cairn/changes").exists());
    Ok(())
}

#[test]
fn status_reports_fixture_change_counts() -> Result<(), Box<dyn Error>> {
    let value = run_json(&["status", "--root", "fixtures/basic"])?;
    assert_eq!(value["root"], "fixtures/basic");
    assert_eq!(value["specs"].as_array().ok_or("specs array")?.len(), 1);
    let changes = value["changes"].as_array().ok_or("changes array")?;
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0]["name"], "demo");
    assert_eq!(changes[0]["has_proposal"], true);
    assert_eq!(changes[0]["has_design"], true);
    assert_eq!(changes[0]["has_tasks"], true);
    assert_eq!(changes[0]["task_counts"]["total"], 2);
    assert_eq!(changes[0]["task_counts"]["unmarked"], 0);
    Ok(())
}

#[test]
fn validate_accepts_fixture_repo() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.validate_ears] r[verify cairn.supersede.validate_ears.accept]
    // r[verify cairn.supersede.validate_ears.boundary]
    let value = run_json(&["validate", "--root", "fixtures/basic"])?;
    assert_eq!(value["valid"], true);
    assert_eq!(value["changes"], 1);
    assert_eq!(value["policy"], "cairn-default");
    assert!(value["issues"].as_array().ok_or("issues array")?.is_empty());
    assert!(value["spec_issues"]
        .as_array()
        .ok_or("spec issues array")?
        .is_empty());
    assert_eq!(value["specs_validated"], 2);
    Ok(())
}

#[test]
fn validate_rejects_weak_ears_style_specs() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.validate_ears.weak_requirement]
    // r[verify cairn.supersede.validate_ears.incomplete_scenario]
    let root = temp_project("weak-ears")?;
    let spec_dir = root.join("cairn/specs/weak");
    fs::create_dir_all(&spec_dir)?;
    fs::write(
        spec_dir.join("spec.md"),
        "# Weak Spec\n\n## Requirements\n\n### Requirement: Weak prose\nThis feature works.\n\n#### Scenario: Missing when\n- GIVEN a precondition\n- THEN an outcome occurs\n",
    )?;
    let root_text = root.to_str().ok_or("utf8 temp root")?;
    let value = run_json(&["validate", "--root", root_text])?;
    assert_eq!(value["valid"], false);
    let issues = value["spec_issues"].as_array().ok_or("spec issues")?;
    assert_eq!(issues.len(), 2);
    assert!(issues[0]
        .as_str()
        .ok_or("issue text")?
        .contains("Requirement: Weak prose missing requirement strength"));
    assert!(issues[1]
        .as_str()
        .ok_or("issue text")?
        .contains("Scenario: Missing when missing WHEN clause"));
    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn gate_context_reports_named_change() -> Result<(), Box<dyn Error>> {
    let value = run_json(&["gate-context", "demo", "--root", "fixtures/basic"])?;
    assert_eq!(value["name"], "demo");
    assert_eq!(value["path"], "fixtures/basic/cairn/changes/demo");
    assert_eq!(
        value["delta_specs"].as_array().ok_or("delta specs")?.len(),
        1
    );
    Ok(())
}

#[test]
fn gate_reports_stage_verdicts() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.gate_policy]
    let pass = run_json(&["gate", "tasks", "demo", "--root", "fixtures/basic"])?;
    assert_eq!(pass["change"], "demo");
    assert_eq!(pass["stage"], "tasks");
    assert_eq!(pass["verdict"], "PASS");
    assert_eq!(pass["valid"], true);
    assert_eq!(pass["policy"], "cairn-default");

    let fail = run_json(&[
        "gate",
        "design",
        "missing",
        "--root",
        "fixtures/missing-artifacts",
    ])?;
    assert_eq!(fail["verdict"], "FAIL");
    let messages = fail["issues"]
        .as_array()
        .ok_or("issues")?
        .iter()
        .map(|issue| issue["message"].as_str().ok_or("message"))
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(
        messages,
        vec!["missing: missing proposal.md", "missing: missing design.md"]
    );
    assert_eq!(fail["issues"][0]["scope"], "tasks");
    assert_eq!(fail["issues"][0]["promoted"], "no");
    Ok(())
}

#[test]
fn gate_checks_references_task_links_and_exemptions() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.gate_policy.references]
    // r[verify cairn.supersede.gate_policy.task_links]
    // r[verify cairn.supersede.gate_policy.exemptions]
    let fixture = temp_fixture("gate-reference-checks", "fixtures/basic")?;
    let change_dir = fixture.join("cairn/changes/demo");
    fs::write(
        change_dir.join("proposal.md"),
        "## Why\n\nr[demo.unknown] gate-exempt[reason=test owner=agent]\n",
    )?;
    fs::write(
        change_dir.join("tasks.md"),
        "## Phase 1\n\n- [ ] [serial] unlinked task\n- [ ] [parallel] r[demo.behavior] linked task\n",
    )?;
    let root = fixture.to_str().ok_or("utf8 fixture path")?;

    let value = run_json(&["gate", "tasks", "demo", "--root", root])?;
    assert_eq!(value["verdict"], "FAIL");
    let messages = value["issues"]
        .as_array()
        .ok_or("issues")?
        .iter()
        .map(|issue| issue["message"].as_str().ok_or("message"))
        .collect::<Result<Vec<_>, _>>()?;
    assert!(messages
        .iter()
        .any(|message| message.contains("unknown requirement reference r[demo.unknown]")));
    assert!(messages
        .iter()
        .any(|message| message.contains("malformed gate exemption")));
    assert!(messages
        .iter()
        .any(|message| message.contains("task missing known requirement reference")));
    fs::remove_dir_all(fixture)?;
    Ok(())
}

#[test]
fn gate_accepts_bounded_exemptions_and_dependency_tasks() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.gate_policy.references]
    // r[verify cairn.supersede.gate_policy.task_links]
    // r[verify cairn.supersede.gate_policy.exemptions]
    let fixture = temp_fixture("gate-bounded-exemptions", "fixtures/basic")?;
    let change_dir = fixture.join("cairn/changes/demo");
    fs::write(
        change_dir.join("proposal.md"),
        "## Why\n\ngate-exempt[reason=test owner=agent until=2026-12-31]\n",
    )?;
    fs::write(
        change_dir.join("tasks.md"),
        "## Phase 1\n\n- [ ] [serial] r[demo.behavior] linked task\n- [ ] [depends:demo.behavior] dependency task\n",
    )?;
    let root = fixture.to_str().ok_or("utf8 fixture path")?;

    let value = run_json(&["gate", "tasks", "demo", "--root", root])?;
    assert_eq!(value["verdict"], "PASS");
    assert_eq!(value["valid"], true);
    fs::remove_dir_all(fixture)?;
    Ok(())
}

#[test]
fn sync_reports_dry_run_plan_for_delta_specs() -> Result<(), Box<dyn Error>> {
    // r[verify sync_archive.plan.sync] r[verify sync_archive.plan.sync.delta_specs]
    // r[verify sync_archive.plan.purity] r[verify sync_archive.plan.purity.no_effects]
    let value = run_json(&["sync", "demo", "--root", "fixtures/basic"])?;
    assert_eq!(value["change"], "demo");
    assert_eq!(value["dry_run"], true);
    assert_eq!(value["mutated"], false);
    assert_eq!(value["blocked"], false);
    assert_eq!(
        value["delta_specs"].as_array().ok_or("delta specs")?.len(),
        1
    );
    let actions = value["actions"].as_array().ok_or("actions array")?;
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0]["kind"], "sync_delta_spec");
    Ok(())
}

#[test]
fn sync_execute_blocks_invalid_delta_specs() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.deterministic_gates]
    let fixture = temp_fixture("sync-invalid-delta", "fixtures/basic")?;
    fs::write(
        fixture.join("cairn/changes/demo/specs/demo/spec.md"),
        "# Demo\n\n## Requirements\n\n### Requirement: Broken\n\nr[demo.add] This feature works.\n\n#### Scenario: Missing when\n\n- GIVEN a precondition\n- THEN an outcome occurs\n",
    )?;
    let root = fixture.to_str().ok_or("utf8 fixture path")?;
    let value = run_json(&["sync", "demo", "--root", root, "--execute"])?;
    assert_eq!(value["blocked"], true);
    assert_eq!(value["mutated"], false);
    assert!(value["actions"].as_array().ok_or("actions")?.is_empty());
    assert!(value["reasons"]
        .as_array()
        .ok_or("reasons")?
        .iter()
        .any(|reason| reason.as_str().unwrap_or_default().contains("validation:")));
    fs::remove_dir_all(fixture)?;
    Ok(())
}

#[test]
fn archive_reports_blocked_dry_run_plan_for_incomplete_tasks() -> Result<(), Box<dyn Error>> {
    // r[verify sync_archive.plan.archive.blockers] r[verify sync_archive.execute.archive.blocked_noop]
    let value = run_json(&["archive", "demo", "--root", "fixtures/basic"])?;
    assert_eq!(value["change"], "demo");
    assert_eq!(value["dry_run"], true);
    assert_eq!(value["mutated"], false);
    assert_eq!(value["blocked"], true);
    assert!(value["actions"]
        .as_array()
        .ok_or("actions array")?
        .is_empty());
    let reasons = value["reasons"].as_array().ok_or("reasons array")?;
    assert_eq!(reasons.len(), 1);
    assert!(reasons[0]
        .as_str()
        .ok_or("reason string")?
        .contains("tasks not archive-ready"));
    Ok(())
}

#[test]
fn archive_reports_ready_dry_run_plan_for_completed_change() -> Result<(), Box<dyn Error>> {
    // r[verify sync_archive.plan.archive]
    let value = run_json(&["archive", "ready", "--root", "fixtures/archive-ready"])?;
    assert_eq!(value["change"], "ready");
    assert_eq!(value["dry_run"], true);
    assert_eq!(value["mutated"], false);
    assert_eq!(value["blocked"], false);
    assert!(value["reasons"]
        .as_array()
        .ok_or("reasons array")?
        .is_empty());
    let actions = value["actions"].as_array().ok_or("actions array")?;
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0]["kind"], "archive_change");
    assert_eq!(
        actions[0]["path"],
        "fixtures/archive-ready/cairn/changes/ready"
    );
    Ok(())
}

#[test]
fn sync_and_archive_report_no_delta_blocker() -> Result<(), Box<dyn Error>> {
    let sync = run_json(&["sync", "no-delta", "--root", "fixtures/no-delta"])?;
    assert_eq!(sync["change"], "no-delta");
    assert_eq!(sync["blocked"], true);
    assert!(sync["actions"].as_array().ok_or("sync actions")?.is_empty());
    assert_eq!(
        sync["reasons"],
        serde_json::json!(["no-delta: no delta specs to sync"])
    );

    let archive = run_json(&["archive", "no-delta", "--root", "fixtures/no-delta"])?;
    assert_eq!(archive["change"], "no-delta");
    assert_eq!(archive["blocked"], true);
    assert!(archive["actions"]
        .as_array()
        .ok_or("archive actions")?
        .is_empty());
    assert_eq!(
        archive["reasons"],
        serde_json::json!(["no-delta: no delta specs to sync"])
    );
    Ok(())
}

#[test]
fn sync_reports_missing_required_artifact_blockers() -> Result<(), Box<dyn Error>> {
    let value = run_json(&["sync", "missing", "--root", "fixtures/missing-artifacts"])?;
    assert_eq!(value["change"], "missing");
    assert_eq!(value["dry_run"], true);
    assert_eq!(value["mutated"], false);
    assert_eq!(value["blocked"], true);
    assert!(value["actions"]
        .as_array()
        .ok_or("actions array")?
        .is_empty());
    assert_eq!(
        value["reasons"],
        serde_json::json!(["missing: missing proposal.md", "missing: missing tasks.md"])
    );
    Ok(())
}

#[test]
fn sync_execute_mutates_temp_fixture() -> Result<(), Box<dyn Error>> {
    // r[verify sync_archive.execute.sync] r[verify sync_archive.execute.sync.applies]
    // r[verify sync_archive.execute.sync.default_dry_run]
    // r[verify sync_archive.execute.sync.merge_preserves_wrapper]
    let fixture = temp_fixture("sync-execute", "fixtures/archive-ready")?;
    let root = fixture.to_str().ok_or("utf8 fixture path")?;
    let value = run_json(&["sync", "ready", "--root", root, "--execute"])?;
    assert_eq!(value["change"], "ready");
    assert_eq!(value["dry_run"], false);
    assert_eq!(value["mutated"], true);
    assert_eq!(value["blocked"], false);
    let accepted = fs::read_to_string(fixture.join("cairn/specs/demo/spec.md"))?;
    assert!(accepted.contains("# Demo Specification"), "{accepted}");
    assert!(accepted.contains("## Purpose"), "{accepted}");
    assert!(accepted.contains("Fixture accepted spec"), "{accepted}");
    assert!(accepted.contains("## Requirements"), "{accepted}");
    assert!(accepted.contains("r[demo.existing]"), "{accepted}");
    assert!(accepted.contains("r[demo.ready]"), "{accepted}");
    assert!(!accepted.contains("## ADDED Requirements"), "{accepted}");
    fs::remove_dir_all(fixture)?;
    Ok(())
}

#[test]
fn sync_execute_creates_wrapped_spec_for_new_domain() -> Result<(), Box<dyn Error>> {
    // r[verify sync_archive.execute.sync.merge_creates_spec]
    let fixture = temp_fixture("sync-new-domain", "fixtures/archive-ready")?;
    let delta_dir = fixture.join("cairn/changes/ready/specs/new-domain");
    fs::create_dir_all(&delta_dir)?;
    fs::write(
        delta_dir.join("spec.md"),
        "## ADDED Requirements\n\n### Requirement: New domain behavior\n\nr[new.domain.behavior] The system MUST create a wrapped accepted spec.\n\n#### Scenario: New domain\n\n- GIVEN a delta for a new domain\n- WHEN sync executes\n- THEN the accepted spec is wrapped\n",
    )?;
    let root = fixture.to_str().ok_or("utf8 fixture path")?;
    let value = run_json(&["sync", "ready", "--root", root, "--execute"])?;
    assert_eq!(value["mutated"], true);
    let accepted = fs::read_to_string(fixture.join("cairn/specs/new-domain/spec.md"))?;
    assert!(
        accepted.starts_with("# New Domain Specification"),
        "{accepted}"
    );
    assert!(accepted.contains("## Purpose"), "{accepted}");
    assert!(accepted.contains("## Requirements"), "{accepted}");
    assert!(accepted.contains("r[new.domain.behavior]"), "{accepted}");
    assert!(!accepted.contains("## ADDED Requirements"), "{accepted}");
    fs::remove_dir_all(fixture)?;
    Ok(())
}

#[test]
fn archive_execute_moves_temp_fixture_change() -> Result<(), Box<dyn Error>> {
    // r[verify sync_archive.execute.archive] r[verify sync_archive.execute.archive.moves]
    // r[verify sync_archive.execute.boundary] r[verify sync_archive.execute.boundary.pure_core]
    let fixture = temp_fixture("archive-execute", "fixtures/archive-ready")?;
    let root = fixture.to_str().ok_or("utf8 fixture path")?;
    let mut command = cairn();
    command
        .env_remove("CAIRN_ARCHIVE_DATE")
        .env("IRONSPEC_ARCHIVE_DATE", "2026-05-17")
        .args(["archive", "ready", "--root", root, "--execute"]);
    let value = run_json_command(&mut command)?;
    assert_eq!(value["change"], "ready");
    assert_eq!(value["dry_run"], false);
    assert_eq!(value["mutated"], true);
    assert_eq!(value["blocked"], false);
    assert!(!fixture.join("cairn/changes/ready").exists());
    assert!(fixture.join("cairn/archive/2026-05-17-ready").exists());
    fs::remove_dir_all(fixture)?;
    Ok(())
}

#[test]
fn archive_rejects_malformed_archive_date_without_mutation() -> Result<(), Box<dyn Error>> {
    // r[verify sync_archive.execute.archive]
    // r[verify cairn.supersede.agent_run_receipts.nondeterminism.environment]
    // r[verify cairn.supersede.determinism_replay.hidden_entropy]
    let fixture = temp_fixture("archive-bad-date", "fixtures/archive-ready")?;
    let root = fixture.to_str().ok_or("utf8 fixture path")?;
    let output = cairn()
        .env("CAIRN_ARCHIVE_DATE", "today")
        .args(["archive", "ready", "--root", root, "--execute"])
        .output()?;
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid archive date: expected YYYY-MM-DD"),
        "{stderr}"
    );
    assert!(fixture.join("cairn/changes/ready").exists());
    Ok(())
}

#[test]
fn execute_blocked_plan_does_not_mutate_temp_fixture() -> Result<(), Box<dyn Error>> {
    let fixture = temp_fixture("blocked-execute", "fixtures/missing-artifacts")?;
    let root = fixture.to_str().ok_or("utf8 fixture path")?;
    let value = run_json(&["sync", "missing", "--root", root, "--execute"])?;
    assert_eq!(value["dry_run"], false);
    assert_eq!(value["mutated"], false);
    assert_eq!(value["blocked"], true);
    assert!(fixture.join("cairn/changes/missing").exists());
    fs::remove_dir_all(fixture)?;
    Ok(())
}

#[test]
fn missing_change_returns_error() -> Result<(), Box<dyn Error>> {
    let output = cairn()
        .args(["sync", "missing", "--root", "fixtures/basic"])
        .output()?;
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("change not found: missing"), "{stderr}");
    Ok(())
}

#[test]
fn legacy_ironspec_alias_warns_and_runs_cairn_behavior() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.rename_cairn.legacy_alias]
    // r[verify cairn.supersede.rename_cairn.cli]
    let mut command = ironspec_alias();
    command.args(["status", "--root", "fixtures/basic"]);
    let output = command.output()?;
    assert!(output.status.success());
    let stdout: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(stdout["layout"], "cairn");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("compatibility alias"), "{stderr}");
    assert!(stderr.contains("cairn"), "{stderr}");
    Ok(())
}

#[test]
fn migrate_legacy_ironspec_to_cairn_explicitly() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.migrate_openspec.ironspec_dry_run]
    // r[verify cairn.supersede.migrate_openspec.ironspec_execute]
    // r[verify cairn.supersede.native_layout.no_implicit_legacy_after_rename]
    let root = temp_fixture("migrate-ironspec", "fixtures/basic")?;
    fs::rename(root.join("cairn"), root.join("ironspec"))?;
    let root_text = root.to_string_lossy().to_string();

    let status = run_json(&["status", "--root", &root_text])?;
    assert_eq!(status["layout"], "cairn");
    assert_eq!(status["specs"].as_array().ok_or("specs")?.len(), 0);
    assert_eq!(status["changes"].as_array().ok_or("changes")?.len(), 0);

    let dry_run = run_json(&["migrate", "ironspec", "--root", &root_text])?;
    assert_eq!(dry_run["command"], "migrate_ironspec");
    assert_eq!(dry_run["source_layout"], "ironspec");
    assert_eq!(dry_run["destination_layout"], "cairn");
    assert_eq!(dry_run["dry_run"], true);
    assert_eq!(dry_run["mutated"], false);
    assert!(!dry_run["actions"].as_array().ok_or("actions")?.is_empty());

    let executed = run_json(&["migrate", "ironspec", "--root", &root_text, "--execute"])?;
    assert_eq!(executed["dry_run"], false);
    assert_eq!(executed["mutated"], true);
    let migrated = run_json(&["status", "--root", &root_text])?;
    assert_eq!(migrated["layout"], "cairn");
    assert_eq!(migrated["specs"].as_array().ok_or("specs")?.len(), 1);
    Ok(())
}

#[test]
fn agent_receipt_validate_replays_fixtures_deterministically() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.agent_receipt_validation_surface]
    // r[verify cairn.supersede.agent_receipt_validation_surface.cli]
    // r[verify cairn.supersede.agent_receipt_validation_surface.cli.schema]
    // r[verify cairn.supersede.agent_receipt_validation_surface.docs]
    // r[verify cairn.supersede.agent_receipt_validation_surface.fixtures]
    // r[verify cairn.supersede.agent_receipt_replay_evidence.schema]
    // r[verify cairn.supersede.agent_receipt_replay_evidence.validation]
    // r[verify cairn.supersede.agent_receipt_replay_evidence.failures]
    // r[verify cairn.supersede.agent_receipt_replay_evidence.replay]
    // r[verify cairn.supersede.agent_receipt_replay_group_coverage.validation]
    // r[verify cairn.supersede.agent_receipt_replay_group_coverage.replay]
    let positive = assert_reproducible_receipt(
        &[
            "agent-receipt",
            "validate",
            "fixtures/agent-receipts/positive.json",
        ],
        &["agent_receipt_hash", "input_hash", "receipt_hash"],
    )?;
    assert_object_keys(
        &positive,
        &[
            "agent_receipt_hash",
            "command",
            "current_replay_policy_hash",
            "input_hash",
            "input_kind",
            "issues",
            "max_content_bytes",
            "path",
            "receipt_hash",
            "replay_evidence",
            "required_replay_cases",
            "required_replay_group",
            "stated_agent_receipt_hash",
            "valid",
            "verdict",
        ],
    )?;
    assert_eq!(positive["valid"], true);
    assert_eq!(positive["verdict"], "PASS");
    assert_eq!(positive["issues"].as_array().ok_or("issues")?.len(), 0);
    assert_hash_field(&positive, "current_replay_policy_hash")?;
    assert_hash_field(&positive["replay_evidence"], "replay_receipt_hash")?;
    assert_hash_field(&positive["replay_evidence"], "replay_policy_hash")?;
    assert_eq!(positive["required_replay_group"], "agent");
    assert_eq!(
        positive["required_replay_cases"],
        json!(policy_replay_group_cases("agent")?)
    );
    assert_eq!(
        positive["replay_evidence"]["replay_selected_groups"],
        json!(["agent"])
    );
    assert_eq!(positive["replay_evidence"]["replay_verdict"], "pass");

    let negative = assert_reproducible_receipt(
        &[
            "agent-receipt",
            "validate",
            "fixtures/agent-receipts/missing-verification.json",
        ],
        &["agent_receipt_hash", "input_hash", "receipt_hash"],
    )?;
    assert_eq!(negative["valid"], false);
    let issues = negative["issues"].as_array().ok_or("negative issues")?;
    assert!(issues
        .iter()
        .any(|issue| issue["code"] == "verification.missing_evidence"));
    Ok(())
}

#[test]
fn agent_receipt_replay_evidence_failures_are_deterministic() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.agent_receipt_replay_evidence.failures]
    // r[verify cairn.supersede.agent_receipt_replay_group_coverage.failures]
    // r[verify cairn.supersede.replay_environment_attestation.agent]
    let base_text = fs::read_to_string(repo_root().join("fixtures/agent-receipts/positive.json"))?;
    let base: Value = serde_json::from_str(&base_text)?;
    let cases = [
        (
            "missing",
            {
                let mut value = base.clone();
                value.as_object_mut().unwrap().remove("replay_evidence");
                value
            },
            "replay_evidence.missing",
        ),
        (
            "stale-policy",
            {
                let mut value = base.clone();
                value["replay_evidence"]["replay_policy_hash"] = Value::String(
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
                );
                value
            },
            "replay_evidence.policy_hash_stale",
        ),
        (
            "insufficient",
            {
                let mut value = base.clone();
                value["replay_evidence"]["replay_selected_cases"] = json!(["status"]);
                value
            },
            "replay_evidence.insufficient_coverage",
        ),
        (
            "missing-group",
            {
                let mut value = base.clone();
                value["replay_evidence"]["replay_selected_groups"] = json!(["smoke"]);
                value
            },
            "replay_evidence.missing_group_coverage",
        ),
        (
            "bad-attestation-hash",
            {
                let mut value = base.clone();
                value["replay_evidence"]["replay_environment_attestation_hash"] =
                    json!("not-a-blake3-hash");
                value
            },
            "replay_evidence.invalid_environment_attestation_hash",
        ),
    ];
    for (name, value, expected_code) in cases {
        let path = temp_project(&format!("agent-replay-{name}"))?.join("receipt.json");
        fs::write(&path, serde_json::to_string_pretty(&value)? + "\n")?;
        let first = run_json(&[
            "agent-receipt",
            "validate",
            path.to_str().ok_or("utf8 receipt path")?,
        ])?;
        let second = run_json(&[
            "agent-receipt",
            "validate",
            path.to_str().ok_or("utf8 receipt path")?,
        ])?;
        assert_eq!(first["receipt_hash"], second["receipt_hash"]);
        assert_eq!(first["valid"], false);
        assert!(
            first["issues"]
                .as_array()
                .ok_or("issues")?
                .iter()
                .any(|issue| issue["code"] == expected_code),
            "{first:#?}"
        );
    }
    Ok(())
}

#[test]
fn mcp_agent_smoke_receipt_proves_operator_flow() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.mcp_agent_smoke_receipts.flow]
    // r[verify cairn.supersede.mcp_agent_smoke_receipts.evidence]
    // r[verify cairn.supersede.mcp_agent_smoke_receipts.read_only]
    // r[verify cairn.supersede.mcp_agent_smoke_receipts.docs]
    // r[verify cairn.supersede.mcp_agent_smoke_receipts.verification]
    let receipt = run_json(&["agent-receipt", "smoke-mcp", "--root", "."])?;
    assert_eq!(receipt["command"], "agent-receipt smoke-mcp");
    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["verdict"], "pass");
    assert_eq!(receipt["fixture"], "fixtures/agent-receipts/positive.json");
    assert_eq!(receipt["mcp"]["read_only"], true);
    assert_eq!(receipt["mutation_denial"]["allowed"], false);
    assert_eq!(receipt["mutation_denial"]["mutated"], false);
    assert!(receipt["catalog_tool_count"].as_u64().unwrap_or_default() >= 9);
    assert!(receipt["invalid_argument_error"]
        .as_str()
        .unwrap_or_default()
        .contains("invalid arguments for cairn_gate"));
    assert_hash_field(&receipt, "receipt_hash")?;
    assert_hash_field(&receipt, "receipt_tool_schema_hash")?;
    assert_hash_field(&receipt, "agent_receipt_validation_hash")?;
    assert_hash_field(&receipt, "agent_receipt_hash")?;
    let checks = receipt["checks"].as_array().ok_or("checks")?;
    assert_eq!(checks.len(), 5);
    assert!(checks
        .iter()
        .all(|check| check["passed"].as_bool() == Some(true)));
    Ok(())
}

#[test]
fn release_readiness_bundle_emits_operator_evidence() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.release_readiness_bundle.schema]
    // r[verify cairn.supersede.release_readiness_bundle.local]
    // r[verify cairn.supersede.release_readiness_bundle.evidence]
    // r[verify cairn.supersede.release_readiness_bundle.docs]
    // r[verify cairn.supersede.release_readiness_bundle.verification]
    // r[verify cairn.supersede.determinism_entropy_inventory.verification]
    // r[verify cairn.supersede.determinism_entropy_inventory.archive]
    // r[verify cairn.supersede.replay_environment_attestation.verification]
    // r[verify cairn.supersede.replay_environment_attestation.archive]
    // r[verify cairn.supersede.replay_receipt_verification.verification]
    // r[verify cairn.supersede.replay_receipt_verification.archive]
    let receipt = run_json(&["release-readiness", "--root", "."])?;
    assert_object_keys(
        &receipt,
        &[
            "command",
            "root",
            "valid",
            "verdict",
            "checks",
            "cairn_validation_hash",
            "tracey_coverage_hash",
            "mcp_smoke_receipt_hash",
            "coverage_audit_valid",
            "coverage_audit_receipt_hash",
            "repository_gate_summary",
            "determinism_replay",
            "mcp",
            "receipt_hash",
        ],
    )?;
    assert_eq!(receipt["command"], "release-readiness");
    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["verdict"], "pass");
    assert_hash_field(&receipt, "receipt_hash")?;
    assert_hash_field(&receipt, "cairn_validation_hash")?;
    assert_hash_field(&receipt, "tracey_coverage_hash")?;
    assert_hash_field(&receipt, "mcp_smoke_receipt_hash")?;
    assert_hash_field(&receipt, "coverage_audit_receipt_hash")?;
    assert_eq!(receipt["coverage_audit_valid"], true);
    let check_names = receipt["checks"]
        .as_array()
        .ok_or("checks array")?
        .iter()
        .map(|check| check["name"].as_str().unwrap_or_default())
        .collect::<Vec<_>>();
    assert!(check_names.contains(&"cairn_validate"));
    assert!(check_names.contains(&"tracey_coverage"));
    assert!(check_names.contains(&"mcp_agent_smoke"));
    assert!(check_names.contains(&"determinism_coverage_audit"));
    assert!(check_names.contains(&"repository_gate_policy_listed"));
    assert_eq!(receipt["mcp"]["read_only"], true);
    assert_eq!(receipt["mcp"]["destructive"], false);
    Ok(())
}

#[test]
fn release_readiness_bundle_is_reproducible() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.release_readiness_bundle.schema]
    // r[verify cairn.supersede.release_readiness_bundle.evidence]
    // r[verify cairn.supersede.release_readiness_bundle.verification]
    let receipt = assert_reproducible_receipt(
        &["release-readiness", "--root", "."],
        &[
            "receipt_hash",
            "cairn_validation_hash",
            "tracey_coverage_hash",
            "mcp_smoke_receipt_hash",
            "coverage_audit_receipt_hash",
        ],
    )?;
    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["verdict"], "pass");
    Ok(())
}

#[test]
fn release_readiness_bundle_reports_missing_evidence() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.release_readiness_bundle.failure]
    let fixture = temp_fixture("release-readiness-missing-evidence", "fixtures/basic")?;
    let root = fixture.to_str().ok_or("utf8 fixture path")?;
    let receipt = run_json(&["release-readiness", "--root", root])?;
    assert_eq!(receipt["valid"], false);
    assert_eq!(receipt["verdict"], "fail");
    assert_hash_field(&receipt, "receipt_hash")?;
    let failed = receipt["checks"]
        .as_array()
        .ok_or("checks array")?
        .iter()
        .filter(|check| check["passed"] == false)
        .collect::<Vec<_>>();
    let has_tracey_failure = failed
        .iter()
        .any(|check| check["name"] == "tracey_coverage");
    let has_mcp_smoke_failure = failed
        .iter()
        .any(|check| check["name"] == "mcp_agent_smoke");
    let is_missing_evidence_failure = has_tracey_failure || has_mcp_smoke_failure;
    assert!(
        is_missing_evidence_failure,
        "expected missing readiness evidence failure: {receipt}"
    );
    fs::remove_dir_all(fixture)?;
    Ok(())
}

#[test]
fn mcp_agent_receipt_validation_is_read_only_and_bounded() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.agent_receipt_validation_surface.mcp]
    // r[verify cairn.supersede.agent_receipt_validation_surface.mcp.read_only]
    // r[verify cairn.supersede.agent_receipt_validation_surface.verification]
    let catalog = run_json(&["mcp", "--describe"])?;
    let tools = catalog["tools"].as_array().ok_or("tools")?;
    assert!(tools
        .iter()
        .any(|tool| tool["name"] == "cairn_agent_receipt_validate"
            && tool["annotations"]["readOnlyHint"] == true
            && tool["annotations"]["destructiveHint"] == false));

    let by_path = run_json(&[
        "mcp",
        "--call",
        "cairn_agent_receipt_validate",
        r#"{"root":".","path":"fixtures/agent-receipts/positive.json"}"#,
    ])?;
    let receipt = &by_path["structuredContent"];
    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["mcp"]["read_only"], true);
    assert_hash_field(receipt, "agent_receipt_hash")?;
    assert_hash_field(receipt, "receipt_hash")?;

    let inline_fixture =
        fs::read_to_string(repo_root().join("fixtures/agent-receipts/positive.json"))?;
    let inline_request = format!(r#"{{"receipt":{inline_fixture}}}"#);
    let inline = run_json(&[
        "mcp",
        "--call",
        "cairn_agent_receipt_validate",
        &inline_request,
    ])?;
    assert_eq!(inline["structuredContent"]["input_kind"], "inline");
    assert_eq!(inline["structuredContent"]["valid"], true);

    let traversal = cairn()
        .args([
            "mcp",
            "--call",
            "cairn_agent_receipt_validate",
            r#"{"root":".","path":"../secret.json"}"#,
        ])
        .output()?;
    assert!(!traversal.status.success());
    let stderr = String::from_utf8_lossy(&traversal.stderr);
    assert!(
        stderr.contains("selector escapes lifecycle bounds"),
        "{stderr}"
    );
    Ok(())
}

#[test]
fn mcp_describes_read_only_tools_without_mutations() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.mcp_server.verification.registration]
    // r[verify cairn.supersede.mcp_server.read_tools]
    // r[verify cairn.supersede.mcp_reproducibility] r[verify cairn.supersede.mcp_reproducibility.describe]
    let value = run_json(&["mcp", "--describe"])?;
    assert_eq!(value, run_json(&["mcp", "--describe"])?);
    assert_eq!(value["allow_mutations"], false);
    let tools = value["tools"].as_array().ok_or("tools")?;
    let status_tool = tools
        .iter()
        .find(|tool| tool["name"] == "cairn_status")
        .ok_or("status tool")?;
    assert_eq!(status_tool["annotations"]["readOnlyHint"], true);
    assert_eq!(status_tool["annotations"]["destructiveHint"], false);
    assert_eq!(status_tool["inputSchema"]["additionalProperties"], false);
    assert!(status_tool["inputSchema"]["properties"]["root"].is_object());

    let sync_plan_tool = tools
        .iter()
        .find(|tool| tool["name"] == "cairn_sync_plan")
        .ok_or("sync plan tool")?;
    assert_eq!(sync_plan_tool["annotations"]["readOnlyHint"], true);
    assert_eq!(sync_plan_tool["inputSchema"]["required"][0], "change");
    assert_eq!(sync_plan_tool["inputSchema"]["additionalProperties"], false);
    assert!(!tools
        .iter()
        .any(|tool| tool["name"] == "cairn_sync_execute"));
    let receipt_tool = tools
        .iter()
        .find(|tool| tool["name"] == "cairn_agent_receipt_validate")
        .ok_or("receipt tool")?;
    assert!(receipt_tool["inputSchema"]["anyOf"].is_array());

    let mutation_value = run_json(&["mcp", "--allow-mutations", "--describe"])?;
    assert_eq!(mutation_value["allow_mutations"], true);
    let mutation_tool = mutation_value["tools"]
        .as_array()
        .ok_or("mutation tools")?
        .iter()
        .find(|tool| tool["name"] == "cairn_sync_execute")
        .ok_or("sync execute")?;
    assert_eq!(mutation_tool["annotations"]["destructiveHint"], true);
    assert_eq!(mutation_tool["inputSchema"]["required"][0], "change");
    Ok(())
}

#[test]
fn mcp_calls_return_structured_receipts() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.mcp_server.read_tools.status]
    // r[verify cairn.supersede.mcp_server.read_tools.gates]
    // r[verify cairn.supersede.mcp_server.verification]
    // r[verify cairn.supersede.mcp_reproducibility.call]
    // r[verify cairn.supersede.mcp_server.verification.docs]
    // r[verify cairn.supersede.mcp_server.strict_schemas.docs]
    let value = run_json(&[
        "mcp",
        "--call",
        "cairn_status",
        r#"{"root":"fixtures/basic"}"#,
    ])?;
    assert_eq!(
        value,
        run_json(&[
            "mcp",
            "--call",
            "cairn_status",
            r#"{"root":"fixtures/basic"}"#,
        ])?
    );
    let receipt = &value["structuredContent"];
    assert_eq!(receipt["layout"], "cairn");
    assert_eq!(receipt["mcp"]["read_only"], true);
    assert_hash_field(receipt, "receipt_hash")?;
    Ok(())
}

#[test]
fn mcp_rejects_invalid_arguments_before_dispatch() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.mcp_server.strict_schemas.validation]
    // r[verify cairn.supersede.mcp_server.strict_schemas.errors]
    // r[verify cairn.supersede.mcp_server.strict_schemas.verification]
    let missing = cairn()
        .args([
            "mcp",
            "--call",
            "cairn_read_spec",
            r#"{"root":"fixtures/basic"}"#,
        ])
        .output()?;
    assert!(!missing.status.success());
    let stderr = String::from_utf8_lossy(&missing.stderr);
    assert!(
        stderr.contains("invalid arguments for cairn_read_spec: missing required argument spec"),
        "{stderr}"
    );

    let unknown = cairn()
        .args([
            "mcp",
            "--call",
            "cairn_status",
            r#"{"root":"fixtures/basic","surprise":"no"}"#,
        ])
        .output()?;
    assert!(!unknown.status.success());
    let stderr = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        stderr.contains("invalid arguments for cairn_status: unknown argument surprise"),
        "{stderr}"
    );

    let bad_stage = cairn()
        .args([
            "mcp",
            "--call",
            "cairn_gate",
            r#"{"root":"fixtures/basic","change":"demo","stage":"archive"}"#,
        ])
        .output()?;
    assert!(!bad_stage.status.success());
    let stderr = String::from_utf8_lossy(&bad_stage.stderr);
    assert!(
        stderr.contains("invalid arguments for cairn_gate: argument stage must be one of proposal, design, tasks"),
        "{stderr}"
    );

    let bad_source = cairn()
        .args([
            "mcp",
            "--call",
            "cairn_migrate_plan",
            r#"{"root":"fixtures/basic","source":"cairn"}"#,
        ])
        .output()?;
    assert!(!bad_source.status.success());
    let stderr = String::from_utf8_lossy(&bad_source.stderr);
    assert!(
        stderr.contains("invalid arguments for cairn_migrate_plan: argument source must be one of openspec, ironspec"),
        "{stderr}"
    );
    Ok(())
}

#[test]
fn mcp_rejects_traversal_and_default_mutation() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.mcp_server.boundary.shell]
    // r[verify cairn.supersede.mcp_server.mutation_guard.default_deny]
    // r[verify cairn.supersede.mcp_server.verification.negative]
    let traversal = cairn()
        .args([
            "mcp",
            "--call",
            "cairn_read_spec",
            r#"{"spec":"../secret","root":"fixtures/basic"}"#,
        ])
        .output()?;
    assert!(!traversal.status.success());
    let stderr = String::from_utf8_lossy(&traversal.stderr);
    assert!(
        stderr.contains("selector escapes lifecycle bounds"),
        "{stderr}"
    );

    let denied = run_json(&[
        "mcp",
        "--call",
        "cairn_sync_execute",
        r#"{"change":"demo","root":"fixtures/basic"}"#,
    ])?;
    let receipt = &denied["structuredContent"];
    assert_eq!(receipt["allowed"], false);
    assert_eq!(receipt["mutated"], false);
    Ok(())
}

#[test]
fn mcp_json_rpc_lists_and_calls_tools() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.mcp_server.verification.registration]
    let mut child = cairn()
        .arg("mcp")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    {
        let stdin = child.stdin.as_mut().ok_or("stdin")?;
        writeln!(stdin, r#"{{"jsonrpc":"2.0","id":1,"method":"tools/list"}}"#)?;
        writeln!(
            stdin,
            r#"{{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{{"name":"cairn_validate","arguments":{{"root":"fixtures/basic"}}}}}}"#
        )?;
        writeln!(
            stdin,
            r#"{{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{{"name":"cairn_gate","arguments":{{"root":"fixtures/basic","change":"demo","stage":"archive"}}}}}}"#
        )?;
    }
    let output = child.wait_with_output()?;
    assert!(output.status.success());
    let lines = String::from_utf8(output.stdout)?;
    let responses = lines
        .lines()
        .map(serde_json::from_str::<Value>)
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(responses.len(), 3);
    assert!(responses[0]["result"]["tools"]
        .as_array()
        .ok_or("tools")?
        .iter()
        .any(|tool| tool["name"] == "cairn_validate"));
    assert_eq!(responses[1]["result"]["structuredContent"]["valid"], true);
    assert!(responses[2]["error"]["message"]
        .as_str()
        .unwrap_or_default()
        .contains("invalid arguments for cairn_gate"));
    Ok(())
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
