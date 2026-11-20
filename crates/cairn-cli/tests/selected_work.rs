use serde_json::{json, Value};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn command(args: &[&str]) -> Result<Output, Box<dyn Error>> {
    Ok(Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(repo())
        .args(args)
        .output()?)
}
fn success(args: &[&str]) -> Result<Value, Box<dyn Error>> {
    let output = command(args)?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(serde_json::from_slice(&output.stdout)?)
}
fn fixture() -> String {
    repo().join("fixtures/basic").to_string_lossy().into_owned()
}

// r[verify cairn.selected_work.contract]
#[test]
fn real_producer_output_passes_independent_portable_admission() -> Result<(), Box<dyn Error>> {
    use cairn_core::verified::selected_work::{
        admit_portable, SelectedWorkPolicy, SelectedWorkSnapshot,
    };
    let output = success(&["selected-work", "demo", "--root", &fixture()])?;
    let snapshot: SelectedWorkSnapshot = serde_json::from_value(output)?;
    let policy = SelectedWorkPolicy::bundled()?;
    let expected = snapshot.cohort.clone();
    admit_portable(&policy, &snapshot, &expected)?;
    let mut malformed = snapshot.clone();
    malformed.schema = "cairn.selected-work.future".into();
    assert!(admit_portable(&policy, &malformed, &expected).is_err());
    let mut crossed = snapshot.clone();
    crossed.completion.subject_hash = Some("b".repeat(64));
    assert!(admit_portable(&policy, &crossed, &expected).is_err());
    let mut incomplete = snapshot.clone();
    incomplete.enumeration.complete = false;
    assert!(admit_portable(&policy, &incomplete, &expected).is_err());
    let mut stale = expected.clone();
    stale.observation = "c".repeat(64);
    assert!(admit_portable(&policy, &snapshot, &stale).is_err());
    Ok(())
}

// r[verify cairn.selected_work.transport]
// r[verify cairn.selected_work.boundary]
#[test]
fn real_cli_mcp_and_machine_share_portable_selected_cohort() -> Result<(), Box<dyn Error>> {
    let root = fixture();
    let cli = success(&["selected-work", "demo", "--root", &root])?;
    let mcp = success(&[
        "mcp",
        "--call",
        "cairn_selected_work",
        &json!({"change":"demo", "root":root}).to_string(),
    ])?;
    assert_eq!(mcp["structuredContent"], cli);
    assert_eq!(cli["schema"], "cairn.selected-work.v1");
    assert!(cli["local"].is_null());
    assert_eq!(
        cli["semantic_extensions"]["dependency_meanings"],
        "unsupported"
    );
    assert_eq!(cli["remaining"][1]["kind"], "completion");
    assert_eq!(cli["remaining"][1]["state"], "unavailable");
    assert!(cli["units"][0]["references"]
        .as_array()
        .is_some_and(|refs| refs
            .iter()
            .any(|reference| reference["kind"] == "requirement")));
    assert!(!cli.to_string().contains(&root));
    let machine = success(&["--machine", "selected-work", "demo", "--root", &root])?;
    assert_eq!(machine["result"], cli);
    let expected = cli["cohort"].to_string();
    let detail = success(&[
        "selected-work",
        "demo",
        "--root",
        &root,
        "--expected-cohort",
        &expected,
        "--detail",
        "--disclosure",
        "local",
    ])?;
    assert_eq!(detail["cohort"], cli["cohort"]);
    assert!(detail["local"]["readiness"]["stages"].is_array());
    let native = success(&["readiness", "demo", "--root", &root])?;
    for (stage, selected) in cli["readiness"]["stages"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        assert_eq!(selected["state"], native["stages"][stage]["state"]);
        assert_eq!(
            selected["decision_hash"],
            native["stages"][stage]["decision_hash"]
        );
    }
    Ok(())
}

// r[verify cairn.selected_work.observation]
// r[verify cairn.selected_work.transport]
#[test]
fn explicit_selectors_and_stale_expected_cohorts_refuse() -> Result<(), Box<dyn Error>> {
    let root = fixture();
    let snapshot = success(&["selected-work", "demo", "--root", &root])?;
    let mut stale = snapshot["cohort"].clone();
    stale["source"] = json!("a".repeat(64));
    for args in [
        vec!["selected-work", "demo", "--root", &root, "--task", "1"],
        vec!["selected-work", "demo", "--root", &root, "--root", &root],
        vec![
            "selected-work",
            "demo",
            "--root",
            &root,
            "--lifecycle-store",
            "other",
        ],
        vec![
            "selected-work",
            "demo",
            "--root",
            &root,
            "--expected-cohort",
            &stale.to_string(),
        ],
        vec!["selected-work", "demo", "--root", &root, "--detail"],
    ] {
        assert!(
            !command(&args)?.status.success(),
            "unexpected acceptance: {args:?}"
        );
    }
    let invalid = command(&[
        "mcp",
        "--call",
        "cairn_selected_work",
        &json!({"change":"demo", "root":root, "unexpected":true}).to_string(),
    ])?;
    assert!(!invalid.status.success());
    Ok(())
}

struct TemporaryRoot(PathBuf);
impl Drop for TemporaryRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn temporary_fixture() -> Result<TemporaryRoot, Box<dyn Error>> {
    let root = std::env::temp_dir().join(format!(
        "cairn-selected-work-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_nanos()
    ));
    let fixture = repo().join("fixtures/basic/.cairn");
    copy_tree(&fixture, &root.join(".cairn"))?;
    Ok(TemporaryRoot(root))
}
fn copy_tree(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&path, &target)?;
        } else if entry.file_type()?.is_file() {
            fs::copy(path, target)?;
        } else {
            return Err("unsupported fixture entry".into());
        }
    }
    Ok(())
}

// r[verify cairn.selected_work.observation]
#[test]
fn authored_ids_scoped_refs_and_independent_malformed_change() -> Result<(), Box<dyn Error>> {
    let fixture = temporary_fixture()?;
    let change = fixture.0.join(".cairn/changes/demo");
    fs::write(change.join("tasks.md"),
        "## Work\n- [x] [task:prepare] r[demo.behavior] Prepare.\n- [ ] [task:implement] [after:prepare] a[demo.acceptance] Implement.\n")?;
    fs::write(
        change.join("metadata.json"),
        r#"{"depends_on":["prerequisite"]}"#,
    )?;
    let prerequisite = fixture.0.join(".cairn/changes/prerequisite");
    fs::create_dir_all(&prerequisite)?;
    fs::write(
        prerequisite.join("tasks.md"),
        "- [ ] [serial] Preliminary work.\n",
    )?;
    let unrelated = fixture.0.join(".cairn/changes/unrelated");
    fs::create_dir_all(&unrelated)?;
    fs::write(unrelated.join("metadata.json"), "{invalid json")?;
    let root = fixture.0.to_string_lossy();
    let output = success(&[
        "selected-work",
        "demo",
        "--root",
        &root,
        "--task",
        "implement",
    ])?;
    assert_eq!(output["selection"]["task"], "implement");
    assert!(output["units"]
        .as_array()
        .unwrap()
        .iter()
        .any(|unit| unit["references"]
            .as_array()
            .unwrap()
            .iter()
            .any(|reference| reference["kind"] == "acceptance"
                && reference["scope"]["kind"] == "task")));
    assert!(output["dependencies"]
        .as_array()
        .unwrap()
        .iter()
        .any(|edge| edge["task"] == "demo" && edge["prerequisite"] == "prerequisite"));
    assert!(!output["source"]["artifacts"]
        .to_string()
        .contains("unrelated"));
    let before = output["cohort"]["source"].clone();
    fs::write(
        change.join("proposal.md"),
        "# Changed intent, no task count changed\n",
    )?;
    let after = success(&[
        "selected-work",
        "demo",
        "--root",
        &root,
        "--task",
        "implement",
    ])?;
    assert_ne!(before, after["cohort"]["source"]);
    let stale = command(&[
        "selected-work",
        "demo",
        "--root",
        &root,
        "--task",
        "implement",
        "--expected-cohort",
        &output["cohort"].to_string(),
        "--detail",
        "--disclosure",
        "local",
    ])?;
    assert!(!stale.status.success());
    Ok(())
}

// r[verify cairn.selected_work.conformance]
#[test]
fn completion_only_keeps_unavailable_completion_distinct() -> Result<(), Box<dyn Error>> {
    let fixture = temporary_fixture()?;
    fs::write(fixture.0.join(".cairn/changes/demo/tasks.md"),
        "## Work\n- [x] [task:first] r[demo.behavior] Completed.\n- [x] [task:second] a[demo.acceptance] Completed.\n")?;
    let root = fixture.0.to_string_lossy();
    let cli = success(&["selected-work", "demo", "--root", &root])?;
    let mcp = success(&[
        "mcp",
        "--call",
        "cairn_selected_work",
        &json!({"change":"demo", "root":root}).to_string(),
    ])?;
    assert_eq!(cli, mcp["structuredContent"]);
    assert!(cli["units"]
        .as_array()
        .unwrap()
        .iter()
        .all(|unit| unit["state"] == "complete"));
    assert!(!cli["remaining"]
        .as_array()
        .unwrap()
        .iter()
        .any(|entry| entry["kind"] == "implementation"));
    assert!(cli["remaining"]
        .as_array()
        .unwrap()
        .iter()
        .any(|entry| entry["kind"] == "completion" && entry["state"] == "unavailable"));
    Ok(())
}

#[test]
fn duplicate_authored_id_and_incomplete_source_refuse() -> Result<(), Box<dyn Error>> {
    let fixture = temporary_fixture()?;
    let tasks = fixture.0.join(".cairn/changes/demo/tasks.md");
    fs::write(
        &tasks,
        "- [ ] [task:duplicate] First.\n- [ ] [task:duplicate] Second.\n",
    )?;
    let root = fixture.0.to_string_lossy();
    assert!(!command(&["selected-work", "demo", "--root", &root])?
        .status
        .success());
    fs::write(&tasks, "- [ ] [task:unique] One.\n")?;
    let baseline = success(&["selected-work", "demo", "--root", &root])?;
    fs::write(&tasks, "- [ ] [task:unique] One.\n- [ ] [task:next] Two.\n")?;
    assert!(!command(&[
        "selected-work",
        "demo",
        "--root",
        &root,
        "--expected-cohort",
        &baseline["cohort"].to_string()
    ])?
    .status
    .success());
    Ok(())
}

// r[verify cairn.selected_work.contract]
#[test]
fn selected_evidence_bytes_invalidate_subject_binding() -> Result<(), Box<dyn Error>> {
    let fixture = temporary_fixture()?;
    let source = repo().join("fixtures/agent-completion");
    fs::copy(
        source.join("contract.json"),
        fixture.0.join("contract.json"),
    )?;
    fs::copy(source.join("results.json"), fixture.0.join("results.json"))?;
    let root = fixture.0.to_string_lossy();
    let subject = "a".repeat(64);
    let args = [
        "selected-work",
        "demo",
        "--root",
        root.as_ref(),
        "--contract",
        "contract.json",
        "--results",
        "results.json",
        "--subject-hash",
        &subject,
    ];
    let before = success(&args)?;
    let results = fixture.0.join("results.json");
    let mut bytes = fs::read(&results)?;
    bytes.push(b'\n');
    fs::write(&results, bytes)?;
    let after = success(&args)?;
    assert_eq!(
        before["units"].as_array().unwrap().len(),
        after["units"].as_array().unwrap().len()
    );
    assert_ne!(before["cohort"]["source"], after["cohort"]["source"]);
    assert_ne!(
        before["cohort"]["verification_subject"],
        after["cohort"]["verification_subject"]
    );
    Ok(())
}

#[test]
fn missing_completion_evidence_preserves_gate_stages() -> Result<(), Box<dyn Error>> {
    let root = fixture();
    let ordinary = success(&["selected-work", "demo", "--root", &root])?;
    let missing = success(&[
        "selected-work",
        "demo",
        "--root",
        &root,
        "--contract",
        "missing-contract.json",
        "--results",
        "missing-results.json",
        "--subject-hash",
        &"a".repeat(64),
    ])?;
    for stage in 0..4 {
        assert_eq!(
            missing["readiness"]["stages"][stage]["state"],
            ordinary["readiness"]["stages"][stage]["state"]
        );
    }
    assert!(missing["source"]["artifacts"]
        .as_array()
        .unwrap()
        .iter()
        .any(|artifact| artifact["path"] == "missing-results.json"
            && artifact["status"] == "missing"));
    assert_eq!(missing["completion"]["state"], "unavailable");
    Ok(())
}

// r[verify cairn.selected_work.observation]
#[test]
fn oversized_selected_source_cannot_be_reported_complete() -> Result<(), Box<dyn Error>> {
    let fixture = temporary_fixture()?;
    let oversized = fixture.0.join(".cairn/changes/demo/oversized.md");
    fs::write(&oversized, vec![b'x'; 1_048_577])?;
    let root = fixture.0.to_string_lossy();
    let output = command(&["selected-work", "demo", "--root", &root])?;
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    Ok(())
}

fn node_ids(nodes: &Value) -> std::collections::BTreeSet<String> {
    nodes
        .as_array()
        .unwrap()
        .iter()
        .map(|node| node["id"].as_str().unwrap().to_owned())
        .collect()
}

/// Evidence observations bound to one snapshot, using the native requirement
/// revisions carried by its typed obligation facts.
fn bound_evidence(snapshot: &Value, predicates: &[(&str, &str)]) -> Value {
    let export = &snapshot["obligations"]["export"];
    let references = export["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|node| node["evidence"].as_array().unwrap().iter())
        .map(|reference| (reference["predicate"].as_str().unwrap(), reference))
        .collect::<std::collections::BTreeMap<_, _>>();
    Value::Array(
        predicates
            .iter()
            .map(|(predicate, state)| {
                let reference = references[predicate];
                json!({
                    "predicate": predicate,
                    "requirement_id": reference["requirement_id"],
                    "revision_blake3": reference["accepted_revision_blake3"],
                    "subject_blake3": snapshot["cohort"]["verification_subject"],
                    "context_blake3": snapshot["cohort"]["context"],
                    "policy_blake3": snapshot["policy_hash"],
                    "observation_id": format!("check:{predicate}"),
                    "state": state,
                })
            })
            .collect(),
    )
}

// r[verify cairn.selected_work.obligations]
#[test]
fn cli_and_mcp_obligations_match_the_obligation_export() -> Result<(), Box<dyn Error>> {
    let fixture = temporary_fixture()?;
    let root = fixture.0.to_string_lossy().into_owned();
    let planning = success(&[
        "selected-work",
        "demo",
        "--root",
        &root,
        "--obligation-definition",
        "verify",
    ])?;
    let mcp = success(&[
        "mcp",
        "--call",
        "cairn_selected_work",
        &json!({"change":"demo", "root":root, "obligation_definition":"verify"}).to_string(),
    ])?;
    assert_eq!(mcp["structuredContent"], planning);
    assert_eq!(planning["obligations"]["state"], "exported");
    assert_eq!(
        planning["semantic_extensions"]["obligation_explanations"],
        "supported"
    );
    let export = &planning["obligations"]["export"];
    assert_eq!(
        export["predicate"]["subject_blake3"],
        planning["cohort"]["verification_subject"]
    );
    assert_eq!(
        export["predicate"]["context_blake3"],
        planning["cohort"]["context"]
    );
    assert_eq!(export["observations_supplied"], false);
    assert_eq!(export["evaluation"]["outcome"], "unknown");
    assert!(!planning.to_string().contains(&root));

    // Worker guidance, the machine predicate and the obligation export agree.
    let case = fixture.0.join("case.json");
    fs::write(
        &case,
        json!({
            "definition": "verify",
            "subject_blake3": planning["cohort"]["verification_subject"],
            "context_blake3": planning["cohort"]["context"],
        })
        .to_string(),
    )?;
    let exported = success(&["obligation", "export", "--case", &case.to_string_lossy()])?;
    assert_eq!(exported["export"]["predicate"], export["predicate"]);
    assert_eq!(
        node_ids(&exported["export"]["obligations"]),
        node_ids(&export["nodes"])
    );
    for node in export["nodes"].as_array().unwrap() {
        let id = node["id"].as_str().unwrap();
        if node["kind"] == "evidence" {
            assert!(export["guidance"]["sections"][0]["items"]
                .as_array()
                .unwrap()
                .iter()
                .any(|item| item.as_str().unwrap().starts_with(&format!("{id}:"))));
        }
    }
    let mut stale_expected = planning["cohort"].clone();
    stale_expected["obligations"] = json!("0".repeat(64));
    assert!(!command(&[
        "selected-work",
        "demo",
        "--root",
        &root,
        "--obligation-definition",
        "verify",
        "--expected-cohort",
        &stale_expected.to_string(),
    ])?
    .status
    .success());
    // Without a definition the section is explicit and the cohort differs.
    let plain = success(&["selected-work", "demo", "--root", &root])?;
    assert_eq!(plain["obligations"], json!({"state": "not_selected"}));
    assert_ne!(
        plain["cohort"]["obligations"],
        planning["cohort"]["obligations"]
    );
    assert_eq!(
        plain["cohort"]["verification_subject"],
        planning["cohort"]["verification_subject"]
    );
    Ok(())
}

// r[verify cairn.selected_work.obligations]
#[test]
fn supplied_obligation_observations_bind_source_and_never_combine_partials(
) -> Result<(), Box<dyn Error>> {
    let fixture = temporary_fixture()?;
    let root = fixture.0.to_string_lossy().into_owned();
    let planning = success(&[
        "selected-work",
        "demo",
        "--root",
        &root,
        "--obligation-definition",
        "verify",
    ])?;
    let observations = fixture.0.join("observations.json");
    let partial = json!({
        "selected_alternatives": {"verify.traceability": "receipt-chain"},
        "evidence": bound_evidence(&planning, &[
            ("agent-receipt.verification-contract", "satisfied"),
            ("agent-receipt.pass-only", "satisfied"),
            ("agent-receipt.completion-authority", "satisfied"),
            ("verification-obligation.claim-revision", "satisfied"),
        ]),
        "agent_claims": {"success_statement": "all done", "checked_task_boxes": 9, "activity_count": 40},
    });
    fs::write(&observations, partial.to_string())?;
    let args = [
        "selected-work",
        "demo",
        "--root",
        root.as_str(),
        "--obligation-definition",
        "verify",
        "--obligation-observations",
        "observations.json",
    ];
    let first = success(&args)?;
    let evaluation = &first["obligations"]["export"]["evaluation"];
    assert_ne!(evaluation["outcome"], "proved");
    assert_eq!(evaluation["successful_delivery"], false);
    assert!(evaluation["remaining"]
        .as_array()
        .unwrap()
        .iter()
        .any(|remaining| remaining["id"] == "verify.evidence.freshness"));
    assert!(!evaluation["ignored_claims"].as_array().unwrap().is_empty());
    assert!(
        first["source"]["artifacts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|artifact| artifact["path"] == "observations.json"
                && artifact["status"] == "present")
    );
    // Meaning identities survive evidence changes; only the evaluation moves.
    let mut complete = partial.clone();
    complete["evidence"] = bound_evidence(
        &planning,
        &[
            ("agent-receipt.verification-contract", "satisfied"),
            ("agent-receipt.pass-only", "satisfied"),
            ("agent-receipt.freshness", "satisfied"),
            ("agent-receipt.completion-authority", "satisfied"),
            ("verification-scope.applicable", "unsatisfied"),
        ],
    );
    fs::write(&observations, complete.to_string())?;
    let second = success(&args)?;
    let proved = &second["obligations"]["export"];
    assert_eq!(proved["evaluation"]["outcome"], "proved");
    assert_eq!(proved["nodes"], first["obligations"]["export"]["nodes"]);
    assert_ne!(second["cohort"]["source"], first["cohort"]["source"]);
    assert_ne!(
        second["cohort"]["obligations"],
        first["cohort"]["obligations"]
    );
    let stale = command(
        &[
            &args[..],
            &["--expected-cohort", &first["cohort"].to_string()],
        ]
        .concat(),
    )?;
    assert!(!stale.status.success());

    // Explicit refusals: orphan observations, unknown definition, malformed or missing files.
    for refused in [
        vec![
            "selected-work",
            "demo",
            "--root",
            root.as_str(),
            "--obligation-observations",
            "observations.json",
        ],
        vec![
            "selected-work",
            "demo",
            "--root",
            root.as_str(),
            "--obligation-definition",
            "unknown",
        ],
        vec![
            "selected-work",
            "demo",
            "--root",
            root.as_str(),
            "--obligation-definition",
            "verify",
            "--obligation-observations",
            "missing.json",
        ],
    ] {
        assert!(!command(&refused)?.status.success(), "{refused:?}");
    }
    fs::write(&observations, r#"{"evidence": [], "forged": true}"#)?;
    assert!(!command(&args)?.status.success());
    let orphan = command(&[
        "mcp",
        "--call",
        "cairn_selected_work",
        &json!({"change":"demo", "root":root, "obligation_observations":"observations.json"})
            .to_string(),
    ])?;
    assert!(!orphan.status.success());
    Ok(())
}
