use std::error::Error;

use cairn_core::validate_oxide_lifecycle_patterns_value;
use serde_json::{json, Value};

use crate::support::{
    print_json, read_option, reject_positional_arguments, reject_unknown_options,
    resolve_under_root, safe_relative_path, with_receipt_hash,
};

const OXIDE_PATTERN_INPUT_MAX_BYTES: usize = 2_097_152;

pub(crate) fn run_oxide_lifecycle_patterns(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let Some(subcommand) = args.first().map(String::as_str) else {
        return Err("lifecycle-patterns requires a subcommand: validate".into());
    };
    match subcommand {
        "validate" => run_validate(args.into_iter().skip(1).collect()),
        unknown => Err(format!("unknown lifecycle-patterns subcommand: {unknown}").into()),
    }
}

fn run_validate(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    reject_unknown_options(&args, &["--root", "--input"])?;
    reject_positional_arguments(&args)?;
    let root = read_option(args.clone(), "--root", ".");
    let input = read_option(args, "--input", "");
    if input.as_os_str().is_empty() {
        return Err("lifecycle-patterns validate requires --input <bundle.json>".into());
    }
    let input_path = resolve_under_root(&root, input);
    let value = crate::support::read_bounded_json_no_symlink(
        &input_path,
        OXIDE_PATTERN_INPUT_MAX_BYTES as u64,
        "lifecycle pattern bundle",
    )
    .map_err(|error| {
        format!(
            "failed to read lifecycle-patterns input {}: {error}",
            input_path.display()
        )
    })?;
    let report = validate_oxide_lifecycle_patterns_value(&value);
    let receipt = with_receipt_hash(json!({
        "command": "lifecycle-patterns validate",
        "root": ".",
        "input": safe_relative_path(&root, &input_path),
        "valid": report.valid,
        "verdict": if report.valid { "pass" } else { "fail" },
        "input_hash": report.input_hash,
        "reference_count": report.reference_count,
        "saga_step_count": report.saga_step_count,
        "document_plan_count": report.document_plan_count,
        "golden_snapshot_count": report.golden_snapshot_count,
        "bundle_member_count": report.bundle_member_count,
        "issues": report.issues,
        "boundary": report.boundary,
    }));
    let valid = receipt.get("valid").and_then(Value::as_bool) == Some(true);
    print_json(receipt)?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}
