use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use cairn_core::{
    archive_dir, build_change_detail, build_change_list, build_change_skeleton,
    build_doctor_report, build_spec_detail, build_spec_list, changes_dir, design_template,
    filter_changes_by_metadata, init_artifacts, layout_name, layout_root, proposal_template,
    specs_dir, tasks_template, validate_changes, validate_spec_sources, ChangeDetail, ChangeList,
    ChangeStatus, DoctorCheckStatus, DoctorReport, DoctorSnapshot, LifecycleArtifactKind,
    LifecycleLayout, RepoStatus, SpecDetail, SpecList, SpecSummary, TaskCounts,
};
use serde_json::{json, Value};

use crate::{
    collect_spec_sources, collect_validation_spec_sources, default_policy_path, detect_layout,
    find_change, find_spec_source, has_flag, load_policy_file, path_string, print_json,
    read_option, read_string_option, scan_repo,
};

pub(crate) fn run_status(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args, "--root", ".");
    let selected = detect_layout(&root);
    let status = scan_repo(root, selected.layout)?;
    print_json(repo_status_json(&status, selected.layout))
}

pub(crate) fn run_doctor(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args.clone(), "--root", ".");
    let policy_path = read_option(args, "--policy", default_policy_path());
    let selected = detect_layout(&root);
    let status = scan_repo(root.clone(), selected.layout)?;
    let spec_sources = collect_validation_spec_sources(&root, selected.layout)?;
    let spec_issues = validate_spec_sources(&spec_sources);
    let policy_result = load_policy_file(policy_path.clone());
    let (policy_name, policy_error, change_issue_count) = match policy_result {
        Ok(policy) => {
            let change_issues = validate_changes(&status.changes, &policy);
            (Some(policy.name), None, change_issues.len())
        }
        Err(error) => (None, Some(error.to_string()), 0),
    };
    let snapshot = DoctorSnapshot {
        root: path_string(&root),
        selected_layout: selected.layout,
        native_layout_present: root.join(layout_root(LifecycleLayout::Native)).exists(),
        legacy_layout_present: root
            .join(layout_root(LifecycleLayout::LegacyOpenSpec))
            .exists(),
        specs_dir_present: root.join(specs_dir(selected.layout)).exists(),
        changes_dir_present: root.join(changes_dir(selected.layout)).exists(),
        archive_dir_present: root.join(archive_dir(selected.layout)).exists(),
        policy_path: path_string(policy_path),
        policy_name,
        policy_error,
        spec_count: status.specs.len(),
        change_count: status.changes.len(),
        validation_issue_count: spec_issues.len() + change_issue_count,
    };
    let report = build_doctor_report(snapshot);
    print_json(doctor_report_json(&report))
}

pub(crate) fn run_change(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("change requires a subcommand: create, list, or show".into());
    };
    match subcommand.as_str() {
        "create" => {
            let Some(change) = iter.next() else {
                return Err("change create requires a change name".into());
            };
            run_change_create(iter.collect(), &change)
        }
        "list" => run_change_list(iter.collect()),
        "show" => {
            let Some(change) = iter.next() else {
                return Err("change show requires a change name".into());
            };
            run_change_show(iter.collect(), &change)
        }
        unknown => Err(format!("unknown change subcommand: {unknown}").into()),
    }
}

pub(crate) fn run_spec(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("spec requires a subcommand: list or show".into());
    };
    match subcommand.as_str() {
        "list" => run_spec_list(iter.collect()),
        "show" => {
            let Some(spec) = iter.next() else {
                return Err("spec show requires a spec id".into());
            };
            run_spec_show(iter.collect(), &spec)
        }
        unknown => Err(format!("unknown spec subcommand: {unknown}").into()),
    }
}

fn run_change_list(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args.clone(), "--root", ".");
    let group = read_string_option(&args, "--group");
    let target_after = read_string_option(&args, "--target-after");
    let target_before = read_string_option(&args, "--target-before");
    let selected = detect_layout(&root);
    let status = scan_repo(root.clone(), selected.layout)?;
    let changes = filter_changes_by_metadata(
        status.changes,
        group.as_deref(),
        target_after.as_deref(),
        target_before.as_deref(),
    );
    let list = build_change_list(changes);
    print_json(change_list_json(&path_string(root), selected.layout, &list))
}

fn run_change_show(args: Vec<String>, change: &str) -> Result<(), Box<dyn Error>> {
    let root = read_option(args, "--root", ".");
    let selected = detect_layout(&root);
    let change_status = find_change(root.clone(), selected.layout, change)?;
    let detail = build_change_detail(change_status);
    print_json(change_detail_json(
        &path_string(root),
        selected.layout,
        &detail,
    ))
}

fn run_spec_list(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args, "--root", ".");
    let selected = detect_layout(&root);
    let specs = collect_spec_sources(&root.join(specs_dir(selected.layout)))?;
    let list = build_spec_list(specs);
    print_json(spec_list_json(&path_string(root), selected.layout, &list))
}

fn run_spec_show(args: Vec<String>, spec: &str) -> Result<(), Box<dyn Error>> {
    let root = read_option(args, "--root", ".");
    let selected = detect_layout(&root);
    let source = find_spec_source(&root, selected.layout, spec)?;
    let detail = build_spec_detail(source);
    print_json(spec_detail_json(
        &path_string(root),
        selected.layout,
        &detail,
    ))
}

pub(crate) fn run_validate(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.rust_nickel_only_gates.gates]
    let root = read_option(args.clone(), "--root", ".");
    let policy_path = read_option(args.clone(), "--policy", default_policy_path());
    let is_strict = has_flag(&args, "--strict");
    let receipt = cairn_validation_receipt(root, policy_path)?;
    print_json(receipt.clone())?;
    if is_strict && receipt["valid"].as_bool() != Some(true) {
        return Err("validation failed".into());
    }
    Ok(())
}

pub(crate) fn cairn_validation_receipt(
    root: PathBuf,
    policy_path: PathBuf,
) -> Result<Value, Box<dyn Error>> {
    let policy = load_policy_file(policy_path)?;
    let selected = detect_layout(&root);
    let status = scan_repo(root.clone(), selected.layout)?;
    let change_issues = validate_changes(&status.changes, &policy);
    let spec_sources = collect_validation_spec_sources(&root, selected.layout)?;
    let spec_issues = validate_spec_sources(&spec_sources);
    let mut issues = Vec::with_capacity(change_issues.len() + spec_issues.len());
    issues.extend(change_issues.iter().cloned());
    issues.extend(spec_issues.iter().cloned());
    Ok(json!({
        "valid": issues.is_empty(),
        "issues": issues,
        "change_issues": change_issues,
        "spec_issues": spec_issues,
        "changes": status.changes.len(),
        "specs_validated": spec_sources.len(),
        "policy": policy.name,
        "layout": layout_name(selected.layout),
    }))
}
pub(crate) fn run_init(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args, "--root", ".");
    let artifacts = init_artifacts();
    let mut created = Vec::with_capacity(artifacts.len());
    let mut existing = Vec::with_capacity(artifacts.len());
    for artifact in artifacts {
        let path = root.join(&artifact.path);
        if path.exists() {
            existing.push(artifact.path);
        } else {
            match artifact.kind {
                LifecycleArtifactKind::Directory => fs::create_dir_all(&path)?,
                LifecycleArtifactKind::File => write_new_file(&path, "")?,
            }
            created.push(artifact.path);
        }
    }
    print_json(json!({
        "command": "init",
        "root": path_string(root),
        "mutated": !created.is_empty(),
        "layout": layout_name(LifecycleLayout::Native),
        "created": created,
        "existing": existing,
    }))
}

fn run_change_create(args: Vec<String>, change: &str) -> Result<(), Box<dyn Error>> {
    let root = read_option(args, "--root", ".");
    let skeleton = build_change_skeleton(change)?;
    let mut created = Vec::with_capacity(skeleton.artifacts.len());
    let mut existing = Vec::with_capacity(skeleton.artifacts.len());
    for artifact in skeleton.artifacts {
        let path = root.join(&artifact.path);
        if path.exists() {
            existing.push(artifact.path);
            continue;
        }
        match artifact.kind {
            LifecycleArtifactKind::Directory => fs::create_dir_all(&path)?,
            LifecycleArtifactKind::File => {
                write_new_file(&path, &change_template(change, &artifact.path))?
            }
        }
        created.push(artifact.path);
    }
    print_json(json!({
        "command": "change_create",
        "root": path_string(root),
        "change": change,
        "mutated": !created.is_empty(),
        "layout": layout_name(LifecycleLayout::Native),
        "created": created,
        "existing": existing,
    }))
}

pub(crate) fn change_template(change: &str, artifact_path: &str) -> String {
    if artifact_path.ends_with("proposal.md") {
        return proposal_template(change);
    }
    if artifact_path.ends_with("design.md") {
        return design_template(change);
    }
    if artifact_path.ends_with("tasks.md") {
        return tasks_template();
    }
    String::new()
}

pub(crate) fn write_new_file(path: &Path, content: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}
pub(crate) fn repo_status_json(status: &RepoStatus, layout: LifecycleLayout) -> Value {
    json!({
        "root": status.root,
        "layout": layout_name(layout),
        "specs": status.specs,
        "changes": status.changes.iter().map(change_status_json).collect::<Vec<_>>(),
    })
}

pub(crate) fn doctor_report_json(report: &DoctorReport) -> Value {
    json!({
        "valid": report.valid,
        "root": report.root,
        "layout": report.layout,
        "spec_count": report.spec_count,
        "change_count": report.change_count,
        "validation_issue_count": report.validation_issue_count,
        "checks": report.checks.iter().map(|check| {
            json!({
                "name": check.name,
                "status": doctor_check_status(check.status),
                "message": check.message,
            })
        }).collect::<Vec<_>>(),
    })
}

fn doctor_check_status(status: DoctorCheckStatus) -> &'static str {
    match status {
        DoctorCheckStatus::Ok => "ok",
        DoctorCheckStatus::Warn => "warn",
        DoctorCheckStatus::Fail => "fail",
    }
}

pub(crate) fn change_list_json(root: &str, layout: LifecycleLayout, list: &ChangeList) -> Value {
    json!({
        "root": root,
        "layout": layout_name(layout),
        "changes": list.changes.iter().map(change_status_json).collect::<Vec<_>>(),
    })
}

pub(crate) fn change_detail_json(
    root: &str,
    layout: LifecycleLayout,
    detail: &ChangeDetail,
) -> Value {
    json!({
        "root": root,
        "layout": layout_name(layout),
        "change": change_status_json(&detail.change),
    })
}

pub(crate) fn spec_list_json(root: &str, layout: LifecycleLayout, list: &SpecList) -> Value {
    json!({
        "root": root,
        "layout": layout_name(layout),
        "specs": list.specs.iter().map(spec_summary_json).collect::<Vec<_>>(),
    })
}

pub(crate) fn spec_detail_json(root: &str, layout: LifecycleLayout, detail: &SpecDetail) -> Value {
    json!({
        "root": root,
        "layout": layout_name(layout),
        "spec": {
            "id": detail.id,
            "path": detail.path,
            "title": detail.title,
            "markdown": detail.markdown,
        }
    })
}

fn spec_summary_json(summary: &SpecSummary) -> Value {
    json!({
        "id": summary.id,
        "path": summary.path,
        "title": summary.title,
    })
}

pub(crate) fn change_status_json(change: &ChangeStatus) -> Value {
    json!({
        "name": change.name,
        "path": change.path,
        "has_proposal": change.has_proposal,
        "has_design": change.has_design,
        "has_tasks": change.has_tasks,
        "delta_specs": change.delta_specs,
        "task_counts": change.task_counts.map(task_counts_json),
        "metadata": change_metadata_json(change),
    })
}

fn change_metadata_json(change: &ChangeStatus) -> Value {
    json!({
        "groups": change.metadata.groups,
        "target_date": change.metadata.target_date,
        "status": change.metadata.status,
    })
}

fn task_counts_json(counts: TaskCounts) -> Value {
    json!({
        "total": counts.total,
        "done": counts.done,
        "in_progress": counts.in_progress,
        "todo": counts.todo,
        "unmarked": counts.unmarked,
    })
}
