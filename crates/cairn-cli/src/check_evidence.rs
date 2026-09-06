//! Read-only producer planning shell. No process or publication fallback exists.
use std::{collections::BTreeMap, error::Error, path::Path};

use cairn_core::verified::check_evidence::{admit_plan, Applicability, CheckProfile};
use cairn_core::{canonical_json_hash, AgentVerificationContract};
use serde::de::DeserializeOwned;
use serde_json::json;

use crate::support::{load_policy_file, print_json, CapabilityRoot, CapabilityRootKind};

const MAX_INPUT_BYTES: u64 = 1_048_576;
const MAX_ARGUMENTS: usize = 12;

pub(crate) fn run_check_evidence(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    match args.first().map(String::as_str) {
        Some("plan") => run_plan(&args[1..]),
        Some("run") => Err("check-evidence run unavailable: bounded-exec 29dac88ecded94457572db3fdfaaaab95fa91525 is not admitted in this build; no checks executed".into()),
        Some("import") => Err("check-evidence import unavailable: no pinned producer admission adapters are installed; no results admitted".into()),
        _ => Err("check-evidence requires plan; run and import are unavailable in this build".into()),
    }
}

fn options(args: &[String]) -> Result<BTreeMap<&str, &str>, Box<dyn Error>> {
    if args.len() > MAX_ARGUMENTS || !args.len().is_multiple_of(2) {
        return Err("check-evidence plan requires named input pairs".into());
    }
    let mut options = BTreeMap::new();
    for pair in args.chunks_exact(2) {
        if !matches!(
            pair[0].as_str(),
            "--root" | "--contract" | "--profiles" | "--applicability" | "--policy"
        ) || pair[1].is_empty()
            || pair[1].starts_with("--")
            || options.insert(pair[0].as_str(), pair[1].as_str()).is_some()
        {
            return Err(
                "check-evidence plan rejected an unknown, empty, or repeated option".into(),
            );
        }
    }
    Ok(options)
}

fn read_input<T: DeserializeOwned>(root: &CapabilityRoot, name: &str) -> Result<T, Box<dyn Error>> {
    let bytes = root
        .read_bounded(Path::new(name), MAX_INPUT_BYTES)
        .map_err(|_| "check-evidence input is unavailable or exceeds its bound")?;
    serde_json::from_slice(&bytes).map_err(|_| "check-evidence input is malformed".into())
}

fn run_plan(args: &[String]) -> Result<(), Box<dyn Error>> {
    let options = options(args)?;
    let contract_path = options.get("--contract").ok_or("missing --contract")?;
    let profiles_path = options.get("--profiles").ok_or("missing --profiles")?;
    let applicability_path = options
        .get("--applicability")
        .ok_or("missing --applicability")?;
    let root = CapabilityRoot::open(
        CapabilityRootKind::Repository,
        Path::new(options.get("--root").copied().unwrap_or(".")),
    )?;
    let contract: AgentVerificationContract = read_input(&root, contract_path)?;
    let profiles: Vec<CheckProfile> = read_input(&root, profiles_path)?;
    let applicability: Applicability = read_input(&root, applicability_path)?;
    let policy_path = options
        .get("--policy")
        .copied()
        .unwrap_or(crate::support::default_policy_path());
    let policy = load_policy_file(policy_path.into())?;
    if applicability.policy_hash != canonical_json_hash(&policy.raw) {
        return Err("check-evidence policy identity is stale".into());
    }
    let plan = admit_plan(&contract, &profiles, &applicability)?;
    // The supplied subject is declared, not an observed filesystem snapshot.
    print_json(json!({
        "schema": "cairn.check-evidence-plan-receipt.v1", "valid": true,
        "checks_executed": false, "subject_observed": false, "publication_performed": false,
        "execution_available": false, "plan": plan,
    }))
}
