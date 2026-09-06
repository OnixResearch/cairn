use super::*;
use crate::support::{find_change, load_sync_merge_snapshots};

fn lifecycle_fixture(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let root = std::env::temp_dir().join(format!("cairn-readiness-{name}-{}", std::process::id()));
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    for path in [
        ".cairn/specs/demo/spec.md",
        ".cairn/changes/demo/proposal.md",
        ".cairn/changes/demo/design.md",
        ".cairn/changes/demo/tasks.md",
        ".cairn/changes/demo/specs/demo/spec.md",
    ] {
        let destination = root.join(path);
        std::fs::create_dir_all(destination.parent().ok_or("missing parent")?)?;
        std::fs::copy(repository.join("fixtures/basic").join(path), destination)?;
    }
    std::fs::create_dir_all(root.join(".cairn/archive"))?;
    Ok(root)
}

fn readiness_input(root: &Path) -> Value {
    json!({"root": root, "change": "demo", "policy":
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../cairn-policy/generated/cairn-policy.json")})
}

// r[verify cairn.unified_readiness.boundary]
#[test]
fn report_budget_is_shared_across_readers_and_both_observation_passes() -> Result<(), Box<dyn Error>>
{
    use crate::support::observation_budget::{
        ObservationBudget, MAX_REPORT_BYTES, MAX_REPORT_ENTRIES,
    };
    let root = lifecycle_fixture("report-budget")?;
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    for file in ["contract.json", "results.json"] {
        std::fs::copy(
            repository.join("fixtures/agent-completion").join(file),
            root.join(file),
        )?;
    }
    let matrix: Value = serde_json::from_slice(&std::fs::read(
        repository.join("fixtures/cross-repo-dependencies/matrix.json"),
    )?)?;
    std::fs::write(
        root.join("upstream.json"),
        serde_json::to_vec(&matrix["positive"][0]["evidence"])?,
    )?;
    let probes = root.join(".cairn/changes/demo/probes/probes.json");
    std::fs::create_dir_all(probes.parent().ok_or("parent")?)?;
    std::fs::write(probes, b"{\"probes\":[]}")?;
    let mut input = readiness_input(&root);
    input["contract"] = json!("contract.json");
    input["results"] = json!("results.json");
    input["subject_hash"] = json!("c".repeat(64));
    input["cross_repo_evidence"] = json!(["upstream.json"]);
    let policy = PathBuf::from(input["policy"].as_str().ok_or("policy")?);
    let measured = ObservationBudget::new();
    let first = gather_once(&root, &policy, "demo", &input, &measured)?;
    let bytes = MAX_REPORT_BYTES - measured.read_limit(MAX_REPORT_BYTES);
    assert!(bytes > std::fs::metadata(&policy)?.len());
    let exact = ObservationBudget::with_limits(bytes * 2, MAX_REPORT_ENTRIES);
    let stable = gather_observation_cohort(|| gather_once(&root, &policy, "demo", &input, &exact))?;
    assert_eq!(stable, first);
    assert_eq!(exact.read_limit(MAX_REPORT_BYTES), 0);
    let short = ObservationBudget::with_limits(bytes * 2 - 1, MAX_REPORT_ENTRIES);
    let rejected =
        gather_observation_cohort(|| gather_once(&root, &policy, "demo", &input, &short))?;
    assert!(!derive_readiness(&rejected).valid);
    assert!(!derive_readiness(&rejected).mutation_authorized);
    std::fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn shared_capability_budget_keeps_per_reader_limits_and_counts_directory_entries(
) -> Result<(), Box<dyn Error>> {
    use crate::support::observation_budget::ObservationBudget;
    const FILE_BYTES: u64 = 4;
    const FILE_COUNT: usize = 2;
    let root = std::env::temp_dir().join(format!("cairn-shared-budget-{}", std::process::id()));
    std::fs::create_dir_all(&root)?;
    std::fs::write(root.join("a"), b"null")?;
    std::fs::write(root.join("b"), b"null")?;
    let budget = ObservationBudget::with_limits(FILE_BYTES * FILE_COUNT as u64, FILE_COUNT);
    let first =
        CapabilityRoot::open_nofollow(CapabilityRootKind::Scan, &root)?.with_budget(Some(&budget));
    let second =
        CapabilityRoot::open_nofollow(CapabilityRootKind::Scan, &root)?.with_budget(Some(&budget));
    assert!(first.read_bounded(Path::new("a"), FILE_BYTES - 1).is_err());
    assert_eq!(first.read_bounded(Path::new("a"), FILE_BYTES)?, b"null");
    assert_eq!(second.read_bounded(Path::new("b"), FILE_BYTES)?, b"null");
    assert!(first.read_bounded(Path::new("a"), FILE_BYTES).is_err());
    assert_eq!(
        first.entries_bounded(Path::new("."), FILE_COUNT)?.len(),
        FILE_COUNT
    );
    assert!(second.entries_bounded(Path::new("."), FILE_COUNT).is_err());
    std::fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn gate_observation_failure_only_disables_stages_that_require_the_gate(
) -> Result<(), Box<dyn Error>> {
    for (sync_gate, archive_gate) in [(true, false), (false, true), (false, false)] {
        let root = lifecycle_fixture(&format!("gate-isolation-{sync_gate}-{archive_gate}"))?;
        let mut input = readiness_input(&root);
        let policy_path = PathBuf::from(input["policy"].as_str().ok_or("policy")?);
        let mut policy: Value = serde_json::from_slice(&std::fs::read(policy_path)?)?;
        policy["mutation_policy"]["sync"]["require_passing_gate"] = json!(sync_gate);
        policy["mutation_policy"]["archive"]["require_passing_gate"] = json!(archive_gate);
        let custom = root.join("policy.json");
        std::fs::write(&custom, serde_json::to_vec(&policy)?)?;
        input["policy"] = json!(custom);
        let accepted = root.join(".cairn/specs/unrelated/spec.md");
        std::fs::create_dir_all(accepted.parent().ok_or("parent")?)?;
        std::fs::write(accepted, [0xff])?;
        let report = readiness_value(&input)?;
        for (index, required) in [(5, sync_gate), (6, archive_gate)] {
            assert_eq!(
                report["stages"][index]["state"] == "unavailable",
                required,
                "{report}"
            );
        }
        std::fs::remove_dir_all(root)?;
    }
    Ok(())
}

#[test]
fn accepted_sources_outside_merge_targets_bind_exact_validation_bytes() -> Result<(), Box<dyn Error>>
{
    let root = lifecycle_fixture("accepted-binding")?;
    let accepted = root.join(".cairn/specs/unrelated/spec.md");
    std::fs::create_dir_all(accepted.parent().ok_or("missing parent")?)?;
    let text = std::fs::read_to_string(root.join(".cairn/specs/demo/spec.md"))?;
    std::fs::write(&accepted, &text)?;
    let input = readiness_input(&root);
    let before = readiness_value(&input)?;
    assert_eq!(before["valid"], true);
    assert!(before["stages"][5]["decision"]["validation_input_hash"].is_string());
    assert_eq!(before, readiness_value(&input)?);
    // A semantically inert edit must change the binding even if validators agree.
    std::fs::write(&accepted, format!("{text}\n"))?;
    let after = readiness_value(&input)?;
    assert_eq!(
        before["scope"]["source_hash"],
        after["scope"]["source_hash"]
    );
    assert_eq!(before["stages"][5]["state"], after["stages"][5]["state"]);
    assert_ne!(
        before["stages"][5]["decision"]["validation_input_hash"],
        after["stages"][5]["decision"]["validation_input_hash"]
    );
    assert_ne!(
        before["scope"]["evidence_hash"],
        after["scope"]["evidence_hash"]
    );
    assert_ne!(before["receipt_hash"], after["receipt_hash"]);
    std::fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn unrelated_source_failure_preserves_selected_artifact_observation() -> Result<(), Box<dyn Error>>
{
    let root = lifecycle_fixture("independent-artifact")?;
    let input = readiness_input(&root);
    let before = readiness_value(&input)?;
    let unrelated = root.join(".cairn/changes/unrelated/tasks.md");
    std::fs::create_dir_all(unrelated.parent().ok_or("parent")?)?;
    std::fs::write(unrelated, [0xff])?;
    let after = readiness_value(&input)?;
    assert_eq!(after["stages"][0]["state"], before["stages"][0]["state"]);
    assert_eq!(
        after["stages"][0]["decision"]["artifacts"],
        before["stages"][0]["decision"]["artifacts"]
    );
    for stage in 1..4 {
        assert_eq!(
            after["stages"][stage]["decision"],
            before["stages"][stage]["decision"]
        );
        assert_eq!(
            after["stages"][stage]["state"],
            before["stages"][stage]["state"]
        );
    }
    assert_eq!(after["stages"][5]["state"], "unavailable");
    assert_eq!(
        after["stages"][5]["decision"]["cross_repo_dependencies"]["valid"],
        true
    );
    assert_eq!(after["mutation_authorized"], false);
    std::fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn unreadable_later_artifacts_preserve_independent_earlier_gate_decisions(
) -> Result<(), Box<dyn Error>> {
    for (artifact, first_unavailable) in [("design.md", 2), ("tasks.md", 3)] {
        let root = lifecycle_fixture(&format!("stage-local-{artifact}"))?;
        let input = readiness_input(&root);
        let before = readiness_value(&input)?;
        std::fs::write(root.join(".cairn/changes/demo").join(artifact), [0xff])?;
        let after = readiness_value(&input)?;
        for stage in 1..first_unavailable {
            assert_eq!(
                after["stages"][stage]["state"],
                before["stages"][stage]["state"]
            );
            assert_eq!(
                after["stages"][stage]["decision"]["verdict"],
                before["stages"][stage]["decision"]["verdict"]
            );
            assert_eq!(
                after["stages"][stage]["decision"]["issues"],
                before["stages"][stage]["decision"]["issues"]
            );
            assert!(after["stages"][stage]["decision"]["source_input_hash"].is_string());
        }
        for stage in first_unavailable..4 {
            assert_eq!(after["stages"][stage]["state"], "unavailable");
        }
        assert_eq!(after["mutation_authorized"], false);
        std::fs::remove_dir_all(root)?;
    }
    Ok(())
}

#[test]
fn lifecycle_failure_preserves_the_independent_completion_decision() -> Result<(), Box<dyn Error>> {
    for failure in ["source", "profile"] {
        let root = lifecycle_fixture(&format!("partial-{failure}"))?;
        let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        for file in ["contract.json", "results.json"] {
            std::fs::copy(
                repository.join("fixtures/agent-completion").join(file),
                root.join(file),
            )?;
        }
        let mut input = readiness_input(&root);
        input["contract"] = json!("contract.json");
        input["results"] = json!("results.json");
        input["subject_hash"] = json!("c".repeat(64));
        let before = readiness_value(&input)?;
        assert!(before["stages"][4]["decision"]["verdict"].is_string());
        if failure == "source" {
            std::fs::write(root.join(".cairn/changes/demo/tasks.md"), [0xff])?;
        } else {
            std::fs::write(root.join(".cairn/changes/demo/metadata.json"), b"{")?;
        }
        let after = readiness_value(&input)?;
        assert_eq!(after["classification"], "unavailable");
        assert_eq!(
            after["stages"][4]["decision"],
            before["stages"][4]["decision"]
        );
        assert_eq!(after["stages"][4]["state"], before["stages"][4]["state"]);
        if failure == "profile" {
            assert_eq!(
                after["stages"][0]["decision"]["cross_repo_dependencies"]["valid"],
                true
            );
        }
        assert_eq!(after["mutation_authorized"], false);
        std::fs::remove_dir_all(root)?;
    }
    Ok(())
}

#[cfg(unix)]
#[test]
fn sync_admission_rejects_dangling_accepted_links_without_effects() -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::symlink;
    let root = lifecycle_fixture("sync-links")?;
    let layout = cairn_core::LifecycleLayout::Native;
    let status = find_change(root.clone(), layout, "demo")?;
    let plan = build_sync_plan_for_effect_with_dependency_blockers(&status, &[], "delta");
    assert!(!load_sync_merge_snapshots(&root, layout, "demo", &plan)?.is_empty());
    let accepted = root.join(".cairn/specs/demo/spec.md");
    std::fs::remove_file(&accepted)?;
    // A missing destination is admitted, but a dangling destination is not absence.
    assert!(load_sync_merge_snapshots(&root, layout, "demo", &plan).is_ok());
    symlink("missing", &accepted)?;
    assert!(load_sync_merge_snapshots(&root, layout, "demo", &plan).is_err());
    assert!(std::fs::symlink_metadata(&accepted)?
        .file_type()
        .is_symlink());
    assert!(!root.join(".cairn/specs/demo/missing").exists());
    std::fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn completion_aggregate_budget_accepts_exact_limit_and_rejects_one_more_byte(
) -> Result<(), Box<dyn Error>> {
    let root = std::env::temp_dir().join(format!("cairn-completion-budget-{}", std::process::id()));
    std::fs::create_dir_all(&root)?;
    let mut contract = vec![b' '; MAX_READINESS_OBSERVATION_BYTES - 4];
    contract[..4].copy_from_slice(b"null");
    std::fs::write(root.join("contract.json"), contract)?;
    std::fs::write(root.join("results.json"), b"null")?;
    let input = json!({"contract": "contract.json", "results": "results.json", "subject_hash": "c".repeat(64)});
    assert!(load_completion(&root, &input)?.is_some());
    std::fs::write(root.join("results.json"), b"null ")?;
    assert!(load_completion(&root, &input).is_err());
    std::fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn cohort_rechecks_every_stage_decision_and_scope() -> Result<(), Box<dyn Error>> {
    let scope = ReadinessScope {
        change: "demo".into(),
        policy_hash: "a".repeat(64),
        source_hash: "b".repeat(64),
        evidence_hash: "c".repeat(64),
    };
    let snapshot = ReadinessSnapshot {
        observations: READINESS_STAGES
            .iter()
            .map(|stage| ReadinessObservation {
                scope: scope.clone(),
                stage: (*stage).into(),
                state: ReadinessState::Ready,
                decision: json!({"valid": true, "verdict": "PASS"}),
                causes: vec![],
            })
            .collect(),
        scope,
    };
    let stable = gather_observation_cohort(|| Ok(snapshot.clone()))?;
    assert!(derive_readiness(&stable).valid);
    for stage in 0..READINESS_STAGES.len() {
        let mut calls = 0;
        let changed = gather_observation_cohort(|| {
            calls += 1;
            let mut observation = snapshot.clone();
            if calls == 2 {
                observation.observations[stage].decision["valid"] = json!(false);
            }
            Ok(observation)
        })?;
        assert_eq!(calls, 2);
        let report = derive_readiness(&changed);
        assert!(!report.valid, "stage {stage}");
        assert!(report
            .stages
            .iter()
            .all(|stage| stage.state == ReadinessState::Unavailable));
        assert!(!report
            .actions
            .iter()
            .any(|action| action.kind.ends_with("_preview")));
    }
    for field in ["policy", "source", "evidence"] {
        let mut current = snapshot.clone();
        match field {
            "policy" => current.scope.policy_hash = "d".repeat(64),
            "source" => current.scope.source_hash = "d".repeat(64),
            _ => current.scope.evidence_hash = "d".repeat(64),
        }
        let mut first = snapshot.clone();
        reject_cohort_drift(&mut first, &current);
        assert!(!derive_readiness(&first).valid);
    }
    Ok(())
}
