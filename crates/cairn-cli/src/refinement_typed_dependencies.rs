//! Read-only advisory refinement inspection. No lifecycle mutation.
use std::{error::Error, path::Path};

use cairn_core::{
    admit_refinement_policy, evaluate_refinement, RefinementError, RefinementPolicy,
    RefinementRequest, REFINEMENT_BOUNDARY,
};
use serde_json::{json, Value};

use crate::support::{
    print_json, read_string_option, reject_positional_arguments, reject_unknown_options,
    CapabilityRoot, CapabilityRootKind,
};

const EXPLANATION_SCHEMA: &str = "cairn.refinement.explanation.v1";
const MAX_CASE_BYTES: u64 = 8 * 1024 * 1024;
const MAX_POLICY_BYTES: u64 = 16 * 1024 * 1024;
const DEFAULT_POLICY: &str =
    include_str!("../../../cairn-policy/generated/refinement-typed-dependencies.json");

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
        "boundary": REFINEMENT_BOUNDARY,
    })
}

fn rejected(error: RefinementError) -> Result<(), Box<dyn Error>> {
    let outcome = error.outcome();
    print_json(rejection(outcome, format!("{error:?}")))?;
    Err("refinement argument admission rejected".into())
}

/// Bounded reads only. A supported argument is advisory and never authorizes a
/// lifecycle action, claim, readiness state or gate verdict.
// r[impl cairn.refinement.inspection]
pub(crate) fn run(mut args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.first().map(String::as_str) != Some("explain") {
        return Err("usage: cairn refinement explain --case FILE [--policy FILE]".into());
    }
    args.remove(0);
    reject_unknown_options(&args, &["--case", "--policy"])?;
    reject_positional_arguments(&args)?;
    let case_path = read_string_option(&args, "--case").ok_or("--case is required")?;
    let policy_bytes = match read_string_option(&args, "--policy") {
        Some(path) => read_bounded_file(&path, MAX_POLICY_BYTES)?,
        None => DEFAULT_POLICY.as_bytes().to_vec(),
    };
    let policy = match serde_json::from_slice::<RefinementPolicy>(&policy_bytes) {
        Ok(policy) => policy,
        Err(error) => {
            print_json(rejection(
                "invalid_input",
                format!("refinement policy rejected: {error}"),
            ))?;
            return Err("refinement policy rejected".into());
        }
    };
    if let Err(error) = admit_refinement_policy(&policy) {
        return rejected(error);
    }
    let case_bytes = read_bounded_file(&case_path, MAX_CASE_BYTES)?;
    let request = match serde_json::from_slice::<RefinementRequest>(&case_bytes) {
        Ok(request) => request,
        Err(error) => {
            print_json(rejection(
                "invalid_input",
                format!("refinement case rejected: {error}"),
            ))?;
            return Err("refinement case rejected".into());
        }
    };
    match evaluate_refinement(&policy, &request) {
        Ok(report) => {
            print_json(json!({
                "schema_version": EXPLANATION_SCHEMA,
                "outcome": "evaluated",
                "advisory": true,
                "report": report,
                "error": null,
                "boundary": REFINEMENT_BOUNDARY,
            }))?;
            Ok(())
        }
        Err(error) => rejected(error),
    }
}
