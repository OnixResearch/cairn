//! Read-only composition. Original validators retain stage authority.
use crate::{
    canonical_json_hash, evaluate_authoritative_agent_completion, GateReport,
    WorkflowArtifactState, WorkflowStatusReport,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const MAX_READINESS_BLOCKERS: usize = 512;
pub const MAX_READINESS_OBSERVATION_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_READINESS_TARGET_BYTES: usize = 4096;
pub const READINESS_STAGES: [&str; 7] = [
    "artifact",
    "proposal",
    "design",
    "tasks",
    "completion",
    "sync",
    "archive",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadinessScope {
    pub change: String,
    pub policy_hash: String,
    pub source_hash: String,
    pub evidence_hash: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessState {
    Ready,
    Blocked,
    Unavailable,
    NotApplicable,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessCause {
    pub code: String,
    pub target: String,
    pub message: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessObservation {
    pub scope: ReadinessScope,
    pub stage: String,
    pub state: ReadinessState,
    pub decision: Value,
    pub causes: Vec<ReadinessCause>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessSnapshot {
    pub scope: ReadinessScope,
    pub observations: Vec<ReadinessObservation>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessBlocker {
    pub id: String,
    pub stage: String,
    pub code: String,
    pub target: String,
    pub message: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessAction {
    pub id: String,
    pub kind: String,
    pub causes: Vec<String>,
    pub target: String,
    pub prerequisites: Vec<String>,
    pub authority: String,
    pub expected_observation: String,
    pub argv: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessStage {
    pub stage: String,
    pub state: ReadinessState,
    pub decision_hash: String,
    pub decision: Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessReport {
    pub schema: String,
    pub scope: ReadinessScope,
    pub valid: bool,
    pub classification: String,
    pub stages: Vec<ReadinessStage>,
    pub blockers: Vec<ReadinessBlocker>,
    pub actions: Vec<ReadinessAction>,
    pub diagnostics: Vec<String>,
    pub readiness_policy_hash: String,
    pub input_hash: String,
    pub receipt_hash: String,
    pub mutation_authorized: bool,
    pub non_claims: Vec<String>,
}

pub fn readiness_cause(code: &str, target: &str, message: &str) -> ReadinessCause {
    ReadinessCause {
        code: code.into(),
        target: target.into(),
        message: message.into(),
    }
}

// r[impl cairn.unified_readiness.decisions]
pub fn readiness_gate(scope: &ReadinessScope, report: &GateReport) -> ReadinessObservation {
    let causes = report
        .issues
        .iter()
        .map(|issue| {
            readiness_cause(
                &issue.code,
                &format!(".cairn/changes/{}/{}.md", scope.change, report.stage),
                &issue.message,
            )
        })
        .collect::<Vec<_>>();
    ReadinessObservation {
        scope: scope.clone(),
        stage: report.stage.clone(),
        state: if causes.is_empty() {
            ReadinessState::Ready
        } else {
            ReadinessState::Blocked
        },
        decision: json!({"change": report.change, "stage": report.stage, "verdict": report.verdict,
            "issues": causes}),
        causes,
    }
}

pub fn readiness_artifacts(
    scope: &ReadinessScope,
    report: &WorkflowStatusReport,
) -> ReadinessObservation {
    let mut causes = report
        .artifacts
        .iter()
        .filter(|a| {
            !matches!(
                a.state,
                WorkflowArtifactState::Done | WorkflowArtifactState::Skipped
            )
        })
        .map(|a| {
            readiness_cause(
                "artifact.incomplete",
                &format!(".cairn/changes/{}/{}", scope.change, a.output_path),
                &format!("{} is {}", a.id, a.state.as_str()),
            )
        })
        .collect::<Vec<_>>();
    causes.extend(
        report
            .issues
            .iter()
            .map(|issue| readiness_cause("workflow.invalid", &scope.change, issue)),
    );
    ReadinessObservation {
        scope: scope.clone(),
        stage: "artifact".into(),
        state: if causes.is_empty() && report.valid {
            ReadinessState::Ready
        } else {
            ReadinessState::Blocked
        },
        decision: json!(report),
        causes,
    }
}

pub fn readiness_profile_causes(
    scope: &ReadinessScope,
    report: &mut ReadinessObservation,
    evaluation: &crate::WorkflowProfileEvaluation,
    schema: &crate::WorkflowSchemaPolicy,
    evidence: &crate::WorkflowProfileEvidence,
) {
    for obligation in &schema.review_obligations {
        if !evaluation.issues.iter().any(|issue| {
            issue.code == "cairn.workflow.profile.review"
                && issue.artifact_id.as_ref() == Some(&obligation.artifact_id)
        }) {
            continue;
        }
        let reviewed = evidence
            .reviews
            .iter()
            .find(|r| r.obligation_id == obligation.id);
        for artifact in evidence
            .artifact_hashes
            .iter()
            .filter(|a| obligation.required_artifacts.contains(&a.artifact_id))
        {
            let current = reviewed.is_some_and(|review| {
                review
                    .reviewed_inputs
                    .iter()
                    .any(|input| input.path == artifact.path && input.hash == artifact.blake3)
            });
            if !current {
                report.causes.push(readiness_cause(
                    "cairn.workflow.profile.review",
                    &artifact.path,
                    &format!(
                        "Review {} must bind current artifact {}",
                        obligation.id, artifact.blake3
                    ),
                ));
            }
        }
    }
    for issue in &evaluation.issues {
        let target = issue
            .artifact_id
            .as_ref()
            .and_then(|id| schema.artifacts.iter().find(|a| &a.id == id))
            .map(|artifact| format!(".cairn/changes/{}/{}", scope.change, artifact.output_path))
            .unwrap_or_else(|| scope.change.clone());
        report
            .causes
            .push(readiness_cause(&issue.code, &target, &issue.message));
    }
    if !report.causes.is_empty() {
        report.state = ReadinessState::Blocked;
    }
    report.decision["profile_evaluation"] = json!(evaluation);
}

pub fn readiness_completion(
    scope: &ReadinessScope,
    contract: &Value,
    results: &Value,
    subject: &str,
) -> ReadinessObservation {
    let report =
        evaluate_authoritative_agent_completion(contract, results, subject, &scope.policy_hash);
    let causes = report
        .diagnostics
        .iter()
        .map(|d| {
            readiness_cause(
                &d.code,
                d.check_id.as_deref().unwrap_or("verification-contract"),
                &d.summary,
            )
        })
        .collect();
    ReadinessObservation {
        scope: scope.clone(),
        stage: "completion".into(),
        state: if report.valid && report.verdict == "PASS" {
            ReadinessState::Ready
        } else {
            ReadinessState::Blocked
        },
        decision: json!(report),
        causes,
    }
}

pub struct ReadinessMutationInputs<'a> {
    pub scope: &'a ReadinessScope,
    pub sync: &'a crate::SyncPlan,
    pub archive: &'a crate::ArchivePlan,
    pub merge_binding: Value,
    pub merge_reasons: &'a [String],
    pub gate_reasons: &'a [String],
    pub policy: &'a crate::Policy,
    pub dependencies: &'a crate::ChangeDependencyGraph,
}

pub fn readiness_mutation_observations(
    input: ReadinessMutationInputs<'_>,
) -> Vec<ReadinessObservation> {
    let mut sync_reasons = input.sync.reasons.clone();
    sync_reasons.extend_from_slice(input.merge_reasons);
    if input.policy.sync_require_gate {
        sync_reasons.extend_from_slice(input.gate_reasons);
    }
    let mut archive_reasons = input.archive.reasons.clone();
    if input.policy.archive_require_gate {
        archive_reasons.extend_from_slice(input.gate_reasons);
    }
    [("sync", sync_reasons), ("archive", archive_reasons)]
        .into_iter()
        .map(|(stage, mut reasons)| {
            reasons.sort();
            reasons.dedup();
            let mut observation = readiness_plan(
                input.scope,
                stage,
                &reasons,
                json!({"reasons": reasons, "merge_preflight": input.merge_binding}),
            );
            readiness_dependency_causes(&mut observation, input.dependencies);
            observation
        })
        .collect()
}

pub fn readiness_dependency_causes(
    observation: &mut ReadinessObservation,
    graph: &crate::ChangeDependencyGraph,
) {
    if let Some(node) = graph.node(&observation.scope.change) {
        for dependency in &node.blocked_by {
            observation.causes.push(readiness_cause(
                "dependency.blocked",
                dependency,
                "The upstream change must satisfy the existing dependency validator",
            ));
        }
    }
    if !observation.causes.is_empty() {
        observation.state = ReadinessState::Blocked;
    }
}

pub fn readiness_plan(
    scope: &ReadinessScope,
    stage: &str,
    reasons: &[String],
    decision: Value,
) -> ReadinessObservation {
    ReadinessObservation {
        scope: scope.clone(),
        stage: stage.into(),
        state: if reasons.is_empty() {
            ReadinessState::Ready
        } else {
            ReadinessState::Blocked
        },
        decision,
        causes: reasons
            .iter()
            .map(|reason| readiness_cause("plan.blocked", &scope.change, reason))
            .collect(),
    }
}

// r[impl cairn.unified_readiness.snapshot]
// r[impl cairn.unified_readiness.boundary]
pub fn derive_readiness(snapshot: &ReadinessSnapshot) -> ReadinessReport {
    let policy: Value = serde_json::from_str(include_str!(
        "../../../../cairn-policy/generated/readiness.json"
    ))
    .expect("checked readiness policy");
    let mut diagnostics = Vec::new();
    if snapshot.observations.len() > READINESS_STAGES.len() {
        diagnostics.push("observation.bound".into());
    }
    let input = json!(snapshot);
    if input.to_string().len() > MAX_READINESS_OBSERVATION_BYTES {
        diagnostics.push("observation.byte_bound".into());
    }
    if crate::validate_change_name(&snapshot.scope.change).is_err() {
        diagnostics.push("scope.change".into());
    }
    for hash in [
        &snapshot.scope.policy_hash,
        &snapshot.scope.source_hash,
        &snapshot.scope.evidence_hash,
    ] {
        if hash.len() != 64 || !hash.bytes().all(|b| b.is_ascii_hexdigit()) {
            diagnostics.push("scope.hash".into());
        }
    }
    let mut stages = Vec::new();
    let mut blockers = Vec::new();
    for stage in READINESS_STAGES {
        let observations = snapshot
            .observations
            .iter()
            .filter(|o| o.stage == stage)
            .collect::<Vec<_>>();
        let mut state = ReadinessState::Unavailable;
        let mut decision = Value::Null;
        if observations.len() == 1 {
            let observation = observations[0];
            state = observation.state;
            decision = observation.decision.clone();
            if observation.stage == "completion"
                && state == ReadinessState::Ready
                && (decision["valid"] != true || decision["verdict"] != "PASS")
            {
                diagnostics.push("completion: observation.contradictory".into());
                state = ReadinessState::Unavailable;
            }
            if decision
                .get("change")
                .and_then(Value::as_str)
                .is_some_and(|change| change != snapshot.scope.change)
            {
                diagnostics.push(format!("{stage}: decision.change_conflict"));
                state = ReadinessState::Unavailable;
            }
            if observation.scope != snapshot.scope {
                diagnostics.push(format!("{stage}: identity.conflict"));
                state = ReadinessState::Unavailable;
            }
            if observation.causes.len() > MAX_READINESS_BLOCKERS {
                diagnostics.push(format!("{stage}: blocker.bound"));
            }
            if matches!(state, ReadinessState::Ready | ReadinessState::NotApplicable)
                && !observation.causes.is_empty()
            {
                diagnostics.push(format!("{stage}: observation.contradictory"));
                state = ReadinessState::Unavailable;
            }
            for cause in observation.causes.iter().take(MAX_READINESS_BLOCKERS) {
                if !safe_target(&cause.target) {
                    diagnostics.push(format!("{stage}: target.unsafe"));
                    continue;
                }
                let id = canonical_json_hash(&json!([snapshot.scope, stage, cause]));
                blockers.push(ReadinessBlocker {
                    id,
                    stage: stage.into(),
                    code: cause.code.clone(),
                    target: cause.target.clone(),
                    message: cause.message.clone(),
                });
            }
        } else if observations.len() > 1 {
            diagnostics.push(format!("{stage}: observation.duplicate"));
        }
        stages.push(ReadinessStage {
            stage: stage.into(),
            state,
            decision_hash: canonical_json_hash(&decision),
            decision,
        });
    }
    if snapshot
        .observations
        .iter()
        .any(|o| !READINESS_STAGES.contains(&o.stage.as_str()))
    {
        diagnostics.push("observation.unknown".into());
    }
    if blockers.len() > MAX_READINESS_BLOCKERS {
        diagnostics.push("blocker.total_bound".into());
        blockers.truncate(MAX_READINESS_BLOCKERS);
    }
    diagnostics.sort();
    diagnostics.dedup();
    blockers.sort_by(|a, b| {
        (&a.stage, &a.code, &a.target, &a.id).cmp(&(&b.stage, &b.code, &b.target, &b.id))
    });
    blockers.dedup();
    // Missing or failed completion never becomes authority for a dependent preview.
    let completion = stages
        .iter()
        .find(|s| s.stage == "completion")
        .map(|s| s.state);
    if !matches!(
        completion,
        Some(ReadinessState::Ready | ReadinessState::NotApplicable)
    ) {
        for stage in stages
            .iter_mut()
            .filter(|s| matches!(s.stage.as_str(), "sync" | "archive"))
        {
            if stage.state == ReadinessState::Ready {
                stage.state = ReadinessState::Blocked;
            }
        }
    }
    if !diagnostics.is_empty() {
        for stage in &mut stages {
            stage.state = ReadinessState::Unavailable;
        }
    }
    let mut actions = blockers
        .iter()
        .map(|b| action_for(b, &snapshot.scope, &policy))
        .collect::<Vec<_>>();
    for stage in stages
        .iter()
        .filter(|s| s.state == ReadinessState::Unavailable)
    {
        let cause = ReadinessBlocker {
            id: canonical_json_hash(&json!([snapshot.scope, stage.stage, "unavailable"])),
            stage: stage.stage.clone(),
            code: "observation.unavailable".into(),
            target: snapshot.scope.change.clone(),
            message: "Observation is unavailable".into(),
        };
        if blockers.len() < MAX_READINESS_BLOCKERS {
            actions.push(action_for(&cause, &snapshot.scope, &policy));
            blockers.push(cause);
        }
    }
    for stage in stages.iter().filter(|s| {
        s.state == ReadinessState::Ready && matches!(s.stage.as_str(), "sync" | "archive")
    }) {
        let kind = format!("{}_preview", stage.stage);
        actions.push(ReadinessAction {
            id: canonical_json_hash(&json!([snapshot.scope, kind])),
            kind,
            causes: Vec::new(),
            target: snapshot.scope.change.clone(),
            prerequisites: vec![stage.decision_hash.clone()],
            authority: "advisory".into(),
            expected_observation: "fresh existing mutation preflight".into(),
            argv: vec![
                "cairn".into(),
                stage.stage.clone(),
                snapshot.scope.change.clone(),
            ],
        });
    }
    let actions = merge_actions(actions);
    let valid = diagnostics.is_empty();
    let classification = if stages
        .iter()
        .any(|s| s.state == ReadinessState::Unavailable)
    {
        "unavailable"
    } else if stages.iter().any(|s| s.state == ReadinessState::Blocked) {
        "blocked"
    } else {
        "ready"
    };
    let mut report = ReadinessReport {
        schema: "cairn.readiness.v1".into(),
        scope: snapshot.scope.clone(),
        valid,
        classification: classification.into(),
        stages,
        blockers,
        actions,
        diagnostics,
        readiness_policy_hash: canonical_json_hash(&policy),
        input_hash: canonical_json_hash(&input),
        receipt_hash: String::new(),
        mutation_authorized: false,
        non_claims: vec![
            "Readiness does not authorize mutation or execute checks".into(),
            "Readiness does not prove evidence truth, code correctness, or release eligibility"
                .into(),
        ],
    };
    report.receipt_hash = canonical_json_hash(&json!(report));
    report
}

fn safe_target(target: &str) -> bool {
    !target.is_empty()
        && target.len() <= MAX_READINESS_TARGET_BYTES
        && !target.starts_with('/')
        && !target.contains('\\')
        && !target.chars().any(char::is_control)
        && !target.split('/').any(|part| part == "..")
}

fn merge_actions(actions: Vec<ReadinessAction>) -> Vec<ReadinessAction> {
    let mut merged = std::collections::BTreeMap::<String, ReadinessAction>::new();
    for action in actions {
        if let Some(existing) = merged.get_mut(&action.id) {
            existing.causes.extend(action.causes);
            existing.causes.sort();
            existing.causes.dedup();
        } else {
            merged.insert(action.id.clone(), action);
        }
    }
    merged.into_values().collect()
}

// r[impl cairn.unified_readiness.actions]
fn action_for(
    blocker: &ReadinessBlocker,
    scope: &ReadinessScope,
    policy: &Value,
) -> ReadinessAction {
    let kind = policy["rules"]
        .as_array()
        .and_then(|rules| rules.iter().find(|r| r["code"] == blocker.code))
        .and_then(|rule| rule["kind"].as_str())
        .unwrap_or("unresolved");
    let expected = match kind {
        "artifact_repair" => "current structurally valid artifact",
        "evidence_refresh" => "current passing evidence bound to the selected input",
        "dependency_wait" => "upstream dependency accepted by its existing validator",
        _ => "reviewed remediation rule for this exact diagnostic",
    };
    ReadinessAction {
        id: canonical_json_hash(&json!([
            scope,
            kind,
            blocker.code,
            blocker.target,
            // Unknown diagnostics have no reviewed equivalence rule.
            if kind == "unresolved" {
                Some(&blocker.message)
            } else {
                None
            }
        ])),
        kind: kind.into(),
        causes: vec![blocker.id.clone()],
        target: blocker.target.clone(),
        prerequisites: vec![
            scope.source_hash.clone(),
            scope.policy_hash.clone(),
            scope.evidence_hash.clone(),
        ],
        authority: "advisory".into(),
        expected_observation: expected.into(),
        argv: Vec::new(),
    }
}

#[cfg(test)]
mod tests;
