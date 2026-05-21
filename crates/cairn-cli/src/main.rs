use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

mod agent_receipt;
mod determinism;
mod evidence;
mod evidence_manifest;
mod evidence_registry;
mod lifecycle;
mod mcp;
mod migration;
mod native;
mod plan_receipts;
mod policy;
mod release_readiness;
mod rename_classification;
mod test_harness;
mod tracey;

mod content_addressed_evidence;

use determinism::run_determinism;
use evidence::run_evidence;
use evidence_manifest::run_evidence_manifest;
use mcp::run_mcp;
use native::{run_change, run_doctor, run_init, run_spec, run_status, run_validate};
use policy::run_policy;
use tracey::run_tracey;

use cairn_core::{
    archive_dir, blake3_hex, build_repo_status, canonical_json_hash, changes_dir,
    evaluate_gate_snapshot, merge_delta_spec, parse_policy_json, specs_dir, validate_spec_sources,
    ChangeSource, ChangeStatus, GateArtifact, GateSnapshot, GateStage, LifecycleLayout, Policy,
    RepoStatus, SpecSource, SyncPlan,
};
use serde_json::{json, Value};

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let invoked_as = env::args().next().unwrap_or_default();
    if invoked_as.ends_with("ironspec") {
        eprintln!("warning: `ironspec` is a compatibility alias; use canonical `cairn`");
    }
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        print_usage();
        return Ok(());
    };

    match command.as_str() {
        "--help" | "-h" | "help" => print_usage(),
        "init" => run_init(args.collect())?,
        "change" => run_change(args.collect())?,
        "spec" => run_spec(args.collect())?,
        "status" => run_status(args.collect())?,
        "doctor" => run_doctor(args.collect())?,
        "validate" => run_validate(args.collect())?,
        "release-readiness" => release_readiness::run_release_readiness(args.collect())?,
        "determinism" => run_determinism(args.collect())?,
        "agent-receipt" => agent_receipt::run_agent_receipt(args.collect())?,
        "evidence" => run_evidence(args.collect())?,
        "evidence-manifest" => run_evidence_manifest(args.collect())?,
        "policy" => run_policy(args.collect())?,
        "tracey" => run_tracey(args.collect())?,
        "test-harness" => test_harness::run_test_harness(args.collect())?,
        "rename-classification" => {
            rename_classification::run_rename_classification(args.collect())?
        }
        "migrate" => migration::run_migrate(args.collect())?,
        "mcp" => run_mcp(args.collect())?,
        "gate" => lifecycle::run_gate(args.collect())?,
        "gate-context" => {
            let Some(change) = args.next() else {
                return Err("gate-context requires a change name".into());
            };
            lifecycle::run_change_context(args.collect(), &change)?;
        }
        "sync" => {
            let Some(change) = args.next() else {
                return Err("sync requires a change name".into());
            };
            lifecycle::run_sync(args.collect(), &change)?;
        }
        "archive" => {
            let Some(change) = args.next() else {
                return Err("archive requires a change name".into());
            };
            lifecycle::run_archive(args.collect(), &change)?;
        }
        unknown => return Err(format!("unknown command: {unknown}\n\n{}", usage_text()).into()),
    }
    Ok(())
}

pub(crate) fn safe_relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

#[derive(Debug, Clone, Copy)]
struct SelectedLayout {
    layout: LifecycleLayout,
}

fn detect_layout(_root: &Path) -> SelectedLayout {
    SelectedLayout {
        layout: LifecycleLayout::Native,
    }
}

pub(crate) fn load_policy_file(path: PathBuf) -> Result<Policy, Box<dyn Error>> {
    let text = fs::read_to_string(&path)
        .map_err(|source| format!("failed to read policy {}: {source}", path.display()))?;
    parse_policy_json(&text)
        .map_err(|source| format!("failed to parse policy {}: {source}", path.display()).into())
}

fn scan_repo(root: PathBuf, layout: LifecycleLayout) -> std::io::Result<RepoStatus> {
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

fn collect_spec_sources(specs_dir: &Path) -> std::io::Result<Vec<SpecSource>> {
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

fn find_spec_source(
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

fn collect_validation_spec_sources(
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

pub(crate) fn execute_sync_plan(
    root: &Path,
    layout: LifecycleLayout,
    change: &str,
    plan: &SyncPlan,
) -> Result<(), Box<dyn Error>> {
    let delta_root = root.join(changes_dir(layout)).join(change).join("specs");
    let specs_root = root.join(specs_dir(layout));
    for action in &plan.actions {
        let source = PathBuf::from(&action.path);
        let relative = source.strip_prefix(&delta_root).map_err(|_| {
            format!(
                "delta spec {} is outside expected change specs root {}",
                source.display(),
                delta_root.display()
            )
        })?;
        let destination = specs_root.join(relative);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        let delta_markdown = fs::read_to_string(&source).map_err(|source_error| {
            format!(
                "failed to read delta spec {}: {source_error}",
                source.display()
            )
        })?;
        let accepted_markdown = if destination.exists() {
            Some(fs::read_to_string(&destination).map_err(|source_error| {
                format!(
                    "failed to read accepted spec {}: {source_error}",
                    destination.display()
                )
            })?)
        } else {
            None
        };
        let spec_id = relative
            .parent()
            .map(path_string)
            .unwrap_or_else(|| path_string(relative));
        let merged = merge_delta_spec(accepted_markdown.as_deref(), &delta_markdown, &spec_id);
        fs::write(&destination, merged.markdown).map_err(|source_error| {
            format!(
                "failed to sync {} to {}: {source_error}",
                source.display(),
                destination.display()
            )
        })?;
    }
    Ok(())
}

pub(crate) fn execute_archive_plan(
    root: &Path,
    layout: LifecycleLayout,
    change: &str,
    change_path: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let source = PathBuf::from(change_path);
    let archive_date = archive_date()?;
    let destination = root
        .join(archive_dir(layout))
        .join(format!("{archive_date}-{change}"));
    if destination.exists() {
        return Err(format!(
            "archive destination already exists: {}",
            destination.display()
        )
        .into());
    }
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::rename(&source, &destination).map_err(|source_error| {
        format!(
            "failed to archive {} to {}: {source_error}",
            source.display(),
            destination.display()
        )
    })?;
    Ok(destination)
}

pub(crate) fn collect_files(root: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut dirs = vec![root.to_path_buf()];
    while let Some(current_dir) = dirs.pop() {
        let entries = fs::read_dir(&current_dir)?.collect::<Result<Vec<_>, _>>()?;
        files.reserve(entries.len());
        dirs.reserve(entries.len());
        for entry in entries {
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                dirs.push(path);
            } else {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

fn archive_date() -> Result<String, Box<dyn Error>> {
    let date = env::var("CAIRN_ARCHIVE_DATE")
        .or_else(|_| env::var("IRONSPEC_ARCHIVE_DATE"))
        .unwrap_or_else(|_| "1970-01-01".to_string());
    if is_archive_date(&date) {
        Ok(date)
    } else {
        Err(format!(
            "invalid archive date: expected YYYY-MM-DD in CAIRN_ARCHIVE_DATE or IRONSPEC_ARCHIVE_DATE, got {date:?}"
        )
        .into())
    }
}

fn is_archive_date(date: &str) -> bool {
    let bytes = date.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| index == 4 || index == 7 || byte.is_ascii_digit())
}

pub(crate) fn portable_json_hash(root: &Path, value: &Value) -> String {
    canonical_json_hash(&portable_json(root, value))
}

fn portable_json(root: &Path, value: &Value) -> Value {
    match value {
        Value::Array(items) => {
            Value::Array(items.iter().map(|item| portable_json(root, item)).collect())
        }
        Value::Object(object) => Value::Object(
            object
                .iter()
                .map(|(key, value)| (key.clone(), portable_json(root, value)))
                .collect(),
        ),
        Value::String(text) => Value::String(portable_text(root, text)),
        other => other.clone(),
    }
}

fn portable_text(root: &Path, text: &str) -> String {
    let mut normalized = text.to_string();
    let root_text = path_string(root);
    if !root_text.is_empty() && root_text != "." {
        normalized = normalized.replace(&root_text, ".");
    }
    if let Ok(canonical_root) = root.canonicalize() {
        let canonical_text = path_string(canonical_root);
        if !canonical_text.is_empty() && canonical_text != "." {
            normalized = normalized.replace(&canonical_text, ".");
        }
    }
    normalized
}

pub(crate) fn file_content_manifest(root: &Path, paths: &[PathBuf]) -> Value {
    let mut unique = paths.to_vec();
    unique.sort();
    unique.dedup();
    let entries = unique
        .iter()
        .map(|path| {
            let exists = path.exists();
            let content_hash = if exists {
                fs::read(path).ok().map(|bytes| blake3_hex(&bytes))
            } else {
                None
            };
            json!({
                "path": portable_text(root, &path_string(path)),
                "exists": exists,
                "content_hash": content_hash,
            })
        })
        .collect::<Vec<_>>();
    let manifest = json!({"entries": entries});
    json!({
        "entries": manifest["entries"].clone(),
        "manifest_hash": portable_json_hash(root, &manifest),
    })
}

pub(crate) fn tree_content_paths(root: &Path, dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(vec![dir.to_path_buf()]);
    }
    let mut files = collect_files(dir)?;
    if files.is_empty() {
        files.push(dir.to_path_buf());
    }
    Ok(files
        .into_iter()
        .map(|path| root.join(path.strip_prefix(root).unwrap_or(&path)))
        .collect())
}

pub(crate) fn sync_mutation_paths(
    root: &Path,
    layout: LifecycleLayout,
    change: &str,
    plan: &SyncPlan,
) -> Vec<PathBuf> {
    let delta_root = root.join(changes_dir(layout)).join(change).join("specs");
    let specs_root = root.join(specs_dir(layout));
    plan.actions
        .iter()
        .filter_map(|action| {
            let source = PathBuf::from(&action.path);
            source
                .strip_prefix(&delta_root)
                .ok()
                .map(|relative| specs_root.join(relative))
        })
        .collect()
}

pub(crate) fn sync_mutation_manifest(
    root: &Path,
    layout: LifecycleLayout,
    change: &str,
    plan: &SyncPlan,
    before: &Value,
) -> Value {
    let paths = sync_mutation_paths(root, layout, change, plan);
    let after = file_content_manifest(root, &paths);
    json!({
        "kind": "sync",
        "before": before,
        "after": after,
        "manifest_hash": portable_json_hash(root, &json!({"before": before, "after": after})),
    })
}

pub(crate) fn archive_mutation_manifest(
    root: &Path,
    source: &Path,
    destination: &Path,
    before: &Value,
) -> Value {
    let mut after_paths =
        tree_content_paths(root, destination).unwrap_or_else(|_| vec![destination.to_path_buf()]);
    after_paths.push(source.to_path_buf());
    let after = file_content_manifest(root, &after_paths);
    json!({
        "kind": "archive",
        "before": before,
        "after": after,
        "manifest_hash": portable_json_hash(root, &json!({"before": before, "after": after})),
    })
}

pub(crate) fn path_string(path: impl Into<PathBuf>) -> String {
    path.into().display().to_string()
}

pub(crate) fn read_string_option(args: &[String], name: &str) -> Option<String> {
    args.windows(2)
        .find_map(|pair| (pair[0] == name).then(|| pair[1].clone()))
}

pub(crate) fn read_option(args: Vec<String>, name: &str, default: &str) -> PathBuf {
    args.windows(2)
        .find_map(|pair| (pair[0] == name).then(|| PathBuf::from(&pair[1])))
        .unwrap_or_else(|| PathBuf::from(default))
}

pub(crate) fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|arg| arg == name)
}

pub(crate) fn print_json(value: Value) -> Result<(), Box<dyn Error>> {
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

fn with_receipt_hash(mut value: Value) -> Value {
    let receipt_hash = canonical_json_hash(&value);
    value["receipt_hash"] = json!(receipt_hash);
    value
}

pub(crate) fn with_portable_receipt_hash(root: &Path, mut value: Value) -> Value {
    let receipt_hash = portable_json_hash(root, &value);
    value["receipt_hash"] = json!(receipt_hash);
    value
}

pub(crate) fn default_policy_path() -> &'static str {
    "cairn-policy/generated/cairn-policy.json"
}

fn usage_text() -> &'static str {
    // r[impl cairn.supersede.hermes_skill] r[impl cairn.supersede.hermes_skill.discover]
    // r[impl cairn.supersede.hermes_skill.contracts] r[impl cairn.supersede.hermes_skill.audit]
    "usage: cairn <command> [args]\n\ncommands:\n  init [--root PATH]\n  change create <name> [--root PATH]\n  change list [--root PATH] [--group LABEL] [--target-after YYYY-MM-DD] [--target-before YYYY-MM-DD]\n  change show <name> [--root PATH]\n  spec list [--root PATH]\n  spec show <id> [--root PATH]\n  status [--root PATH]\n  doctor [--root PATH] [--policy PATH]\n  validate [--root PATH] [--policy PATH] [--strict]\n  release-readiness [--root PATH] [--policy PATH]\n  determinism coverage-audit [--root PATH]\n  determinism replay [--root PATH] [--case ID] [--group ID] [--format json|jsonl|summary] [--out PATH]\n  determinism verify-receipt <receipt.json> [--root PATH]\n  agent-receipt validate <path>\n  agent-receipt verify-bundle <bundle.json> [--root PATH]\n  agent-receipt smoke-mcp [--root PATH]\n  evidence export --root PATH --out DIR [--agent-receipt PATH]\n  evidence verify --root PATH --bundle DIR\n  evidence import --root PATH --bundle DIR --into DIR\n  evidence republish --root PATH --imported DIR --out DIR\n  evidence index --root PATH --bundle DIR --out FILE\n  evidence index verify --root PATH --index FILE\n  evidence registry build --root PATH --index FILE [--index FILE ...] --out FILE [--channel CHANNEL] [--release YYYY-MM-DD] [--platform PLATFORM] [--compatibility-class CLASS]\n  evidence registry verify --root PATH --registry FILE\n  evidence registry mirror-verify --root PATH --registry FILE --mirror DIR\n  evidence registry select --root PATH --registry FILE --out FILE [--bundle-kind KIND] [--policy-hash HASH] [--channel CHANNEL] [--release YYYY-MM-DD] [--platform PLATFORM] [--compatibility-class CLASS] [--index-receipt-hash HASH]\n  evidence registry fetch-plan --root PATH --registry FILE --out FILE [--bundle-kind KIND] [--policy-hash HASH] [--channel CHANNEL] [--release YYYY-MM-DD] [--platform PLATFORM] [--compatibility-class CLASS]\n  evidence-manifest generate --bundle PATH --agent-receipt PATH --replay PATH --replay-verification PATH [--release-readiness PATH] [--coverage-audit PATH] [--registry PATH] [--registry-verification PATH] [--registry-selection PATH] [--registry-fetch-plan PATH] [--signer-metadata PATH] [--anchor-metadata PATH] [--out PATH|-] [--root PATH]\n  evidence-manifest verify <manifest.json> [--root PATH]\n  policy [--policy PATH]\n  policy export [--source PATH] [--output PATH] [--check]\n  tracey coverage [--root PATH] [--json]\n  test-harness [--root PATH] [--case pass|product-fail|harness-fail|environment-fail|timeout-flake] [--rail validate|tracey-coverage|release-readiness|policy-export-check] [--seed SEED] [--out FILE]\n  rename-classification [--output PATH]\n  migrate <openspec|ironspec> [--root PATH] [--policy PATH] [--execute]\n  mcp [--describe] [--call TOOL JSON] [--allow-mutations]\n  gate <proposal|design|tasks> <change> [--root PATH] [--policy PATH]\n  gate-context <change> [--root PATH]\n  sync <change> [--root PATH] [--policy PATH] [--execute]\n  archive <change> [--root PATH] [--policy PATH] [--execute]"
}

fn print_usage() {
    println!("{}", usage_text());
}
