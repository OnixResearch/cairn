use std::{error::Error, fs, path::PathBuf};

use cairn_core::{derive_adoption_report, AdoptionSnapshot};
use serde_json::{json, Value};

use crate::support::{path_string, print_json, read_string_option};

const KIBIBYTE_BYTES: u64 = 1024;
const MAX_ADOPTION_SNAPSHOT_KIBIBYTES: u64 = 256;
const MAX_ADOPTION_SNAPSHOT_BYTES: u64 = MAX_ADOPTION_SNAPSHOT_KIBIBYTES * KIBIBYTE_BYTES;

pub(crate) fn run_adoption(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err("adoption requires report".into());
    };
    match command {
        "report" => {
            adoption_report_value(&adoption_input_from_args(&args[1..])?).and_then(print_json)
        }
        unknown => Err(format!("unknown adoption command: {unknown}").into()),
    }
}

pub(crate) fn adoption_report_value(input: &Value) -> Result<Value, Box<dyn Error>> {
    let root = PathBuf::from(input["root"].as_str().unwrap_or("."));
    let snapshot_path = PathBuf::from(
        input["snapshot"]
            .as_str()
            .ok_or("adoption report requires snapshot")?,
    );
    let snapshot_ref = if snapshot_path.is_absolute() {
        snapshot_path.clone()
    } else {
        root.join(&snapshot_path)
    };
    let bytes = read_bounded_snapshot(&snapshot_ref)?;
    let snapshot_value: Value = serde_json::from_slice(&bytes)?;
    let snapshot: AdoptionSnapshot = serde_json::from_value(snapshot_value.clone())?;
    let report = derive_adoption_report(&snapshot);
    Ok(json!({
        "command": "adoption report",
        "root": path_string(root),
        "snapshot": path_string(snapshot_path),
        "snapshot_hash": cairn_core::canonical_json_hash(&snapshot_value),
        "read_only": true,
        "mutation_authorized": false,
        "checks_executed": false,
        "coverage": report.coverage,
        "actions": report.actions,
        "report": report,
    }))
}

fn adoption_input_from_args(args: &[String]) -> Result<Value, Box<dyn Error>> {
    let snapshot =
        read_string_option(args, "--snapshot").ok_or("adoption report requires --snapshot")?;
    let root = read_string_option(args, "--root").unwrap_or_else(|| ".".to_string());
    for index in (0..args.len()).step_by(2) {
        let Some(flag) = args.get(index) else { break };
        if !matches!(flag.as_str(), "--snapshot" | "--root") {
            return Err(format!("unsupported adoption report option: {flag}").into());
        }
        if args.get(index + 1).is_none() {
            return Err("adoption report options require values".into());
        }
    }
    if !args.len().is_multiple_of(2) {
        return Err("adoption report options require values".into());
    }
    Ok(json!({"root": root, "snapshot": snapshot}))
}

fn read_bounded_snapshot(path: &PathBuf) -> Result<Vec<u8>, Box<dyn Error>> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() {
        return Err("adoption snapshot must not be a symlink".into());
    }
    if metadata.len() > MAX_ADOPTION_SNAPSHOT_BYTES {
        return Err(format!(
            "adoption snapshot exceeds bounded read limit: {} > {MAX_ADOPTION_SNAPSHOT_BYTES}",
            metadata.len()
        )
        .into());
    }
    Ok(fs::read(path)?)
}
