use std::{error::Error, fs};

use serde_json::Value;

// r[impl cli_maintainability.lifecycle_tests.focused_modules]
// r[impl cli_maintainability.lifecycle_tests.focused_modules.visible_boundaries]
// r[verify cli_maintainability.lifecycle_tests.behavior_preserved]
// r[verify cli_maintainability.lifecycle_tests.behavior_preserved.focused_checks]
// r[verify cli_maintainability.lifecycle_tests.behavior_preserved.repository_checks]
#[path = "lifecycle/support.rs"]
mod support;

use support::{
    assert_hash_field, assert_matches_golden, assert_reproducible_receipt, cairn, ironspec_alias,
    run_json, temp_fixture, temp_project,
};

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
fn change_metadata_is_validated_and_filterable() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.change_metadata.schema.valid_metadata]
    // r[verify cairn.supersede.change_metadata.schema.optional_metadata]
    // r[verify cairn.supersede.change_metadata.validation.invalid_metadata]
    // r[verify cairn.supersede.change_metadata.read_receipts.list_show]
    // r[verify cairn.supersede.change_metadata.filters.group]
    // r[verify cairn.supersede.change_metadata.filters.target_date]
    // r[verify cairn.supersede.change_metadata.verification]
    // r[verify cairn.supersede.change_metadata.verification.gates]
    let fixture = temp_fixture("change-metadata", "fixtures/basic")?;
    let root = fixture.to_str().ok_or("utf8 fixture path")?;
    fs::write(
        fixture.join("cairn/changes/demo/metadata.json"),
        "{\"groups\":[\"feature:registry\",\"current\"],\"target_date\":\"2026-06-01\",\"status\":\"planned\"}\n",
    )?;

    let filtered = run_json(&[
        "change",
        "list",
        "--root",
        root,
        "--group",
        "feature:registry",
        "--target-after",
        "2026-05-01",
        "--target-before",
        "2026-07-01",
    ])?;
    assert_eq!(filtered["changes"].as_array().ok_or("changes")?.len(), 1);
    assert_eq!(filtered["changes"][0]["metadata"]["groups"][0], "current");
    assert_eq!(
        filtered["changes"][0]["metadata"]["groups"][1],
        "feature:registry"
    );
    assert_eq!(
        filtered["changes"][0]["metadata"]["target_date"],
        "2026-06-01"
    );

    let excluded = run_json(&["change", "list", "--root", root, "--group", "future"])?;
    assert!(excluded["changes"].as_array().ok_or("changes")?.is_empty());

    fs::write(
        fixture.join("cairn/changes/demo/metadata.json"),
        "{\"groups\":[\"unknown\"],\"target_date\":\"not-a-date\",\"status\":\"unknown\"}\n",
    )?;
    let invalid = run_json(&["validate", "--root", root])?;
    assert_eq!(invalid["valid"], false);
    let issues = invalid["change_issues"].as_array().ok_or("change_issues")?;
    assert_eq!(issues.len(), 3);
    let issue_text = serde_json::to_string(issues)?;
    assert!(
        issue_text.contains("metadata group unknown"),
        "{issue_text}"
    );
    assert!(issue_text.contains("metadata target_date"), "{issue_text}");
    assert!(
        issue_text.contains("metadata status unknown"),
        "{issue_text}"
    );
    fs::remove_dir_all(fixture)?;
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
