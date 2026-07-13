use std::{error::Error, fs, path::Path};

use cairn_core::{
    build_change_skeleton, change_metadata_with_dependencies, changes_dir, design_template,
    init_artifacts, layout_name, normalized_change_dependencies, plan_change_dependency_edit,
    proposal_template, render_change_metadata_json, tasks_template, ChangeDependencyEditAction,
    LifecycleArtifactKind, LifecycleLayout,
};
use serde_json::{json, Value};

use crate::support::{
    collect_archived_change_ids, detect_layout, file_content_manifest, has_flag, path_string,
    portable_json_hash, print_json, read_option, safe_relative_path, scan_repo,
    with_portable_receipt_hash,
};

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

pub(super) fn run_change_create(args: Vec<String>, change: &str) -> Result<(), Box<dyn Error>> {
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

pub(super) fn run_change_depend(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("change depend requires a subcommand: list, add, or remove".into());
    };
    match subcommand.as_str() {
        "list" => {
            let Some(change) = iter.next() else {
                return Err("change depend list requires a change name".into());
            };
            run_change_depend_list(iter.collect(), &change)
        }
        "add" => {
            let Some(change) = iter.next() else {
                return Err("change depend add requires a change name".into());
            };
            let Some(dependency) = iter.next() else {
                return Err("change depend add requires a dependency name".into());
            };
            run_change_depend_edit(
                iter.collect(),
                &change,
                &dependency,
                ChangeDependencyEditAction::Add,
            )
        }
        "remove" => {
            let Some(change) = iter.next() else {
                return Err("change depend remove requires a change name".into());
            };
            let Some(dependency) = iter.next() else {
                return Err("change depend remove requires a dependency name".into());
            };
            run_change_depend_edit(
                iter.collect(),
                &change,
                &dependency,
                ChangeDependencyEditAction::Remove,
            )
        }
        unknown => Err(format!("unknown change depend subcommand: {unknown}").into()),
    }
}

fn run_change_depend_list(args: Vec<String>, change: &str) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.change_dependency_graph.authoring_cli]
    let root = read_option(args, "--root", ".");
    let selected = detect_layout(&root);
    let status = scan_repo(root.clone(), selected.layout)?;
    let Some(change_status) = status
        .changes
        .iter()
        .find(|candidate| candidate.name == change)
    else {
        return Err(format!("change not found: {change}").into());
    };
    let dependencies = normalized_change_dependencies(change_status);
    let valid = change_status.metadata_issues.is_empty();
    let receipt = with_portable_receipt_hash(
        &root,
        json!({
            "command": "change depend list",
            "root": path_string(&root),
            "layout": layout_name(selected.layout),
            "change": change,
            "valid": valid,
            "mutated": false,
            "dependencies": dependencies,
            "issues": change_status.metadata_issues,
        }),
    );
    print_json(receipt)?;
    if valid {
        Ok(())
    } else {
        Err("change dependency metadata invalid".into())
    }
}

fn run_change_depend_edit(
    args: Vec<String>,
    change: &str,
    dependency: &str,
    action: ChangeDependencyEditAction,
) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.change_dependency_graph.authoring_cli.preview]
    // r[impl cairn.change_dependency_graph.authoring_cli.invalid]
    let root = read_option(args.clone(), "--root", ".");
    let execute = has_flag(&args, "--execute");
    let selected = detect_layout(&root);
    let status = scan_repo(root.clone(), selected.layout)?;
    let archived_changes = collect_archived_change_ids(&root, selected.layout)?;
    let plan = plan_change_dependency_edit(
        &status.changes,
        &archived_changes,
        change,
        dependency,
        action,
    );
    let metadata_path = root
        .join(changes_dir(selected.layout))
        .join(change)
        .join("metadata.json");
    let before_manifest = file_content_manifest(&root, std::slice::from_ref(&metadata_path));
    let after_metadata_text = status
        .changes
        .iter()
        .find(|candidate| candidate.name == change)
        .map(|change_status| {
            let metadata = change_metadata_with_dependencies(change_status, plan.after.clone());
            render_change_metadata_json(&metadata)
        })
        .transpose()?;

    let mut mutated = false;
    if plan.valid && execute && plan.changed {
        let Some(text) = &after_metadata_text else {
            return Err("valid dependency edit missing rendered metadata".into());
        };
        if let Some(parent) = metadata_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&metadata_path, text)?;
        mutated = true;
    }

    let mutation_manifest = if execute && plan.valid {
        let after_manifest = file_content_manifest(&root, std::slice::from_ref(&metadata_path));
        json!({
            "kind": "change_dependency_metadata",
            "before": before_manifest,
            "after": after_manifest,
            "manifest_hash": portable_json_hash(&root, &json!({"before": before_manifest, "after": after_manifest})),
        })
    } else {
        Value::Null
    };
    let receipt = dependency_edit_receipt(DependencyEditReceiptInput {
        root: &root,
        layout: selected.layout,
        metadata_path: &metadata_path,
        plan: &plan,
        execute,
        mutated,
        after_metadata_text,
        mutation_manifest,
    });
    let valid = plan.valid;
    print_json(receipt)?;
    if valid {
        Ok(())
    } else {
        Err("change dependency edit invalid".into())
    }
}

struct DependencyEditReceiptInput<'a> {
    root: &'a Path,
    layout: LifecycleLayout,
    metadata_path: &'a Path,
    plan: &'a cairn_core::ChangeDependencyEditPlan,
    execute: bool,
    mutated: bool,
    after_metadata_text: Option<String>,
    mutation_manifest: Value,
}

fn dependency_edit_receipt(input: DependencyEditReceiptInput<'_>) -> Value {
    with_portable_receipt_hash(
        input.root,
        json!({
            "command": format!("change depend {}", input.plan.action.as_str()),
            "root": path_string(input.root),
            "layout": layout_name(input.layout),
            "change": input.plan.change,
            "dependency": input.plan.dependency,
            "metadata_path": safe_relative_path(input.root, input.metadata_path),
            "valid": input.plan.valid,
            "dry_run": !input.execute,
            "mutated": input.mutated,
            "changed": input.plan.changed,
            "before": input.plan.before,
            "after": input.plan.after,
            "after_metadata": input.after_metadata_text,
            "issues": input.plan.issues.iter().map(super::json::change_dependency_issue_json).collect::<Vec<_>>(),
            "mutation_manifest": input.mutation_manifest,
        }),
    )
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
