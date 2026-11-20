//! Read-only selected-work observation. The pure contract and admission live in core.
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    path::{Path, PathBuf},
};

use cairn_core::verified::readiness::{ReadinessReport, ReadinessState};
use cairn_core::verified::selected_work::{
    self, Availability, CompletionFact, ContextEntry, ContextFact, DependencyFact, Enumeration,
    ExpectedCohort, ImplementationUnit, ObligationObservationInput, ObligationSource,
    ReferenceScope, RemainingKind, RemainingWork, SelectedWorkInput, SelectedWorkPolicy, Selection,
    SourceArtifact, SourceCohort, SourceRevision, SourceRevisionKind, SourceRoot, SourceStatus,
    WorkState,
};
use cairn_core::{
    blake3_hex, canonical_json_hash, changes_dir, validate_change_name,
    AdmittedWorkflowObligationCatalog, AgentGuidanceRequest, LifecycleLayout,
};
use serde_json::{json, Value};

use crate::support::observation_budget::{ObservationBudget, MAX_REPORT_BYTES};
use crate::support::{
    default_policy_path, load_policy_file_with_budget, print_json,
    read_selected_change_with_budget, read_selected_dependency_graph_with_budget, CapabilityRoot,
    CapabilityRootKind,
};

const MAX_ARTIFACT_BYTES: u64 = 1_048_576;
const MAX_ARTIFACT_ENTRIES: usize = 512;

pub(crate) fn run_selected_work(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let change = args
        .first()
        .filter(|value| !value.is_empty() && !value.starts_with("--"))
        .ok_or("selected-work requires CHANGE")?
        .clone();
    let mut request = json!({"change": change});
    let mut index = 1;
    while index < args.len() {
        let option = &args[index];
        let key = match option.as_str() {
            "--root" => "root",
            "--lifecycle-store" => "lifecycle_store",
            "--policy" => "policy",
            "--task" => "task",
            "--contract" => "contract",
            "--results" => "results",
            "--subject-hash" => "subject_hash",
            "--disclosure" => "disclosure",
            "--obligation-definition" => "obligation_definition",
            "--obligation-observations" => "obligation_observations",
            "--expected-cohort" => "expected_cohort",
            "--detail" => {
                if request.get("detail").is_some() {
                    return Err("duplicate --detail".into());
                }
                request["detail"] = json!(true);
                index += 1;
                continue;
            }
            "--cross-repo-evidence" => {
                let value = args
                    .get(index + 1)
                    .ok_or("--cross-repo-evidence requires a path")?;
                request
                    .as_object_mut()
                    .expect("object")
                    .entry("cross_repo_evidence")
                    .or_insert_with(|| json!([]))
                    .as_array_mut()
                    .expect("array")
                    .push(json!(value));
                index += 2;
                continue;
            }
            _ => return Err(format!("unsupported selected-work option: {option}").into()),
        };
        let value = args
            .get(index + 1)
            .filter(|value| !value.is_empty() && !value.starts_with("--"))
            .ok_or_else(|| format!("{option} requires a value"))?;
        if request.get(key).is_some() {
            return Err(format!("duplicate selected-work option: {option}").into());
        }
        if key == "expected_cohort" {
            request[key] = serde_json::from_str(value)?;
        } else {
            request[key] = json!(value);
        }
        index += 2;
    }
    print_json(selected_work_value(&request)?)
}

/// Shared semantic result for the CLI and the strict MCP tool.
// r[impl cairn.selected_work.transport]
pub(crate) fn selected_work_value(request: &Value) -> Result<Value, Box<dyn Error>> {
    let change = required_string(request, "change")?;
    validate_change_name(change)?;
    let task = request
        .get("task")
        .map(|_| required_string(request, "task"))
        .transpose()?;
    let root = request
        .get("root")
        .map(|_| required_string(request, "root"))
        .transpose()?;
    let store = request
        .get("lifecycle_store")
        .map(|_| required_string(request, "lifecycle_store"))
        .transpose()?;
    let invocation = crate::lifecycle_store_shell::resolve_selected_work_root(
        root.map(str::to_owned),
        store.map(str::to_owned),
    )?;
    let selected_root = invocation.selected_root()?.to_path_buf();
    let local_root = std::path::absolute(&selected_root)?;
    let project_root = std::path::absolute(invocation.project_root().unwrap_or(&selected_root))?;
    let policy_path = PathBuf::from(
        request
            .get("policy")
            .and_then(Value::as_str)
            .unwrap_or(default_policy_path()),
    );
    let selected_policy = SelectedWorkPolicy::bundled()?;
    if request.get("obligation_observations").is_some()
        && request.get("obligation_definition").is_none()
    {
        return Err("--obligation-observations requires --obligation-definition".into());
    }
    let definition = request
        .get("obligation_definition")
        .map(|_| required_string(request, "obligation_definition"))
        .transpose()?;
    let budget = ObservationBudget::new();
    let first = observe_once(
        &local_root,
        &project_root,
        &policy_path,
        change,
        task,
        request,
        &budget,
    )?;
    let second = observe_once(
        &local_root,
        &project_root,
        &policy_path,
        change,
        task,
        request,
        &budget,
    )?;
    // Compare facts before writing an I/O accounting value, which necessarily
    // differs across passes. Both passes consume the same aggregate budget.
    if first != second {
        return Err("selected-work observation cohort changed".into());
    }
    let Observed {
        input: mut observed,
        obligations,
    } = first;
    observed.enumeration.consumed_bytes = budget.consumed_bytes();
    let source = match (definition, &obligations) {
        (Some(definition), Some((catalog, observations))) => Some(ObligationSource {
            catalog,
            definition,
            observations: observations.as_ref(),
        }),
        (None, None) => None,
        _ => return Err("selected-work obligation observation is incomplete".into()),
    };
    let snapshot = selected_work::compose(&selected_policy, &observed, source.as_ref())?;
    let expected = request
        .get("expected_cohort")
        .map(|value| serde_json::from_value::<ExpectedCohort>(value.clone()))
        .transpose()?;
    if request.get("detail").and_then(Value::as_bool) == Some(true) && expected.is_none() {
        return Err("selected-work detail requires expected_cohort".into());
    }
    let disclosure = request
        .get("disclosure")
        .and_then(Value::as_str)
        .unwrap_or("portable");
    if request.get("detail").and_then(Value::as_bool) == Some(true) && disclosure != "local" {
        return Err("selected-work detail requires local disclosure".into());
    }
    let result = match disclosure {
        "portable" => {
            let cohort = snapshot.cohort.clone();
            let portable = snapshot.into_portable();
            selected_work::admit_portable(&selected_policy, &portable, &cohort)?;
            if let Some(expected) = &expected {
                selected_work::admit_portable(&selected_policy, &portable, expected)?;
            }
            portable
        }
        "local" => {
            let portable = snapshot.portable();
            selected_work::admit_portable(&selected_policy, &portable, &snapshot.cohort)?;
            if let Some(expected) = &expected {
                selected_work::admit_portable(&selected_policy, &portable, expected)?;
            }
            snapshot
        }
        _ => return Err("selected-work disclosure must be local or portable".into()),
    };
    Ok(serde_json::to_value(result)?)
}

fn required_string<'a>(input: &'a Value, key: &str) -> Result<&'a str, Box<dyn Error>> {
    input
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("selected-work requires string {key}").into())
}

/// One observation pass. The obligation catalog and observations are part of
/// the compared cohort, so a change between passes is refused.
#[derive(PartialEq)]
struct Observed {
    input: SelectedWorkInput,
    obligations: Option<(
        AdmittedWorkflowObligationCatalog,
        Option<ObligationObservationInput>,
    )>,
}

// r[impl cairn.selected_work.observation]
fn observe_once(
    root: &Path,
    project: &Path,
    policy_path: &Path,
    change: &str,
    task: Option<&str>,
    request: &Value,
    budget: &ObservationBudget,
) -> Result<Observed, Box<dyn Error>> {
    let layout = LifecycleLayout::Native;
    let policy = load_policy_file_with_budget(policy_path.to_path_buf(), Some(budget))?;
    let policy_hash = canonical_json_hash(&policy.raw);
    let obligation_catalog = request
        .get("obligation_definition")
        .map(|_| crate::workflow_obligations::admit_bundled_obligation_catalog(&policy))
        .transpose()?;
    let graph = read_selected_dependency_graph_with_budget(root, layout, change, Some(budget))?;
    let selected = read_selected_change_with_budget(root, layout, change, Some(budget))?;
    let profile = crate::workflow::load_workflow_profile_context_with_budget(
        root,
        layout,
        change,
        &policy,
        Some(budget),
    );
    let profile_id = profile
        .as_ref()
        .map(|value| value.selection.profile_id.clone())
        .unwrap_or_else(|_| "unavailable".to_owned());
    let profile_hash = profile
        .as_ref()
        .map(|value| value.profile_identity_hash.clone())
        .unwrap_or_else(|_| canonical_json_hash(&json!("profile unavailable")));
    let mut readiness_request = json!({"root": root, "change": change, "policy": policy_path});
    for key in ["contract", "results", "subject_hash", "cross_repo_evidence"] {
        if let Some(value) = request.get(key) {
            readiness_request[key] = value.clone();
        }
    }
    let readiness: ReadinessReport = serde_json::from_value(
        crate::readiness::selected_readiness_value(&readiness_request, budget)?,
    )?;

    let guidance_request = AgentGuidanceRequest {
        profile: profile
            .as_ref()
            .ok()
            .map(|value| value.selection.profile_id.clone()),
        artifact: Some("tasks".into()),
        operation: Some("context".into()),
    };
    let guidance = crate::agent_guidance_shell::load_agent_guidance_with_budget(
        project,
        root,
        guidance_request.clone(),
        &[],
        &policy,
        Some(budget),
    )?;
    let context = if let Some(guidance) = guidance {
        if !guidance.valid {
            return Err("selected declared context is incomplete".into());
        }
        ContextFact {
            selection_hash: canonical_json_hash(&serde_json::to_value(&guidance.request)?),
            local_hash: guidance.local_result_hash,
            portable_hash: guidance.portable_identity,
            entries: guidance
                .entries
                .into_iter()
                .map(|entry| ContextEntry {
                    id: entry.id,
                    authority: entry.authority,
                    source_root: entry.source_root,
                    path: entry.path,
                    status: match entry.status {
                        cairn_core::AgentGuidanceObservationStatus::Present => Availability::Ready,
                        cairn_core::AgentGuidanceObservationStatus::MissingOptional => {
                            Availability::Unavailable
                        }
                        cairn_core::AgentGuidanceObservationStatus::MissingRequired
                        | cairn_core::AgentGuidanceObservationStatus::Invalid => {
                            Availability::Blocked
                        }
                    },
                    content_hash: entry.content_blake3,
                    text: entry.text,
                })
                .collect(),
        }
    } else {
        let selection_hash = canonical_json_hash(&serde_json::to_value(&guidance_request)?);
        ContextFact {
            selection_hash,
            local_hash: canonical_json_hash(&json!(["no-declared-guidance", project, change])),
            portable_hash: canonical_json_hash(&json!([
                "no-declared-guidance",
                change,
                profile_id
            ])),
            entries: Vec::new(),
        }
    };

    let capability = CapabilityRoot::open_nofollow(CapabilityRootKind::Repository, root)?
        .with_budget(Some(budget));
    let task_path = PathBuf::from(changes_dir(layout))
        .join(change)
        .join("tasks.md");
    let task_text = match capability.read_bounded(&task_path, MAX_ARTIFACT_BYTES) {
        Ok(bytes) => Some(String::from_utf8(bytes)?),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    };
    // Work can start only when the native tasks stage and the change's
    // declared dependencies are both ready.
    let work_ready = stage_availability(&readiness, "tasks") == Availability::Ready
        && graph
            .nodes
            .iter()
            .find(|node| node.name == change)
            .is_some_and(|node| node.dependency_ready);
    let (units, mut dependencies) = implementation_units(
        change,
        task,
        task_text.as_deref(),
        &policy,
        &selected,
        work_ready,
        request,
    )?;
    for edge in graph.edges.iter().filter(|edge| edge.dependent == change) {
        dependencies.push(DependencyFact {
            task: change.into(),
            prerequisite: edge.dependency.clone(),
            state: if edge.satisfied {
                Availability::Ready
            } else {
                Availability::Blocked
            },
            identity: canonical_json_hash(&json!([
                "change-prerequisite",
                change,
                edge.dependency,
                edge.satisfied,
                selected.metadata.depends_on
            ])),
        });
    }
    for dependency in &selected.metadata.depends_on {
        if !dependencies
            .iter()
            .any(|fact| fact.task == change && fact.prerequisite == *dependency)
        {
            dependencies.push(DependencyFact {
                task: change.into(),
                prerequisite: dependency.clone(),
                state: Availability::Unavailable,
                identity: canonical_json_hash(&json!(["missing-prerequisite", change, dependency])),
            });
        }
    }
    let mut manifest = Vec::new();
    let mut names = graph
        .nodes
        .iter()
        .map(|node| node.name.clone())
        .collect::<Vec<_>>();
    if !names.iter().any(|name| name == change) {
        names.push(change.into());
    }
    names.sort();
    names.dedup();
    for name in &names {
        manifest_tree(
            &capability,
            &PathBuf::from(changes_dir(layout)).join(name),
            SourceRoot::Lifecycle,
            &mut manifest,
        )?;
    }
    // Native sync/merge readiness reads the accepted counterpart of each
    // selected delta. Bind that source even when it does not yet exist.
    let delta_root = root.join(changes_dir(layout)).join(change).join("specs");
    for delta in &selected.delta_specs {
        let relative = Path::new(delta)
            .strip_prefix(&delta_root)
            .map_err(|_| "delta source escaped selected change")?;
        let accepted = PathBuf::from(cairn_core::specs_dir(layout)).join(relative);
        manifest_optional(&capability, &accepted, SourceRoot::Lifecycle, &mut manifest)?;
    }
    if project != root {
        let project_capability =
            CapabilityRoot::open_nofollow(CapabilityRootKind::Repository, project)?
                .with_budget(Some(budget));
        for path in [".cairn/project.ncl", ".cairn/project.json"] {
            manifest_optional(
                &project_capability,
                Path::new(path),
                SourceRoot::Repository,
                &mut manifest,
            )?;
        }
    } else {
        for path in [".cairn/project.ncl", ".cairn/project.json"] {
            manifest_optional(
                &capability,
                Path::new(path),
                SourceRoot::Lifecycle,
                &mut manifest,
            )?;
        }
    }
    let declarations = PathBuf::from(changes_dir(layout))
        .join(change)
        .join("probes/probes.json");
    if capability.metadata_nofollow(&declarations).is_ok() {
        let bytes = capability.read_bounded(&declarations, 64 * 1_024)?;
        let value: Value = serde_json::from_slice(&bytes)
            .map_err(|_| "saved probe declaration is incomplete for selected evidence")?;
        let probes = value
            .get("probes")
            .and_then(Value::as_array)
            .or_else(|| value.as_array())
            .ok_or("saved probe declaration has no admitted probe array")?;
        if probes.len() > 64 {
            return Err("selected saved probe enumeration exceeds bound".into());
        }
        for probe in probes {
            let receipt = probe
                .get("receipt_path")
                .and_then(Value::as_str)
                .ok_or("selected saved probe receipt locator is unavailable")?;
            manifest_optional(
                &capability,
                Path::new(receipt),
                SourceRoot::Lifecycle,
                &mut manifest,
            )?;
        }
    }

    // Obligation observations are evidence: bind their exact bytes into the
    // source manifest and refuse a missing or malformed file.
    let obligation_observations = match request
        .get("obligation_observations")
        .and_then(Value::as_str)
    {
        Some(path) => {
            let bytes = capability.read_bounded(Path::new(path), MAX_ARTIFACT_BYTES)?;
            let parsed: ObligationObservationInput =
                serde_json::from_slice(&bytes).map_err(|error| {
                    format!("selected obligation observations are invalid: {error}")
                })?;
            manifest.push(SourceArtifact {
                root: SourceRoot::Lifecycle,
                path: path.into(),
                bytes: bytes.len() as u64,
                blake3: blake3_hex(&bytes),
                status: SourceStatus::Present,
            });
            Some(parsed)
        }
        None => None,
    };
    for key in ["contract", "results"] {
        if let Some(path) = request.get(key).and_then(Value::as_str) {
            manifest_optional(
                &capability,
                Path::new(path),
                SourceRoot::Lifecycle,
                &mut manifest,
            )?;
        }
    }
    if let Some(paths) = request.get("cross_repo_evidence").and_then(Value::as_array) {
        for path in paths {
            manifest_optional(
                &capability,
                Path::new(path.as_str().ok_or("invalid evidence path")?),
                SourceRoot::Lifecycle,
                &mut manifest,
            )?;
        }
    }
    manifest
        .sort_by(|left, right| (left.root as u8, &left.path).cmp(&(right.root as u8, &right.path)));
    manifest.dedup_by(|left, right| left.root == right.root && left.path == right.path);
    let revision = canonical_json_hash(&json!({"artifacts": manifest}));
    let completion_stage = readiness
        .stages
        .iter()
        .find(|stage| stage.stage == "completion");
    let completion_state = completion_stage
        .map(|stage| stage.state)
        .unwrap_or(ReadinessState::Unavailable);
    let completion = CompletionFact {
        state: availability(completion_state),
        contract_hash: manifest
            .iter()
            .find(|entry| {
                request["contract"].as_str() == Some(entry.path.as_str())
                    && entry.status == SourceStatus::Present
            })
            .map(|entry| entry.blake3.clone()),
        results_hash: manifest
            .iter()
            .find(|entry| {
                request["results"].as_str() == Some(entry.path.as_str())
                    && entry.status == SourceStatus::Present
            })
            .map(|entry| entry.blake3.clone()),
        subject_hash: request
            .get("subject_hash")
            .and_then(Value::as_str)
            .map(str::to_owned),
    };
    let mut remaining = Vec::new();
    if units.iter().any(|unit| unit.state != WorkState::Complete) {
        let state = if units.iter().any(|unit| unit.state == WorkState::Ready) {
            Availability::Ready
        } else if units
            .iter()
            .any(|unit| unit.state == WorkState::Unavailable)
        {
            Availability::Unavailable
        } else {
            Availability::Blocked
        };
        remaining.push(RemainingWork {
            kind: RemainingKind::Implementation,
            state,
            subject: task.map(str::to_owned),
        });
    }
    remaining.extend(
        [
            (RemainingKind::Completion, completion.state),
            (RemainingKind::Sync, stage_availability(&readiness, "sync")),
            (
                RemainingKind::Archive,
                stage_availability(&readiness, "archive"),
            ),
        ]
        .into_iter()
        .map(|(kind, state)| RemainingWork {
            kind,
            state,
            subject: None,
        }),
    );
    let observed = u32::try_from(names.len())?;
    let input = SelectedWorkInput {
        producer: "cairn".into(),
        selection: Selection {
            change: change.into(),
            task: task.map(str::to_owned),
        },
        source: SourceCohort {
            revision: SourceRevision {
                kind: SourceRevisionKind::Projection,
                identity: revision,
            },
            artifacts: manifest,
        },
        policy_hash,
        profile: profile_id,
        profile_hash,
        dependencies,
        completion,
        context,
        readiness,
        units,
        remaining,
        enumeration: Enumeration {
            complete: true,
            observed,
            total: Some(observed),
            budget_bytes: MAX_REPORT_BYTES,
            consumed_bytes: 0,
        },
        local_root: root.to_string_lossy().into_owned(),
    };
    Ok(Observed {
        input,
        obligations: obligation_catalog.map(|catalog| (catalog, obligation_observations)),
    })
}

fn availability(state: ReadinessState) -> Availability {
    match state {
        ReadinessState::Ready => Availability::Ready,
        ReadinessState::Blocked => Availability::Blocked,
        ReadinessState::Unavailable | ReadinessState::NotApplicable => Availability::Unavailable,
    }
}
fn stage_availability(report: &ReadinessReport, name: &str) -> Availability {
    report
        .stages
        .iter()
        .find(|stage| stage.stage == name)
        .map(|stage| availability(stage.state))
        .unwrap_or(Availability::Unavailable)
}

fn implementation_units(
    change: &str,
    task: Option<&str>,
    markdown: Option<&str>,
    policy: &cairn_core::Policy,
    selected: &cairn_core::ChangeStatus,
    work_ready: bool,
    request: &Value,
) -> Result<(Vec<ImplementationUnit>, Vec<DependencyFact>), Box<dyn Error>> {
    let mut units = Vec::new();
    let mut dependencies = Vec::new();
    let contract_paths = request
        .get("contract")
        .and_then(Value::as_str)
        .map(|path| vec![path.to_owned()])
        .unwrap_or_default();
    let markdown = markdown.unwrap_or("");
    let ordering = cairn_core::build_task_ordering_graph(markdown, policy);
    if ordering.active {
        if !ordering.valid {
            return Err("selected task IDs or prerequisites are malformed or ambiguous".into());
        }
        if let Some(task) = task {
            if !ordering.nodes.iter().any(|node| node.id == task) {
                return Err(format!("unknown selected task ID: {task}").into());
            }
        }
        let mut included = BTreeSet::new();
        if let Some(task) = task {
            included.insert(task.to_owned());
            loop {
                let before = included.len();
                for node in &ordering.nodes {
                    if included.contains(&node.id) {
                        included.extend(node.predecessors.iter().cloned());
                    }
                }
                if included.len() == before {
                    break;
                }
            }
        } else {
            included.extend(ordering.nodes.iter().map(|node| node.id.clone()));
        }
        let lines = markdown.lines().collect::<Vec<_>>();
        let mut by_id = BTreeMap::new();
        for node in &ordering.nodes {
            let start = node.line.saturating_sub(1);
            let end = ordering
                .nodes
                .iter()
                .filter(|next| next.line > node.line)
                .map(|next| next.line.saturating_sub(1))
                .min()
                .unwrap_or(lines.len());
            by_id.insert(
                node.id.as_str(),
                lines.get(start..end).unwrap_or(&[]).join("\n"),
            );
        }
        for node in &ordering.nodes {
            if !included.contains(&node.id) {
                continue;
            }
            let state = match node.state {
                cairn_core::TaskOrderingState::Done => WorkState::Complete,
                _ if !work_ready || !node.ready => WorkState::Blocked,
                _ => WorkState::Ready,
            };
            let refs = selected_work::extract_scoped_references(
                by_id
                    .get(node.id.as_str())
                    .map(String::as_str)
                    .unwrap_or(""),
                ReferenceScope::Task(node.id.clone()),
            )?;
            for predecessor in &node.predecessors {
                dependencies.push(DependencyFact {
                    task: node.id.clone(),
                    prerequisite: predecessor.clone(),
                    state: if ordering.nodes.iter().any(|other| {
                        other.id == *predecessor
                            && matches!(other.state, cairn_core::TaskOrderingState::Done)
                    }) {
                        Availability::Ready
                    } else {
                        Availability::Blocked
                    },
                    identity: canonical_json_hash(&json!([change, node.id, predecessor, markdown])),
                });
            }
            units.push(ImplementationUnit {
                task: Some(node.id.clone()),
                state,
                references: refs,
                verification_contracts: contract_paths.clone(),
            });
        }
    } else {
        if task.is_some() {
            return Err("authored task selector unavailable for legacy whole-change work".into());
        }
        let counts = selected.task_counts;
        let complete = counts.is_some_and(|counts| {
            counts.total > 0 && counts.todo == 0 && counts.in_progress == 0 && counts.unmarked == 0
        });
        units.push(ImplementationUnit {
            task: None,
            state: if complete {
                WorkState::Complete
            } else if work_ready {
                WorkState::Ready
            } else {
                WorkState::Blocked
            },
            references: selected_work::extract_scoped_references(markdown, ReferenceScope::Change)?,
            verification_contracts: contract_paths,
        });
    }
    Ok((units, dependencies))
}

fn manifest_tree(
    capability: &CapabilityRoot,
    directory: &Path,
    root: SourceRoot,
    manifest: &mut Vec<SourceArtifact>,
) -> Result<(), Box<dyn Error>> {
    let mut pending = vec![directory.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in capability.entries_bounded(&directory, MAX_ARTIFACT_ENTRIES)? {
            if manifest.len() >= MAX_ARTIFACT_ENTRIES {
                return Err("selected source manifest exceeds bound".into());
            }
            let name = entry.file_name().to_str().ok_or("non-UTF8 source name")?;
            let path = directory.join(name);
            if entry.file_type().is_dir() {
                pending.push(path);
            } else if entry.file_type().is_file() {
                manifest_required(capability, &path, root, manifest)?;
            } else {
                return Err("linked or unsupported selected source entry".into());
            }
        }
    }
    Ok(())
}
fn manifest_optional(
    capability: &CapabilityRoot,
    path: &Path,
    root: SourceRoot,
    manifest: &mut Vec<SourceArtifact>,
) -> Result<(), Box<dyn Error>> {
    match capability.metadata_nofollow(path) {
        Ok(metadata) if metadata.is_file() => manifest_required(capability, path, root, manifest),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            manifest.push(SourceArtifact {
                root,
                path: path.to_str().ok_or("non-UTF8 source path")?.into(),
                bytes: 0,
                blake3: String::new(),
                status: SourceStatus::Missing,
            });
            Ok(())
        }
        _ => Err("unsafe selected source path".into()),
    }
}
fn manifest_required(
    capability: &CapabilityRoot,
    path: &Path,
    root: SourceRoot,
    manifest: &mut Vec<SourceArtifact>,
) -> Result<(), Box<dyn Error>> {
    let bytes = capability.read_bounded(path, MAX_ARTIFACT_BYTES)?;
    let path = path.to_str().ok_or("non-UTF8 source path")?;
    manifest.push(SourceArtifact {
        root,
        path: path.into(),
        bytes: bytes.len() as u64,
        blake3: blake3_hex(&bytes),
        status: SourceStatus::Present,
    });
    Ok(())
}
