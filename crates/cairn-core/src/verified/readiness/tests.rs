use super::*;

#[test]
fn current_and_stale_reviews_select_exact_artifact_repairs() {
    let policy: Value = serde_json::from_str(include_str!(
        "../../../../../cairn-policy/generated/cairn-policy.json"
    ))
    .unwrap();
    let schema: crate::WorkflowSchemaPolicy = serde_json::from_value(
        policy["schemas"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["id"] == "assurance")
            .unwrap()
            .clone(),
    )
    .unwrap();
    let mut evidence = crate::WorkflowProfileEvidence {
        spec_effect: Some("delta".into()),
        spec_effect_rationale: None,
        delta_spec_paths: vec!["specs/demo/spec.md".into()],
        acceptance_markdown: None,
        tasks_markdown: None,
        outcome_machine_review: None,
        outcome_machine_evidence: Vec::new(),
        reviews: Vec::new(),
        artifact_hashes: schema
            .artifacts
            .iter()
            .map(|a| crate::WorkflowProfileArtifactHash {
                artifact_id: a.id.clone(),
                path: format!(".cairn/changes/demo/{}", a.output_path),
                blake3: "a".repeat(64),
            })
            .collect(),
    };
    evidence.reviews = schema
        .review_obligations
        .iter()
        .map(|obligation| crate::WorkflowReviewEvidence {
            obligation_id: obligation.id.clone(),
            valid: true,
            review_kind: obligation.review_kind.clone(),
            decision: "approved".into(),
            receipt_hash: "b".repeat(64),
            reviewed_inputs: evidence
                .artifact_hashes
                .iter()
                .filter(|a| obligation.required_artifacts.contains(&a.artifact_id))
                .map(|a| crate::WorkflowReviewInputEvidence {
                    path: a.path.clone(),
                    hash: a.blake3.clone(),
                })
                .collect(),
        })
        .collect();
    let mut s = snapshot();
    let evaluation = crate::evaluate_workflow_profile(&schema, &evidence);
    readiness_profile_causes(
        &s.scope,
        &mut s.observations[0],
        &evaluation,
        &schema,
        &evidence,
    );
    assert!(!s.observations[0]
        .causes
        .iter()
        .any(|c| c.code == "cairn.workflow.profile.review"));
    let changed = evidence.reviews[0].reviewed_inputs[0].path.clone();
    evidence.reviews[0].reviewed_inputs[0].hash = "d".repeat(64);
    let evaluation = crate::evaluate_workflow_profile(&schema, &evidence);
    readiness_profile_causes(
        &s.scope,
        &mut s.observations[0],
        &evaluation,
        &schema,
        &evidence,
    );
    let report = derive_readiness(&s);
    assert!(report
        .actions
        .iter()
        .any(|a| a.kind == "evidence_refresh" && a.target == changed));
}

#[test]
fn generated_policy_bounds_match_enforced_limits() {
    let policy: Value = serde_json::from_str(include_str!(
        "../../../../../cairn-policy/generated/readiness.json"
    ))
    .unwrap();
    assert_eq!(policy["max_blockers"], MAX_READINESS_BLOCKERS);
    assert_eq!(
        policy["max_observation_bytes"],
        MAX_READINESS_OBSERVATION_BYTES
    );
    assert_eq!(policy["max_target_bytes"], MAX_READINESS_TARGET_BYTES);
}

#[test]
fn causal_actions_merge_stages_but_not_distinct_repairs() {
    let mut s = snapshot();
    for stage in [0, 1] {
        s.observations[stage].state = ReadinessState::Blocked;
        s.observations[stage].causes = vec![readiness_cause(
            "artifact.incomplete",
            "tasks.md",
            "missing tasks",
        )];
    }
    let report = derive_readiness(&s);
    let repairs = report
        .actions
        .iter()
        .filter(|a| a.kind == "artifact_repair")
        .collect::<Vec<_>>();
    assert_eq!(repairs.len(), 1);
    assert_eq!(repairs[0].causes.len(), 2);
    assert!(repairs[0]
        .causes
        .iter()
        .all(|id| report.blockers.iter().any(|b| &b.id == id)));
    s.observations[1].causes[0].target = "design.md".into();
    assert_eq!(
        derive_readiness(&s)
            .actions
            .iter()
            .filter(|a| a.kind == "artifact_repair")
            .count(),
        2
    );
    for stage in [0, 1] {
        s.observations[stage].causes = vec![readiness_cause(
            "unknown",
            "demo",
            &format!("cause {stage}"),
        )];
    }
    let report = derive_readiness(&s);
    assert_eq!(
        report
            .actions
            .iter()
            .filter(|a| a.kind == "unresolved")
            .count(),
        2
    );
    s.observations.reverse();
    assert_eq!(report.actions, derive_readiness(&s).actions);
}

fn snapshot() -> ReadinessSnapshot {
    let scope = ReadinessScope {
        change: "demo".into(),
        policy_hash: "a".repeat(64),
        source_hash: "b".repeat(64),
        evidence_hash: "c".repeat(64),
    };
    let observations = READINESS_STAGES
        .iter()
        .map(|stage| ReadinessObservation {
            scope: scope.clone(),
            stage: (*stage).into(),
            state: ReadinessState::Ready,
            decision: json!({"valid": true, "verdict": "PASS"}),
            causes: Vec::new(),
        })
        .collect();
    ReadinessSnapshot {
        scope,
        observations,
    }
}

#[test]
// r[verify cairn.unified_readiness.snapshot]
// r[verify cairn.unified_readiness.conformance]
fn equal_inputs_are_deterministic_and_conflicts_fail_closed() {
    let mut s = snapshot();
    let first = derive_readiness(&s);
    assert_eq!(first, derive_readiness(&s));
    assert_eq!(first.classification, "ready");
    assert!(first.actions.iter().any(|a| a.kind == "sync_preview"));
    s.observations[0].scope.source_hash = "d".repeat(64);
    let rejected = derive_readiness(&s);
    assert!(!rejected.valid);
    assert!(rejected
        .stages
        .iter()
        .all(|s| s.state == ReadinessState::Unavailable));
    assert!(!rejected.actions.iter().any(|a| a.kind == "sync_preview"));
}

#[test]
// r[verify cairn.unified_readiness.actions]
fn repair_targets_and_unknown_causes_remain_advisory() {
    let mut s = snapshot();
    s.observations[0].state = ReadinessState::Blocked;
    s.observations[0].causes = vec![
        readiness_cause(
            "artifact.incomplete",
            ".cairn/changes/demo/tasks.md",
            "missing tasks",
        ),
        readiness_cause("unknown", "demo", "unknown cause"),
    ];
    let report = derive_readiness(&s);
    assert!(report.valid);
    assert_eq!(report.classification, "blocked");
    assert!(report
        .actions
        .iter()
        .any(|a| a.kind == "artifact_repair" && a.target.ends_with("tasks.md")));
    assert!(report
        .actions
        .iter()
        .any(|a| a.kind == "unresolved" && a.argv.is_empty()));
    assert!(report.actions.iter().all(|a| a.authority == "advisory"));
    assert!(!report.mutation_authorized);
}

#[test]
// r[verify cairn.unified_readiness.decisions]
fn authoritative_class_does_not_override_stale_subject_or_missing_rows() {
    let contract: Value = serde_json::from_str(include_str!(
        "../../../../../fixtures/agent-completion/contract.json"
    ))
    .unwrap();
    let results: Value = serde_json::from_str(include_str!(
        "../../../../../fixtures/agent-completion/results.json"
    ))
    .unwrap();
    let mut s = snapshot();
    s.scope.policy_hash = contract["policy_hash"].as_str().unwrap().into();
    for o in &mut s.observations {
        o.scope = s.scope.clone();
    }
    let passing = readiness_completion(&s.scope, &contract, &results, &"c".repeat(64));
    assert_eq!(passing.state, ReadinessState::Ready);
    let rejected = readiness_completion(&s.scope, &contract, &results, &"d".repeat(64));
    assert_eq!(rejected.decision["classification"], "authoritative");
    assert_eq!(rejected.decision["valid"], false);
    let mut contradictory = rejected.clone();
    contradictory.state = ReadinessState::Ready;
    contradictory.causes.clear();
    s.observations[4] = contradictory;
    assert!(!derive_readiness(&s).valid);
    s.observations[4] = rejected;
    let report = derive_readiness(&s);
    assert_eq!(report.stages[4].state, ReadinessState::Blocked);
    assert_eq!(report.stages[5].state, ReadinessState::Blocked);
    assert_eq!(report.stages[6].state, ReadinessState::Blocked);
    assert!(!report.actions.iter().any(|a| a.kind == "sync_preview"));
    assert_eq!(
        readiness_completion(&s.scope, &contract, &json!([]), &"c".repeat(64)).state,
        ReadinessState::Blocked
    );
}

#[test]
// r[verify cairn.unified_readiness.boundary]
fn unavailable_unsafe_duplicate_and_overbound_observations_fail_closed() {
    let mut s = snapshot();
    s.observations.remove(4);
    assert_eq!(
        derive_readiness(&s).stages[4].state,
        ReadinessState::Unavailable
    );
    s.observations.push(s.observations[0].clone());
    assert!(!derive_readiness(&s).valid);
    let mut s = snapshot();
    s.observations[0].state = ReadinessState::Blocked;
    s.observations[0].causes = vec![readiness_cause("artifact.incomplete", "../escape", "bad")];
    assert!(!derive_readiness(&s).valid);
    s.observations[0].causes =
        vec![readiness_cause("unknown", "demo", "bad"); MAX_READINESS_BLOCKERS + 1];
    assert!(!derive_readiness(&s).valid);
}
