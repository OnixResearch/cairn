//! Read-only advisory support-maintenance inspection. No lifecycle mutation.
use std::{error::Error, path::Path};

use cairn_core::{
    admit_support_policy, evaluate_support_maintenance, DerivationError, SupportCase,
    SupportPolicy, SUPPORT_BOUNDARY,
};
use serde_json::{json, Value};

use crate::support::{
    print_json, read_string_option, reject_positional_arguments, reject_unknown_options,
    CapabilityRoot, CapabilityRootKind,
};

const EXPLANATION_SCHEMA: &str = "cairn.support-maintenance.explanation.v1";
const MAX_CASE_BYTES: u64 = 4 * 1024 * 1024;
const MAX_POLICY_BYTES: u64 = 16 * 1024 * 1024;
const DEFAULT_POLICY: &str =
    include_str!("../../../cairn-policy/generated/support-maintenance.json");

fn read_bounded_file(path: &str, maximum: u64) -> Result<Vec<u8>, Box<dyn Error>> {
    let (root, relative) = CapabilityRoot::open_file_parent(
        CapabilityRootKind::EvidenceBundle,
        Path::new(path),
        false,
    )?;
    Ok(root.read_bounded(&relative, maximum)?)
}

fn rejection(outcome: &str, detail: String) -> Value {
    json!({
        "schema_version": EXPLANATION_SCHEMA,
        "outcome": "rejected",
        "advisory": true,
        "report": null,
        "error": {"outcome": outcome, "detail": detail},
        "boundary": SUPPORT_BOUNDARY,
    })
}

fn rejected(error: DerivationError) -> Result<(), Box<dyn Error>> {
    let outcome = error.outcome();
    print_json(rejection(outcome, format!("{error:?}")))?;
    Err("support-maintenance admission rejected".into())
}

/// Bounded reads only. Support is advisory and never authorizes a lifecycle action.
// r[impl cairn.support_maintenance.inspection]
pub(crate) fn run(mut args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.first().map(String::as_str) != Some("explain") {
        return Err(
            "usage: cairn support-maintenance explain --case FILE [--policy FILE]".into(),
        );
    }
    args.remove(0);
    reject_unknown_options(&args, &["--case", "--policy"])?;
    reject_positional_arguments(&args)?;
    let case_path = read_string_option(&args, "--case").ok_or("--case is required")?;
    let policy_bytes = match read_string_option(&args, "--policy") {
        Some(path) => read_bounded_file(&path, MAX_POLICY_BYTES)?,
        None => DEFAULT_POLICY.as_bytes().to_vec(),
    };
    let policy = match serde_json::from_slice::<SupportPolicy>(&policy_bytes) {
        Ok(policy) => policy,
        Err(error) => {
            print_json(rejection(
                "invalid_input",
                format!("support-maintenance policy rejected: {error}"),
            ))?;
            return Err("support-maintenance policy rejected".into());
        }
    };
    if let Err(error) = admit_support_policy(&policy) {
        return rejected(error);
    }
    let case_bytes = read_bounded_file(&case_path, MAX_CASE_BYTES)?;
    let case = match serde_json::from_slice::<SupportCase>(&case_bytes) {
        Ok(case) => case,
        Err(error) => {
            print_json(rejection(
                "invalid_input",
                format!("support-maintenance case rejected: {error}"),
            ))?;
            return Err("support-maintenance case rejected".into());
        }
    };
    match evaluate_support_maintenance(&policy, &case) {
        Ok(report) => {
            print_json(json!({
                "schema_version": EXPLANATION_SCHEMA,
                "outcome": "evaluated",
                "advisory": true,
                "report": report,
                "error": null,
                "boundary": SUPPORT_BOUNDARY,
            }))?;
            Ok(())
        }
        Err(error) => rejected(error),
    }
}
