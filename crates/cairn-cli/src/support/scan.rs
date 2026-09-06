use super::source::SourceReader;
use super::*;

const ARCHIVE_DATE_LEN: usize = 10;
const ARCHIVE_DATE_SEPARATOR_INDEX: usize = ARCHIVE_DATE_LEN;
const ARCHIVE_NAME_PREFIX_LEN: usize = ARCHIVE_DATE_LEN + 1;

pub(crate) fn scan_repo(root: PathBuf, layout: LifecycleLayout) -> std::io::Result<RepoStatus> {
    scan_repo_with_budget(root, layout, None)
}

pub(crate) fn scan_repo_with_budget(
    root: PathBuf,
    layout: LifecycleLayout,
    budget: Option<&super::observation_budget::ObservationBudget>,
) -> std::io::Result<RepoStatus> {
    let mut capability = SourceReader::open_with_budget(&root, budget)?;
    let specs_dir_text = specs_dir(layout);
    let changes_dir_text = changes_dir(layout);
    let specs = collect_spec_files(&mut capability, Path::new(&specs_dir_text))?;
    let changes = collect_change_sources(&mut capability, Path::new(&changes_dir_text))?;
    Ok(build_repo_status(path_string(root), specs, changes))
}

pub(crate) fn read_change_dependency_graph(
    root: &Path,
    layout: LifecycleLayout,
) -> std::io::Result<ChangeDependencyGraph> {
    read_change_dependency_graph_with_budget(root, layout, None)
}

pub(crate) fn read_change_dependency_graph_with_budget(
    root: &Path,
    layout: LifecycleLayout,
    budget: Option<&super::observation_budget::ObservationBudget>,
) -> std::io::Result<ChangeDependencyGraph> {
    let status = scan_repo_with_budget(root.to_path_buf(), layout, budget)?;
    let archived_changes = collect_archived_change_ids_with_budget(root, layout, budget)?;
    Ok(build_change_dependency_graph(
        &status.changes,
        &archived_changes,
    ))
}

pub(crate) fn collect_archived_change_ids(
    root: &Path,
    layout: LifecycleLayout,
) -> std::io::Result<Vec<String>> {
    collect_archived_change_ids_with_budget(root, layout, None)
}

fn collect_archived_change_ids_with_budget(
    root: &Path,
    layout: LifecycleLayout,
    budget: Option<&super::observation_budget::ObservationBudget>,
) -> std::io::Result<Vec<String>> {
    let mut capability = SourceReader::open_with_budget(root, budget)?;
    let archive_dir_text = archive_dir(layout);
    let archive_root = Path::new(&archive_dir_text);
    let entries = match capability.entries(archive_root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error),
    };
    let mut archived = Vec::with_capacity(entries.len());
    for entry in entries {
        if !entry.file_type().is_dir() {
            continue;
        }
        let directory_name = entry.file_name().to_string_lossy().to_string();
        if is_hidden_or_archive_change(&directory_name) {
            continue;
        }
        archived.push(archived_change_id_from_directory(&directory_name));
    }
    archived.sort();
    archived.dedup();
    Ok(archived)
}

fn collect_change_sources(
    capability: &mut SourceReader,
    changes_dir: &Path,
) -> std::io::Result<Vec<ChangeSource>> {
    let entries = match capability.entries(changes_dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error),
    };
    let mut changes = Vec::with_capacity(entries.len());
    for entry in entries {
        if !entry.file_type().is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if is_hidden_or_archive_change(&name) {
            continue;
        }
        let relative = entry_relative_path(changes_dir, entry.file_name());
        changes.push(read_change_source(capability, name, relative)?);
    }
    Ok(changes)
}

fn read_change_source(
    capability: &mut SourceReader,
    name: String,
    path: PathBuf,
) -> std::io::Result<ChangeSource> {
    let tasks_path = path.join("tasks.md");
    let tasks_markdown = read_optional_text(capability, &tasks_path)?;
    let metadata_path = path.join("metadata.json");
    let metadata_json = read_optional_text(capability, &metadata_path)?;
    let delta_specs = collect_spec_files(capability, &path.join("specs"))?;
    Ok(ChangeSource {
        name,
        path: path_string(capability.display_path(&path)?),
        has_proposal: path_exists(capability, &path.join("proposal.md"))?,
        has_design: path_exists(capability, &path.join("design.md"))?,
        has_tasks: tasks_markdown.is_some(),
        delta_specs,
        tasks_markdown,
        metadata_json,
    })
}

fn collect_spec_files(
    capability: &mut SourceReader,
    relative_dir: &Path,
) -> std::io::Result<Vec<String>> {
    let mut specs = Vec::new();
    let mut dirs = vec![relative_dir.to_path_buf()];
    while let Some(current_dir) = dirs.pop() {
        let entries = match capability.entries(&current_dir) {
            Ok(entries) => entries,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => return Err(error),
        };
        for entry in entries {
            let relative = entry_relative_path(&current_dir, entry.file_name());
            if entry.file_type().is_dir() {
                dirs.push(relative);
            } else if relative.file_name().is_some_and(|name| name == "spec.md") {
                specs.push(path_string(capability.display_path(&relative)?));
            }
        }
    }
    specs.sort();
    Ok(specs)
}

pub(crate) fn collect_spec_sources(specs_dir: &Path) -> std::io::Result<Vec<SpecSource>> {
    let mut capability = SourceReader::open(specs_dir)?;
    collect_spec_sources_from_root(&mut capability, Path::new("."), specs_dir, specs_dir)
}

fn collect_spec_sources_from_root(
    capability: &mut SourceReader,
    relative_dir: &Path,
    display_root: &Path,
    spec_id_root: &Path,
) -> std::io::Result<Vec<SpecSource>> {
    let spec_paths = collect_relative_spec_files(capability, relative_dir)?;
    let mut specs = Vec::with_capacity(spec_paths.len());
    for relative_path in spec_paths {
        let markdown = capability.read_to_string(&relative_path)?;
        let path = display_root.join(&relative_path);
        specs.push(SpecSource {
            id: spec_id(spec_id_root, &path),
            path: path_string(path),
            markdown,
        });
    }
    Ok(specs)
}

fn collect_relative_spec_files(
    capability: &mut SourceReader,
    relative_dir: &Path,
) -> std::io::Result<Vec<PathBuf>> {
    let mut specs = Vec::new();
    let mut dirs = vec![relative_dir.to_path_buf()];
    while let Some(current_dir) = dirs.pop() {
        let entries = match capability.entries(&current_dir) {
            Ok(entries) => entries,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => return Err(error),
        };
        for entry in entries {
            let relative = entry_relative_path(&current_dir, entry.file_name());
            if entry.file_type().is_dir() {
                dirs.push(relative);
            } else if relative.file_name().is_some_and(|name| name == "spec.md") {
                specs.push(relative);
            }
        }
    }
    specs.sort();
    Ok(specs)
}

pub(crate) fn find_spec_source(
    root: &Path,
    layout: LifecycleLayout,
    spec: &str,
) -> Result<SpecSource, Box<dyn Error>> {
    let specs_dir = root.join(specs_dir(layout));
    let Some(source) = collect_spec_sources(&specs_dir)?
        .into_iter()
        .find(|candidate| candidate.id == spec)
    else {
        return Err(format!("spec not found: {spec}").into());
    };
    Ok(source)
}

pub(crate) fn collect_validation_spec_sources(
    root: &Path,
    layout: LifecycleLayout,
) -> std::io::Result<Vec<SpecSource>> {
    let mut capability = SourceReader::open(root)?;
    let specs_dir_text = specs_dir(layout);
    let specs_relative = Path::new(&specs_dir_text);
    let specs_display = root.join(specs_relative);
    let mut sources =
        collect_spec_sources_from_root(&mut capability, specs_relative, root, &specs_display)?;
    sources.extend(collect_delta_spec_sources_from_root(
        &mut capability,
        root,
        layout,
    )?);
    sources.sort_by(|left, right| left.path.cmp(&right.path).then(left.id.cmp(&right.id)));
    Ok(sources)
}

fn collect_delta_spec_sources_from_root(
    capability: &mut SourceReader,
    root: &Path,
    layout: LifecycleLayout,
) -> std::io::Result<Vec<SpecSource>> {
    let changes_dir_text = changes_dir(layout);
    let changes_dir = Path::new(&changes_dir_text);
    let entries = match capability.entries(changes_dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error),
    };
    let mut sources = Vec::new();
    for entry in entries {
        if !entry.file_type().is_dir() {
            continue;
        }
        let change = entry.file_name().to_string_lossy().to_string();
        if is_hidden_or_archive_change(&change) {
            continue;
        }
        let change_relative = entry_relative_path(changes_dir, entry.file_name());
        let specs_relative = change_relative.join("specs");
        let display_specs_dir = root.join(&specs_relative);
        let mut delta_sources =
            collect_spec_sources_from_root(capability, &specs_relative, root, &display_specs_dir)?;
        for source in &mut delta_sources {
            source.id = format!("change/{change}/{}", source.id);
        }
        sources.extend(delta_sources);
    }
    Ok(sources)
}

fn spec_id(specs_dir: &Path, path: &Path) -> String {
    path.strip_prefix(specs_dir)
        .ok()
        .and_then(|relative| relative.parent())
        .map(path_string)
        .unwrap_or_else(|| path_string(path))
}

fn archived_change_id_from_directory(directory_name: &str) -> String {
    if has_archive_date_prefix(directory_name) {
        return directory_name[ARCHIVE_NAME_PREFIX_LEN..].to_string();
    }
    directory_name.to_string()
}

fn has_archive_date_prefix(directory_name: &str) -> bool {
    let bytes = directory_name.as_bytes();
    if bytes.len() <= ARCHIVE_NAME_PREFIX_LEN {
        return false;
    }
    bytes[ARCHIVE_DATE_SEPARATOR_INDEX] == b'-'
        && bytes[..ARCHIVE_DATE_LEN]
            .iter()
            .enumerate()
            .all(|(index, byte)| is_archive_date_byte(index, *byte))
}

fn is_archive_date_byte(index: usize, byte: u8) -> bool {
    const FIRST_DASH_INDEX: usize = 4;
    const SECOND_DASH_INDEX: usize = 7;
    if index == FIRST_DASH_INDEX || index == SECOND_DASH_INDEX {
        return byte == b'-';
    }
    byte.is_ascii_digit()
}

fn is_hidden_or_archive_change(name: &str) -> bool {
    name == "archive" || name == "_done" || name.starts_with('.')
}

pub(crate) fn find_change(
    root: PathBuf,
    layout: LifecycleLayout,
    change: &str,
) -> Result<ChangeStatus, Box<dyn Error>> {
    find_change_with_budget(root, layout, change, None)
}

pub(crate) fn find_change_with_budget(
    root: PathBuf,
    layout: LifecycleLayout,
    change: &str,
    budget: Option<&super::observation_budget::ObservationBudget>,
) -> Result<ChangeStatus, Box<dyn Error>> {
    let status = scan_repo_with_budget(root, layout, budget)?;
    let Some(change_status) = status
        .changes
        .into_iter()
        .find(|candidate| candidate.name == change)
    else {
        return Err(format!("change not found: {change}").into());
    };
    Ok(change_status)
}

/// Admit the selected change independently of unrelated repository sources.
pub(crate) fn read_selected_change_with_budget(
    root: &Path,
    layout: LifecycleLayout,
    change: &str,
    budget: Option<&super::observation_budget::ObservationBudget>,
) -> Result<ChangeStatus, Box<dyn Error>> {
    cairn_core::validate_change_name(change)?;
    let mut reader = SourceReader::open_with_budget(root, budget)?;
    let relative = PathBuf::from(changes_dir(layout)).join(change);
    reader.entries(&relative)?;
    let source = read_change_source(&mut reader, change.into(), relative)?;
    build_repo_status(path_string(root), Vec::new(), vec![source])
        .changes
        .pop()
        .ok_or_else(|| "selected change was not observed".into())
}

/// Load only inputs consumed by the selected gate. Delta references apply at every stage.
pub(crate) fn read_gate_stage_snapshot_with_budget(
    root: &Path,
    layout: LifecycleLayout,
    change: &str,
    stage: GateStage,
    budget: Option<&super::observation_budget::ObservationBudget>,
) -> Result<GateSnapshot, Box<dyn Error>> {
    cairn_core::validate_change_name(change)?;
    let mut reader = SourceReader::open_with_budget(root, budget)?;
    let relative = PathBuf::from(changes_dir(layout)).join(change);
    CapabilityRoot::open_nofollow(CapabilityRootKind::Scan, &root.join(&relative))?;
    let proposal = read_gate_artifact(&mut reader, &relative.join("proposal.md"))?;
    let design = if stage != GateStage::Proposal {
        read_gate_artifact(&mut reader, &relative.join("design.md"))?
    } else {
        None
    };
    let tasks = if stage == GateStage::Tasks {
        read_gate_artifact(&mut reader, &relative.join("tasks.md"))?
    } else {
        None
    };
    let paths = collect_spec_files(&mut reader, &relative.join("specs"))?;
    let mut delta_specs = Vec::new();
    for path in &paths {
        if let Some(artifact) = read_gate_artifact_path(&mut reader, Path::new(path))? {
            delta_specs.push(artifact);
        }
    }
    let source = ChangeSource {
        name: change.into(),
        path: path_string(root.join(&relative)),
        has_proposal: proposal.is_some(),
        has_design: design.is_some(),
        has_tasks: tasks.is_some(),
        delta_specs: paths,
        tasks_markdown: tasks.as_ref().map(|tasks| tasks.markdown.clone()),
        metadata_json: read_optional_text(&mut reader, &relative.join("metadata.json"))?,
    };
    let change = build_repo_status(path_string(root), Vec::new(), vec![source])
        .changes
        .pop()
        .ok_or("selected change was not observed")?;
    Ok(GateSnapshot {
        change,
        proposal,
        design,
        tasks,
        delta_specs,
    })
}

pub(crate) fn read_gate_snapshot(
    root: &Path,
    layout: LifecycleLayout,
    change: ChangeStatus,
) -> Result<GateSnapshot, Box<dyn Error>> {
    read_gate_snapshot_with_budget(root, layout, change, None)
}

pub(crate) fn read_gate_snapshot_with_budget(
    root: &Path,
    layout: LifecycleLayout,
    change: ChangeStatus,
    budget: Option<&super::observation_budget::ObservationBudget>,
) -> Result<GateSnapshot, Box<dyn Error>> {
    // r[impl cairn.supersede.gate_policy.references]
    // r[impl cairn.supersede.gate_policy.task_links]
    // r[impl cairn.supersede.gate_policy.exemptions]
    let mut capability = SourceReader::open_with_budget(root, budget)?;
    let changes_dir_text = changes_dir(layout);
    let change_dir = Path::new(&changes_dir_text).join(&change.name);
    let proposal = read_gate_artifact(&mut capability, &change_dir.join("proposal.md"))?;
    let design = read_gate_artifact(&mut capability, &change_dir.join("design.md"))?;
    let tasks = read_gate_artifact(&mut capability, &change_dir.join("tasks.md"))?;
    let mut delta_specs = Vec::with_capacity(change.delta_specs.len());
    for path in &change.delta_specs {
        if let Some(artifact) = read_gate_artifact_path(&mut capability, Path::new(path))? {
            delta_specs.push(artifact);
        }
    }
    Ok(GateSnapshot {
        change,
        proposal,
        design,
        tasks,
        delta_specs,
    })
}

fn read_gate_artifact(
    capability: &mut SourceReader,
    path: &Path,
) -> Result<Option<GateArtifact>, Box<dyn Error>> {
    if !path_exists(capability, path)? {
        return Ok(None);
    }
    Ok(Some(GateArtifact {
        path: path_string(capability.display_path(path)?),
        markdown: capability.read_to_string_path(&capability.display_path(path)?)?,
    }))
}

fn read_gate_artifact_path(
    capability: &mut SourceReader,
    path: &Path,
) -> Result<Option<GateArtifact>, Box<dyn Error>> {
    if !path_exists_display(capability, path)? {
        return Ok(None);
    }
    Ok(Some(GateArtifact {
        path: path_string(path),
        markdown: capability.read_to_string_path(path)?,
    }))
}

pub(crate) fn mutation_gate_blockers(
    // r[impl cairn.supersede.deterministic_gates] r[impl cairn.supersede.deterministic_gates.pre_sync]
    // r[impl cairn.supersede.deterministic_gates.pre_archive] r[impl cairn.supersede.deterministic_gates.stage]
    // r[impl cairn.supersede.deterministic_gates.traceability]
    // r[impl cairn.probe_execution_boundary.authoritative_mutation_block]
    // r[impl cairn.probe_execution_boundary.authoritative_mutation_block.missing]
    // r[impl cairn.probe_execution_boundary.authoritative_mutation_block.tampered]
    // r[impl cairn.probe_execution_boundary.authoritative_mutation_block.valid]
    root: &Path,
    layout: LifecycleLayout,
    change_status: &ChangeStatus,
    stage: GateStage,
    policy: &Policy,
) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(mutation_gate_observation(root, layout, change_status, stage, policy)?.0)
}

pub(crate) fn mutation_gate_observation(
    root: &Path,
    layout: LifecycleLayout,
    change_status: &ChangeStatus,
    stage: GateStage,
    policy: &Policy,
) -> Result<(Vec<String>, Value), Box<dyn Error>> {
    mutation_gate_observation_with_budget(root, layout, change_status, stage, policy, None)
}

pub(crate) fn mutation_gate_observation_with_budget(
    root: &Path,
    layout: LifecycleLayout,
    change_status: &ChangeStatus,
    stage: GateStage,
    policy: &Policy,
    budget: Option<&super::observation_budget::ObservationBudget>,
) -> Result<(Vec<String>, Value), Box<dyn Error>> {
    let snapshot = read_gate_snapshot_with_budget(root, layout, change_status.clone(), budget)?;
    let profile_context = crate::workflow::load_workflow_profile_context_with_budget(
        root,
        layout,
        &change_status.name,
        policy,
        budget,
    )?;
    let report = profile_context.gate_report(&snapshot, stage, policy);
    let mut blockers = report
        .issues
        .into_iter()
        .map(|issue| format!("gate {}: {}", stage.as_str(), issue.message))
        .collect::<Vec<_>>();
    let probes = matches!(stage, GateStage::Tasks).then(|| {
        crate::probe::observe_gate_probes_with_budget(
            root,
            layout,
            &change_status.name,
            policy,
            budget,
        )
    });
    if let Some(probes) = &probes {
        blockers.extend(
            probes
                .issues
                .iter()
                .map(|issue| format!("gate {}: {}: {}", stage.as_str(), issue.code, issue.message)),
        );
    }
    let mut reader = SourceReader::open_with_budget(root, budget)?;
    let accepted = PathBuf::from(specs_dir(layout));
    let mut validation_sources =
        collect_spec_sources_from_root(&mut reader, &accepted, root, &root.join(&accepted))?;
    let delta = PathBuf::from(changes_dir(layout))
        .join(&change_status.name)
        .join("specs");
    let mut delta_sources =
        collect_spec_sources_from_root(&mut reader, &delta, root, &root.join(&delta))?;
    for source in &mut delta_sources {
        source.id = format!("change/{}/{}", change_status.name, source.id);
    }
    validation_sources.extend(delta_sources);
    let binding = json!({
        "gate": crate::plan_receipts::gate_input_json(layout, stage, policy, &snapshot),
        "profile": profile_context.profile_identity_hash,
        "profile_evidence": profile_context.evidence,
        "probes": probes.as_ref().map(|probes| &probes.binding),
        "validation_sources": validation_sources.iter().map(|source| json!({
            "id": source.id,
            "path": safe_relative_path(root, Path::new(&source.path)),
            "blake3": blake3_hex(source.markdown.as_bytes()),
        })).collect::<Vec<_>>(),
    });
    blockers.extend(
        validate_spec_sources_with_policy(&validation_sources, &policy.artifact_substance_policy)
            .into_iter()
            .map(|issue| format!("validation: {issue}")),
    );
    blockers.sort();
    blockers.dedup();
    Ok((blockers, binding))
}

fn path_exists(capability: &SourceReader, relative: &Path) -> std::io::Result<bool> {
    match capability.metadata(relative) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error),
    }
}

fn path_exists_display(capability: &SourceReader, path: &Path) -> std::io::Result<bool> {
    match capability.metadata_path(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error),
    }
}

fn read_optional_text(
    capability: &mut SourceReader,
    relative: &Path,
) -> std::io::Result<Option<String>> {
    match capability.read_to_string(relative) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn entry_relative_path(directory: &Path, name: &std::ffi::OsStr) -> PathBuf {
    if directory == Path::new(".") {
        PathBuf::from(name)
    } else {
        directory.join(name)
    }
}
