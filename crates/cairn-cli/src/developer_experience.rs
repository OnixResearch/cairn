use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use cairn_core::{
    blake3_hex, canonical_json_hash, onboarding_templates, plan_prompt_scaffold,
    plan_reverse_spec_discovery, plan_template_application, DiscoverySnapshot, PlannedFile,
    DISCOVERY_MAX_BYTES, DISCOVERY_MAX_FILES, DISCOVERY_POLICY_ID, SCAFFOLD_POLICY_ID,
    TEMPLATE_POLICY_ID,
};
use serde_json::{json, Value};

use crate::support::{
    file_content_manifest, has_flag, path_string, portable_json_hash, print_json, read_option,
    read_string_option, safe_relative_path, with_portable_receipt_hash,
};

pub(crate) fn run_template(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("template requires a subcommand: list or apply".into());
    };
    let rest = iter.collect::<Vec<_>>();
    match subcommand.as_str() {
        "list" => run_template_list(rest),
        "apply" => {
            let Some(template) = rest.first() else {
                return Err("template apply requires a template id".into());
            };
            run_template_apply(rest[1..].to_vec(), template)
        }
        unknown => Err(format!("unknown template subcommand: {unknown}").into()),
    }
}

pub(crate) fn run_scaffold(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("scaffold requires a subcommand: change".into());
    };
    match subcommand.as_str() {
        "change" => {
            let Some(change) = iter.next() else {
                return Err("scaffold change requires a change name".into());
            };
            run_scaffold_change(iter.collect(), &change)
        }
        unknown => Err(format!("unknown scaffold subcommand: {unknown}").into()),
    }
}

pub(crate) fn run_discover_specs(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.developer_experience.reverse_spec_discovery.cli]
    let root = read_option(args.clone(), "--root", ".");
    let out = read_string_option(&args, "--out").map(PathBuf::from);
    let include_paths = include_values(&args);
    let mut snapshots = Vec::new();
    let mut skipped = Vec::new();
    for include in include_paths {
        let path = root.join(&include);
        if !path.is_file() {
            skipped.push(include);
            continue;
        }
        match fs::read_to_string(&path) {
            Ok(content) => snapshots.push(DiscoverySnapshot {
                path: include,
                content,
            }),
            Err(_) => skipped.push(include),
        }
    }
    let plan = plan_reverse_spec_discovery(snapshots);
    let mut issues = plan.issues.clone();
    let mut written = Vec::new();
    if let Some(out) = out.as_ref() {
        if plan.valid {
            let draft = discovery_draft(
                &plan
                    .candidates
                    .iter()
                    .map(candidate_json)
                    .collect::<Vec<_>>(),
            );
            write_output_file(&root, out, &draft, &mut issues, &mut written)?;
        }
    }
    let receipt = with_portable_receipt_hash(
        &root,
        json!({
            "command": "discover_specs",
            "root": path_string(&root),
            "valid": issues.is_empty(),
            "policy": DISCOVERY_POLICY_ID,
            "limits": {"max_files": DISCOVERY_MAX_FILES, "max_bytes": DISCOVERY_MAX_BYTES},
            "issues": issues,
            "selected_paths": plan.selected_paths,
            "skipped_paths": merge_skipped(plan.skipped_paths, skipped),
            "candidates": plan.candidates.into_iter().map(|candidate| json!({
                "id": candidate.id,
                "evidence_path": candidate.evidence_path,
                "summary": candidate.summary,
                "review_status": candidate.review_status,
            })).collect::<Vec<_>>(),
            "input_hash": plan.input_hash,
            "written": written,
        }),
    );
    print_json(receipt)
}

fn run_template_list(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.developer_experience.onboarding_templates.catalog]
    let root = read_option(args, "--root", ".");
    let templates = onboarding_templates()
        .into_iter()
        .map(|template| {
            json!({
                "id": template.id,
                "description": template.description,
                "files": template.files.into_iter().map(|file| file.path).collect::<Vec<_>>(),
            })
        })
        .collect::<Vec<_>>();
    print_json(with_portable_receipt_hash(
        &root,
        json!({
            "command": "template_list",
            "root": path_string(&root),
            "valid": true,
            "policy": TEMPLATE_POLICY_ID,
            "templates": templates,
        }),
    ))
}

fn run_template_apply(args: Vec<String>, template: &str) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.developer_experience.onboarding_templates.apply]
    let root = read_option(args.clone(), "--root", ".");
    let execute = has_flag(&args, "--execute");
    let existing = existing_template_paths(&root, template);
    let plan = plan_template_application(template, &existing);
    let files = onboarding_templates()
        .into_iter()
        .find(|candidate| candidate.id == template)
        .map(|profile| profile.files)
        .unwrap_or_default();
    let mut issues = plan.issues.clone();
    let touched = files
        .iter()
        .map(|file| root.join(&file.path))
        .collect::<Vec<_>>();
    let before = file_content_manifest(&root, &touched);
    let mut written = Vec::new();
    if execute && plan.valid {
        write_planned_files(&root, &files, &existing, &mut issues, &mut written)?;
    }
    let after = file_content_manifest(&root, &touched);
    print_json(with_portable_receipt_hash(
        &root,
        json!({
            "command": "template_apply",
            "root": path_string(&root),
            "template": template,
            "dry_run": !execute,
            "valid": issues.is_empty(),
            "policy": TEMPLATE_POLICY_ID,
            "issues": issues,
            "files": plan.files.into_iter().map(|entry| json!({
                "path": entry.path,
                "content_hash": entry.content_hash,
                "exists": entry.exists,
                "action": entry.action,
            })).collect::<Vec<_>>(),
            "mutated": execute && !written.is_empty(),
            "written": written,
            "mutation_manifest": if execute { json!({"before": before, "after": after, "manifest_hash": portable_json_hash(&root, &after)}) } else { Value::Null },
        }),
    ))
}

fn run_scaffold_change(args: Vec<String>, change: &str) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.developer_experience.prompt_to_change_scaffold.cli]
    let root = read_option(args.clone(), "--root", ".");
    let execute = has_flag(&args, "--execute");
    let domain =
        read_string_option(&args, "--domain").unwrap_or_else(|| "developer-experience".to_string());
    let request = read_request(&args)?;
    let existing = existing_change_paths(&root, change);
    let plan = plan_prompt_scaffold(change, &domain, &request, &existing);
    let mut issues = plan.issues.clone();
    let touched = plan
        .files
        .iter()
        .map(|file| root.join(&file.path))
        .collect::<Vec<_>>();
    let before = file_content_manifest(&root, &touched);
    let mut written = Vec::new();
    if execute && plan.valid {
        write_planned_files(&root, &plan.files, &existing, &mut issues, &mut written)?;
    }
    let after = file_content_manifest(&root, &touched);
    let artifact_plan = plan
        .files
        .iter()
        .map(|file| {
            json!({
                "path": file.path,
                "content_hash": blake3_hex(file.content.as_bytes()),
                "exists": existing.iter().any(|path| path == &file.path),
            })
        })
        .collect::<Vec<_>>();
    print_json(with_portable_receipt_hash(
        &root,
        json!({
            "command": "scaffold_change",
            "root": path_string(&root),
            "change": plan.change,
            "domain": plan.domain,
            "dry_run": !execute,
            "valid": issues.is_empty(),
            "policy": SCAFFOLD_POLICY_ID,
            "issues": issues,
            "input_hash": plan.input_hash,
            "artifacts": artifact_plan,
            "mutated": execute && !written.is_empty(),
            "written": written,
            "mutation_manifest": if execute { json!({"before": before, "after": after, "manifest_hash": portable_json_hash(&root, &after)}) } else { Value::Null },
        }),
    ))
}

fn read_request(args: &[String]) -> Result<String, Box<dyn Error>> {
    if let Some(request) = read_string_option(args, "--request") {
        return Ok(request);
    }
    if let Some(path) = read_string_option(args, "--request-file") {
        return Ok(fs::read_to_string(path)?);
    }
    Ok(String::new())
}

fn include_values(args: &[String]) -> Vec<String> {
    args.windows(2)
        .filter(|pair| pair[0] == "--include")
        .map(|pair| pair[1].clone())
        .collect()
}

fn existing_template_paths(root: &Path, template: &str) -> Vec<String> {
    onboarding_templates()
        .into_iter()
        .find(|candidate| candidate.id == template)
        .map(|profile| existing_paths(root, &profile.files))
        .unwrap_or_default()
}

fn existing_change_paths(root: &Path, change: &str) -> Vec<String> {
    let prefix = format!("cairn/changes/{change}");
    ["proposal.md", "design.md", "tasks.md"]
        .into_iter()
        .map(|name| format!("{prefix}/{name}"))
        .filter(|path| root.join(path).exists())
        .collect()
}

fn existing_paths(root: &Path, files: &[PlannedFile]) -> Vec<String> {
    files
        .iter()
        .filter(|file| root.join(&file.path).exists())
        .map(|file| file.path.clone())
        .collect()
}

fn write_planned_files(
    root: &Path,
    files: &[PlannedFile],
    existing: &[String],
    issues: &mut Vec<String>,
    written: &mut Vec<String>,
) -> Result<(), Box<dyn Error>> {
    for file in files {
        if existing.iter().any(|path| path == &file.path) {
            continue;
        }
        write_output_file(root, Path::new(&file.path), &file.content, issues, written)?;
    }
    Ok(())
}

fn write_output_file(
    root: &Path,
    relative: &Path,
    content: &str,
    issues: &mut Vec<String>,
    written: &mut Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let relative_text = relative.to_string_lossy().replace('\\', "/");
    if relative.is_absolute() || relative_text.contains("..") {
        issues.push(format!("unsafe output path: {relative_text}"));
        return Ok(());
    }
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, content)?;
    written.push(safe_relative_path(root, &path));
    written.sort();
    Ok(())
}

fn merge_skipped(mut left: Vec<String>, mut right: Vec<String>) -> Vec<String> {
    left.append(&mut right);
    left.sort();
    left.dedup();
    left
}

fn candidate_json(candidate: &cairn_core::DiscoveryCandidate) -> Value {
    json!({
        "id": candidate.id,
        "evidence_path": candidate.evidence_path,
        "summary": candidate.summary,
        "review_status": candidate.review_status,
    })
}

fn discovery_draft(candidates: &[Value]) -> String {
    let mut output = String::from("# Reverse Spec Discovery Draft\n\n");
    for candidate in candidates {
        output.push_str(&format!(
            "- `{}` from `{}`: {} ({})\n",
            candidate["id"].as_str().unwrap_or("candidate"),
            candidate["evidence_path"].as_str().unwrap_or("unknown"),
            candidate["summary"].as_str().unwrap_or("candidate"),
            candidate["review_status"]
                .as_str()
                .unwrap_or("review-needed"),
        ));
    }
    output
}

#[allow(dead_code)]
fn stable_hash(value: &Value) -> String {
    canonical_json_hash(value)
}
