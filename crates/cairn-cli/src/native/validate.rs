use std::{error::Error, path::PathBuf};

use cairn_core::{
    evaluate_lifecycle_substance, layout_name, validate_changes_with_archives,
    validate_spec_source_with_policy,
};
use serde_json::{json, Value};

use crate::support::{
    collect_archived_change_ids, collect_validation_spec_sources, default_policy_path,
    detect_layout, has_flag, load_policy_file, print_json, read_gate_snapshot, read_option,
    scan_repo, spec_substance_inventory_json, substance_inventory_json, validation_findings_json,
};

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
    let archived_changes = collect_archived_change_ids(&root, selected.layout)?;
    let change_issues = validate_changes_with_archives(&status.changes, &archived_changes, &policy);
    let spec_sources = collect_validation_spec_sources(&root, selected.layout)?;
    let mut spec_issues = Vec::new();
    let mut spec_substance = Vec::new();
    for source in &spec_sources {
        let (inventory, source_issues) =
            validate_spec_source_with_policy(source, &policy.artifact_substance_policy);
        spec_substance.push(inventory);
        spec_issues.extend(source_issues);
    }
    spec_issues.sort();
    spec_issues.dedup();
    spec_substance.sort_by(|left, right| left.path.cmp(&right.path));
    let mut substance_issues = Vec::new();
    let mut substance = Vec::new();
    for change in &status.changes {
        let snapshot = read_gate_snapshot(&root, selected.layout, change.clone())?;
        let report = evaluate_lifecycle_substance(&snapshot, &policy);
        substance_issues.extend(report.issues);
        substance.extend(report.artifacts);
    }
    substance_issues.sort();
    substance_issues.dedup();
    substance.sort_by(|left, right| left.path.cmp(&right.path).then(left.kind.cmp(&right.kind)));
    let mut issues =
        Vec::with_capacity(change_issues.len() + spec_issues.len() + substance_issues.len());
    issues.extend(change_issues.iter().cloned());
    issues.extend(spec_issues.iter().cloned());
    issues.extend(substance_issues.iter().cloned());
    issues.sort();
    issues.dedup();
    Ok(json!({
        "valid": issues.is_empty(),
        "findings": validation_findings_json(&issues),
        "issues": issues,
        "change_issues": change_issues,
        "spec_findings": validation_findings_json(&spec_issues),
        "spec_issues": spec_issues,
        "spec_substance": spec_substance_inventory_json(&spec_substance),
        "substance_findings": validation_findings_json(&substance_issues),
        "substance_issues": substance_issues,
        "substance": substance_inventory_json(&substance),
        "changes": status.changes.len(),
        "specs_validated": spec_sources.len(),
        "policy": policy.name,
        "layout": layout_name(selected.layout),
    }))
}
