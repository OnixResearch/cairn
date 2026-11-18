//! Read-only advisory consistency inspection. No lifecycle mutation.
use crate::{
    requirement_id_registry::read_accepted_specifications,
    support::{
        default_policy_path, print_json, read_option, read_string_option,
        reject_positional_arguments, reject_unknown_options, CapabilityRoot, CapabilityRootKind,
    },
};
use cairn_core::{
    analyze_consistency, parse_content_bound_requirement_registry_value, replay_witness,
    verify_content_bound_requirement_registry, AnalysisWitness, ConsistencyCase, ConsistencyError,
    ConsistencyPolicy, CONSISTENCY_BOUNDARY,
};
use serde_json::json;
use std::{error::Error, path::Path};

const INSPECTION_SCHEMA: &str = "cairn.requirement-analysis.inspection.v1";
const MAX_INPUT_BYTES: u64 = 4 * 1024 * 1024;
const MAX_POLICY_BYTES: u64 = 16 * 1024 * 1024;
const DEFAULT_ANALYSIS_POLICY: &str = "cairn-policy/generated/consistency-analysis.json";

fn read_bounded(path: &str, maximum: u64) -> Result<Vec<u8>, Box<dyn Error>> {
    let (root, relative) = CapabilityRoot::open_file_parent(
        CapabilityRootKind::EvidenceBundle,
        Path::new(path),
        false,
    )?;
    Ok(root.read_bounded(&relative, maximum)?)
}

fn rejection(outcome: &str, detail: String) -> serde_json::Value {
    json!({
        "schema_version": INSPECTION_SCHEMA,
        "outcome": "rejected",
        "advisory": true,
        "report": null,
        "witness_replay": null,
        "error": {"outcome": outcome, "detail": detail},
        "boundary": CONSISTENCY_BOUNDARY,
    })
}

fn rejected(error: ConsistencyError) -> Result<(), Box<dyn Error>> {
    let outcome = error.outcome();
    print_json(rejection(outcome, format!("{error:?}")))?;
    Err("requirement consistency analysis rejected".into())
}

/// Bounded reads only. The result is advisory and authorizes no lifecycle action.
// r[impl cairn.requirement_consistency.inspection]
pub(crate) fn run(mut args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.first().map(String::as_str) != Some("inspect") {
        return Err("usage: cairn requirement-analysis inspect --case FILE --registry FILE [--root DIR] [--policy FILE] [--analysis-policy FILE] [--witness FILE]".into());
    }
    args.remove(0);
    reject_unknown_options(
        &args,
        &[
            "--case",
            "--registry",
            "--root",
            "--policy",
            "--analysis-policy",
            "--witness",
        ],
    )?;
    reject_positional_arguments(&args)?;
    let case_path = read_string_option(&args, "--case").ok_or("--case is required")?;
    let registry_path = read_string_option(&args, "--registry").ok_or("--registry is required")?;
    let root_path = read_option(args.clone(), "--root", ".");
    let lifecycle_policy_path = read_option(args.clone(), "--policy", default_policy_path());
    let repository = CapabilityRoot::open(CapabilityRootKind::Repository, &root_path)?;
    let registry_value: serde_json::Value = serde_json::from_slice(&read_bounded(
        &registry_path,
        MAX_INPUT_BYTES,
    )?)?;
    let registry = parse_content_bound_requirement_registry_value(&registry_value)
        .map_err(|issue| issue.message)?;
    let (policy_root, relative) = CapabilityRoot::open_file_parent(
        CapabilityRootKind::EvidenceBundle,
        Path::new(&lifecycle_policy_path),
        false,
    )?;
    let lifecycle_policy_bytes = policy_root.read_bounded(&relative, MAX_POLICY_BYTES)?;
    let registry_report = verify_content_bound_requirement_registry(
        &registry,
        read_accepted_specifications(&repository)?,
        &lifecycle_policy_bytes,
        &registry.repository_id,
    );
    if !registry_report.valid {
        print_json(json!({
            "schema_version": INSPECTION_SCHEMA,
            "outcome": "rejected",
            "advisory": true,
            "registry": "invalid_or_stale",
            "issues": cairn_core::content_bound_requirement_registry_issues_json(&registry_report.issues),
            "boundary": CONSISTENCY_BOUNDARY,
        }))?;
        return Err("requirement registry admission failed".into());
    }
    let policy_bytes = match read_string_option(&args, "--analysis-policy") {
        Some(path) => read_bounded(&path, MAX_POLICY_BYTES)?,
        None => repository
            .read_bounded(Path::new(DEFAULT_ANALYSIS_POLICY), MAX_POLICY_BYTES)
            .map_err(|_| {
                "analysis policy unavailable: pass --analysis-policy FILE with the checked export of cairn-policy/consistency-analysis.ncl"
                    .to_string()
            })?,
    };
    let policy: ConsistencyPolicy = match serde_json::from_slice(&policy_bytes) {
        Ok(policy) => policy,
        Err(error) => {
            print_json(rejection(
                "unsupported",
                format!("analysis policy rejected: {error}"),
            ))?;
            return Err("analysis policy rejected".into());
        }
    };
    let case_bytes = read_bounded(&case_path, MAX_INPUT_BYTES)?;
    let case: ConsistencyCase = match serde_json::from_slice(&case_bytes) {
        Ok(case) => case,
        Err(error) => {
            print_json(rejection(
                "invalid_input",
                format!("consistency case rejected: {error}"),
            ))?;
            return Err("consistency case rejected".into());
        }
    };
    for offered in &case.contracts {
        let admitted = registry
            .rows
            .iter()
            .find(|row| row.requirement_id == offered.native_requirement.requirement_id);
        let Some(admitted) = admitted else {
            print_json(rejection(
                "scope_mismatch",
                format!(
                    "case contract names a requirement absent from the admitted native registry: {}",
                    offered.native_requirement.requirement_id
                ),
            ))?;
            return Err("case contract is not bound to an admitted native requirement".into());
        };
        if admitted != &offered.native_requirement {
            print_json(rejection(
                "scope_mismatch",
                format!(
                    "case contract names a stale native revision: {}",
                    offered.native_requirement.requirement_id
                ),
            ))?;
            return Err("case contract names a stale native requirement revision".into());
        }
    }
    let report = match analyze_consistency(&policy, &case) {
        Ok(report) => report,
        Err(error) => return rejected(error),
    };
    let witness_replay = match read_string_option(&args, "--witness") {
        Some(path) => {
            let witness: AnalysisWitness = match serde_json::from_slice(&read_bounded(
                &path,
                MAX_INPUT_BYTES,
            )?) {
                Ok(witness) => witness,
                Err(error) => {
                    print_json(rejection(
                        "invalid_input",
                        format!("retained witness rejected: {error}"),
                    ))?;
                    return Err("retained witness rejected".into());
                }
            };
            match replay_witness(&policy, &case, &witness) {
                Ok(replay) => json!(replay),
                Err(error) => return rejected(error),
            }
        }
        None => json!(null),
    };
    print_json(json!({
        "schema_version": INSPECTION_SCHEMA,
        "outcome": "analyzed",
        "advisory": true,
        "report": report,
        "witness_replay": witness_replay,
        "admitted_proof_evidence": "not_supplied",
        "lifecycle_authority": "none",
        "boundary": CONSISTENCY_BOUNDARY,
    }))?;
    Ok(())
}
