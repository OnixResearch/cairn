//! Read-only advisory concurrent-change footprint inspection. No lifecycle mutation.
use std::{error::Error, path::Path};

use cairn_core::{
    analyze_concurrent_changes, ConcurrentChangeCase, FootprintError, FootprintPolicy,
    CONCURRENT_FOOTPRINTS_BOUNDARY,
};
use serde_json::json;

use crate::support::{
    print_json, read_option, read_string_option, reject_positional_arguments,
    reject_unknown_options, CapabilityRoot, CapabilityRootKind,
};

const INSPECTION_SCHEMA: &str = "cairn.concurrent-change-footprints.inspection.v1";
const MAX_INPUT_BYTES: u64 = 4 * 1024 * 1024;
const MAX_POLICY_BYTES: u64 = 16 * 1024 * 1024;
const DEFAULT_POLICY: &str = "cairn-policy/generated/concurrent-change-footprints.json";

fn read_bounded_file(path: &str, maximum: u64) -> Result<Vec<u8>, Box<dyn Error>> {
    let (root, relative) = CapabilityRoot::open_file_parent(
        CapabilityRootKind::EvidenceBundle,
        Path::new(path),
        false,
    )?;
    Ok(root.read_bounded(&relative, maximum)?)
}

fn rejection(error: FootprintError) -> serde_json::Value {
    json!({
        "schema_version": INSPECTION_SCHEMA,
        "outcome": "rejected",
        "advisory": true,
        "report": null,
        "error": error,
        "lifecycle_authority": "none",
        "boundary": CONCURRENT_FOOTPRINTS_BOUNDARY,
    })
}

fn rejected(error: FootprintError) -> Result<(), Box<dyn Error>> {
    print_json(rejection(error))?;
    Err("concurrent change footprint analysis rejected".into())
}

/// Read-only advisory inspection. It never writes lifecycle state, executes
/// checks, transfers evidence or changes a completion decision.
// r[impl cairn.concurrent_footprints.inspection]
pub(crate) fn run(mut args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.first().map(String::as_str) != Some("inspect") {
        return Err(
            "usage: cairn footprints inspect --case FILE [--policy FILE] [--root DIR]".into(),
        );
    }
    args.remove(0);
    reject_unknown_options(&args, &["--case", "--policy", "--root"])?;
    reject_positional_arguments(&args)?;
    let case_path = read_string_option(&args, "--case").ok_or("--case is required")?;
    let root_path = read_option(args.clone(), "--root", ".");
    let policy_bytes = match read_string_option(&args, "--policy") {
        Some(path) => read_bounded_file(&path, MAX_POLICY_BYTES)?,
        None => {
            let repository =
                CapabilityRoot::open(CapabilityRootKind::Repository, Path::new(&root_path))
                    .map_err(|error| {
                        format!("cannot open --root {}: {error}", root_path.display())
                    })?;
            repository
                .read_bounded(Path::new(DEFAULT_POLICY), MAX_POLICY_BYTES)
                .map_err(|_| {
                    "concurrent change footprints policy unavailable: pass --policy FILE with the checked export of cairn-policy/concurrent-change-footprints.ncl".to_string()
                })?
        }
    };
    let policy: FootprintPolicy = match serde_json::from_slice(&policy_bytes) {
        Ok(policy) => policy,
        Err(error) => {
            print_json(rejection(FootprintError::Unsupported(format!(
                "concurrent change footprints policy rejected: {error}"
            ))))?;
            return Err("concurrent change footprints policy rejected".into());
        }
    };
    if let Err(error) = policy.admit() {
        return rejected(error);
    }
    let case_bytes = read_bounded_file(&case_path, MAX_INPUT_BYTES)?;
    let case: ConcurrentChangeCase = match serde_json::from_slice(&case_bytes) {
        Ok(case) => case,
        Err(error) => {
            print_json(rejection(FootprintError::InvalidInput(format!(
                "concurrent change case rejected: {error}"
            ))))?;
            return Err("concurrent change case rejected".into());
        }
    };
    match analyze_concurrent_changes(&policy, &case) {
        Ok(report) => {
            print_json(json!({
                "schema_version": INSPECTION_SCHEMA,
                "outcome": "analyzed",
                "advisory": true,
                "report": report,
                "lifecycle_authority": "none",
                "boundary": CONCURRENT_FOOTPRINTS_BOUNDARY,
            }))?;
            Ok(())
        }
        Err(error) => rejected(error),
    }
}
