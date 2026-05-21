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

fn assert_hash_field(value: &Value, field: &str) -> Result<(), Box<dyn Error>> {
    let hash = value[field].as_str().ok_or("hash field missing")?;
    assert_eq!(hash.len(), 64, "{field} should be a BLAKE3 hex hash");
    assert!(
        hash.chars().all(|character| character.is_ascii_hexdigit()),
        "{field} should be hex: {hash}"
    );
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
