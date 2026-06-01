use std::{error::Error, path::PathBuf};

use cairn_core::{
    build_archive_plan_with_dependency_blockers, build_sync_plan_with_dependency_blockers,
    classify_gate_evidence, dependency_blockers_for_change, evaluate_gate_snapshot, GateIssue,
    GateStage, PlanAction, PlanActionKind,
};
use serde_json::Value;

use crate::native::change_status_json;
use crate::plan_receipts::{archive_plan_json, gate_input_json, gate_report_json, sync_plan_json};
use crate::probe::gate_probe_issues;
use crate::support::{
    archive_mutation_manifest, default_policy_path, detect_layout, execute_archive_plan,
    execute_sync_plan, file_content_manifest, find_change, has_flag, load_policy_file,
    mutation_gate_blockers, path_string, portable_json_hash, print_json,
    read_change_dependency_graph, read_gate_snapshot, read_option, sync_mutation_manifest,
    sync_mutation_paths, tree_content_paths, with_portable_receipt_hash,
};

pub(crate) fn run_gate(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(stage_text) = iter.next() else {
        return Err("gate requires a stage: proposal, design, or tasks".into());
    };
    let stage = parse_gate_stage(&stage_text)?;
    let Some(change) = iter.next() else {
        return Err("gate requires a change name".into());
    };
    let rest = iter.collect::<Vec<_>>();
    let run_probes_requested = has_flag(&rest, "--run-probes");
    let root = read_option(rest.clone(), "--root", ".");
    let policy_path = read_option(rest, "--policy", default_policy_path());
    let policy = load_policy_file(policy_path)?;
    let selected = detect_layout(&root);
    let change_status = find_change(root.clone(), selected.layout, &change)?;
    let snapshot = read_gate_snapshot(&root, selected.layout, change_status)?;
    let mut report = evaluate_gate_snapshot(&snapshot, stage, &policy);
    if run_probes_requested {
        report.issues.push(GateIssue {
            message: "inline probe execution is disabled by policy".to_string(),
            scope: policy.gate_finding_scope.clone(),
            promoted: policy.gate_finding_promoted.clone(),
        });
    }
    if matches!(stage, GateStage::Tasks) {
        report
            .issues
            .extend(gate_probe_issues(&root, selected.layout, &change, &policy)?);
    }
    let input_hash = portable_json_hash(
        &root,
        &gate_input_json(selected.layout, stage, &policy, &snapshot),
    );
    let mut receipt = gate_report_json(selected.layout, &policy, &report, &input_hash);
    let evidence = classify_gate_evidence(&snapshot);
    receipt["evidence_summary"] = serde_json::json!({
        "mode": "advisory",
        "docs_only": evidence.docs_only,
        "fixture": evidence.fixture,
        "probe": evidence.probe,
        "property": evidence.property,
        "model": evidence.model,
        "formal": evidence.formal,
    });
    print_json(with_portable_receipt_hash(&root, receipt))
}

pub(crate) fn parse_gate_stage(stage: &str) -> Result<GateStage, Box<dyn Error>> {
    match stage {
        "proposal" => Ok(GateStage::Proposal),
        "design" => Ok(GateStage::Design),
        "tasks" => Ok(GateStage::Tasks),
        unknown => Err(format!("unknown gate stage: {unknown}").into()),
    }
}

pub(crate) fn run_change_context(args: Vec<String>, change: &str) -> Result<(), Box<dyn Error>> {
    let root = read_option(args, "--root", ".");
    let selected = detect_layout(&root);
    let change_status = find_change(root, selected.layout, change)?;
    print_json(change_status_json(&change_status))
}

pub(crate) fn run_sync(args: Vec<String>, change: &str) -> Result<(), Box<dyn Error>> {
    let root = read_option(args.clone(), "--root", ".");
    let is_execute = has_flag(&args, "--execute");
    let policy_path = read_option(args.clone(), "--policy", default_policy_path());
    let policy = load_policy_file(policy_path)?;
    let selected = detect_layout(&root);
    let change_status = find_change(root.clone(), selected.layout, change)?;
    let input_hash = portable_json_hash(&root, &change_status_json(&change_status));
    let dependency_graph = read_change_dependency_graph(&root, selected.layout)?;
    let dependency_blockers = dependency_blockers_for_change(&dependency_graph, change);
    let mut plan = build_sync_plan_with_dependency_blockers(&change_status, &dependency_blockers);
    let sync_paths = sync_mutation_paths(&root, selected.layout, change, &plan);
    let before_manifest = file_content_manifest(&root, &sync_paths);
    let mut mutation_manifest = Value::Null;
    if policy.sync_require_gate && !plan.blocked {
        plan.reasons.extend(mutation_gate_blockers(
            &root,
            selected.layout,
            &change_status,
            GateStage::Tasks,
            &policy,
        )?);
        plan.reasons.sort();
        plan.reasons.dedup();
        plan.blocked = !plan.reasons.is_empty();
        if plan.blocked {
            plan.actions.clear();
        }
    }
    if is_execute {
        plan.dry_run = false;
        if !plan.blocked {
            execute_sync_plan(&root, selected.layout, change, &plan)?;
            plan.mutated = true;
            mutation_manifest =
                sync_mutation_manifest(&root, selected.layout, change, &plan, &before_manifest);
        }
    }
    print_json(with_portable_receipt_hash(
        &root,
        sync_plan_json(
            &root,
            selected.layout,
            &policy,
            &plan,
            &input_hash,
            mutation_manifest,
        ),
    ))
}

pub(crate) fn run_archive(args: Vec<String>, change: &str) -> Result<(), Box<dyn Error>> {
    let root = read_option(args.clone(), "--root", ".");
    let is_execute = has_flag(&args, "--execute");
    let policy_path = read_option(args.clone(), "--policy", default_policy_path());
    let policy = load_policy_file(policy_path)?;
    let selected = detect_layout(&root);
    let change_status = find_change(root.clone(), selected.layout, change)?;
    let input_hash = portable_json_hash(&root, &change_status_json(&change_status));
    let dependency_graph = read_change_dependency_graph(&root, selected.layout)?;
    let dependency_blockers = dependency_blockers_for_change(&dependency_graph, change);
    let mut plan =
        build_archive_plan_with_dependency_blockers(&change_status, &dependency_blockers);
    let archive_source = PathBuf::from(&change_status.path);
    let before_paths =
        tree_content_paths(&root, &archive_source).unwrap_or_else(|_| vec![archive_source.clone()]);
    let before_manifest = file_content_manifest(&root, &before_paths);
    let mut mutation_manifest = Value::Null;
    if policy.archive_require_gate && !plan.blocked {
        plan.reasons.extend(mutation_gate_blockers(
            &root,
            selected.layout,
            &change_status,
            GateStage::Tasks,
            &policy,
        )?);
        plan.reasons.sort();
        plan.reasons.dedup();
        plan.blocked = !plan.reasons.is_empty();
        if plan.blocked {
            plan.actions.clear();
        }
    }
    if is_execute {
        plan.dry_run = false;
        if !plan.blocked {
            let archive_path =
                execute_archive_plan(&root, selected.layout, change, &change_status.path)?;
            mutation_manifest =
                archive_mutation_manifest(&root, &archive_source, &archive_path, &before_manifest);
            plan.actions = vec![PlanAction {
                kind: PlanActionKind::ArchiveChange,
                path: path_string(archive_path),
                description: format!("move active change to archive: {change}"),
            }];
            plan.mutated = true;
        }
    }
    print_json(with_portable_receipt_hash(
        &root,
        archive_plan_json(
            &root,
            selected.layout,
            &policy,
            &plan,
            &input_hash,
            mutation_manifest,
        ),
    ))
}
