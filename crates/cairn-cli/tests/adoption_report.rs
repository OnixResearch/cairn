// r[verify cairn.adoption_report.conformance]
// r[verify cairn.adoption_report.queries]
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const HASH_C: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

fn repository() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture(name: &str) -> TestResult<PathBuf> {
    let root = std::env::temp_dir().join(format!(
        "cairn-adoption-report-{name}-{}",
        std::process::id()
    ));
    fs::create_dir_all(root.join(".cairn/specs"))?;
    fs::create_dir_all(root.join(".cairn/changes"))?;
    fs::create_dir_all(root.join(".cairn/archive"))?;
    Ok(root)
}

fn run(args: &[&str]) -> TestResult<Value> {
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(repository())
        .args(args)
        .output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(serde_json::from_slice(&output.stdout)?)
}

fn tree(root: &Path) -> TestResult<BTreeMap<PathBuf, Vec<u8>>> {
    let mut found = BTreeMap::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            let relative = path.strip_prefix(root)?.to_path_buf();
            if entry.file_type()?.is_symlink() {
                found.insert(
                    relative,
                    format!("link:{}", fs::read_link(path)?.display()).into_bytes(),
                );
            } else if entry.file_type()?.is_dir() {
                found.insert(relative, b"directory".to_vec());
                pending.push(path);
            } else {
                found.insert(relative, fs::read(path)?);
            }
        }
    }
    Ok(found)
}

fn relationship(id: &str, consumer: &str) -> Value {
    json!({
        "relationship_id": id,
        "upstream_repository": "onixresearch/cairn",
        "upstream_change": "adoption-upstream",
        "consumer_repository": consumer,
        "required_revision": "2222222222222222222222222222222222222222",
        "subject_id": HASH_C,
        "policy_id": "adoption-policy-v1"
    })
}

fn observation(relationship: &Value, stage: &str, state: &str, producer: &str) -> Value {
    json!({
        "relationship": relationship,
        "stage": stage,
        "state": state,
        "producer_id": producer,
        "producer_status": state,
        "evidence_hashes": [HASH_B],
        "observed_revision": "2222222222222222222222222222222222222222",
        "message": format!("{stage} {state}")
    })
}

fn snapshot(name: &str, relationships: Vec<Value>, observations: Vec<Value>) -> Value {
    json!({
        "schema": "cairn.adoption.snapshot.v1",
        "query": {
            "scope_id": name,
            "relationships": relationships,
            "bounds": {"max_relationships": 512, "max_observations": 4096, "max_provenance": 128}
        },
        "provenance": [{
            "producer_id": "fixture-producer",
            "schema": "cairn.cross-repo-receipt-summary.v1",
            "source_revision": "22515fb7bc01d378312053d389d4bbdd55637a50",
            "tool_identity": "fixture-tool@22515fb7bc01d378312053d389d4bbdd55637a50",
            "policy_identity": "fixture-policy@22515fb7bc01d378312053d389d4bbdd55637a50",
            "observation_scope": "saved-fixture",
            "receipt_hash": HASH_A
        }],
        "observations": observations,
        "coverage": {"partial": false, "selected_relationships": 0, "observed_relationships": 0, "omitted_relationships": [], "bound_exhausted": []}
    })
}

fn complete_observations(relationship: &Value) -> Vec<Value> {
    [
        "upstream_lifecycle",
        "consumer_pin",
        "consumer_compatibility",
        "integration_evidence",
    ]
    .into_iter()
    .map(|stage| observation(relationship, stage, "supported", "fixture-producer"))
    .collect()
}

fn observe_both(root: &Path, name: &str, value: Value) -> TestResult<Value> {
    let path = root.join(format!("{name}.json"));
    fs::write(&path, serde_json::to_vec_pretty(&value)?)?;
    let before = tree(root)?;
    let cli = run(&[
        "adoption",
        "report",
        "--root",
        root.to_str().ok_or("root")?,
        "--snapshot",
        path.file_name()
            .and_then(|name| name.to_str())
            .ok_or("snapshot")?,
    ])?;
    assert_eq!(tree(root)?, before, "CLI effects");
    let mcp_input = json!({"root": root, "snapshot": path.file_name().and_then(|name| name.to_str()).ok_or("snapshot")?});
    let mcp = run(&[
        "mcp",
        "--call",
        "cairn_adoption_report",
        &mcp_input.to_string(),
    ])?;
    assert_eq!(tree(root)?, before, "MCP effects");
    assert_eq!(mcp["structuredContent"], cli, "transport parity");
    assert_eq!(cli["read_only"], true);
    assert_eq!(cli["mutation_authorized"], false);
    for action in cli["actions"].as_array().ok_or("actions")? {
        assert_eq!(action["authority"], "advisory");
        assert_eq!(action["effect_authorized"], false);
    }
    Ok(cli)
}

#[test]
fn cli_and_mcp_report_complete_saved_cairn_to_octet_cohort() -> TestResult {
    let root = fixture("complete")?;
    let rel = relationship("cairn-to-octet", "onixresearch/octet");
    let report = observe_both(
        &root,
        "complete",
        snapshot("complete", vec![rel.clone()], complete_observations(&rel)),
    )?;
    assert_eq!(report["report"]["complete"], true);
    assert_eq!(
        report["report"]["actions"]
            .as_array()
            .ok_or("actions")?
            .len(),
        0
    );
    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn cli_and_mcp_controls_preserve_gaps_conflicts_cycles_and_bounds() -> TestResult {
    let root = fixture("controls")?;
    let first = relationship("cairn-to-octet", "onixresearch/octet");
    let second = relationship("octet-to-lattice", "onixresearch/lattice");

    let mut old_pin = complete_observations(&first);
    old_pin[1] = observation(&first, "consumer_pin", "gap", "stack-deps");
    assert_eq!(
        observe_both(
            &root,
            "old-pin",
            snapshot("old-pin", vec![first.clone()], old_pin)
        )?["report"]["complete"],
        false
    );

    let missing_check = snapshot(
        "missing-check",
        vec![first.clone()],
        complete_observations(&first)
            .into_iter()
            .filter(|item| item["stage"] != "integration_evidence")
            .collect(),
    );
    let missing = observe_both(&root, "missing-check", missing_check)?;
    assert_eq!(
        missing["report"]["relationships"][0]["stages"][3]["state"],
        "missing"
    );

    let mut stale_policy = complete_observations(&first);
    stale_policy[3] = observation(&first, "integration_evidence", "stale", "stack-doctor");
    assert_eq!(
        observe_both(
            &root,
            "stale-policy",
            snapshot("stale-policy", vec![first.clone()], stale_policy)
        )?["report"]["complete"],
        false
    );

    let conflict = snapshot(
        "conflicting-revision",
        vec![first.clone()],
        vec![
            observation(&first, "consumer_pin", "supported", "stack-deps-a"),
            observation(&first, "consumer_pin", "diverged", "stack-deps-b"),
        ],
    );
    assert_eq!(
        observe_both(&root, "conflict", conflict)?["report"]["relationships"][0]["stages"][1]
            ["state"],
        "conflict"
    );

    let mut unsupported = snapshot(
        "unsupported-schema",
        vec![first.clone()],
        complete_observations(&first),
    );
    unsupported["schema"] = json!("cairn.adoption.snapshot.v0");
    assert_eq!(
        observe_both(&root, "unsupported", unsupported)?["report"]["valid"],
        false
    );

    let cycle = snapshot(
        "audit-only-cycle",
        vec![first.clone(), second.clone()],
        complete_observations(&first)
            .into_iter()
            .chain(complete_observations(&second))
            .collect(),
    );
    let cycle_report = observe_both(&root, "cycle", cycle)?;
    assert_eq!(cycle_report["report"]["complete"], true);
    assert_eq!(
        cycle_report["report"]["actions"]
            .as_array()
            .ok_or("actions")?
            .len(),
        0
    );

    let mut partial = snapshot(
        "partial-scope",
        vec![first.clone(), second.clone()],
        complete_observations(&first)
            .into_iter()
            .chain(complete_observations(&second))
            .collect(),
    );
    partial["query"]["bounds"]["max_relationships"] = json!(1);
    let partial_report = observe_both(&root, "partial", partial)?;
    assert_eq!(partial_report["report"]["coverage"]["partial"], true);
    assert_eq!(partial_report["report"]["complete"], false);

    fs::remove_dir_all(root)?;
    Ok(())
}
