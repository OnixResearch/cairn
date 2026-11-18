//! Read-only advisory revision classification. No lifecycle mutation.
use std::{error::Error, path::Path};

use cairn_core::{
    compare_policy_revisions, compare_requirement_revisions, PolicyComparisonReport,
    PolicyRevisionRequest, RequirementComparisonReport, RequirementRevisionRequest,
    RevisionClassificationError, RevisionClassificationPolicy, REVISION_CLASSIFICATION_BOUNDARY,
};
use serde_json::{json, Value};

use crate::support::{
    print_json, read_string_option, reject_positional_arguments, reject_unknown_options,
    CapabilityRoot, CapabilityRootKind,
};

const INSPECTION_SCHEMA: &str = "cairn.revision-classification.inspection.v1";
const MAX_INPUT_BYTES: u64 = 4 * 1024 * 1024;
const MAX_POLICY_BYTES: u64 = 16 * 1024 * 1024;
const DEFAULT_POLICY: &str =
    include_str!("../../../cairn-policy/generated/revision-classification.json");

fn read_bounded_file(path: &str, maximum: u64) -> Result<Vec<u8>, Box<dyn Error>> {
    let (root, relative) = CapabilityRoot::open_file_parent(
        CapabilityRootKind::EvidenceBundle,
        Path::new(path),
        false,
    )?;
    Ok(root.read_bounded(&relative, maximum)?)
}

fn rejection(error: RevisionClassificationError) -> Value {
    json!({
        "schema_version": INSPECTION_SCHEMA,
        "outcome": "rejected",
        "advisory": true,
        "report": null,
        "error": error,
        "boundary": REVISION_CLASSIFICATION_BOUNDARY,
    })
}

fn rejected(error: RevisionClassificationError) -> Result<(), Box<dyn Error>> {
    print_json(rejection(error))?;
    Err("revision classification rejected".into())
}

fn requirement_report(report: &RequirementComparisonReport) -> Value {
    json!({
        "schema_version": INSPECTION_SCHEMA,
        "outcome": "classified",
        "advisory": true,
        "kind": "requirement",
        "report": report,
        "error": null,
        "boundary": REVISION_CLASSIFICATION_BOUNDARY,
    })
}

fn policy_report(report: &PolicyComparisonReport) -> Value {
    json!({
        "schema_version": INSPECTION_SCHEMA,
        "outcome": "classified",
        "advisory": true,
        "kind": "policy_fragment",
        "report": report,
        "error": null,
        "boundary": REVISION_CLASSIFICATION_BOUNDARY,
    })
}

/// Read-only advisory comparison. It never executes checks, writes lifecycle
/// state, transfers evidence, or changes a native completion decision.
// r[impl cairn.revision_classification.inspection]
pub(crate) fn run(mut args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.first().map(String::as_str) != Some("classify") {
        return Err("usage: cairn revision classify (--requirement FILE | --policy-fragment FILE) [--policy FILE]".into());
    }
    args.remove(0);
    reject_unknown_options(&args, &["--requirement", "--policy-fragment", "--policy"])?;
    reject_positional_arguments(&args)?;
    let requirement_path = read_string_option(&args, "--requirement");
    let fragment_path = read_string_option(&args, "--policy-fragment");
    match (requirement_path, fragment_path) {
        (Some(_), Some(_)) => {
            return Err("select exactly one of --requirement or --policy-fragment".into())
        }
        (None, None) => {
            return Err("one of --requirement or --policy-fragment is required".into())
        }
        _ => {}
    }
    let policy_bytes = match read_string_option(&args, "--policy") {
        Some(path) => read_bounded_file(&path, MAX_POLICY_BYTES)?,
        None => DEFAULT_POLICY.as_bytes().to_vec(),
    };
    let policy: RevisionClassificationPolicy = match serde_json::from_slice(&policy_bytes) {
        Ok(policy) => policy,
        Err(error) => {
            print_json(rejection(RevisionClassificationError::InvalidPolicy(format!(
                "revision classification policy rejected: {error}"
            ))))?;
            return Err("revision classification policy rejected".into());
        }
    };
    if let Err(error) = policy.admit() {
        return rejected(error);
    }

    if let Some(path) = read_string_option(&args, "--requirement") {
        let bytes = read_bounded_file(&path, MAX_INPUT_BYTES)?;
        let request: RequirementRevisionRequest = match serde_json::from_slice(&bytes) {
            Ok(request) => request,
            Err(error) => {
                print_json(rejection(RevisionClassificationError::InvalidInput(format!(
                    "requirement revision request rejected: {error}"
                ))))?;
                return Err("requirement revision request rejected".into());
            }
        };
        return match compare_requirement_revisions(&policy, &request) {
            Ok(report) => {
                print_json(requirement_report(&report))?;
                Ok(())
            }
            Err(error) => rejected(error),
        };
    }

    let path = read_string_option(&args, "--policy-fragment")
        .ok_or("--policy-fragment is required")?;
    let bytes = read_bounded_file(&path, MAX_INPUT_BYTES)?;
    let request: PolicyRevisionRequest = match serde_json::from_slice(&bytes) {
        Ok(request) => request,
        Err(error) => {
            print_json(rejection(RevisionClassificationError::InvalidInput(format!(
                "policy revision request rejected: {error}"
            ))))?;
            return Err("policy revision request rejected".into());
        }
    };
    match compare_policy_revisions(&policy, &request) {
        Ok(report) => {
            print_json(policy_report(&report))?;
            Ok(())
        }
        Err(error) => rejected(error),
    }
}
