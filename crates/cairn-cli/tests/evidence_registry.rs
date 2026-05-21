use std::{
    env,
    error::Error,
    fs,
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

    let fetch_execute_dir = dir.join("materialized-fetch");
    let fetch_execute = run_json_strings(&[
        "evidence".into(),
        "registry".into(),
        "fetch-execute".into(),
        "--root".into(),
        ".".into(),
        "--plan".into(),
        fetch_plan_path.display().to_string(),
        "--source".into(),
        republish_dir.display().to_string(),
        "--out".into(),
        fetch_execute_dir.display().to_string(),
    ])?;
    assert_eq!(fetch_execute["valid"], true);
    assert_eq!(fetch_execute["verdict"], "pass");
    assert_eq!(fetch_execute["plan"], "<plan>");
    assert_eq!(fetch_execute["source"], "<source>");
    assert_eq!(fetch_execute["out"], "<out>");
    assert_eq!(
        fetch_execute["fetch_plan_receipt_hash"],
        fetch_plan["receipt_hash"]
    );
    assert_eq!(
        fetch_execute["registry_receipt_hash"],
        registry["receipt_hash"]
    );
    let planned_artifact_count = fetch_plan["entries"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["artifacts"].as_array().unwrap().len())
        .sum::<usize>();
    assert_eq!(fetch_execute["materialized_count"], planned_artifact_count);
    for artifact in fetch_execute["materialized"].as_array().unwrap() {
        assert_hash_field(artifact, "expected_hash")?;
        assert_hash_field(artifact, "observed_hash")?;
        assert_eq!(artifact["expected_hash"], artifact["observed_hash"]);
        let path = artifact["path"].as_str().unwrap();
        assert!(fetch_execute_dir.join(path).exists());
    }
    assert_hash_field(&fetch_execute, "receipt_hash")?;
    let fetch_execute_schema = receipt_schema("evidence registry fetch-execute")?;
    assert_schema_keys(&fetch_execute, &fetch_execute_schema, "top_level_keys")?;
    // r[cairn.supersede.fetch_execute_policy_docs.policy.schema]
    // r[cairn.supersede.fetch_execute_policy_docs.policy.inventory]
    let fetch_execute_text = serde_json::to_string(&fetch_execute)?;
    assert!(
        !fetch_execute_text.contains(&dir.display().to_string()),
        "{fetch_execute_text}"
    );

    let mut unsafe_fetch_plan = fetch_plan.clone();
    unsafe_fetch_plan["entries"][0]["artifacts"][0]["path"] = Value::String("../escape".into());
    let unsafe_fetch_plan_path = dir.join("unsafe-fetch-plan.json");
    fs::write(
        &unsafe_fetch_plan_path,
        serde_json::to_string_pretty(&unsafe_fetch_plan)? + "\n",
    )?;
    let unsafe_fetch_execute = run_json_failure_strings(&[
        "evidence".into(),
        "registry".into(),
        "fetch-execute".into(),
        "--root".into(),
        ".".into(),
        "--plan".into(),
        unsafe_fetch_plan_path.display().to_string(),
        "--source".into(),
        republish_dir.display().to_string(),
        "--out".into(),
        dir.join("unsafe-materialized-fetch").display().to_string(),
    ])?;
    assert_eq!(unsafe_fetch_execute["valid"], false);
    let unsafe_fetch_codes = unsafe_fetch_execute["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(unsafe_fetch_codes.contains(&"evidence_registry_fetch_execute.artifact_path_unsafe"));

    let mut unverified_fetch_plan = fetch_plan.clone();
    unverified_fetch_plan["valid"] = Value::Bool(false);
    let unverified_fetch_plan_path = dir.join("unverified-fetch-plan.json");
    fs::write(
        &unverified_fetch_plan_path,
        serde_json::to_string_pretty(&unverified_fetch_plan)? + "\n",
    )?;
    let unverified_fetch_execute = run_json_failure_strings(&[
        "evidence".into(),
        "registry".into(),
        "fetch-execute".into(),
        "--root".into(),
        ".".into(),
        "--plan".into(),
        unverified_fetch_plan_path.display().to_string(),
        "--source".into(),
        republish_dir.display().to_string(),
        "--out".into(),
        dir.join("unverified-materialized-fetch")
            .display()
            .to_string(),
    ])?;
    assert_eq!(unverified_fetch_execute["valid"], false);
    let unverified_fetch_codes = unverified_fetch_execute["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(unverified_fetch_codes.contains(&"evidence_registry_fetch_execute.plan_unverified"));

    let missing_fetch_source = dir.join("missing-fetch-source");
    copy_dir(&republish_dir, &missing_fetch_source)?;
    let first_artifact = fetch_plan["entries"][0]["artifacts"][0]["path"]
        .as_str()
        .unwrap();
    fs::remove_file(missing_fetch_source.join(first_artifact))?;
    let missing_fetch_execute = run_json_failure_strings(&[
        "evidence".into(),
        "registry".into(),
        "fetch-execute".into(),
        "--root".into(),
        ".".into(),
        "--plan".into(),
        fetch_plan_path.display().to_string(),
        "--source".into(),
        missing_fetch_source.display().to_string(),
        "--out".into(),
        dir.join("missing-materialized-fetch").display().to_string(),
    ])?;
    assert_eq!(missing_fetch_execute["valid"], false);
    let missing_fetch_codes = missing_fetch_execute["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(missing_fetch_codes.contains(&"evidence_registry_fetch_execute.artifact_missing"));

    let tampered_fetch_source = dir.join("tampered-fetch-source");
    copy_dir(&republish_dir, &tampered_fetch_source)?;
    let tampered_artifact = fetch_plan["entries"][0]["artifacts"][0]["path"]
        .as_str()
        .unwrap();
    fs::write(tampered_fetch_source.join(tampered_artifact), b"tampered")?;
    let tampered_fetch_execute = run_json_failure_strings(&[
        "evidence".into(),
        "registry".into(),
        "fetch-execute".into(),
        "--root".into(),
        ".".into(),
        "--plan".into(),
        fetch_plan_path.display().to_string(),
        "--source".into(),
        tampered_fetch_source.display().to_string(),
        "--out".into(),
        dir.join("tampered-materialized-fetch")
            .display()
            .to_string(),
    ])?;
    assert_eq!(tampered_fetch_execute["valid"], false);
    let tampered_fetch_codes = tampered_fetch_execute["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["code"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(
        tampered_fetch_codes.contains(&"evidence_registry_fetch_execute.artifact_hash_mismatch")
    );

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
