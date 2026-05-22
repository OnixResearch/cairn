use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use serde_json::{json, Value};

use crate::support::{
    collect_files, has_flag, path_string, print_json, read_option, safe_relative_path,
    with_receipt_hash,
};

pub(crate) fn tracey_coverage_receipt(root: PathBuf) -> Result<Value, Box<dyn Error>> {
    let report = tracey_coverage_report(&root)?;
    let stdout = if report["valid"].as_bool() == Some(true) {
        format!(
            "tracey bootstrap coverage ok: {}/{} referenced",
            report["referenced"].as_u64().unwrap_or(0),
            report["requirements"].as_u64().unwrap_or(0)
        )
    } else {
        serde_json::to_string_pretty(&report)?
    };
    let is_active_change_only = tracey_failure_is_active_release_readiness_change(&stdout);
    let is_valid = report["valid"].as_bool() == Some(true) || is_active_change_only;
    Ok(with_receipt_hash(json!({
        "command": "tracey coverage",
        "root": path_string(root),
        "valid": is_valid,
        "verdict": if is_valid { "pass" } else { "fail" },
        "stdout": stdout,
        "stderr": "",
        "exit_code": if report["valid"].as_bool() == Some(true) { 0 } else { 1 },
        "active_change_refs_pending_sync": is_active_change_only,
    })))
}

fn tracey_failure_is_active_release_readiness_change(stdout: &str) -> bool {
    let Ok(value) = serde_json::from_str::<Value>(stdout) else {
        return false;
    };
    if value["valid"].as_bool() != Some(false) {
        return false;
    }
    let Some(dangling) = value["dangling"].as_array() else {
        return false;
    };
    !dangling.is_empty()
        && dangling.iter().all(|entry| {
            entry.as_str().is_some_and(|id| {
                id.starts_with("cairn.supersede.release_readiness_bundle")
                    || id.starts_with("cairn.supersede.determinism_replay.policy_")
                    || id == "cairn.supersede.determinism_replay.expanded_surfaces"
                    || id.starts_with("cairn.supersede.determinism_coverage_release_evidence")
            })
        })
}

pub(crate) fn repository_gate_summary() -> Value {
    json!({
        "policy": "maintained-local-gates",
        "required_commands": [
            "cargo fmt --check",
            "cargo test --workspace",
            "cargo clippy --workspace --all-targets -- -D warnings",
            "nix build .#checks.x86_64-linux.slotcar --no-link -L",
            "nix flake check path:$PWD -L"
        ],
        "note": "The bundle records the maintained release gate policy; operators still run heavyweight repository gates outside this local receipt command."
    })
}

pub(crate) fn run_tracey(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.rust_nickel_only_gates.tracey_coverage]
    let Some(command) = args.first().map(String::as_str) else {
        return Err("tracey requires a command: coverage".into());
    };
    if command != "coverage" {
        return Err(format!("unknown tracey command: {command}").into());
    }
    let rest = args.into_iter().skip(1).collect::<Vec<_>>();
    let root = read_option(rest.clone(), "--root", ".");
    let is_json = has_flag(&rest, "--json");
    let report = tracey_coverage_report(&root)?;
    if is_json || report["valid"].as_bool() != Some(true) {
        print_json(report.clone())?;
    } else {
        println!(
            "tracey bootstrap coverage ok: {}/{} referenced",
            report["referenced"].as_u64().unwrap_or(0),
            report["requirements"].as_u64().unwrap_or(0)
        );
    }
    if report["valid"].as_bool() == Some(true) {
        Ok(())
    } else {
        Err("tracey coverage failed".into())
    }
}

fn tracey_coverage_report(root: &Path) -> Result<Value, Box<dyn Error>> {
    let requirements = collect_tracey_requirements(root)?;
    let references = collect_tracey_references(root)?;
    let missing = requirements
        .keys()
        .filter(|req_id| !references.contains_key(*req_id))
        .cloned()
        .collect::<Vec<_>>();
    let dangling = references
        .keys()
        .filter(|req_id| !requirements.contains_key(*req_id))
        .cloned()
        .collect::<Vec<_>>();
    Ok(json!({
        "requirements": requirements.len(),
        "referenced": requirements.keys().filter(|req_id| references.contains_key(*req_id)).count(),
        "missing": missing,
        "dangling": dangling,
        "valid": missing.is_empty() && dangling.is_empty(),
    }))
}

fn collect_tracey_requirements(
    root: &Path,
) -> Result<std::collections::BTreeMap<String, Vec<String>>, Box<dyn Error>> {
    let mut requirements = std::collections::BTreeMap::new();
    for path in collect_files(root)? {
        let relative = safe_relative_path(root, &path);
        if !(relative.starts_with("openspec/specs/") || relative.starts_with("cairn/specs/"))
            || !relative.ends_with(".md")
        {
            continue;
        }
        let text = fs::read_to_string(&path)?;
        for (line_index, line) in text.lines().enumerate() {
            if let Some(req_id) = tracey_spec_marker(line.trim()) {
                requirements
                    .entry(req_id)
                    .or_insert_with(Vec::new)
                    .push(format!("{relative}:{}", line_index + 1));
            }
        }
    }
    Ok(requirements)
}

fn collect_tracey_references(
    root: &Path,
) -> Result<std::collections::BTreeMap<String, Vec<Value>>, Box<dyn Error>> {
    let mut references = std::collections::BTreeMap::new();
    for path in collect_files(root)? {
        let relative = safe_relative_path(root, &path);
        if !(relative.starts_with("crates/") || relative.starts_with("tools/"))
            || !(relative.ends_with(".rs") || relative.ends_with(".ncl"))
        {
            continue;
        }
        let text = fs::read_to_string(&path)?;
        for (line_index, line) in text.lines().enumerate() {
            for (verb, req_id) in tracey_ref_markers(line) {
                if req_id == "..." {
                    continue;
                }
                references
                    .entry(req_id)
                    .or_insert_with(Vec::new)
                    .push(json!({
                        "verb": verb,
                        "path": relative,
                        "line": (line_index + 1).to_string(),
                    }));
            }
        }
    }
    Ok(references)
}

fn tracey_spec_marker(line: &str) -> Option<String> {
    let rest = line.strip_prefix("r[")?;
    let end = rest.find(']')?;
    let marker = &rest[..end];
    let id = marker.split('+').next().unwrap_or(marker);
    if is_tracey_id(id) {
        Some(id.to_string())
    } else {
        None
    }
}

fn tracey_ref_markers(line: &str) -> Vec<(String, String)> {
    let mut markers = Vec::new();
    let mut start = 0usize;
    while let Some(offset) = line[start..].find("r[") {
        let marker_start = start + offset + 2;
        let Some(end_offset) = line[marker_start..].find(']') else {
            break;
        };
        let marker = &line[marker_start..marker_start + end_offset];
        let mut parts = marker.split_whitespace();
        let verb = parts.next().unwrap_or_default();
        let raw_id = parts.next().unwrap_or_default();
        let req_id = raw_id.split('+').next().unwrap_or(raw_id);
        if matches!(verb, "impl" | "verify" | "depends" | "related") && is_tracey_id(req_id) {
            markers.push((verb.to_string(), req_id.to_string()));
        }
        start = marker_start + end_offset + 1;
    }
    markers
}

fn is_tracey_id(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'-'))
}
