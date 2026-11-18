//! Stateless shell adapter for incremental lifecycle observations.
//!
//! The core owns identity, ordering, pages, and differences. This module owns
//! bounded source reads, explicit prior-snapshot admission, and guarded output.
use crate::support::{
    default_policy_path, detect_layout, load_policy_file, path_string, read_guarded_bytes,
    scan_repo, write_guarded_bytes,
};
use cairn_core::verified::incremental_observations::{
    admit_snapshot, bounds, build_snapshot, difference, observation_entry, page, policy_hash,
    unchanged, ObservationCohort, ObservationEntry, ObservationPageRequest, ObservationSnapshot,
    ObservationView,
};
use cairn_core::verified::validation::ContextSelector;
use cairn_core::{blake3_hex, canonical_json_hash, RepoStatus};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    error::Error,
    io::Write,
    path::{Path, PathBuf},
};

fn canonical_root(root: &Path) -> Result<String, Box<dyn Error>> {
    Ok(std::fs::canonicalize(root)?.display().to_string())
}

fn required_str<'a>(input: &'a Value, key: &str) -> Result<&'a str, Box<dyn Error>> {
    input[key]
        .as_str()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("observation requires a non-empty {key}").into())
}

fn usize_option(input: &Value, key: &str) -> Result<Option<usize>, Box<dyn Error>> {
    let Some(value) = input.get(key) else {
        return Ok(None);
    };
    let number = value
        .as_u64()
        .ok_or_else(|| format!("{key} must be a non-negative integer"))?;
    usize::try_from(number)
        .map(Some)
        .map_err(|_| format!("{key} exceeds the platform integer bound").into())
}

fn array<'a>(value: &'a Value, key: &str) -> Result<&'a Vec<Value>, Box<dyn Error>> {
    value[key]
        .as_array()
        .ok_or_else(|| format!("admitted observation view is missing its {key} array").into())
}

fn count(value: &Value, key: &str) -> Result<usize, Box<dyn Error>> {
    value[key]
        .as_u64()
        .and_then(|number| usize::try_from(number).ok())
        .ok_or_else(|| format!("admitted observation view is missing count {key}").into())
}

type ObservationParts = (ObservationCohort, Vec<ObservationEntry>, usize, bool);

fn active_observation(root: &Path, input: &Value) -> Result<ObservationParts, Box<dyn Error>> {
    let policy_path = PathBuf::from(
        input
            .get("policy")
            .and_then(Value::as_str)
            .unwrap_or(default_policy_path()),
    );
    let policy = load_policy_file(policy_path)?;
    let lifecycle_policy_hash = canonical_json_hash(&policy.raw);
    let root_identity = canonical_root(root)?;
    let layout = detect_layout(root).layout;
    let status = scan_repo(PathBuf::from(root_identity.clone()), layout)?;
    let entries = status
        .changes
        .iter()
        .map(|change| {
            observation_entry(
                format!("change:{}", change.name),
                crate::native::change_status_json(change),
            )
            .map_err(|error| -> Box<dyn Error> { Box::new(error) })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    let source_manifest = active_source_manifest(Path::new(root_identity.as_str()), &status)?;
    let cohort = ObservationCohort::new(
        root_identity,
        None,
        source_manifest,
        lifecycle_policy_hash,
        policy_hash().to_string(),
        canonical_json_hash(&Value::Null),
        ObservationView::ActiveChanges,
    )?;
    Ok((cohort, entries, 0, true))
}

fn active_source_manifest(root: &Path, status: &RepoStatus) -> Result<String, Box<dyn Error>> {
    let mut files = BTreeMap::new();
    let mut budget = bounds().max_observation_bytes as u64;
    for change in &status.changes {
        let base = Path::new(&change.path);
        for name in ["proposal.md", "design.md", "tasks.md", "metadata.json"] {
            insert_source(root, &mut files, &base.join(name), &mut budget)?;
        }
        for spec in &change.delta_specs {
            insert_source(root, &mut files, Path::new(spec), &mut budget)?;
        }
    }
    Ok(canonical_json_hash(&json!(files)))
}

fn insert_source(
    root: &Path,
    files: &mut BTreeMap<String, Option<String>>,
    path: &Path,
    budget: &mut u64,
) -> Result<(), Box<dyn Error>> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| "observed source escaped the admitted root")?;
    let key = relative.to_string_lossy().replace('\\', "/");
    if files.contains_key(&key) {
        return Ok(());
    }
    let limit = (*budget).min(1024 * 1024);
    if limit == 0 {
        return Err("active source manifest exceeds the observation byte bound".into());
    }
    let hash = match read_guarded_bytes(root, relative, limit) {
        Ok(bytes) => {
            *budget = budget.saturating_sub(bytes.len() as u64);
            Some(blake3_hex(&bytes))
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    };
    files.insert(key, hash);
    Ok(())
}

fn readiness_observation(root: &Path, input: &Value) -> Result<ObservationParts, Box<dyn Error>> {
    let change = required_str(input, "change")?;
    let mut readiness_input = json!({
        "change": change,
        "root": path_string(root),
        "view": "agent",
    });
    if let Some(policy) = input.get("policy").and_then(Value::as_str) {
        readiness_input["policy"] = json!(policy);
    }
    for (source, target) in [("max_bytes", "max_bytes"), ("max_entries", "max_entries")] {
        if let Some(value) = usize_option(input, source)? {
            readiness_input[target] = json!(value);
        }
    }
    let view = crate::readiness::readiness_value(&readiness_input)?;
    let stages = array(&view, "stages")?;
    let blockers = array(&view, "blockers")?;
    let actions = array(&view, "actions")?;
    let blocker_count = count(&view, "blocker_count")?;
    let action_count = count(&view, "action_count")?;
    let omitted_blockers = count(&view, "omitted_blockers")?;
    let omitted_actions = count(&view, "omitted_actions")?;
    if blockers.len().checked_add(omitted_blockers) != Some(blocker_count)
        || actions.len().checked_add(omitted_actions) != Some(action_count)
    {
        return Err("readiness view selection counts disagree with its entries".into());
    }
    let mut entries = Vec::with_capacity(stages.len() + blockers.len() + actions.len());
    for stage in stages {
        let id = stage["stage"]
            .as_str()
            .ok_or("readiness stage entry has no stable identity")?;
        entries.push(observation_entry(format!("stage:{id}"), stage.clone())?);
    }
    for blocker in blockers {
        let id = blocker["id"]
            .as_str()
            .ok_or("readiness blocker entry has no stable identity")?;
        entries.push(observation_entry(format!("blocker:{id}"), blocker.clone())?);
    }
    for action in actions {
        let id = action["id"]
            .as_str()
            .ok_or("readiness action entry has no stable identity")?;
        entries.push(observation_entry(format!("action:{id}"), action.clone())?);
    }
    let source_manifest = canonical_json_hash(&json!({
        "input_hash": view["input_hash"],
        "source_hash": view["scope"]["source_hash"],
    }));
    let evidence_hash = view["scope"]["evidence_hash"]
        .as_str()
        .ok_or("readiness view has no evidence identity")?;
    let policy_hash = view["readiness_policy_hash"]
        .as_str()
        .ok_or("readiness view has no lifecycle policy identity")?;
    let producer_policy_hash = view["view_policy_hash"]
        .as_str()
        .ok_or("readiness view has no producer policy identity")?;
    let omitted = omitted_blockers + omitted_actions;
    let cohort = ObservationCohort::new(
        canonical_root(root)?,
        None,
        source_manifest,
        policy_hash,
        producer_policy_hash,
        evidence_hash,
        ObservationView::Readiness {
            change: change.into(),
            max_bytes: usize_option(input, "max_bytes")?,
            max_entries: usize_option(input, "max_entries")?,
        },
    )?;
    // Stage decisions remain detail-only. Completeness covers the selected
    // stage, blocker, and action entries this view exposes, not stage detail.
    Ok((cohort, entries, omitted, omitted == 0))
}

fn context_selectors(input: &Value) -> Result<Vec<ContextSelector>, Box<dyn Error>> {
    let shorthand = ["requirement_id", "task_id", "heading_path"];
    if input.get("selectors").is_some() {
        if shorthand.iter().any(|key| input.get(*key).is_some()) {
            return Err(
                "context observation accepts selectors or one shorthand selector, not both".into(),
            );
        }
        let values = input["selectors"]
            .as_array()
            .ok_or("context selectors must be an array")?;
        return values
            .iter()
            .map(|value| {
                serde_json::from_value::<ContextSelector>(value.clone())
                    .map_err(|error| format!("invalid context selector: {error}").into())
            })
            .collect();
    }
    let mut selectors = Vec::new();
    if let Some(id) = input.get("requirement_id").and_then(Value::as_str) {
        selectors.push(ContextSelector::Requirement { id: id.into() });
    }
    if let Some(id) = input.get("task_id").and_then(Value::as_str) {
        selectors.push(ContextSelector::Task { id: id.into() });
    }
    if let Some(path) = input.get("heading_path").and_then(Value::as_array) {
        let path = path
            .iter()
            .map(|part| {
                part.as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| "heading_path entries must be strings".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;
        selectors.push(ContextSelector::Section { path });
    }
    if selectors.is_empty() {
        return Err("context observation requires a selector".into());
    }
    Ok(selectors)
}

fn context_identity(selector: &ContextSelector) -> String {
    match selector {
        ContextSelector::Requirement { id } => format!("context:requirement:{id}"),
        ContextSelector::Task { id } => format!("context:task:{id}"),
        ContextSelector::Section { path } => {
            format!("context:section:{}", canonical_json_hash(&json!(path)))
        }
    }
}

fn context_observation(root: &Path, input: &Value) -> Result<ObservationParts, Box<dyn Error>> {
    if input.get("policy").is_some() {
        return Err(
            "context observation uses the admitted targeted-context policy and accepts no override"
                .into(),
        );
    }
    let selectors = context_selectors(input)?;
    if selectors.len() > bounds().max_context_selectors {
        return Err("context observation exceeds the selector policy bound".into());
    }
    let scope = required_str(input, "scope")?.to_string();
    let artifact = required_str(input, "artifact")?.to_string();
    let change = input
        .get("change")
        .and_then(Value::as_str)
        .map(str::to_owned);
    let spec = input.get("spec").and_then(Value::as_str).map(str::to_owned);
    let mut entries = Vec::with_capacity(selectors.len());
    let mut source_binding: Option<(String, String)> = None;
    for selector in &selectors {
        let request = crate::targeted_context::ReadRequest {
            root: path_string(root),
            scope: scope.clone(),
            artifact: artifact.clone(),
            change: change.clone(),
            spec: spec.clone(),
            requirement_id: match selector {
                ContextSelector::Requirement { id } => Some(id.clone()),
                _ => None,
            },
            task_id: match selector {
                ContextSelector::Task { id } => Some(id.clone()),
                _ => None,
            },
            heading_path: match selector {
                ContextSelector::Section { path } => Some(path.clone()),
                _ => None,
            },
            expected_source_blake3: None,
        };
        let value = crate::targeted_context::read_value(request)?;
        if value["schema"].as_str() != Some("cairn.targeted-context-read.v1") {
            return Err("targeted context read returned an unsupported schema".into());
        }
        let path = value["path"]
            .as_str()
            .ok_or("targeted context read has no admitted path")?
            .to_string();
        let source = value["source_blake3"]
            .as_str()
            .ok_or("targeted context read has no source identity")?
            .to_string();
        match &source_binding {
            Some((bound_path, bound_source)) if bound_path != &path || bound_source != &source => {
                return Err(
                    "context source changed between selectors; request a complete observation"
                        .into(),
                )
            }
            Some(_) => {}
            None => source_binding = Some((path, source)),
        }
        // The admitted source identity is bound once at cohort level; repeating it
        // inside every entry would mark all selectors changed on any document edit.
        let mut entry_value = value;
        if let Some(object) = entry_value.as_object_mut() {
            object.remove("source_blake3");
        }
        entries.push(observation_entry(context_identity(selector), entry_value)?);
    }
    let (path, source) = source_binding.ok_or("context observation selected no source")?;
    let source_manifest = canonical_json_hash(&json!({
        "path": path,
        "source_blake3": source,
    }));
    let policy_hash = entries[0].content["global_policy_blake3"]
        .as_str()
        .ok_or("targeted context read has no global policy identity")?;
    let producer_policy_hash = entries[0].content["targeted_policy_blake3"]
        .as_str()
        .ok_or("targeted context read has no targeted policy identity")?;
    let evidence_hash = canonical_json_hash(&json!(entries
        .iter()
        .map(|entry| entry.content["selection_blake3"].clone())
        .collect::<Vec<_>>()));
    let cohort = ObservationCohort::new(
        canonical_root(root)?,
        None,
        source_manifest,
        policy_hash,
        producer_policy_hash,
        evidence_hash,
        ObservationView::Context {
            scope,
            change,
            spec,
            artifact,
            selectors,
        },
    )?;
    Ok((cohort, entries, 0, true))
}

fn reject_view_arguments(input: &Value, view: &str, keys: &[&str]) -> Result<(), Box<dyn Error>> {
    for key in keys {
        if input.get(*key).is_some() {
            return Err(format!("{view} observation does not accept {key}").into());
        }
    }
    Ok(())
}

fn build_observation(root: &Path, input: &Value) -> Result<ObservationSnapshot, Box<dyn Error>> {
    let view = required_str(input, "view")?;
    let (cohort, entries, omitted, complete) = match view {
        "active_changes" | "active" => {
            reject_view_arguments(
                input,
                "active",
                &[
                    "change",
                    "max_bytes",
                    "max_entries",
                    "scope",
                    "spec",
                    "artifact",
                    "requirement_id",
                    "task_id",
                    "heading_path",
                    "selectors",
                ],
            )?;
            active_observation(root, input)?
        }
        "readiness" => {
            reject_view_arguments(
                input,
                "readiness",
                &[
                    "scope",
                    "spec",
                    "artifact",
                    "requirement_id",
                    "task_id",
                    "heading_path",
                    "selectors",
                ],
            )?;
            readiness_observation(root, input)?
        }
        "context" => {
            reject_view_arguments(input, "context", &["max_bytes", "max_entries"])?;
            context_observation(root, input)?
        }
        other => return Err(format!("unsupported observation view: {other}").into()),
    };
    Ok(build_snapshot(cohort, entries, omitted, complete)?)
}

fn resolve_mode(input: &Value) -> Result<&'static str, Box<dyn Error>> {
    let inferred = if input.get("prior").is_some() || input.get("prior_path").is_some() {
        "difference"
    } else if input.get("cursor").is_some()
        || input.get("page_entries").is_some()
        || input.get("page_bytes").is_some()
    {
        "page"
    } else {
        "complete"
    };
    let mode = match input.get("mode") {
        None => inferred,
        Some(value) => value.as_str().ok_or("observation mode must be a string")?,
    };
    match mode {
        "complete" | "page" | "unchanged" | "difference" => Ok(match mode {
            "complete" => "complete",
            "page" => "page",
            "unchanged" => "unchanged",
            _ => "difference",
        }),
        other => Err(format!("unsupported observation mode: {other}").into()),
    }
}

fn prior_value(root: &Path, input: &Value) -> Result<Value, Box<dyn Error>> {
    match (input.get("prior"), input.get("prior_path")) {
        (Some(_), Some(_)) => Err("observation accepts one explicit prior source".into()),
        (Some(prior), None) => {
            let size = serde_json::to_vec(prior)?.len();
            if size > bounds().max_prior_bytes {
                return Err("inline prior observation exceeds the policy byte bound".into());
            }
            Ok(prior.clone())
        }
        (None, Some(path)) => {
            let path = path.as_str().ok_or("prior_path must be a string")?;
            let bytes = read_guarded_bytes(root, Path::new(path), bounds().max_prior_bytes as u64)?;
            Ok(serde_json::from_slice(&bytes)
                .map_err(|error| format!("prior observation is malformed or truncated: {error}"))?)
        }
        (None, None) => Err(
            "unchanged and difference observations require an explicit prior observation".into(),
        ),
    }
}

fn page_request(input: &Value) -> Result<ObservationPageRequest, Box<dyn Error>> {
    Ok(ObservationPageRequest {
        cursor: input
            .get("cursor")
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| "cursor must be a string".to_string())
            })
            .transpose()?,
        max_entries: usize_option(input, "page_entries")?,
        max_bytes: usize_option(input, "page_bytes")?,
    })
}

fn reject_unknown(input: &Value) -> Result<(), Box<dyn Error>> {
    const ALLOWED: &[&str] = &[
        "root",
        "policy",
        "view",
        "change",
        "max_bytes",
        "max_entries",
        "scope",
        "spec",
        "artifact",
        "requirement_id",
        "task_id",
        "heading_path",
        "selectors",
        "mode",
        "page_entries",
        "page_bytes",
        "cursor",
        "prior",
        "prior_path",
    ];
    let object = input
        .as_object()
        .ok_or("observation request must be an object")?;
    for key in object.keys() {
        if !ALLOWED.contains(&key.as_str()) {
            return Err(format!("unsupported observation request field: {key}").into());
        }
    }
    Ok(())
}

// r[impl cairn.incremental_observations.storage]
pub(crate) fn observe_value(input: &Value) -> Result<Value, Box<dyn Error>> {
    reject_unknown(input)?;
    let root = PathBuf::from(input["root"].as_str().unwrap_or("."));
    let snapshot = build_observation(&root, input)?;
    let mode = resolve_mode(input)?;
    match mode {
        "complete" => {
            if input.get("cursor").is_some()
                || input.get("page_entries").is_some()
                || input.get("page_bytes").is_some()
            {
                return Err("complete observation does not accept page options".into());
            }
            if input.get("prior").is_some() || input.get("prior_path").is_some() {
                return Err("complete observation does not accept a prior observation".into());
            }
            Ok(json!(snapshot))
        }
        "page" => {
            if input.get("prior").is_some() || input.get("prior_path").is_some() {
                return Err("page observation does not accept a prior observation".into());
            }
            Ok(json!(page(&snapshot, page_request(input)?)?))
        }
        "unchanged" | "difference" => {
            if input.get("cursor").is_some()
                || input.get("page_entries").is_some()
                || input.get("page_bytes").is_some()
            {
                return Err(
                    "unchanged and difference observations do not accept page options".into(),
                );
            }
            let prior = admit_snapshot(&prior_value(&root, input)?)?;
            if mode == "unchanged" {
                Ok(json!(unchanged(&snapshot, &prior)?))
            } else {
                Ok(json!(difference(&snapshot, &prior)?))
            }
        }
        _ => unreachable!("resolved observation mode"),
    }
}

pub(crate) fn mcp_observe(input: &Value) -> Result<Value, Box<dyn Error>> {
    if input.get("output").is_some() {
        return Err("cairn_observe does not write output files".into());
    }
    observe_value(input)
}

// r[impl cairn.incremental_observations.storage]
pub(crate) fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let (input, output) = parse_cli(args)?;
    let value = observe_value(&input)?;
    let mut rendered = serde_json::to_vec(&value)?;
    rendered.push(b'\n');
    if let Some(output) = output {
        let root = PathBuf::from(input["root"].as_str().unwrap_or("."));
        write_guarded_bytes(&root, Path::new(&output), &rendered)?;
    }
    std::io::stdout().write_all(&rendered)?;
    Ok(())
}

fn parse_cli(args: Vec<String>) -> Result<(Value, Option<String>), Box<dyn Error>> {
    let view = match args.first().map(String::as_str) {
        Some("active") => "active_changes",
        Some("readiness") => "readiness",
        Some("context") => "context",
        Some(_) | None => {
            return Err("usage: cairn observe active|readiness CHANGE|context [options]".into())
        }
    };
    let mut input = serde_json::Map::new();
    input.insert("view".into(), json!(view));
    let mut iter = args.into_iter().skip(1);
    let mut output = None;
    let mut headings = Vec::new();
    let mut selector_values = Vec::new();
    while let Some(argument) = iter.next() {
        let (key, target) = match argument.as_str() {
            "--root" => ("root", "root"),
            "--policy" => ("policy", "policy"),
            "--change" => ("change", "change"),
            "--max-bytes" => ("max_bytes", "max_bytes"),
            "--max-entries" => ("max_entries", "max_entries"),
            "--scope" => ("scope", "scope"),
            "--spec" => ("spec", "spec"),
            "--artifact" => ("artifact", "artifact"),
            "--mode" => ("mode", "mode"),
            "--page-entries" => ("page_entries", "page_entries"),
            "--page-bytes" => ("page_bytes", "page_bytes"),
            "--cursor" => ("cursor", "cursor"),
            "--prior" => ("prior_path", "prior_path"),
            "--output" => ("output", "output"),
            "--requirement-id" => ("requirement_id", "requirement_id"),
            "--task-id" => ("task_id", "task_id"),
            "--heading" => {
                let value = iter.next().ok_or("--heading requires a value")?;
                headings.push(value);
                continue;
            }
            other
                if !other.starts_with('-')
                    && view == "readiness"
                    && !input.contains_key("change") =>
            {
                input.insert("change".into(), json!(other));
                continue;
            }
            other => return Err(format!("unknown observe argument: {other}").into()),
        };
        let value = iter
            .next()
            .ok_or_else(|| format!("{argument} requires a value"))?;
        if target == "output" {
            if output.is_some() {
                return Err("duplicate --output".into());
            }
            output = Some(value);
            continue;
        }
        if matches!(target, "requirement_id" | "task_id") {
            selector_values.push(if target == "requirement_id" {
                json!({"kind": "requirement", "id": value})
            } else {
                json!({"kind": "task", "id": value})
            });
            continue;
        }
        if matches!(target, "max_bytes" | "max_entries" | "page_entries" | "page_bytes") {
            let number: u64 = value
                .parse()
                .map_err(|_| format!("{argument} requires a non-negative integer"))?;
            if input.insert(key.into(), json!(number)).is_some() {
                return Err(format!("duplicate observe argument: {argument}").into());
            }
            continue;
        }
        if input.insert(key.into(), json!(value)).is_some() {
            return Err(format!("duplicate observe argument: {argument}").into());
        }
    }
    if !headings.is_empty() {
        selector_values.push(json!({"kind": "section", "path": headings}));
    }
    if !selector_values.is_empty() {
        input.insert("selectors".into(), json!(selector_values));
    }
    if view == "readiness" && !input.contains_key("change") {
        return Err("readiness observation requires a change".into());
    }
    Ok((Value::Object(input), output))
}
