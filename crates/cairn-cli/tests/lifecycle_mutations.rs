use std::{error::Error, fs};

// r[impl cli_maintainability.lifecycle_tests.focused_modules.visible_boundaries]
// r[verify cli_maintainability.lifecycle_tests.behavior_preserved.focused_checks]
#[path = "lifecycle/support.rs"]
mod support;

use support::{
    assert_hash_field, assert_matches_golden, cairn, run_json, run_json_command, run_json_strings,
    temp_fixture,
};

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
