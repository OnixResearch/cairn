use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use serde_json::Value;

const REQUIREMENT_ID: &str = "cairn.trace_index.demo";
const REPOSITORY_ID: &str = "onixresearch/trace-index-fixture";
const REVISION_KIND: &str = "opaque";
const REVISION: &str = "trace-index-fixture-revision";
const POLICY_RELATIVE: &str = "cairn-policy/generated/cairn-policy.json";
const REGISTRY_RELATIVE: &str = "target/requirement-registry.json";
const INDEX_RELATIVE: &str = "target/trace-index.json";
const SECOND_INDEX_RELATIVE: &str = "target/trace-index-second.json";
const SOURCE_RELATIVE: &str = "crates/demo/src/lib.rs";
const COPIED_SOURCE_RELATIVE: &str = "crates/demo/src/copied.rs";
const BLAKE3_HEX_LENGTH: usize = 64;
const EXPECTED_LINK_COUNT: u64 = 2;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn cairn() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cairn"));
    command.current_dir(repo_root());
    command
}

fn temp_project(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let destination =
        env::temp_dir().join(format!("cairn-trace-index-{name}-{}", std::process::id()));
    if destination.exists() {
        fs::remove_dir_all(&destination)?;
    }
    fs::create_dir_all(&destination)?;
    Ok(destination)
}

fn write_text(path: &Path, text: &str) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, text)?;
    Ok(())
}

fn fixture_specification() -> String {
    format!(
        "# Trace index fixture\n\n## Requirements\n\n### Requirement: Content-bound trace\n\nr[{REQUIREMENT_ID}] Cairn MUST bind declared source links.\n\n#### Scenario: Declared link\n\n- GIVEN a source marker\n- WHEN Cairn builds the index\n- THEN the index MUST contain the link\n"
    )
}

fn fixture_source(prefix: &str) -> String {
    format!("{prefix}// r[impl {REQUIREMENT_ID}]\n// r[verify {REQUIREMENT_ID}]\n")
}

fn write_fixture(root: &Path) -> Result<PathBuf, Box<dyn Error>> {
    write_text(
        &root.join(".cairn/specs/trace-index/spec.md"),
        &fixture_specification(),
    )?;
    write_text(&root.join(SOURCE_RELATIVE), &fixture_source(""))?;
    let policy_path = root.join(POLICY_RELATIVE);
    if let Some(parent) = policy_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(
        repo_root().join("cairn-policy/generated/cairn-policy.json"),
        &policy_path,
    )?;
    Ok(policy_path)
}

fn build_registry(root: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let registry = root.join(REGISTRY_RELATIVE);
    let output = cairn()
        .args(["requirement-registry", "export", "--root"])
        .arg(root)
        .args([
            "--repository-id",
            REPOSITORY_ID,
            "--revision-kind",
            REVISION_KIND,
            "--revision",
            REVISION,
            "--out",
        ])
        .arg(&registry)
        .args(["--policy", POLICY_RELATIVE])
        .output()?;
    assert_success(&output, "requirement-registry export");
    Ok(registry)
}

fn export_index(
    root: &Path,
    policy: &Path,
    registry: &Path,
    output_relative: &str,
) -> Result<(PathBuf, Value), Box<dyn Error>> {
    let index = root.join(output_relative);
    let output = cairn()
        .args(["traceability", "export", "--root"])
        .arg(root)
        .args(["--policy"])
        .arg(policy)
        .args(["--registry"])
        .arg(registry)
        .args(["--out"])
        .arg(&index)
        .output()?;
    assert_success(&output, "traceability export");
    Ok((index, serde_json::from_slice(&output.stdout)?))
}

fn verify_index(
    root: &Path,
    policy: &Path,
    registry: &Path,
    index: &Path,
) -> Result<std::process::Output, Box<dyn Error>> {
    Ok(cairn()
        .args(["traceability", "verify", "--root"])
        .arg(root)
        .args(["--policy"])
        .arg(policy)
        .args(["--registry"])
        .arg(registry)
        .args(["--index"])
        .arg(index)
        .output()?)
}

fn show_requirement(
    index: &Path,
    requirement_id: &str,
) -> Result<std::process::Output, Box<dyn Error>> {
    Ok(cairn()
        .args(["traceability", "show", requirement_id, "--index"])
        .arg(index)
        .output()?)
}

fn assert_success(output: &std::process::Output, command: &str) {
    assert!(
        output.status.success(),
        "{command} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn issue_codes(receipt: &Value, field: &str) -> Vec<String> {
    receipt[field]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|issue| issue["code"].as_str().map(str::to_string))
        .collect()
}

fn assert_blake3(value: &Value, field: &str) {
    let hash = value[field].as_str().expect("BLAKE3 field");
    assert_eq!(hash.len(), BLAKE3_HEX_LENGTH);
    assert!(hash
        .bytes()
        .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')));
}

#[test]
fn trace_index_export_is_deterministic_and_directly_queryable() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.trace_index.cli]
    // r[verify cairn.trace_index.query]
    let root = temp_project("export-query")?;
    let policy = write_fixture(&root)?;
    let registry = build_registry(&root)?;
    let (index, first_receipt) = export_index(&root, &policy, &registry, INDEX_RELATIVE)?;
    let (second_index, second_receipt) =
        export_index(&root, &policy, &registry, SECOND_INDEX_RELATIVE)?;

    assert_eq!(fs::read(&index)?, fs::read(&second_index)?);
    assert_eq!(first_receipt["command"], "traceability export");
    assert_eq!(first_receipt["valid"], true);
    assert_eq!(first_receipt["link_count"], EXPECTED_LINK_COUNT);
    assert_eq!(
        first_receipt["index_blake3"],
        second_receipt["index_blake3"]
    );
    assert_blake3(&first_receipt, "index_blake3");

    let verification = verify_index(&root, &policy, &registry, &index)?;
    assert_success(&verification, "traceability verify");
    let verification_receipt: Value = serde_json::from_slice(&verification.stdout)?;
    assert_eq!(verification_receipt["valid"], true);
    assert_eq!(verification_receipt["verdict"], "pass");

    let shown = show_requirement(&index, REQUIREMENT_ID)?;
    assert_success(&shown, "traceability show");
    let show_receipt: Value = serde_json::from_slice(&shown.stdout)?;
    assert_eq!(show_receipt["valid"], true);
    assert_eq!(
        show_receipt["query"]["requirement"]["requirement_id"],
        REQUIREMENT_ID
    );
    assert_eq!(
        show_receipt["query"]["links"]
            .as_array()
            .expect("query links")
            .len(),
        usize::try_from(EXPECTED_LINK_COUNT)?
    );

    let unknown = show_requirement(&index, "cairn.trace_index.unknown")?;
    assert!(!unknown.status.success());
    let unknown_receipt: Value = serde_json::from_slice(&unknown.stdout)?;
    assert!(issue_codes(&unknown_receipt, "issues")
        .contains(&"trace_index.query_unknown_requirement".to_string()));

    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn trace_index_verify_rejects_stale_moved_and_copied_markers() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.trace_index.freshness]
    let root = temp_project("stale-markers")?;
    let policy = write_fixture(&root)?;
    let registry = build_registry(&root)?;
    let (index, _receipt) = export_index(&root, &policy, &registry, INDEX_RELATIVE)?;

    write_text(&root.join(SOURCE_RELATIVE), &fixture_source("\n"))?;
    let moved = verify_index(&root, &policy, &registry, &index)?;
    assert!(!moved.status.success());
    let moved_receipt: Value = serde_json::from_slice(&moved.stdout)?;
    let moved_codes = issue_codes(&moved_receipt, "issues");
    assert!(moved_codes.contains(&"trace_index.source_stale".to_string()));
    assert!(moved_codes.contains(&"trace_index.link_stale".to_string()));

    write_text(&root.join(SOURCE_RELATIVE), &fixture_source(""))?;
    write_text(
        &root.join(COPIED_SOURCE_RELATIVE),
        &format!("// r[impl {REQUIREMENT_ID}]\n"),
    )?;
    let copied = verify_index(&root, &policy, &registry, &index)?;
    assert!(!copied.status.success());
    let copied_receipt: Value = serde_json::from_slice(&copied.stdout)?;
    let copied_codes = issue_codes(&copied_receipt, "issues");
    assert!(copied_codes.contains(&"trace_index.source_unexpected".to_string()));
    assert!(copied_codes.contains(&"trace_index.link_stale".to_string()));

    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn trace_index_verify_rejects_policy_drift_and_malformed_indexes() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.trace_index.freshness]
    // r[verify cairn.trace_index.validation]
    let root = temp_project("policy-malformed")?;
    let policy = write_fixture(&root)?;
    let registry = build_registry(&root)?;
    let (index, _receipt) = export_index(&root, &policy, &registry, INDEX_RELATIVE)?;

    let policy_text = fs::read_to_string(&policy)?;
    let mut drifted_policy = policy_text.clone();
    drifted_policy.push('\n');
    fs::write(&policy, drifted_policy)?;
    let drift = verify_index(&root, &policy, &registry, &index)?;
    assert!(!drift.status.success());
    let drift_receipt: Value = serde_json::from_slice(&drift.stdout)?;
    assert!(issue_codes(&drift_receipt, "registry_issues")
        .contains(&"content_requirement_registry.stale_policy".to_string()));

    fs::write(&policy, policy_text)?;
    fs::write(
        &index,
        b"{\"schema_version\":\"wrong\",\"unexpected\":true}\n",
    )?;
    let malformed = verify_index(&root, &policy, &registry, &index)?;
    assert!(!malformed.status.success());
    assert!(
        String::from_utf8_lossy(&malformed.stderr).contains("invalid content-bound trace index")
    );

    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn rust_item_profile_exports_verifies_queries_and_revises() -> Result<(), Box<dyn Error>> {
    let root = temp_project("rust-items")?;
    let policy_path = write_fixture(&root)?;
    let first_source = format!("// r[impl {REQUIREMENT_ID}]\r\n/* é */ #[cfg(not(any()))] fn x() {{}}\r\n");
    write_text(&root.join(SOURCE_RELATIVE), &first_source)?;
    write_text(&root.join("rust-config.json"),
        include_str!("../../../fixtures/trace-index-rust-items/rust-config.json"))?;
    let mut policy: Value = serde_json::from_slice(&fs::read(&policy_path)?)?;
    let mut profile = policy["traceability_policy"]["profiles"][0].clone();
    profile["id"] = Value::String("rust-items".into());
    profile["anchor_kind"] = Value::String("rust_item_v1".into());
    profile["rust_configuration_path"] = Value::String("rust-config.json".into());
    profile["evidence_sources"] = serde_json::json!([{
        "root": "crates/demo/src", "source_kind": "rust_source", "extensions": ["rs"]
    }]);
    policy["traceability_policy"]["profiles"].as_array_mut().unwrap().push(profile);
    fs::write(&policy_path, serde_json::to_vec_pretty(&policy)?)?;
    let registry = build_registry(&root)?;
    let index = root.join(INDEX_RELATIVE);
    let export = cairn().args(["traceability", "export", "--root"]).arg(&root)
        .args(["--policy"]).arg(&policy_path).args(["--profile", "rust-items"])
        .args(["--registry"]).arg(&registry).args(["--out"]).arg(&index).output()?;
    assert_success(&export, "structural traceability export");
    let saved: Value = serde_json::from_slice(&fs::read(&index)?)?;
    assert_eq!(saved["schema_version"], "cairn.content-bound-trace-index.v2");
    let item = &saved["structural"]["bindings"][0]["item"];
    assert_eq!(item["start_byte"].as_u64().unwrap() as usize, first_source.find("#[cfg").unwrap());
    assert_eq!(item["item_kind"], "fn");
    assert_success(&verify_index(&root, &policy_path, &registry, &index)?, "structural verification");
    let show = cairn().args(["traceability", "show", REQUIREMENT_ID, "--root"]).arg(&root)
        .args(["--index"]).arg(&index).args(["--registry"]).arg(&registry)
        .args(["--policy"]).arg(&policy_path).output()?;
    assert_success(&show, "structural query");
    let shown: Value = serde_json::from_slice(&show.stdout)?;
    assert_eq!(shown["binding_status"], "current");
    assert_eq!(shown["bindings"][0]["binding_blake3"], saved["structural"]["bindings"][0]["binding_blake3"]);

    let changed = first_source.replace("fn x() {}", "fn x() { }");
    write_text(&root.join(SOURCE_RELATIVE), &changed)?;
    let stale = verify_index(&root, &policy_path, &registry, &index)?;
    assert!(!stale.status.success());
    let stale_receipt: Value = serde_json::from_slice(&stale.stdout)?;
    assert!(issue_codes(&stale_receipt, "issues").contains(&"trace_index.binding_stale".into()));
    let next = root.join(SECOND_INDEX_RELATIVE);
    let export = cairn().args(["traceability", "export", "--root"]).arg(&root)
        .args(["--policy"]).arg(&policy_path).args(["--profile", "rust-items"])
        .args(["--registry"]).arg(&registry).args(["--previous-index"]).arg(&index)
        .args(["--out"]).arg(&next).output()?;
    assert_success(&export, "structural revision export");
    let revised: Value = serde_json::from_slice(&fs::read(&next)?)?;
    assert_eq!(revised["structural"]["history"][0]["binding_blake3"], saved["structural"]["bindings"][0]["binding_blake3"]);
    assert_eq!(revised["structural"]["history"][0]["item"]["normalized_item_blake3"],
        revised["structural"]["bindings"][0]["item"]["normalized_item_blake3"]);
    assert_success(&verify_index(&root, &policy_path, &registry, &next)?, "revised structural verification");
    let reconfigured = include_str!("../../../fixtures/trace-index-rust-items/rust-config.json")
        .replace("x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu");
    write_text(&root.join("rust-config.json"), &reconfigured)?;
    let configuration_stale = verify_index(&root, &policy_path, &registry, &next)?;
    assert!(!configuration_stale.status.success());
    let receipt: Value = serde_json::from_slice(&configuration_stale.stdout)?;
    assert!(issue_codes(&receipt, "issues").contains(&"trace_index.configuration_stale".into()));
    write_text(&root.join("rust-config.json"), include_str!("../../../fixtures/trace-index-rust-items/rust-config.json"))?;

    write_text(&root.join(SOURCE_RELATIVE), &(changed.clone() + "fn helper() { 1 + 2; }\r\n"))?;
    let helper_stale = verify_index(&root, &policy_path, &registry, &next)?;
    assert!(!helper_stale.status.success());
    let receipt: Value = serde_json::from_slice(&helper_stale.stdout)?;
    assert!(issue_codes(&receipt, "issues").contains(&"trace_index.source_stale".into()));
    assert_eq!(revised["structural"]["bindings"][0]["item"]["normalized_item_blake3"],
        saved["structural"]["bindings"][0]["item"]["normalized_item_blake3"]);
    write_text(&root.join(SOURCE_RELATIVE), &changed)?;

    let unsupported = format!("// r[impl {REQUIREMENT_ID}]\nfoo!();\n");
    write_text(&root.join(SOURCE_RELATIVE), &unsupported)?;
    let rejected = cairn().args(["traceability", "export", "--root"]).arg(&root)
        .args(["--policy"]).arg(&policy_path).args(["--profile", "rust-items"])
        .args(["--registry"]).arg(&registry).args(["--out"]).arg(root.join("target/unsupported.json")).output()?;
    assert!(!rejected.status.success());
    let receipt: Value = serde_json::from_slice(&rejected.stdout)?;
    assert!(issue_codes(&receipt, "issues").contains(&"trace_index.item_unsupported".into()));
    fs::remove_dir_all(&root)?;
    Ok(())
}
