use std::{error::Error, fs, path::PathBuf};

use cairn_core::{
    canonical_json_hash, requirement_id_registry_issues_json,
    validate_requirement_id_registry_value,
};
use serde_json::{json, Value};

use crate::support::{print_json, read_option, with_receipt_hash};

pub(crate) fn run_requirement_registry(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let Some(subcommand) = args.first().map(String::as_str) else {
        return Err("requirement-registry requires a subcommand: verify".into());
    };
    match subcommand {
        "verify" => run_requirement_registry_verify(args.into_iter().skip(1).collect()),
        other => Err(format!("unknown requirement-registry subcommand: {other}").into()),
    }
}

fn run_requirement_registry_verify(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.requirement_id_registry.validation]
    // r[impl cairn.requirement_id_registry.validation.receipt]
    let input = read_option(args, "--input", "");
    if input.as_os_str().is_empty() {
        return Err("requirement-registry verify requires --input <registry-input.json>".into());
    }
    let value = read_requirement_registry_input(&input)?;
    let report = validate_requirement_id_registry_value(&value);
    let receipt = json!({
        "command": "requirement-registry verify",
        "schema": "cairn.requirement-registry.verify-receipt.v1",
        "input": input.to_string_lossy(),
        "input_hash": canonical_json_hash(&value),
        "registry_hash": report.registry_hash,
        "policy_hash": policy_hash(&value),
        "valid": report.valid,
        "verdict": if report.valid { "pass" } else { "fail" },
        "row_count": report.row_count,
        "checked_reference_count": report.checked_reference_count,
        "dangling_reference_count": report.dangling_reference_count,
        "stale_alias_count": report.stale_alias_count,
        "boundary": report.boundary,
        "issues": requirement_id_registry_issues_json(&report),
    });
    print_json(with_receipt_hash(receipt))?;
    if !report.valid {
        std::process::exit(1);
    }
    Ok(())
}

fn policy_hash(value: &Value) -> String {
    value
        .get("policy_hash")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| canonical_json_hash(&Value::Null))
}

fn read_requirement_registry_input(path: &PathBuf) -> Result<Value, Box<dyn Error>> {
    let text = fs::read_to_string(path).map_err(|source| {
        format!(
            "failed to read requirement registry input {}: {source}",
            path.display()
        )
    })?;
    serde_json::from_str(&text).map_err(|source| {
        format!(
            "failed to parse requirement registry input {}: {source}",
            path.display()
        )
        .into()
    })
}
