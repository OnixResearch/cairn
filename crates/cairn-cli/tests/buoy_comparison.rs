use std::{
    collections::BTreeMap,
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use serde_json::Value;

const FIXTURES_RELATIVE: &str = "fixtures/buoy-comparison/v1";
const POLICY_RELATIVE: &str = "cairn-policy/generated/cairn-policy.json";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn cairn() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cairn"));
    command.current_dir(repo_root());
    command
}

fn temp_project(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let destination = env::temp_dir().join(format!(
        "cairn-buoy-comparison-{name}-{}",
        std::process::id()
    ));
    if destination.exists() {
        fs::remove_dir_all(&destination)?;
    }
    fs::create_dir_all(&destination)?;
    Ok(destination)
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn fixture_files(root: &Path) -> Result<BTreeMap<String, Vec<u8>>, Box<dyn Error>> {
    let mut files = BTreeMap::new();
    let mut pending = vec![(root.to_path_buf(), String::new())];
    while let Some((directory, prefix)) = pending.pop() {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            let relative = if prefix.is_empty() {
                name
            } else {
                format!("{prefix}/{name}")
            };
            if entry.file_type()?.is_dir() {
                pending.push((entry.path(), relative));
            } else {
                files.insert(relative, fs::read(entry.path())?);
            }
        }
    }
    Ok(files)
}

fn run_compare(
    fixtures: &Path,
    out: &Path,
    observations: Option<&Path>,
    policy: Option<&Path>,
) -> Result<Output, Box<dyn Error>> {
    let mut command = cairn();
    command.args(["traceability", "compare", "--root"]);
    command.arg(repo_root());
    command.args(["--fixtures"]);
    command.arg(fixtures);
    command.args(["--out"]);
    command.arg(out);
    if let Some(observations) = observations {
        command.args(["--observations"]);
        command.arg(observations);
    }
    if let Some(policy) = policy {
        command.args(["--policy"]);
        command.arg(policy);
    }
    Ok(command.output()?)
}

fn read_receipt(out: &Path) -> Result<Value, Box<dyn Error>> {
    Ok(serde_json::from_slice(&fs::read(out)?)?)
}

fn assert_success(output: &Output, command: &str) {
    assert!(
        output.status.success(),
        "{command} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn issue_codes(receipt: &Value) -> Vec<String> {
    receipt["issues"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|issue| issue["code"].as_str().map(str::to_string))
        .collect()
}

fn case<'a>(receipt: &'a Value, id: &str) -> &'a Value {
    receipt["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .find(|case| case["id"] == id)
        .unwrap_or_else(|| panic!("case {id}"))
}

#[test]
fn comparison_executes_native_fixtures_and_retains_raw_observations() -> Result<(), Box<dyn Error>>
{
    // r[verify cairn.trace_index.validation]
    let root = temp_project("executed")?;
    let fixtures = repo_root().join(FIXTURES_RELATIVE);
    let before = fixture_files(&fixtures)?;
    let out = root.join("target/receipt.json");
    let observations = root.join("observations");
    let output = run_compare(&fixtures, &out, Some(&observations), None)?;
    assert_success(&output, "traceability compare");

    let receipt = read_receipt(&out)?;
    assert_eq!(receipt["valid"], true);
    assert_eq!(receipt["verdict"], "pass");
    assert_eq!(receipt["execution"]["native_executed"], true);
    assert_eq!(receipt["execution"]["upstream_executed"], false);
    assert_eq!(receipt["complete"], false);
    assert_eq!(receipt["execution"]["executed_case_count"], 8u64);
    assert_eq!(receipt["upstream"]["executed"], false);
    assert_eq!(receipt["upstream"]["artifact_available"], false);
    assert_eq!(
        receipt["upstream"]["revision"],
        "01096fbdd0b9bf39b2fec468f90e7b1db958f615"
    );
    assert_eq!(receipt["upstream"]["rust_toolchain"], "nightly-2026-04-16");
    assert_eq!(receipt["upstream"]["digest_feature"], "blake3-256");
    assert_eq!(
        receipt["mutation_qualification"]["planted_drift_detected"],
        true
    );
    assert_eq!(
        receipt["mutation_qualification"]["restated_observation_rejected"],
        true
    );
    assert_eq!(
        receipt["mutation_qualification"]["restated_native_expectation_rejected"],
        true
    );

    assert_eq!(case(&receipt, "clean")["outcome"], "agreement");
    assert_eq!(
        case(&receipt, "missing_marker")["native_semantic"],
        "stale_item"
    );
    assert_eq!(
        case(&receipt, "duplicate_marker")["native_semantic"],
        "stale_item"
    );
    assert_eq!(
        case(&receipt, "moved_span")["native_semantic"],
        "stale_item"
    );
    assert_eq!(
        case(&receipt, "changed_requirement_body")["native_semantic"],
        "stale_requirement"
    );
    assert_eq!(
        case(&receipt, "schema_version_drift")["native_semantic"],
        "unsupported"
    );
    assert_eq!(
        case(&receipt, "schema_version_drift")["outcome"],
        "unsupported"
    );
    assert_eq!(
        case(&receipt, "unicode_crlf")["outcome"],
        "expected_divergence"
    );
    assert_eq!(
        case(&receipt, "cfg_not_any")["outcome"],
        "expected_divergence"
    );
    assert!(receipt["known_disagreements"]
        .as_array()
        .expect("known disagreements")
        .iter()
        .any(|value| value == "unicode_crlf"));

    for observation in [
        "clean",
        "missing_marker",
        "duplicate_marker",
        "moved_span",
        "changed_requirement_body",
        "schema_version_drift",
        "unicode_crlf",
        "cfg_not_any",
    ] {
        assert!(observations
            .join(observation)
            .join("identity.json")
            .is_file());
    }
    let missing: Value =
        serde_json::from_slice(&fs::read(observations.join("missing_marker/stdout.json"))?)?;
    assert_eq!(missing["valid"], false);
    assert!(missing["issues"]
        .as_array()
        .expect("issues")
        .iter()
        .any(|issue| issue["code"] == "trace_index.source_stale"));
    let duplicate: Value = serde_json::from_slice(&fs::read(
        observations.join("duplicate_marker/stdout.json"),
    )?)?;
    assert!(duplicate["issues"]
        .as_array()
        .expect("issues")
        .iter()
        .any(|issue| issue["code"] == "trace_index.source_unexpected"));
    let changed: Value = serde_json::from_slice(&fs::read(
        observations.join("changed_requirement_body/stdout.json"),
    )?)?;
    assert!(changed["issues"]
        .as_array()
        .expect("issues")
        .iter()
        .any(|issue| issue["code"] == "trace_index.registry_stale"));
    let drifted: Value = serde_json::from_slice(&fs::read(
        observations.join("schema_version_drift/stdout.json"),
    )?)?;
    assert!(drifted["issues"]
        .as_array()
        .expect("issues")
        .iter()
        .any(|issue| issue["code"] == "trace_index.schema_unsupported"));

    assert_eq!(before, fixture_files(&fixtures)?);
    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn comparison_rejects_restated_observations_and_unsupported_pass() -> Result<(), Box<dyn Error>> {
    let root = temp_project("restated")?;
    let fixtures = root.join("fixtures");
    copy_tree(&repo_root().join(FIXTURES_RELATIVE), &fixtures)?;
    let manifest_path = fixtures.join("manifest.json");
    let original: Value = serde_json::from_slice(&fs::read(&manifest_path)?)?;

    let mut restated = original.clone();
    for case in restated["cases"].as_array_mut().expect("cases") {
        if case["id"] == "missing_marker" {
            case["buoy_observation"]["semantic"] = Value::String("current".to_string());
        }
    }
    fs::write(&manifest_path, serde_json::to_vec_pretty(&restated)?)?;
    let out = root.join("target/restated.json");
    let output = run_compare(&fixtures, &out, None, None)?;
    assert!(!output.status.success());
    let receipt = read_receipt(&out)?;
    let codes = issue_codes(&receipt);
    assert!(codes.contains(&"comparison.mutation_restated".to_string()));
    assert_eq!(
        receipt["mutation_qualification"]["restated_observation_rejected"],
        true
    );

    let mut unsupported_passed = original;
    for case in unsupported_passed["cases"].as_array_mut().expect("cases") {
        if case["id"] == "schema_version_drift" {
            case["comparison"] = Value::String("agreement".to_string());
        }
    }
    fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&unsupported_passed)?,
    )?;
    let out = root.join("target/unsupported.json");
    let output = run_compare(&fixtures, &out, None, None)?;
    assert!(!output.status.success());
    let receipt = read_receipt(&out)?;
    assert!(issue_codes(&receipt).contains(&"comparison.disposition_invalid".to_string()));

    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn comparison_identity_binds_source_mapping_fixture_and_policy() -> Result<(), Box<dyn Error>> {
    let root = temp_project("identity")?;
    let fixtures = root.join("fixtures");
    copy_tree(&repo_root().join(FIXTURES_RELATIVE), &fixtures)?;
    let manifest_path = fixtures.join("manifest.json");
    let original: Value = serde_json::from_slice(&fs::read(&manifest_path)?)?;

    let baseline_out = root.join("target/baseline.json");
    assert_success(
        &run_compare(&fixtures, &baseline_out, None, None)?,
        "baseline comparison",
    );
    let baseline = read_receipt(&baseline_out)?["comparison_identity"].clone();
    assert!(baseline.as_str().is_some_and(|value| value.len() == 64));

    let mut source_changed = original.clone();
    source_changed["upstream"]["revision"] =
        Value::String("0000000000000000000000000000000000000000".to_string());
    fs::write(&manifest_path, serde_json::to_vec_pretty(&source_changed)?)?;
    let out = root.join("target/source.json");
    assert_success(
        &run_compare(&fixtures, &out, None, None)?,
        "source identity comparison",
    );
    assert_ne!(read_receipt(&out)?["comparison_identity"], baseline);

    let mut tool_changed = original.clone();
    tool_changed["upstream"]["rust_toolchain"] = Value::String("nightly-2099-01-01".to_string());
    fs::write(&manifest_path, serde_json::to_vec_pretty(&tool_changed)?)?;
    let out = root.join("target/tool.json");
    assert_success(
        &run_compare(&fixtures, &out, None, None)?,
        "toolchain identity comparison",
    );
    assert_ne!(read_receipt(&out)?["comparison_identity"], baseline);

    let mut mapping_changed = original.clone();
    mapping_changed["mapping"]["id"] =
        Value::String("cairn.buoy-comparison.mapping.changed.v1".to_string());
    fs::write(&manifest_path, serde_json::to_vec_pretty(&mapping_changed)?)?;
    let out = root.join("target/mapping.json");
    assert_success(
        &run_compare(&fixtures, &out, None, None)?,
        "mapping identity comparison",
    );
    assert_ne!(read_receipt(&out)?["comparison_identity"], baseline);

    fs::write(&manifest_path, serde_json::to_vec_pretty(&original)?)?;
    let clean_source = fixtures.join("cases/clean/crates/demo/src/lib.rs");
    let mut source = fs::read(&clean_source)?;
    source.push(b'\n');
    fs::write(&clean_source, source)?;
    let out = root.join("target/fixture.json");
    assert_success(
        &run_compare(&fixtures, &out, None, None)?,
        "fixture identity comparison",
    );
    assert_ne!(read_receipt(&out)?["comparison_identity"], baseline);

    let clean_source = fixtures.join("cases/clean/crates/demo/src/lib.rs");
    let mut restored = Vec::new();
    restored.extend_from_slice(b"// r[");
    restored.extend_from_slice(b"impl cairn.buoy_comparison.demo]\npub fn demo() {}\n");
    fs::write(&clean_source, restored)?;
    let policy = root.join("policy.json");
    let mut policy_bytes = fs::read(repo_root().join(POLICY_RELATIVE))?;
    policy_bytes.push(b'\n');
    fs::write(&policy, policy_bytes)?;
    let out = root.join("target/policy.json");
    assert_success(
        &run_compare(&fixtures, &out, None, Some(&policy))?,
        "policy identity comparison",
    );
    assert_ne!(read_receipt(&out)?["comparison_identity"], baseline);

    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn comparison_rejects_unknown_manifest_schema() -> Result<(), Box<dyn Error>> {
    let root = temp_project("schema")?;
    let fixtures = root.join("fixtures");
    copy_tree(&repo_root().join(FIXTURES_RELATIVE), &fixtures)?;
    let manifest_path = fixtures.join("manifest.json");
    let mut manifest: Value = serde_json::from_slice(&fs::read(&manifest_path)?)?;
    manifest["schema"] = Value::String("cairn.buoy-comparison.fixtures.v999".to_string());
    fs::write(&manifest_path, serde_json::to_vec_pretty(&manifest)?)?;
    let out = root.join("target/schema.json");
    let output = run_compare(&fixtures, &out, None, None)?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("invalid Buoy comparison manifest"));
    fs::remove_dir_all(root)?;
    Ok(())
}
