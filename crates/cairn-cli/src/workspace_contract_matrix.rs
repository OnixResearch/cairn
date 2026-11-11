use std::{error::Error, path::Path};

use cairn_core::{
    blake3_hex, canonical_json_hash, validate_workspace_contract_matrix_value,
    workspace_contract_issues_json, WORKSPACE_CONTRACT_MATRIX_RECEIPT_SCHEMA_V1,
};
use serde_json::{json, Value};

use crate::support::{
    path_string, print_json, read_option, reject_positional_arguments, reject_unknown_options,
    safe_relative_path, with_receipt_hash, CapabilityRoot, CapabilityRootKind,
};

const MAX_OBSERVED_ARTIFACT_BYTES: u64 = 1_048_576;
const MAX_WORKSPACE_CONTRACT_INPUT_BYTES: u64 = 2_097_152;

pub(crate) fn run_workspace_contract(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let Some(subcommand) = args.first().map(String::as_str) else {
        return Err("workspace-contract requires a subcommand: verify".into());
    };
    match subcommand {
        "verify" => run_workspace_contract_verify(args.into_iter().skip(1).collect()),
        other => Err(format!("unknown workspace-contract subcommand: {other}").into()),
    }
}

fn run_workspace_contract_verify(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.workspace_contract_matrix.validation]
    // r[impl cairn.workspace_contract_matrix.validation.receipt]
    // r[impl cairn.workspace_contract_matrix.validation.privacy]
    reject_unknown_options(&args, &["--root", "--input"])?;
    reject_positional_arguments(&args)?;
    let root = read_option(args.clone(), "--root", ".");
    let input = read_option(args, "--input", "");
    if input.as_os_str().is_empty() {
        return Err("workspace-contract verify requires --input <matrix.json>".into());
    }
    let input_path = crate::support::resolve_under_root(&root, input);
    let value = read_workspace_contract_input(&input_path)?;
    let enriched = enrich_observed_artifacts(&root, value)?;
    let report = validate_workspace_contract_matrix_value(&enriched);
    let receipt = json!({
        "command": "workspace-contract verify",
        "schema": WORKSPACE_CONTRACT_MATRIX_RECEIPT_SCHEMA_V1,
        "input": safe_relative_path(&root, &input_path),
        "root": path_string(&root),
        "input_hash": canonical_json_hash(&enriched),
        "matrix_hash": report.matrix_hash,
        "policy_hash": policy_hash(&enriched),
        "valid": report.valid,
        "verdict": if report.valid { "pass" } else { "fail" },
        "checked_edge_count": report.edge_count,
        "stale_edge_count": report.stale_edge_count,
        "checked_edges": report.checked_edges,
        "boundary": report.boundary,
        "issues": workspace_contract_issues_json(&report),
        "observed_artifacts": enriched.get("observed_artifacts").cloned().unwrap_or(Value::Array(Vec::new())),
    });
    print_json(with_receipt_hash(receipt))?;
    if !report.valid {
        std::process::exit(1);
    }
    Ok(())
}

fn enrich_observed_artifacts(root: &Path, mut value: Value) -> Result<Value, Box<dyn Error>> {
    let matrix = value
        .get("matrix")
        .ok_or("workspace contract input missing matrix")?;
    let edges = matrix
        .get("edges")
        .and_then(Value::as_array)
        .ok_or("workspace contract input matrix.edges must be an array")?;
    let capability = CapabilityRoot::open_nofollow(CapabilityRootKind::Repository, root)?;
    let mut observations = Vec::new();
    for edge in edges {
        let edge_id = edge
            .get("edge_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let artifact_path = edge
            .get("artifact_path")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let artifact_identity = if is_cli_safe_relative_path(artifact_path) {
            let bytes = capability
                .read_bounded(Path::new(artifact_path), MAX_OBSERVED_ARTIFACT_BYTES)
                .map_err(|source| {
                    format!("failed to read workspace contract artifact {artifact_path}: {source}")
                })?;
            blake3_hex(&bytes)
        } else {
            String::new()
        };
        observations.push(json!({
            "edge_id": edge_id,
            "artifact_path": artifact_path,
            "artifact_identity": artifact_identity,
        }));
    }
    value["observed_artifacts"] = Value::Array(observations);
    Ok(value)
}

fn is_cli_safe_relative_path(path: &str) -> bool {
    if path.is_empty() || path.starts_with('/') || path.starts_with('~') || path.contains('\\') {
        return false;
    }
    !path
        .split('/')
        .any(|component| component.is_empty() || component == "." || component == "..")
}

fn policy_hash(value: &Value) -> String {
    value
        .get("policy_hash")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| canonical_json_hash(&Value::Null))
}

fn read_workspace_contract_input(path: &Path) -> Result<Value, Box<dyn Error>> {
    crate::support::read_bounded_json_no_symlink(
        path,
        MAX_WORKSPACE_CONTRACT_INPUT_BYTES,
        "workspace contract input",
    )
}
