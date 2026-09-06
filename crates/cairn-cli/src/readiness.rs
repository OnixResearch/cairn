//! Bounded read-only observation adapter. No producer or mutation commands run here.
use crate::support::{
    default_policy_path, detect_layout, find_change, load_policy_file, load_sync_merge_snapshots,
    mutation_gate_blockers, portable_json_hash, print_json, read_change_dependency_graph,
    read_gate_snapshot, sync_merge_bindings, sync_merge_blockers, CapabilityRoot,
    CapabilityRootKind,
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
    let policy = load_policy_file(policy_path.to_path_buf())?;
    let layout = detect_layout(root).layout;
    let status = find_change(root.to_path_buf(), layout, change)?;
    let gate_snapshot = read_gate_snapshot(root, layout, status.clone())?;
    let profile = crate::workflow::load_workflow_profile_context(root, layout, change, &policy)?;
    let graph = read_change_dependency_graph(root, layout)?;
    let probes = crate::probe::observe_gate_probes(root, layout, change, &policy);
    let policy_hash = canonical_json_hash(&policy.raw);
    let cross = crate::cross_repo_dependencies::evaluate_change_cross_repo_dependencies(
        &status,
        &[],
        &policy_hash,
    );
    let mut dependency_blockers = dependency_blockers_for_change(&graph, change);
    dependency_blockers.extend(cairn_core::dependency_blocker_messages(&cross));
    let profile_blockers = if profile.selection.compatibility_fallback {
        Vec::new()
    } else {
        profile.completion_blockers()
    };
    let completion = load_completion(root, input)?;
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
                "completion": completion, "cross_repo": cross.to_json(),
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
        observations.push(readiness_gate(&scope, &report));
    }
    if let Some(completion) = &completion {
        observations.push(readiness_completion(
            &scope,
            &completion.contract,
            &completion.results,
            &completion.subject,
        ));
    }
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
    let merges = load_sync_merge_snapshots(root, layout, change, &sync)?;
    let merge_reasons = sync_merge_blockers(root, &merges);
    let gate_reasons = mutation_gate_blockers(root, layout, &status, GateStage::Tasks, &policy)?;
    observations.extend(readiness_mutation_observations(ReadinessMutationInputs {
        scope: &scope,
        sync: &sync,
        archive: &archive,
        merge_binding: json!(sync_merge_bindings(root, &merges)),
        merge_reasons: &merge_reasons,
        gate_reasons: &gate_reasons,
        policy: &policy,
        dependencies: &graph,
    }));
    // A second bounded source/profile observation detects drift during composition.
    let current_status = find_change(root.to_path_buf(), layout, change)?;
    let current = read_gate_snapshot(root, layout, current_status)?;
    let current_profile =
        crate::workflow::load_workflow_profile_context(root, layout, change, &policy)?;
    if scope.source_hash
        != portable_json_hash(
            root,
            &crate::plan_receipts::gate_input_json(layout, GateStage::Tasks, &policy, &current),
        )
        || profile.profile_identity_hash != current_profile.profile_identity_hash
    {
        observations[0].scope.source_hash = canonical_json_hash(&json!("source changed"));
    }
    Ok(ReadinessSnapshot {
        scope,
        observations,
    })
}

#[derive(serde::Serialize)]
struct CompletionInputs {
    contract: Value,
    results: Value,
    subject: String,
}

fn load_completion(root: &Path, input: &Value) -> Result<Option<CompletionInputs>, Box<dyn Error>> {
    match (
        input["contract"].as_str(),
        input["results"].as_str(),
        input["subject_hash"].as_str(),
    ) {
        (None, None, None) => Ok(None),
        (Some(contract), Some(results), Some(subject)) => {
            let capability = CapabilityRoot::open(CapabilityRootKind::Scan, root)?;
            let contract = capability
                .read_bounded(Path::new(contract), MAX_READINESS_OBSERVATION_BYTES as u64)?;
            let results = capability
                .read_bounded(Path::new(results), MAX_READINESS_OBSERVATION_BYTES as u64)?;
            Ok(Some(CompletionInputs {
                contract: serde_json::from_slice(&contract)?,
                results: serde_json::from_slice(&results)?,
                subject: subject.into(),
            }))
        }
        _ => Err("contract, results, and subject-hash must be supplied together".into()),
    }
}
