use super::support::*;
use std::{error::Error, fs};

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
            "release_evidence_index_check_hash",
            "semantic_review_ledger_hash",
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
    assert_hash_field(&receipt, "release_evidence_index_check_hash")?;
    assert_hash_field(&receipt, "semantic_review_ledger_hash")?;
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
    assert!(check_names.contains(&"release_evidence_index_freshness"));
    assert!(check_names.contains(&"semantic_review_ledger"));
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
            "release_evidence_index_check_hash",
            "semantic_review_ledger_hash",
        ],
    )?;
    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["verdict"], "pass");
    Ok(())
}

#[test]
fn release_readiness_skips_absent_semantic_review_ledger() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.semantic_review.readiness_gate.release_readiness.skipped_absent]
    // r[verify cairn.semantic_review.readiness_gate.boundary.no_reviewer_execution]
    let receipt = run_json(&["release-readiness", "--root", "."])?;
    let semantic_review = receipt["checks"]
        .as_array()
        .ok_or("checks array")?
        .iter()
        .find(|check| check["name"] == "semantic_review_ledger")
        .ok_or("semantic review ledger check")?;
    assert_eq!(semantic_review["passed"], true);
    assert_eq!(semantic_review["skipped"], true);
    assert!(semantic_review["source"]
        .as_str()
        .unwrap_or_default()
        .contains("--review-ledger"));
    assert_hash_field(&receipt, "semantic_review_ledger_hash")?;
    Ok(())
}

#[test]
fn release_readiness_accepts_supplied_semantic_review_ledger() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.semantic_review.readiness_gate.release_readiness]
    // r[verify cairn.semantic_review.readiness_gate.release_readiness.supplied_valid]
    let ledger = temp_project("release-readiness-valid-semantic-ledger")?.join("ledger.json");
    fs::write(
        &ledger,
        r#"{"command":"review ledger","valid":true,"verdict":"PASS","receipt_hash":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"}"#,
    )?;
    let receipt = run_json_strings(&[
        "release-readiness".to_string(),
        "--root".to_string(),
        ".".to_string(),
        "--review-ledger".to_string(),
        ledger.display().to_string(),
    ])?;
    assert_eq!(receipt["valid"], true);
    assert_eq!(
        receipt["semantic_review_ledger_hash"],
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    );
    let semantic_review = receipt["checks"]
        .as_array()
        .ok_or("checks array")?
        .iter()
        .find(|check| check["name"] == "semantic_review_ledger")
        .ok_or("semantic review ledger check")?;
    assert_eq!(semantic_review["passed"], true);
    assert_eq!(semantic_review["verdict"], "PASS");
    fs::remove_file(ledger)?;
    Ok(())
}

#[test]
fn release_readiness_fails_supplied_semantic_review_ledger() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.semantic_review.readiness_gate.release_readiness.supplied_invalid]
    let ledger = temp_project("release-readiness-invalid-semantic-ledger")?.join("ledger.json");
    fs::write(
        &ledger,
        r#"{"command":"review ledger","valid":false,"verdict":"FAIL","receipt_hash":"abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"}"#,
    )?;
    let receipt = run_json(&[
        "release-readiness",
        "--root",
        ".",
        "--review-ledger",
        ledger.to_str().ok_or("utf8 ledger path")?,
    ])?;
    assert_eq!(receipt["valid"], false);
    assert_eq!(receipt["verdict"], "fail");
    let semantic_review = receipt["checks"]
        .as_array()
        .ok_or("checks array")?
        .iter()
        .find(|check| check["name"] == "semantic_review_ledger")
        .ok_or("semantic review ledger check")?;
    assert_eq!(semantic_review["passed"], false);
    assert_eq!(semantic_review["verdict"], "FAIL");
    fs::remove_file(ledger)?;
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
