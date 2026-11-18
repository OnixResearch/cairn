//! Readiness-backed drain projection and its direct CLI route.
//!
//! The shell reads explicit selectors and admits them against the checked
//! Nickel declaration. The pure core orders admitted changes and selects next
//! actions. Nothing here discovers evidence, infers subjects, executes an
//! action, or mutates lifecycle state.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::path::{Path, PathBuf};

use cairn_core::verified::readiness::{
    select_drain_readiness, workflow_readiness_policy, DrainReadinessChange, DrainTaskOrdering,
    WorkflowReadinessPolicy,
};
use cairn_core::{
    build_change_dependency_graph, build_task_ordering_graph, layout_name, ChangeStatus,
};
use serde::Deserialize;
use serde_json::{json, Map, Value};

use crate::native::{
    change_dependency_edge_json, change_dependency_issue_json, change_dependency_node_json,
};
use crate::readiness::readiness_report_with_budget;
use crate::support::observation_budget::ObservationBudget;
use crate::support::{
    collect_archived_change_ids_with_budget, default_policy_path, detect_layout,
    is_safe_relative_reference, load_policy_file_with_budget, path_string, print_json, read_option,
    read_repeated_path_options, read_string_option, reject_positional_arguments,
    reject_unknown_options_with_repeats, scan_repo_with_budget, with_portable_receipt_hash,
    CapabilityRoot, CapabilityRootKind,
};

const MANIFEST_UNAVAILABLE: &str = "workflow_readiness.manifest.unavailable";
const MANIFEST_INVALID: &str = "workflow_readiness.manifest.invalid";
const MANIFEST_SCHEMA: &str = "workflow_readiness.manifest.schema";
const MANIFEST_BOUND: &str = "workflow_readiness.manifest.bound";
const MANIFEST_UNKNOWN_CHANGE: &str = "workflow_readiness.manifest.unknown_change";
const MANIFEST_DUPLICATE_CHANGE: &str = "workflow_readiness.manifest.duplicate_change";
const MANIFEST_PARTIAL_COMPLETION: &str = "workflow_readiness.manifest.partial_completion";
const MANIFEST_UNSAFE_PATH: &str = "workflow_readiness.manifest.unsafe_path";
const MANIFEST_INVALID_SUBJECT: &str = "workflow_readiness.manifest.invalid_subject";
const MANIFEST_DUPLICATE_EVIDENCE: &str = "workflow_readiness.manifest.duplicate_evidence";
const MAX_MANIFEST_DIAGNOSTIC_BYTES: usize = 512;
const TASKS_FILE_BYTES: u64 = 1_048_576;

// r[impl cairn.agent_workflow_readiness.compatibility]
pub(crate) fn run_drain_plan(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    reject_unknown_options_with_repeats(
        &args,
        &[
            "--root",
            "--policy",
            "--view",
            "--evidence-manifest",
            "--cross-repo-evidence",
        ],
        &[],
        &["--cross-repo-evidence"],
    )?;
    reject_positional_arguments(&args)?;
    let root = read_option(args.clone(), "--root", ".");
    let mut input = json!({"root": path_string(&root)});
    if let Some(policy) = read_string_option(&args, "--policy") {
        input["policy"] = json!(policy);
    }
    let evidence = read_repeated_path_options(&args, "--cross-repo-evidence");
    if !evidence.is_empty() {
        let paths = evidence
            .iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<_>>();
        input["cross_repo_evidence"] = json!(paths);
    }
    let view = read_string_option(&args, "--view");
    let manifest = read_string_option(&args, "--evidence-manifest");
    if let Some(view) = &view {
        let policy = workflow_readiness_policy().map_err(|code| {
            format!("{code}: generated workflow readiness policy is not admitted")
        })?;
        if !policy.views.iter().any(|candidate| candidate == view) {
            return Err(format!(
                "unsupported drain plan view: {view}; known views: {}",
                policy.views.join(", ")
            )
            .into());
        }
        input["view"] = json!(view);
        if view == &policy.readiness_view && !evidence.is_empty() {
            return Err(
                "readiness view selects per-change cross-repository evidence from --evidence-manifest"
                    .into(),
            );
        }
        if view != &policy.readiness_view && manifest.is_some() {
            return Err("--evidence-manifest requires the readiness view".into());
        }
    } else if manifest.is_some() {
        return Err("--evidence-manifest requires the readiness view".into());
    }
    if let Some(manifest) = &manifest {
        input["evidence_manifest"] = json!(manifest);
    }
    print_json(crate::mcp::drain_plan(&input, "cairn drain-plan")?)
}

// r[impl cairn.agent_workflow_readiness.compatibility]
// r[impl cairn.agent_workflow_readiness.shared]
pub(crate) fn readiness_drain_value(input: &Value, command: &str) -> Result<Value, Box<dyn Error>> {
    readiness_drain_with_budget(input, command, &ObservationBudget::new())
}

fn readiness_drain_with_budget(
    input: &Value,
    command: &str,
    budget: &ObservationBudget,
) -> Result<Value, Box<dyn Error>> {
    let policy_handle = workflow_readiness_policy()
        .map_err(|code| format!("{code}: generated workflow readiness policy is not admitted"))?;
    if let Some(view) = input.get("view") {
        let view = view.as_str().ok_or("drain plan view must be a string")?;
        if view != policy_handle.readiness_view {
            return Err(format!("unsupported drain plan view: {view}").into());
        }
    }
    let root = input_root(input);
    let policy_path = input_policy(input);
    let selected = detect_layout(&root);
    let policy = load_policy_file_with_budget(policy_path.clone(), Some(budget))?;
    let status = scan_repo_with_budget(root.clone(), selected.layout, Some(budget))?;
    let archived_changes =
        collect_archived_change_ids_with_budget(&root, selected.layout, Some(budget))?;
    let graph = build_change_dependency_graph(&status.changes, &archived_changes);
    let manifest = match input.get("evidence_manifest") {
        Some(value) => {
            let path = value
                .as_str()
                .ok_or("evidence_manifest must be a repository-relative path")?;
            Some(load_evidence_manifest(
                &root,
                path,
                &status.changes,
                policy_handle,
                budget,
            )?)
        }
        None => None,
    };

    let mut observed = Vec::with_capacity(status.changes.len());
    for change in &status.changes {
        let evidence = manifest
            .as_ref()
            .and_then(|manifest| manifest.get(&change.name));
        let change_input = change_observation_input(input, &change.name, evidence);
        let report =
            readiness_report_with_budget(&root, &policy_path, &change.name, &change_input, budget);
        // The per-change task-marker read has its own fixed byte cap; it must not be
        // starved by the aggregate readiness budget already consumed for other changes.
        let task_order =
            observe_task_order(&root, &policy, &change.path, &ObservationBudget::new());
        observed.push(ObservedChange {
            name: change.name.clone(),
            path: change.path.clone(),
            report,
            task_ordering: task_order,
        });
    }
    let dependency_issue_counts =
        graph
            .issues
            .iter()
            .fold(BTreeMap::<&str, usize>::new(), |mut counts, issue| {
                *counts.entry(issue.change.as_str()).or_default() += 1;
                counts
            });
    let admitted = observed
        .iter()
        .map(|change| DrainReadinessChange {
            name: &change.name,
            report: &change.report,
            blocked_by: graph
                .node(&change.name)
                .map(|node| node.blocked_by.as_slice())
                .unwrap_or(&[]),
            dependency_issue_count: dependency_issue_counts
                .get(change.name.as_str())
                .copied()
                .unwrap_or(0),
            task_ordering: DrainTaskOrdering {
                available: change.task_ordering.available,
                active: change.task_ordering.active,
                valid: change.task_ordering.valid,
                ready_tasks: change.task_ordering.ready_tasks.as_slice(),
            },
        })
        .collect::<Vec<_>>();
    let selection = select_drain_readiness(&admitted, &graph, policy_handle)?;

    let by_name = observed
        .iter()
        .map(|change| (change.name.as_str(), change))
        .collect::<BTreeMap<_, _>>();
    let mut changes = Vec::with_capacity(selection.changes.len());
    let mut next_actions = Vec::new();
    for selected_change in &selection.changes {
        let Some(change) = by_name.get(selected_change.name.as_str()) else {
            continue;
        };
        let node = graph.node(&selected_change.name);
        let dependency_issues = graph
            .issues
            .iter()
            .filter(|issue| issue.change == selected_change.name)
            .map(change_dependency_issue_json)
            .collect::<Vec<_>>();
        let evidence = manifest
            .as_ref()
            .and_then(|manifest| manifest.get(&selected_change.name));
        let next_task = selected_change.next_task.as_ref().map(|task| {
            json!({
                "task": task.task,
                "instruction": {
                    "tool": task.instruction.tool,
                    "arguments": task.instruction.arguments,
                    "argv": task.instruction.argv,
                },
            })
        });
        changes.push(json!({
            "name": selected_change.name,
            "path": change.path,
            "dependencies": node.map(|node| node.dependencies.clone()).unwrap_or_default(),
            "blocked_by": node.map(|node| node.blocked_by.clone()).unwrap_or_default(),
            "blocks": node.map(|node| node.blocks.clone()).unwrap_or_default(),
            "dependency_ready": node.is_some_and(|node| node.dependency_ready),
            "dependency_issues": dependency_issues,
            "ordering_blocked": selected_change.ordering_blocked,
            "task_ordering": {
                "available": change.task_ordering.available,
                "active": change.task_ordering.active,
                "valid": change.task_ordering.valid,
                "ready_tasks": change.task_ordering.ready_tasks,
                "order": change.task_ordering.order,
            },
            "evidence": evidence_json(evidence),
            "classification": change.report.classification,
            "valid": change.report.valid,
            "readiness": serde_json::to_value(&change.report)?,
            "next_task": next_task,
        }));
        for action in &selected_change.actions {
            let mut action_json = json!({
                "change": selected_change.name,
                "kind": action.kind,
                "id": action.id,
                "target": action.target,
                "causes": action.causes,
                "prerequisites": action.prerequisites,
                "authority": action.authority,
                "expected_observation": action.expected_observation,
                "argv": action.argv,
            });
            if action.kind == "task" {
                if let Some(task) = selected_change
                    .next_task
                    .as_ref()
                    .filter(|task| task.task == action.target)
                {
                    action_json["instruction"] = json!({
                        "tool": task.instruction.tool,
                        "arguments": task.instruction.arguments,
                        "argv": task.instruction.argv,
                    });
                }
            }
            next_actions.push(action_json);
        }
    }

    let value = json!({
        "command": command,
        "schema": policy_handle.view_schema,
        "view": policy_handle.readiness_view,
        "root": path_string(&root),
        "layout": layout_name(selected.layout),
        "policy": policy.name,
        "readiness_policy_hash": cairn_core::canonical_json_hash(&serde_json::to_value(policy_handle)?),
        "manifest": {
            "declared": manifest.is_some(),
            "schema": policy_handle.manifest_schema,
            "change_count": manifest.as_ref().map_or(0, |admitted| admitted.len()),
        },
        "active_change_count": status.changes.len(),
        "observed_change_count": observed.len(),
        "ready_change_count": selection.ready_change_count,
        "blocked_change_count": selection.blocked_change_count,
        "unavailable_change_count": selection.unavailable_change_count,
        "drain_complete": status.changes.is_empty(),
        "archived_changes": archived_changes,
        "dependency_graph": {
            "valid": graph.valid,
            "nodes": graph.nodes.iter().map(change_dependency_node_json).collect::<Vec<_>>(),
            "edges": graph.edges.iter().map(change_dependency_edge_json).collect::<Vec<_>>(),
            "issues": graph.issues.iter().map(change_dependency_issue_json).collect::<Vec<_>>(),
            "drain_order": graph.drain_order,
        },
        "drain_order": selection.change_order,
        "changes": changes,
        "next_actions": next_actions,
        "truncated": selection.truncated,
        "non_claims": policy_handle.non_claims,
        "mcp": {"read_only": true, "destructive": false},
    });
    Ok(with_portable_receipt_hash(&root, value))
}

struct ObservedChange {
    name: String,
    path: String,
    report: cairn_core::verified::readiness::ReadinessReport,
    task_ordering: TaskOrderObservation,
}

struct TaskOrderObservation {
    available: bool,
    active: bool,
    valid: bool,
    ready_tasks: Vec<String>,
    order: Vec<String>,
}

fn change_observation_input(
    input: &Value,
    change: &str,
    evidence: Option<&EvidenceSelection>,
) -> Value {
    let mut observation = Map::new();
    if let Some(root) = input.get("root") {
        observation.insert("root".to_string(), root.clone());
    }
    if let Some(policy) = input.get("policy") {
        observation.insert("policy".to_string(), policy.clone());
    }
    observation.insert("change".to_string(), json!(change));
    if let Some(evidence) = evidence {
        if let Some(contract) = &evidence.contract {
            observation.insert("contract".to_string(), json!(contract));
        }
        if let Some(results) = &evidence.results {
            observation.insert("results".to_string(), json!(results));
        }
        if let Some(subject_hash) = &evidence.subject_hash {
            observation.insert("subject_hash".to_string(), json!(subject_hash));
        }
        if !evidence.cross_repo_evidence.is_empty() {
            observation.insert(
                "cross_repo_evidence".to_string(),
                json!(evidence.cross_repo_evidence),
            );
        }
    }
    Value::Object(observation)
}

fn evidence_json(evidence: Option<&EvidenceSelection>) -> Value {
    match evidence {
        Some(evidence) => json!({
            "declared": true,
            "completion_inputs": evidence.contract.is_some(),
            "contract": evidence.contract,
            "results": evidence.results,
            "subject_hash": evidence.subject_hash,
            "cross_repo_evidence": evidence.cross_repo_evidence,
        }),
        None => json!({
            "declared": false,
            "completion_inputs": false,
            "contract": Value::Null,
            "results": Value::Null,
            "subject_hash": Value::Null,
            "cross_repo_evidence": Vec::<String>::new(),
        }),
    }
}

fn observe_task_order(
    root: &Path,
    policy: &cairn_core::Policy,
    change_path: &str,
    budget: &ObservationBudget,
) -> TaskOrderObservation {
    let unavailable = || TaskOrderObservation {
        available: false,
        active: false,
        valid: false,
        ready_tasks: Vec::new(),
        order: Vec::new(),
    };
    let Ok(capability) = CapabilityRoot::open_nofollow(CapabilityRootKind::Scan, root) else {
        return unavailable();
    };
    let capability = capability.with_budget(Some(budget));
    let change_relative = Path::new(change_path)
        .strip_prefix(root)
        .unwrap_or_else(|_| Path::new(change_path));
    let relative = change_relative.join("tasks.md");
    let Ok(bytes) = capability.read_bounded(&relative, TASKS_FILE_BYTES) else {
        return unavailable();
    };
    let Ok(markdown) = String::from_utf8(bytes) else {
        return unavailable();
    };
    let graph = build_task_ordering_graph(&markdown, policy);
    TaskOrderObservation {
        available: true,
        active: graph.active,
        valid: graph.valid,
        ready_tasks: graph.ready_tasks,
        order: graph.order,
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EvidenceManifest {
    schema: String,
    changes: Vec<ManifestEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestEntry {
    change: String,
    #[serde(default)]
    contract: Option<String>,
    #[serde(default)]
    results: Option<String>,
    #[serde(default)]
    subject_hash: Option<String>,
    #[serde(default)]
    cross_repo_evidence: Vec<String>,
}

#[derive(Debug)]
struct EvidenceSelection {
    contract: Option<String>,
    results: Option<String>,
    subject_hash: Option<String>,
    cross_repo_evidence: Vec<String>,
}

fn load_evidence_manifest(
    root: &Path,
    path: &str,
    changes: &[ChangeStatus],
    policy: &WorkflowReadinessPolicy,
    budget: &ObservationBudget,
) -> Result<BTreeMap<String, EvidenceSelection>, Box<dyn Error>> {
    if !manifest_path_is_safe(path, policy.bounds.max_path_bytes) {
        return Err(manifest_error(
            MANIFEST_UNSAFE_PATH,
            &["evidence manifest path is not repository-relative".to_string()],
        ));
    }
    let capability =
        CapabilityRoot::open_nofollow(CapabilityRootKind::Scan, root)?.with_budget(Some(budget));
    let bytes = capability
        .read_bounded(Path::new(path), policy.bounds.max_manifest_bytes)
        .map_err(|_| {
            manifest_error(
                MANIFEST_UNAVAILABLE,
                &["evidence manifest could not be admitted".to_string()],
            )
        })?;
    let manifest: EvidenceManifest = serde_json::from_slice(&bytes).map_err(|error| {
        manifest_error(MANIFEST_INVALID, &[bounded_diagnostic(&error.to_string())])
    })?;
    admit_evidence_manifest(manifest, changes, policy)
}

fn admit_evidence_manifest(
    manifest: EvidenceManifest,
    changes: &[ChangeStatus],
    policy: &WorkflowReadinessPolicy,
) -> Result<BTreeMap<String, EvidenceSelection>, Box<dyn Error>> {
    let limits = &policy.bounds;
    let mut issues = Vec::new();
    if manifest.schema != policy.manifest_schema {
        issues.push(format!("{MANIFEST_SCHEMA}: unsupported manifest schema"));
    }
    if manifest.changes.is_empty() || manifest.changes.len() > limits.max_changes {
        issues.push(format!(
            "{MANIFEST_BOUND}: change selector count must be between 1 and {}",
            limits.max_changes
        ));
    }
    let active = changes
        .iter()
        .map(|change| change.name.as_str())
        .collect::<BTreeSet<_>>();
    let mut admitted = BTreeMap::new();
    for entry in manifest.changes {
        let name = entry.change;
        if cairn_core::validate_change_name(&name).is_err() || !active.contains(name.as_str()) {
            issues.push(format!(
                "{MANIFEST_UNKNOWN_CHANGE}: {name} is not an exact active change identifier"
            ));
            continue;
        }
        if admitted.contains_key(&name) {
            issues.push(format!("{MANIFEST_DUPLICATE_CHANGE}: {name}"));
            continue;
        }
        let completion_inputs = [
            entry.contract.is_some(),
            entry.results.is_some(),
            entry.subject_hash.is_some(),
        ];
        if completion_inputs.iter().any(|present| *present)
            && !completion_inputs.iter().all(|present| *present)
        {
            issues.push(format!("{MANIFEST_PARTIAL_COMPLETION}: {name}"));
            continue;
        }
        let mut admitted_evidence = EvidenceSelection {
            contract: entry.contract,
            results: entry.results,
            subject_hash: entry.subject_hash,
            cross_repo_evidence: Vec::new(),
        };
        let mut valid = true;
        for path in [&admitted_evidence.contract, &admitted_evidence.results]
            .into_iter()
            .flatten()
        {
            if !manifest_path_is_safe(path, limits.max_path_bytes) {
                issues.push(format!("{MANIFEST_UNSAFE_PATH}: {name}"));
                valid = false;
            }
        }
        if let Some(subject_hash) = &admitted_evidence.subject_hash {
            if subject_hash.is_empty()
                || subject_hash.len() > limits.max_subject_hash_bytes
                || subject_hash.chars().any(char::is_control)
            {
                issues.push(format!("{MANIFEST_INVALID_SUBJECT}: {name}"));
                valid = false;
            }
        }
        if entry.cross_repo_evidence.len() > limits.max_evidence_files {
            issues.push(format!(
                "{MANIFEST_BOUND}: {name} selects more than {} evidence files",
                limits.max_evidence_files
            ));
            valid = false;
        }
        for path in entry.cross_repo_evidence {
            if !manifest_path_is_safe(&path, limits.max_path_bytes) {
                issues.push(format!("{MANIFEST_UNSAFE_PATH}: {name}"));
                valid = false;
                continue;
            }
            if admitted_evidence.cross_repo_evidence.contains(&path) {
                issues.push(format!("{MANIFEST_DUPLICATE_EVIDENCE}: {name}"));
                valid = false;
                continue;
            }
            admitted_evidence.cross_repo_evidence.push(path);
        }
        if valid {
            admitted.insert(name, admitted_evidence);
        }
    }
    if issues.is_empty() {
        return Ok(admitted);
    }
    issues.truncate(limits.max_manifest_issues);
    Err(manifest_error(MANIFEST_INVALID, &issues))
}

fn manifest_path_is_safe(path: &str, max_bytes: usize) -> bool {
    !path.is_empty()
        && path.len() <= max_bytes
        && !path.chars().any(char::is_control)
        && !path.contains('\\')
        && is_safe_relative_reference(path)
}

fn bounded_diagnostic(text: &str) -> String {
    let mut bounded = String::with_capacity(text.len().min(MAX_MANIFEST_DIAGNOSTIC_BYTES));
    for character in text.chars() {
        if bounded.len() + character.len_utf8() > MAX_MANIFEST_DIAGNOSTIC_BYTES {
            break;
        }
        bounded.push(if character.is_control() {
            ' '
        } else {
            character
        });
    }
    bounded
}

fn manifest_error(code: &str, issues: &[String]) -> Box<dyn Error> {
    let mut text = code.to_string();
    for issue in issues {
        text.push_str(": ");
        text.push_str(issue);
    }
    bounded_diagnostic(&text).into()
}

fn input_root(input: &Value) -> PathBuf {
    input["root"]
        .as_str()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn input_policy(input: &Value) -> PathBuf {
    input["policy"]
        .as_str()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(default_policy_path()))
}

#[cfg(test)]
#[path = "drain_plan_tests.rs"]
mod tests;
