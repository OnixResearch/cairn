use super::*;

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
