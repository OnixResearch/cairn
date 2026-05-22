use super::*;

pub(crate) fn scan_repo(root: PathBuf, layout: LifecycleLayout) -> std::io::Result<RepoStatus> {
    let specs = collect_spec_files(&root.join(specs_dir(layout)))?;
    let changes = collect_change_sources(&root.join(changes_dir(layout)))?;
    Ok(build_repo_status(path_string(root), specs, changes))
}

fn collect_change_sources(changes_dir: &Path) -> std::io::Result<Vec<ChangeSource>> {
    if !changes_dir.exists() {
        return Ok(Vec::new());
    }
    let entries = fs::read_dir(changes_dir)?.collect::<Result<Vec<_>, _>>()?;
    let mut changes = Vec::with_capacity(entries.len());
    for entry in entries {
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if is_hidden_or_archive_change(&name) {
            continue;
        }
        changes.push(read_change_source(name, entry.path())?);
    }
    Ok(changes)
}

fn read_change_source(name: String, path: PathBuf) -> std::io::Result<ChangeSource> {
    let tasks_path = path.join("tasks.md");
    let tasks_markdown = if tasks_path.exists() {
        Some(fs::read_to_string(&tasks_path)?)
    } else {
        None
    };
    let metadata_path = path.join("metadata.json");
    let metadata_json = if metadata_path.exists() {
        Some(fs::read_to_string(&metadata_path)?)
    } else {
        None
    };
    Ok(ChangeSource {
        name,
        path: path_string(&path),
        has_proposal: path.join("proposal.md").exists(),
        has_design: path.join("design.md").exists(),
        has_tasks: tasks_path.exists(),
        delta_specs: collect_spec_files(&path.join("specs"))?,
        tasks_markdown,
        metadata_json,
    })
}

fn collect_spec_files(dir: &Path) -> std::io::Result<Vec<String>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut specs = Vec::new();
    let mut dirs = Vec::with_capacity(1);
    dirs.push(dir.to_path_buf());
    while let Some(current_dir) = dirs.pop() {
        let entries = fs::read_dir(&current_dir)?.collect::<Result<Vec<_>, _>>()?;
        specs.reserve(entries.len());
        for entry in entries {
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                dirs.push(path);
            } else if path.file_name().is_some_and(|name| name == "spec.md") {
                specs.push(path_string(path));
            }
        }
    }
    specs.sort();
    Ok(specs)
}

pub(crate) fn collect_spec_sources(specs_dir: &Path) -> std::io::Result<Vec<SpecSource>> {
    let spec_paths = collect_spec_files(specs_dir)?;
    let mut specs = Vec::with_capacity(spec_paths.len());
    for spec_path in spec_paths {
        let path = PathBuf::from(&spec_path);
        specs.push(SpecSource {
            id: spec_id(specs_dir, &path),
            path: spec_path,
            markdown: fs::read_to_string(path)?,
        });
    }
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
    let mut sources = collect_spec_sources(&root.join(specs_dir(layout)))?;
    sources.extend(collect_delta_spec_sources(root, layout)?);
    sources.sort_by(|left, right| left.path.cmp(&right.path).then(left.id.cmp(&right.id)));
    Ok(sources)
}

fn collect_delta_spec_sources(
    root: &Path,
    layout: LifecycleLayout,
) -> std::io::Result<Vec<SpecSource>> {
    let changes_dir = root.join(changes_dir(layout));
    if !changes_dir.exists() {
        return Ok(Vec::new());
    }
    let entries = fs::read_dir(&changes_dir)?.collect::<Result<Vec<_>, _>>()?;
    let mut sources = Vec::with_capacity(entries.len());
    for entry in entries {
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let change = entry.file_name().to_string_lossy().to_string();
        if is_hidden_or_archive_change(&change) {
            continue;
        }
        let specs_dir = entry.path().join("specs");
        let mut delta_sources = collect_spec_sources(&specs_dir)?;
        for source in &mut delta_sources {
            source.id = format!("change/{change}/{}", source.id);
        }
        sources.reserve(delta_sources.len());
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

fn is_hidden_or_archive_change(name: &str) -> bool {
    name == "archive" || name == "_done" || name.starts_with('.')
}

pub(crate) fn find_change(
    root: PathBuf,
    layout: LifecycleLayout,
    change: &str,
) -> Result<ChangeStatus, Box<dyn Error>> {
    let status = scan_repo(root, layout)?;
    let Some(change_status) = status
        .changes
        .into_iter()
        .find(|candidate| candidate.name == change)
    else {
        return Err(format!("change not found: {change}").into());
    };
    Ok(change_status)
}

pub(crate) fn read_gate_snapshot(
    root: &Path,
    layout: LifecycleLayout,
    change: ChangeStatus,
) -> Result<GateSnapshot, Box<dyn Error>> {
    // r[impl cairn.supersede.gate_policy.references]
    // r[impl cairn.supersede.gate_policy.task_links]
    // r[impl cairn.supersede.gate_policy.exemptions]
    let change_dir = root.join(changes_dir(layout)).join(&change.name);
    let proposal = read_gate_artifact(change_dir.join("proposal.md"))?;
    let design = read_gate_artifact(change_dir.join("design.md"))?;
    let tasks = read_gate_artifact(change_dir.join("tasks.md"))?;
    let mut delta_specs = Vec::with_capacity(change.delta_specs.len());
    for path in &change.delta_specs {
        if let Some(artifact) = read_gate_artifact(PathBuf::from(path))? {
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

fn read_gate_artifact(path: PathBuf) -> Result<Option<GateArtifact>, Box<dyn Error>> {
    if !path.exists() {
        return Ok(None);
    }
    Ok(Some(GateArtifact {
        path: path_string(&path),
        markdown: fs::read_to_string(path)?,
    }))
}

pub(crate) fn mutation_gate_blockers(
    // r[impl cairn.supersede.deterministic_gates] r[impl cairn.supersede.deterministic_gates.pre_sync]
    // r[impl cairn.supersede.deterministic_gates.pre_archive] r[impl cairn.supersede.deterministic_gates.stage]
    // r[impl cairn.supersede.deterministic_gates.traceability]
    root: &Path,
    layout: LifecycleLayout,
    change_status: &ChangeStatus,
    stage: GateStage,
    policy: &Policy,
) -> Result<Vec<String>, Box<dyn Error>> {
    let snapshot = read_gate_snapshot(root, layout, change_status.clone())?;
    let report = evaluate_gate_snapshot(&snapshot, stage, policy);
    let mut blockers = report
        .issues
        .into_iter()
        .map(|issue| format!("gate {}: {}", stage.as_str(), issue.message))
        .collect::<Vec<_>>();
    let mut validation_sources = collect_spec_sources(&root.join(specs_dir(layout)))?;
    for source in collect_delta_spec_sources(root, layout)? {
        if source
            .path
            .contains(&format!("/changes/{}/", change_status.name))
        {
            validation_sources.push(source);
        }
    }
    blockers.extend(
        validate_spec_sources(&validation_sources)
            .into_iter()
            .map(|issue| format!("validation: {issue}")),
    );
    blockers.sort();
    blockers.dedup();
    Ok(blockers)
}
