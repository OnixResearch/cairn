use std::{error::Error, fs};

use serde_json::{json, Value};

use super::support::{repo_root, run_json_failure_strings, EvidenceChain, RegistryChain};

pub(crate) fn verify(
    chain: &EvidenceChain,
    registry_chain: &RegistryChain,
) -> Result<(), Box<dyn Error>> {
    // r[verify cairn.trace_index.evidence_bundle.freshness]
    // r[verify cairn.trace_index.evidence_bundle.validation]
    let dir = &chain.dir;
    let export_dir = &chain.export_dir;
    let import_dir = &chain.import_dir;
    let republish_dir = &chain.republish_dir;
    let index_path = &chain.index_path;
    let index = &chain.index;
    let registry_path = &registry_chain.registry_path;
    let registry = &registry_chain.registry;
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

    let outside_artifact = dir.join("outside-artifact.json");
    fs::write(&outside_artifact, "{}\n")?;
    let escape_link = export_dir.join("escape-link.json");
    std::os::unix::fs::symlink(&outside_artifact, &escape_link)?;
    let symlink_import_failure = run_json_failure_strings(&[
        "evidence".into(),
        "import".into(),
        "--root".into(),
        ".".into(),
        "--bundle".into(),
        export_dir.display().to_string(),
        "--into".into(),
        dir.join("symlink-import").display().to_string(),
    ])?;
    assert_eq!(symlink_import_failure["valid"], false);
    let symlink_import_issue_codes = symlink_import_failure["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(symlink_import_issue_codes.contains(&"evidence_import.bundle_read_failed"));
    fs::remove_file(escape_link)?;

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

    let manifest_path = export_dir.join("evidence-manifest.json");
    let manifest: Value = serde_json::from_slice(&fs::read(&manifest_path)?)?;
    let mut registry_mismatch_manifest = manifest.clone();
    let registry_binding = registry_mismatch_manifest["artifacts"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|artifact| artifact["role"] == "trace_index")
        .unwrap();
    registry_binding["requirement_registry_blake3"] =
        json!("0000000000000000000000000000000000000000000000000000000000000000");
    let registry_mismatch_path = export_dir.join("registry-mismatch-manifest.json");
    fs::write(
        &registry_mismatch_path,
        serde_json::to_string_pretty(&registry_mismatch_manifest)? + "\n",
    )?;
    let registry_mismatch = run_json_failure_strings(&[
        "evidence-manifest".into(),
        "verify".into(),
        registry_mismatch_path.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    let registry_mismatch_codes = registry_mismatch["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(registry_mismatch_codes.contains(&"artifact.trace_index_registry_mismatch"));
    fs::remove_file(&registry_mismatch_path)?;

    let mut verification_mismatch_manifest = manifest;
    let verification_binding = verification_mismatch_manifest["artifacts"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|artifact| artifact["role"] == "trace_index_verification")
        .unwrap();
    verification_binding["index_blake3"] =
        json!("0000000000000000000000000000000000000000000000000000000000000000");
    let verification_mismatch_path = export_dir.join("verification-mismatch-manifest.json");
    fs::write(
        &verification_mismatch_path,
        serde_json::to_string_pretty(&verification_mismatch_manifest)? + "\n",
    )?;
    let verification_mismatch = run_json_failure_strings(&[
        "evidence-manifest".into(),
        "verify".into(),
        verification_mismatch_path.display().to_string(),
        "--root".into(),
        ".".into(),
    ])?;
    let verification_mismatch_codes = verification_mismatch["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(verification_mismatch_codes.contains(&"artifact.trace_index_verification_mismatch"));
    fs::remove_file(&verification_mismatch_path)?;

    let stale_root = dir.join("stale-root");
    super::support::copy_dir(
        &repo_root().join(".cairn/specs"),
        &stale_root.join(".cairn/specs"),
    )?;
    super::support::copy_dir(
        &repo_root().join("cairn-policy"),
        &stale_root.join("cairn-policy"),
    )?;
    super::support::copy_dir(&repo_root().join("crates"), &stale_root.join("crates"))?;
    super::support::copy_dir(&repo_root().join("tools"), &stale_root.join("tools"))?;
    fs::write(
        stale_root.join("crates/cairn-cli/src/trace_index.rs"),
        format!(
            "{}\n// changed after trace export\n",
            fs::read_to_string(repo_root().join("crates/cairn-cli/src/trace_index.rs"))?
        ),
    )?;
    let stale_verify = run_json_failure_strings(&[
        "evidence".into(),
        "verify".into(),
        "--root".into(),
        stale_root.display().to_string(),
        "--bundle".into(),
        export_dir.display().to_string(),
    ])?;
    let stale_codes = stale_verify["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(stale_codes.contains(&"evidence_verify.trace_index_stale"));

    let mut malformed_index: Value =
        serde_json::from_slice(&fs::read(export_dir.join("trace-index.json"))?)?;
    malformed_index["schema_version"] = json!("wrong");
    fs::write(
        export_dir.join("trace-index.json"),
        serde_json::to_string_pretty(&malformed_index)? + "\n",
    )?;
    let malformed_verify = run_json_failure_strings(&[
        "evidence".into(),
        "verify".into(),
        "--root".into(),
        ".".into(),
        "--bundle".into(),
        export_dir.display().to_string(),
    ])?;
    let malformed_codes = malformed_verify["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(malformed_codes.contains(&"evidence_verify.artifact_hash_mismatch"));
    assert!(malformed_codes.contains(&"evidence_verify.manifest_rerun_invalid"));

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
