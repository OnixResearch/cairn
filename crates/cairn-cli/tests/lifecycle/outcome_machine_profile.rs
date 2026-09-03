use std::{error::Error, fs, path::Path};

use serde_json::{json, Value};

use super::support::{run_json, temp_project};

const VALID_PROPOSAL: &str = "## Why\n\nA repeatable capability is required.\n\n## What Changes\n\n- Add one bounded capability.\n\n## Impact\n\n- Lifecycle artifacts and tests change.\n";
const VALID_DESIGN: &str = "# Design\n\n## Context\n\nThe change needs repeatable behavior.\n\n## Decisions\n\n### Decision: Keep inputs explicit\n\n**Choice:** Use tracked inputs and deterministic checks.\n\n**Rationale:** This removes hidden session state.\n";
const VALID_SPEC: &str = "# Outcome-machine fixture delta\n\n## ADDED Requirements\n\n### Requirement: Preserve repeatability\n\nr[test.outcome_machine] Cairn MUST preserve repeatable capability.\n\n#### Scenario: Repeatable capability passes\n\n- GIVEN tracked inputs and deterministic checks\n- WHEN lifecycle admission runs\n- THEN the change MUST retain repository authority.\n";
const VALID_TASKS: &str = "# Tasks\n\n- [x] [serial] Implement repeatable capability. r[test.outcome_machine]\n- [x] [serial] Verify rejection behavior. r[test.outcome_machine]\n";
const FRESH_EVIDENCE: &[u8] = b"fresh-state replay passed\n";
const COMPLETION_EVIDENCE: &[u8] = b"focused lifecycle tests passed\n";
const STALE_COMPLETION_EVIDENCE: &[u8] = b"stale completion evidence\n";

#[test]
fn outcome_machine_scaffold_passes_planning_gates_but_blocks_completion(
) -> Result<(), Box<dyn Error>> {
    // r[verify cairn.outcome_machine_profile.selection]
    // r[verify cairn.outcome_machine_profile.scaffold]
    // r[verify cairn.outcome_machine_profile.completion]
    let root = temp_project("outcome-machine-planned")?;
    let root_text = root.to_str().ok_or("utf8 root")?;
    run_json(&["init", "--root", root_text])?;
    let created = run_json(&[
        "change",
        "create",
        "machine-demo",
        "--profile",
        "outcome-machine",
        "--root",
        root_text,
    ])?;
    assert_eq!(created["workflow_profile"], "outcome-machine");
    let change = root.join(".cairn/changes/machine-demo");
    let scaffold: Value = serde_json::from_slice(&fs::read(change.join("outcome-machine.json"))?)?;
    assert_eq!(scaffold["schema"], "cairn.outcome-machine-review.v1");
    assert!(scaffold["route"]["details"]["immediate_outcome"]
        .as_str()
        .ok_or("immediate outcome")?
        .contains("CAIRN_PLACEHOLDER"));

    write_spec_artifacts(&change)?;
    write_review(&change, "planned", "planned", delivery_route())?;
    assert_eq!(
        run_json(&["gate", "proposal", "machine-demo", "--root", root_text])?["verdict"],
        "PASS"
    );
    assert_eq!(
        run_json(&["gate", "design", "machine-demo", "--root", root_text])?["verdict"],
        "PASS"
    );
    assert_eq!(
        run_json(&["gate", "tasks", "machine-demo", "--root", root_text])?["verdict"],
        "PASS"
    );

    let instructions = run_json(&[
        "workflow",
        "instructions",
        "outcome-machine",
        "--change",
        "machine-demo",
        "--root",
        root_text,
    ])?;
    assert_eq!(instructions["artifact"]["artifact_id"], "outcome-machine");
    assert_eq!(
        instructions["artifact"]["output_path"],
        ".cairn/changes/machine-demo/outcome-machine.json"
    );

    let status = run_json(&["workflow", "status", "machine-demo", "--root", root_text])?;
    assert_eq!(status["valid"], false);
    assert!(status["issues"]
        .to_string()
        .contains("cairn.workflow.outcome_machine.completion"));
    assert!(status["issues"]
        .to_string()
        .contains("cairn.workflow.outcome_machine.fresh_state"));

    let sync = run_json(&["sync", "machine-demo", "--root", root_text])?;
    assert_eq!(sync["blocked"], true);
    assert!(sync["reasons"]
        .to_string()
        .contains("observed completion evidence is missing"));

    let ordinary = run_json(&[
        "change",
        "create",
        "ordinary-demo",
        "--profile",
        "spec-driven",
        "--root",
        root_text,
    ])?;
    assert_eq!(ordinary["workflow_profile"], "spec-driven");
    assert!(!root
        .join(".cairn/changes/ordinary-demo/outcome-machine.json")
        .exists());
    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn observed_outcome_machine_evidence_allows_sync_and_archive() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.outcome_machine_profile.completion]
    // r[verify cairn.outcome_machine_profile.fresh_state]
    // r[verify cairn.outcome_machine_profile.verification]
    let root = temp_project("outcome-machine-observed")?;
    let root_text = root.to_str().ok_or("utf8 root")?;
    run_json(&["init", "--root", root_text])?;
    run_json(&[
        "change",
        "create",
        "machine-demo",
        "--profile",
        "outcome-machine",
        "--root",
        root_text,
    ])?;
    let change = root.join(".cairn/changes/machine-demo");
    write_spec_artifacts(&change)?;
    write_review(&change, "observed", "observed", delivery_route())?;

    let tasks_gate = run_json(&["gate", "tasks", "machine-demo", "--root", root_text])?;
    assert_eq!(tasks_gate["verdict"], "PASS");
    let current_status = run_json(&["workflow", "status", "machine-demo", "--root", root_text])?;

    fs::write(
        change.join("evidence/completion.json"),
        STALE_COMPLETION_EVIDENCE,
    )?;
    let stale_status = run_json(&["workflow", "status", "machine-demo", "--root", root_text])?;
    assert_ne!(current_status["input_hash"], stale_status["input_hash"]);
    let stale = run_json(&["sync", "machine-demo", "--root", root_text])?;
    assert_eq!(stale["blocked"], true);
    assert!(stale["reasons"]
        .to_string()
        .contains("stale evidence binding"));
    fs::write(change.join("evidence/completion.json"), COMPLETION_EVIDENCE)?;

    let sync = run_json(&["sync", "machine-demo", "--root", root_text])?;
    assert_eq!(sync["blocked"], false);
    let synced = run_json(&["sync", "machine-demo", "--root", root_text, "--execute"])?;
    assert_eq!(synced["blocked"], false);
    assert_eq!(synced["mutated"], true);

    let archive = run_json(&["archive", "machine-demo", "--root", root_text])?;
    assert_eq!(archive["blocked"], false);
    let archived = run_json(&["archive", "machine-demo", "--root", root_text, "--execute"])?;
    assert_eq!(archived["blocked"], false);
    assert_eq!(archived["mutated"], true);
    assert!(!change.exists());
    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn invalid_route_hidden_state_and_malformed_json_fail_design() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.outcome_machine_profile.routes]
    // r[verify cairn.outcome_machine_profile.boundary]
    // r[verify cairn.outcome_machine_profile.verification]
    let root = temp_project("outcome-machine-invalid")?;
    let root_text = root.to_str().ok_or("utf8 root")?;
    run_json(&["init", "--root", root_text])?;
    run_json(&[
        "change",
        "create",
        "machine-demo",
        "--profile",
        "outcome-machine",
        "--root",
        root_text,
    ])?;
    let change = root.join(".cairn/changes/machine-demo");
    write_spec_artifacts(&change)?;
    let invalid_foundation = json!({
        "kind": "foundation",
        "details": {
            "current_consumer": "",
            "target_outcome": "CAIRN_PLACEHOLDER",
            "adoption_path": "Adopt through repository policy.",
            "repeatability_plan": "Replay the focused checks."
        }
    });
    let mut review = review_value("observed", "observed", invalid_foundation);
    review["ai_assistance"]["hidden_session_state_required"] = json!(true);
    review["ai_assistance"]["completion_authority"] = json!("model");
    review["activity_counts_are_completion_evidence"] = json!(true);
    fs::write(
        change.join("outcome-machine.json"),
        serde_json::to_vec_pretty(&review)?,
    )?;

    let invalid = run_json(&["gate", "design", "machine-demo", "--root", root_text])?;
    assert_eq!(invalid["verdict"], "FAIL");
    assert!(invalid["issues"]
        .to_string()
        .contains("cairn.workflow.outcome_machine.route"));
    assert!(invalid["issues"]
        .to_string()
        .contains("cairn.workflow.outcome_machine.authority"));

    fs::write(change.join("outcome-machine.json"), "{ malformed\n")?;
    let malformed = run_json(&["gate", "design", "machine-demo", "--root", root_text])?;
    assert_eq!(malformed["verdict"], "FAIL");
    assert!(malformed["issues"]
        .to_string()
        .contains("cairn.workflow.outcome_machine.shape"));
    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn bounded_exception_requires_non_claim_and_fresh_state_rationale() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.outcome_machine_profile.routes]
    // r[verify cairn.outcome_machine_profile.fresh_state]
    let root = temp_project("outcome-machine-exception")?;
    let root_text = root.to_str().ok_or("utf8 root")?;
    run_json(&["init", "--root", root_text])?;
    run_json(&[
        "change",
        "create",
        "machine-demo",
        "--profile",
        "outcome-machine",
        "--root",
        root_text,
    ])?;
    let change = root.join(".cairn/changes/machine-demo");
    write_spec_artifacts(&change)?;
    let route = json!({
        "kind": "bounded_exception",
        "details": {
            "immediate_outcome": "Restore a blocked service.",
            "exception_reason": "The incident deadline prevents mechanism work.",
            "recurrence_decision": "Create a follow-up after another occurrence.",
            "revisit_trigger": "A second occurrence.",
            "capability_non_claim": "This repair does not create repeatable capability."
        }
    });
    let mut review = review_value("not_applicable", "observed", route);
    review["fresh_state_reproduction"]["rationale"] = json!("");
    fs::write(
        change.join("outcome-machine.json"),
        serde_json::to_vec_pretty(&review)?,
    )?;
    let invalid = run_json(&["gate", "design", "machine-demo", "--root", root_text])?;
    assert_eq!(invalid["verdict"], "FAIL");

    review["fresh_state_reproduction"]["rationale"] =
        json!("The incident repair has no replayable execution path.");
    fs::write(
        change.join("outcome-machine.json"),
        serde_json::to_vec_pretty(&review)?,
    )?;
    let valid = run_json(&["gate", "design", "machine-demo", "--root", root_text])?;
    assert_eq!(valid["verdict"], "PASS");
    fs::remove_dir_all(root)?;
    Ok(())
}

fn write_spec_artifacts(change: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(change.join("specs/demo"))?;
    fs::write(change.join("proposal.md"), VALID_PROPOSAL)?;
    fs::write(change.join("design.md"), VALID_DESIGN)?;
    fs::write(change.join("specs/demo/spec.md"), VALID_SPEC)?;
    fs::write(change.join("tasks.md"), VALID_TASKS)?;
    Ok(())
}

fn write_review(
    change: &Path,
    fresh_status: &str,
    completion_status: &str,
    route: Value,
) -> Result<(), Box<dyn Error>> {
    let evidence = change.join("evidence");
    fs::create_dir_all(&evidence)?;
    if fresh_status == "observed" {
        fs::write(evidence.join("fresh-state.json"), FRESH_EVIDENCE)?;
    }
    if completion_status == "observed" {
        fs::write(evidence.join("completion.json"), COMPLETION_EVIDENCE)?;
    }
    fs::write(
        change.join("outcome-machine.json"),
        serde_json::to_vec_pretty(&review_value(fresh_status, completion_status, route))?,
    )?;
    Ok(())
}

fn review_value(fresh_status: &str, completion_status: &str, route: Value) -> Value {
    let fresh_evidence = if fresh_status == "observed" {
        vec![json!({
            "kind": "replay",
            "path": "evidence/fresh-state.json",
            "blake3": cairn_core::blake3_hex(FRESH_EVIDENCE)
        })]
    } else {
        Vec::new()
    };
    let completion_evidence = if completion_status == "observed" {
        vec![json!({
            "kind": "test",
            "path": "evidence/completion.json",
            "blake3": cairn_core::blake3_hex(COMPLETION_EVIDENCE)
        })]
    } else {
        Vec::new()
    };
    json!({
        "schema": "cairn.outcome-machine-review.v1",
        "route": route,
        "maintenance_owner": "repository maintainer",
        "tracked_inputs": ["source", "policy"],
        "deterministic_checks": ["cargo test -p cairn-cli --test lifecycle"],
        "ai_assistance": {
            "used": true,
            "hidden_session_state_required": false,
            "completion_authority": "repository"
        },
        "fresh_state_reproduction": {
            "status": fresh_status,
            "evidence": fresh_evidence,
            "rationale": if fresh_status == "planned" {
                "Run from a clean checkout."
            } else {
                ""
            }
        },
        "completion": {
            "status": completion_status,
            "evidence": completion_evidence
        },
        "activity_counts_are_completion_evidence": false,
        "non_claims": [
            "This record does not prove product value or release eligibility."
        ]
    })
}

fn delivery_route() -> Value {
    json!({
        "kind": "delivery",
        "details": {
            "immediate_outcome": "The operator receives a bounded result.",
            "current_consumer": "workspace operator",
            "durable_capability_contribution": "A reusable lifecycle profile.",
            "repeatability_plan": "Run the same checks from tracked inputs."
        }
    })
}
