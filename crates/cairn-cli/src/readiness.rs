//! Bounded read-only observation adapter. No producer or mutation commands run here.
use crate::support::{
    default_policy_path, detect_layout, load_policy_file_with_budget,
    load_sync_merge_snapshots_with_budget, mutation_gate_observation_with_budget,
    portable_json_hash, print_json, read_change_dependency_graph_with_budget,
    read_gate_snapshot_with_budget, read_selected_change_with_budget, sync_merge_bindings,
    sync_merge_blockers, CapabilityRoot, CapabilityRootKind,
};
use cairn_core::verified::readiness::*;
use cairn_core::{
    build_archive_plan_for_effect_with_dependency_blockers,
    build_sync_plan_for_effect_with_dependency_blockers, canonical_json_hash,
    dependency_blockers_for_change, finalize_gate_report, GateStage,
};
use serde_json::{json, Value};
use std::{
    error::Error,
    path::{Path, PathBuf},
};

pub(crate) fn run_readiness(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let change = args.first().ok_or("readiness requires a change")?;
    let mut input = json!({"change": change});
    let mut format = "json";
    let mut pairs = args[1..].chunks_exact(2);
    for pair in &mut pairs {
        let key = match pair[0].as_str() {
            "--root" => "root",
            "--policy" => "policy",
            "--contract" => "contract",
            "--results" => "results",
            "--subject-hash" => "subject_hash",
            "--cross-repo-evidence" => {
                if input.get("cross_repo_evidence").is_none() {
                    input["cross_repo_evidence"] = json!([]);
                }
                input["cross_repo_evidence"]
                    .as_array_mut()
                    .expect("array initialized")
                    .push(json!(pair[1]));
                continue;
            }
            "--format" => {
                format = &pair[1];
                continue;
            }
            _ => return Err("unsupported readiness option (inspection cannot execute)".into()),
        };
        if input.get(key).is_some() {
            return Err("duplicate readiness option".into());
        }
        input[key] = json!(pair[1]);
    }
    if !pairs.remainder().is_empty() || !matches!(format, "json" | "summary") {
        return Err("invalid readiness options".into());
    }
    let value = readiness_value(&input)?;
    if format == "json" {
        return print_json(value);
    }
    println!(
        "{}: {}",
        change,
        value["classification"].as_str().unwrap_or("unavailable")
    );
    if let Some(stages) = value["stages"].as_array() {
        for stage in stages {
            println!(
                "  {}: {}",
                stage["stage"].as_str().unwrap_or("unknown"),
                stage["state"].as_str().unwrap_or("unavailable")
            );
        }
    }
    if let Some(actions) = value["actions"].as_array() {
        for action in actions {
            println!(
                "  {}: {}",
                action["kind"].as_str().unwrap_or("unresolved"),
                action["target"].as_str().unwrap_or("unknown")
            );
        }
    }
    Ok(())
}

// r[impl cairn.unified_readiness.transport]
// r[impl cairn.unified_readiness.boundary]
pub(crate) fn readiness_value(input: &Value) -> Result<Value, Box<dyn Error>> {
    let change = input["change"]
        .as_str()
        .ok_or("readiness requires change")?;
    cairn_core::validate_change_name(change)?;
    let root = PathBuf::from(input["root"].as_str().unwrap_or("."));
    let policy_path = PathBuf::from(input["policy"].as_str().unwrap_or(default_policy_path()));
    let scope = ReadinessScope {
        change: change.into(),
        policy_hash: canonical_json_hash(&Value::Null),
        source_hash: canonical_json_hash(&Value::Null),
        evidence_hash: canonical_json_hash(input),
    };
    let snapshot = match gather(&root, &policy_path, change, input) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            let error_class = error
                .downcast_ref::<std::io::Error>()
                .map(|error| format!("{:?}", error.kind()))
                .unwrap_or_else(|| "ObservationAdmission".into());
            ReadinessSnapshot {
                observations: vec![ReadinessObservation {
                    scope: scope.clone(), stage: "artifact".into(), state: ReadinessState::Unavailable,
                    decision: json!({"evaluated": false, "error_class": error_class}),
                    causes: vec![readiness_cause("observation.load_failed", change, "An observation could not be admitted. Inspect the selected inputs with the original validator.")],
                }], scope,
            }
        }
    };
    Ok(serde_json::to_value(derive_readiness(&snapshot))?)
}

fn gather(
    root: &Path,
    policy_path: &Path,
    change: &str,
    input: &Value,
) -> Result<ReadinessSnapshot, Box<dyn Error>> {
    let budget = crate::support::observation_budget::ObservationBudget::new();
    gather_observation_cohort(|| gather_once(root, policy_path, change, input, &budget))
}

fn gather_observation_cohort(
    mut observe: impl FnMut() -> Result<ReadinessSnapshot, Box<dyn Error>>,
) -> Result<ReadinessSnapshot, Box<dyn Error>> {
    let mut first = observe()?;
    let current = observe()?;
    reject_cohort_drift(&mut first, &current);
    Ok(first)
}

fn reject_cohort_drift(first: &mut ReadinessSnapshot, current: &ReadinessSnapshot) {
    if first != current {
        for observation in &mut first.observations {
            observation.scope.evidence_hash =
                canonical_json_hash(&json!("observation cohort changed"));
        }
    }
}

fn unavailable(scope: &ReadinessScope, stage: &str) -> ReadinessObservation {
    ReadinessObservation {
        scope: scope.clone(),
        stage: stage.into(),
        state: ReadinessState::Unavailable,
        decision: json!({"evaluated": false, "error_class": "ObservationAdmission"}),
        causes: vec![readiness_cause(
            "observation.load_failed",
            &scope.change,
            "The selected observation could not be admitted",
        )],
    }
}

fn gather_once(
    root: &Path,
    policy_path: &Path,
    change: &str,
    input: &Value,
    budget: &crate::support::observation_budget::ObservationBudget,
) -> Result<ReadinessSnapshot, Box<dyn Error>> {
    let policy = load_policy_file_with_budget(policy_path.to_path_buf(), Some(budget))?;
    let layout = detect_layout(root).layout;
    let completion = load_completion_with_budget(root, input, Some(budget));
    let status = read_selected_change_with_budget(root, layout, change, Some(budget));
    let profile = crate::workflow::load_workflow_profile_context_with_budget(
        root,
        layout,
        change,
        &policy,
        Some(budget),
    );
    let external = crate::cross_repo_dependencies::load_readiness_cross_repo_evidence_with_budget(
        root,
        input,
        Some(budget),
    );
    let policy_hash = canonical_json_hash(&policy.raw);
    let cross = status
        .as_ref()
        .ok()
        .zip(external.as_ref().ok())
        .map(|(status, evidence)| {
            crate::cross_repo_dependencies::evaluate_change_cross_repo_dependencies(
                status,
                evidence,
                &policy_hash,
            )
        });
    let gate_snapshot = status
        .as_ref()
        .ok()
        .map(|status| read_gate_snapshot_with_budget(root, layout, status.clone(), Some(budget)));
    let (status, gate_snapshot, profile) = match (status, gate_snapshot, profile) {
        (Ok(status), Some(Ok(snapshot)), Ok(profile)) => (status, snapshot, profile),
        (_, _, profile) => {
            let scope = ReadinessScope {
                change: change.into(),
                policy_hash: canonical_json_hash(&policy.raw),
                source_hash: canonical_json_hash(&json!({"available": false})),
                evidence_hash: canonical_json_hash(&json!({
                    "completion": completion.as_ref().ok(),
                    "completion_unavailable": completion.is_err(),
                    "cross_repo": cross.as_ref().map(|cross| cross.to_json()),
                    "external_unavailable": external.is_err(),
                    "profile": profile.as_ref().ok().map(|profile| &profile.profile_identity_hash),
                })),
            };
            let mut artifacts = match &profile {
                Ok(profile) => {
                    let mut artifacts = readiness_artifacts(&scope, &profile.report);
                    readiness_profile_causes(
                        &scope,
                        &mut artifacts,
                        &profile.evaluation,
                        &profile.schema,
                        &profile.evidence,
                    );
                    artifacts
                }
                Err(_) => unavailable(&scope, "artifact"),
            };
            artifacts.decision["cross_repo_dependencies"] = cross
                .as_ref()
                .map(|cross| cross.to_json())
                .unwrap_or(Value::Null);
            let mut observations = vec![artifacts];
            observations.extend(gather_partial_gates(
                root,
                layout,
                &scope,
                &policy,
                profile.as_ref().ok(),
                budget,
            ));
            if let Ok(Some(completion)) = completion {
                observations.push(readiness_completion(
                    &scope,
                    &completion.contract,
                    &completion.results,
                    &completion.subject,
                ));
            } else {
                observations.push(unavailable(&scope, "completion"));
            }
            let mut scope = scope;
            scope.source_hash = portable_json_hash(
                root,
                &json!(observations
                    .iter()
                    .filter(|observation| matches!(
                        observation.stage.as_str(),
                        "proposal" | "design" | "tasks"
                    ))
                    .map(|observation| &observation.decision["source_input_hash"])
                    .collect::<Vec<_>>()),
            );
            for observation in &mut observations {
                observation.scope = scope.clone();
            }
            return Ok(ReadinessSnapshot {
                scope,
                observations,
            });
        }
    };
    let graph = read_change_dependency_graph_with_budget(root, layout, Some(budget));
    let probes =
        crate::probe::observe_gate_probes_with_budget(root, layout, change, &policy, Some(budget));

    let mut dependency_blockers = graph
        .as_ref()
        .map(|graph| dependency_blockers_for_change(graph, change))
        .unwrap_or_default();
    if let Some(cross) = &cross {
        dependency_blockers.extend(cairn_core::dependency_blocker_messages(cross));
    }
    let profile_blockers = if profile.selection.compatibility_fallback {
        Vec::new()
    } else {
        profile.completion_blockers()
    };
    let source_input =
        crate::plan_receipts::gate_input_json(layout, GateStage::Tasks, &policy, &gate_snapshot);
    let scope = ReadinessScope {
        change: change.into(),
        policy_hash,
        source_hash: portable_json_hash(root, &source_input),
        evidence_hash: portable_json_hash(
            root,
            &json!({
                "profile": profile.profile_identity_hash, "probe": probes.binding, "dependencies": dependency_blockers,
                "dependency_graph": graph.as_ref().ok().map(|graph| json!({
                    "nodes": graph.nodes.iter().map(crate::native::change_dependency_node_json).collect::<Vec<_>>(),
                    "edges": graph.edges.iter().map(crate::native::change_dependency_edge_json).collect::<Vec<_>>(),
                    "issues": graph.issues.iter().map(crate::native::change_dependency_issue_json).collect::<Vec<_>>(),
                    "drain_order": graph.drain_order,
                })),
                "completion": completion.as_ref().ok(), "completion_unavailable": completion.is_err(),
                "cross_repo": cross.as_ref().map(|cross| cross.to_json()),
                "external": external.as_ref().ok().map(|evidence| evidence.iter().map(|summary| json!({
                    "repo": summary.repo, "change": summary.change, "receipt_hash": summary.receipt_hash,
                    "policy_hash": summary.policy_hash, "subject_hash": summary.subject_hash,
                    "status": summary.status, "receipt_kind": summary.receipt_kind,
                    "artifact_ref": summary.artifact_ref, "diagnostic_summary": summary.diagnostic_summary,
                })).collect::<Vec<_>>()),
                "external_unavailable": external.is_err(),
            }),
        ),
    };
    let mut artifacts = readiness_artifacts(&scope, &profile.report);
    readiness_profile_causes(
        &scope,
        &mut artifacts,
        &profile.evaluation,
        &profile.schema,
        &profile.evidence,
    );
    let mut observations = vec![artifacts];
    for stage in [GateStage::Proposal, GateStage::Design, GateStage::Tasks] {
        let report = profile.gate_report(&gate_snapshot, stage, &policy);
        let report = finalize_gate_report(
            report,
            if stage == GateStage::Tasks {
                probes.issues.clone()
            } else {
                Vec::new()
            },
        );
        observations.push(readiness_gate_with_external_causes(
            &scope,
            &report,
            if stage == GateStage::Tasks {
                &probes.readiness_causes
            } else {
                &[]
            },
        ));
    }
    if let Ok(Some(completion)) = &completion {
        observations.push(readiness_completion(
            &scope,
            &completion.contract,
            &completion.results,
            &completion.subject,
        ));
    }
    if completion.is_err() {
        observations.push(unavailable(&scope, "completion"));
    }
    let mutations = (|| -> Result<Vec<ReadinessObservation>, Box<dyn Error>> {
        let graph = graph?;
        external?;
        let mut plan_blockers = dependency_blockers.clone();
        plan_blockers.extend(profile_blockers);
        let sync = build_sync_plan_for_effect_with_dependency_blockers(
            &status,
            &plan_blockers,
            &profile.schema.spec_effect,
        );
        let archive = build_archive_plan_for_effect_with_dependency_blockers(
            &status,
            &plan_blockers,
            &profile.schema.spec_effect,
        );
        let merges =
            load_sync_merge_snapshots_with_budget(root, layout, change, &sync, Some(budget));
        let merge_reasons = merges
            .as_ref()
            .map(|merges| sync_merge_blockers(root, merges))
            .unwrap_or_default();
        let merge_binding = merges
            .as_ref()
            .ok()
            .map(|merges| sync_merge_bindings(root, merges));
        let gate = if policy.sync_require_gate || policy.archive_require_gate {
            Some(mutation_gate_observation_with_budget(
                root,
                layout,
                &status,
                GateStage::Tasks,
                &policy,
                Some(budget),
            ))
        } else {
            None
        };
        let gate_reasons = gate
            .as_ref()
            .and_then(|gate| gate.as_ref().ok())
            .map(|gate| gate.0.as_slice())
            .unwrap_or_default();
        let gate_binding = gate
            .as_ref()
            .and_then(|gate| gate.as_ref().ok())
            .map(|gate| &gate.1);
        let gate_unavailable = gate.as_ref().is_some_and(|gate| gate.is_err());
        let mut mutations = readiness_mutation_observations(ReadinessMutationInputs {
            scope: &scope,
            sync: &sync,
            archive: &archive,
            merge_binding,
            merge_reasons: &merge_reasons,
            gate_reasons,
            policy: &policy,
            dependencies: &graph,
        });
        for observation in &mut mutations {
            let requires_gate = if observation.stage == "sync" {
                policy.sync_require_gate
            } else {
                policy.archive_require_gate
            };
            if requires_gate && gate_unavailable {
                observation.state = ReadinessState::Unavailable;
                observation.causes.push(readiness_cause(
                    "observation.load_failed",
                    change,
                    "The required mutation gate observation could not be admitted",
                ));
            }
            if requires_gate {
                observation.decision["validation_input_hash"] = gate_binding
                    .map(|binding| json!(portable_json_hash(root, binding)))
                    .unwrap_or(Value::Null);
            }
            observation.decision["gate_required"] = json!(requires_gate);
            observation.decision["cross_repo_dependencies"] = cross
                .as_ref()
                .map(|cross| cross.to_json())
                .unwrap_or(Value::Null);
        }
        Ok(mutations)
    })();
    match mutations {
        Ok(mutations) => observations.extend(mutations),
        Err(_) => {
            for stage in ["sync", "archive"] {
                let mut observation = unavailable(&scope, stage);
                observation.decision["cross_repo_dependencies"] = cross
                    .as_ref()
                    .map(|cross| cross.to_json())
                    .unwrap_or(Value::Null);
                observations.push(observation);
            }
        }
    }
    let mut scope = scope;
    scope.evidence_hash = portable_json_hash(
        root,
        &json!({
            "evidence": scope.evidence_hash,
            "decisions": observations.iter().map(|o| &o.decision).collect::<Vec<_>>()
        }),
    );
    for observation in &mut observations {
        observation.scope = scope.clone();
    }
    Ok(ReadinessSnapshot {
        scope,
        observations,
    })
}

fn gather_partial_gates(
    root: &Path,
    layout: cairn_core::LifecycleLayout,
    scope: &ReadinessScope,
    policy: &cairn_core::Policy,
    profile: Option<&crate::workflow::WorkflowProfileContext>,
    budget: &crate::support::observation_budget::ObservationBudget,
) -> Vec<ReadinessObservation> {
    let compatibility = profile.is_none()
        && crate::workflow::compatibility_selection_with_budget(
            root,
            layout,
            &scope.change,
            policy,
            Some(budget),
        )
        .unwrap_or(false);
    [GateStage::Proposal, GateStage::Design, GateStage::Tasks]
        .into_iter()
        .map(|stage| {
            if profile.is_none() && !compatibility {
                return unavailable(scope, stage.as_str());
            }
            let snapshot = match crate::support::read_gate_stage_snapshot_with_budget(
                root,
                layout,
                &scope.change,
                stage,
                Some(budget),
            ) {
                Ok(snapshot) => snapshot,
                Err(_) => return unavailable(scope, stage.as_str()),
            };
            let report = profile.map_or_else(
                || cairn_core::evaluate_gate_snapshot(&snapshot, stage, policy),
                |profile| profile.gate_report(&snapshot, stage, policy),
            );
            let probes = (stage == GateStage::Tasks).then(|| {
                crate::probe::observe_gate_probes_with_budget(
                    root,
                    layout,
                    &scope.change,
                    policy,
                    Some(budget),
                )
            });
            let report = finalize_gate_report(
                report,
                probes
                    .as_ref()
                    .map(|probe| probe.issues.clone())
                    .unwrap_or_default(),
            );
            let mut observation = readiness_gate_with_external_causes(
                scope,
                &report,
                probes
                    .as_ref()
                    .map(|probe| probe.readiness_causes.as_slice())
                    .unwrap_or_default(),
            );
            observation.decision["source_input_hash"] = json!(portable_json_hash(
                root,
                &crate::plan_receipts::gate_input_json(layout, stage, policy, &snapshot)
            ));
            observation
        })
        .collect()
}

#[cfg(test)]
#[path = "readiness_tests.rs"]
mod tests;

#[derive(serde::Serialize)]
struct CompletionInputs {
    contract: Value,
    results: Value,
    subject: String,
}

#[cfg(test)]
fn load_completion(root: &Path, input: &Value) -> Result<Option<CompletionInputs>, Box<dyn Error>> {
    load_completion_with_budget(root, input, None)
}

fn load_completion_with_budget(
    root: &Path,
    input: &Value,
    budget: Option<&crate::support::observation_budget::ObservationBudget>,
) -> Result<Option<CompletionInputs>, Box<dyn Error>> {
    match (
        input["contract"].as_str(),
        input["results"].as_str(),
        input["subject_hash"].as_str(),
    ) {
        (None, None, None) => Ok(None),
        (Some(contract), Some(results), Some(subject)) => {
            let capability =
                CapabilityRoot::open_nofollow(CapabilityRootKind::Scan, root)?.with_budget(budget);
            let contract = capability
                .read_bounded(Path::new(contract), MAX_READINESS_OBSERVATION_BYTES as u64)?;
            let remaining = MAX_READINESS_OBSERVATION_BYTES
                .checked_sub(contract.len())
                .ok_or("completion aggregate byte bound")?;
            let results = capability.read_bounded(Path::new(results), remaining as u64)?;
            Ok(Some(CompletionInputs {
                contract: serde_json::from_slice(&contract)?,
                results: serde_json::from_slice(&results)?,
                subject: subject.into(),
            }))
        }
        _ => Err("contract, results, and subject-hash must be supplied together".into()),
    }
}
