//! Read-only advisory claim-derivation inspection. No lifecycle mutation.
use std::{error::Error, path::Path};

use cairn_core::{
    admit_claim_policy, evaluate_derivations, ClaimPolicy, DerivationError, EvaluationRequest,
    DERIVATION_BOUNDARY,
};
use serde_json::{json, Value};

use crate::support::{
    print_json, read_string_option, reject_positional_arguments, reject_unknown_options,
    CapabilityRoot, CapabilityRootKind,
};

const EXPLANATION_SCHEMA: &str = "cairn.claim-derivation.explanation.v1";
const MAX_CASE_BYTES: u64 = 4 * 1024 * 1024;
const MAX_POLICY_BYTES: u64 = 16 * 1024 * 1024;
const DEFAULT_POLICY: &str =
    include_str!("../../../cairn-policy/generated/scoped-claim-derivations.json");

fn read_bounded_file(path: &str, maximum: u64) -> Result<Vec<u8>, Box<dyn Error>> {
    let (root, relative) =
        CapabilityRoot::open_file_parent(CapabilityRootKind::EvidenceBundle, Path::new(path), false)?;
    Ok(root.read_bounded(&relative, maximum)?)
}

fn rejection(outcome: &str, detail: String) -> Value {
    json!({
        "schema_version": EXPLANATION_SCHEMA,
        "outcome": "rejected",
        "advisory": true,
        "report": null,
        "error": {"outcome": outcome, "detail": detail},
        "boundary": DERIVATION_BOUNDARY,
    })
}

fn rejected(error: DerivationError) -> Result<(), Box<dyn Error>> {
    let outcome = error.outcome();
    print_json(rejection(outcome, format!("{error:?}")))?;
    Err("claim derivation admission rejected".into())
}

/// Bounded reads only. Support is advisory and never authorizes a lifecycle action.
// r[impl cairn.scoped_claims.inspection]
pub(crate) fn run(mut args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.first().map(String::as_str) != Some("explain") {
        return Err("usage: cairn claim-derivation explain --case FILE [--policy FILE]".into());
    }
    args.remove(0);
    reject_unknown_options(&args, &["--case", "--policy"])?;
    reject_positional_arguments(&args)?;
    let case_path = read_string_option(&args, "--case").ok_or("--case is required")?;
    let policy_bytes = match read_string_option(&args, "--policy") {
        Some(path) => read_bounded_file(&path, MAX_POLICY_BYTES)?,
        None => DEFAULT_POLICY.as_bytes().to_vec(),
    };
    let policy = match serde_json::from_slice::<ClaimPolicy>(&policy_bytes) {
        Ok(policy) => policy,
        Err(error) => {
            print_json(rejection(
                "invalid_input",
                format!("claim policy rejected: {error}"),
            ))?;
            return Err("claim policy rejected".into());
        }
    };
    if let Err(error) = admit_claim_policy(&policy) {
        return rejected(error);
    }
    let case_bytes = read_bounded_file(&case_path, MAX_CASE_BYTES)?;
    let request = match serde_json::from_slice::<EvaluationRequest>(&case_bytes) {
        Ok(request) => request,
        Err(error) => {
            print_json(rejection(
                "invalid_input",
                format!("derivation case rejected: {error}"),
            ))?;
            return Err("derivation case rejected".into());
        }
    };
    match evaluate_derivations(&policy, &request) {
        Ok(report) => {
            print_json(json!({
                "schema_version": EXPLANATION_SCHEMA,
                "outcome": "evaluated",
                "advisory": true,
                "report": report,
                "error": null,
                "boundary": DERIVATION_BOUNDARY,
            }))?;
            Ok(())
        }
        Err(error) => rejected(error),
    }
}
