use std::error::Error;

use cairn_core::{
    canonical_json_hash, SteelOrchestrationProfile, STEEL_ORCHESTRATION_RECEIPT_SCHEMA_V1,
};
use serde_json::{json, Value};

use crate::support::{
    default_policy_path, load_policy_file, print_json, read_option, read_string_option,
};

const DEFAULT_PROFILE_ID: &str = "disabled.lifecycle.default";
const NO_SCRIPT_HASH: &str = "none";

pub(crate) fn run_steel_orchestration(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(steel_usage().into());
    };

    match command {
        "receipt" => run_receipt(args[1..].to_vec()),
        "--help" | "-h" | "help" => {
            println!("{}", steel_usage());
            Ok(())
        }
        unknown => Err(format!(
            "unknown steel-orchestration command: {unknown}\n\n{}",
            steel_usage()
        )
        .into()),
    }
}

fn run_receipt(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args.clone(), "--root", ".");
    let policy_path = read_option(
        args.clone(),
        "--policy",
        &root.join(default_policy_path()).to_string_lossy(),
    );
    let policy = load_policy_file(policy_path)?;
    let requested_profile_id = read_string_option(&args, "--profile");
    let script_hash =
        read_string_option(&args, "--script-hash").unwrap_or_else(|| NO_SCRIPT_HASH.into());

    let profiles = &policy.steel_orchestration_policy.profiles;
    let selected_profile = select_profile(profiles, requested_profile_id.as_deref());
    let mut issues = Vec::new();
    if selected_profile.is_none() {
        issues.push(json!({
            "code": "profile_not_found",
            "class": "denied",
            "message": "requested Steel orchestration profile is not declared by policy"
        }));
    }

    let profile = selected_profile.cloned().unwrap_or_else(fallback_profile);
    let outcome = receipt_outcome(policy.steel_orchestration_policy.enabled, &profile);
    let fallback_reason = fallback_reason(policy.steel_orchestration_policy.enabled, &profile);
    if outcome != "disabled" {
        issues.push(json!({
            "code": "steel_runtime_unavailable",
            "class": "fallback",
            "message": "Steel orchestration runtime evaluation is not wired in this shell"
        }));
    }

    let mut receipt = json!({
        "schema_version": STEEL_ORCHESTRATION_RECEIPT_SCHEMA_V1,
        "command": "steel-orchestration receipt",
        "valid": issues.iter().all(|issue| issue.get("class").and_then(Value::as_str) != Some("denied")),
        "verdict": if issues.iter().any(|issue| issue.get("class").and_then(Value::as_str) == Some("denied")) { "FAIL" } else { "PASS" },
        "outcome": outcome,
        "mode": profile.mode,
        "profile_id": profile.id,
        "steel_enabled": policy.steel_orchestration_policy.enabled,
        "steel_runtime_available": false,
        "script_hash": script_hash,
        "authorized_actions": [],
        "denied_actions": [],
        "fallback_reason": fallback_reason,
        "redaction_classes": profile.redaction_classes,
        "receipt_schema_version": profile.receipt_schema_version,
        "issues": issues,
        "receipt_hash": "",
    });
    let hash_material = receipt_without_hash(&receipt);
    receipt["receipt_hash"] = json!(canonical_json_hash(&hash_material));
    print_json(receipt)
}

fn select_profile<'a>(
    profiles: &'a [SteelOrchestrationProfile],
    requested_profile_id: Option<&str>,
) -> Option<&'a SteelOrchestrationProfile> {
    match requested_profile_id {
        Some(profile_id) => profiles.iter().find(|profile| profile.id == profile_id),
        None => profiles
            .iter()
            .find(|profile| profile.id == DEFAULT_PROFILE_ID)
            .or_else(|| profiles.first()),
    }
}

fn receipt_outcome(is_policy_enabled: bool, profile: &SteelOrchestrationProfile) -> &'static str {
    if !is_policy_enabled || profile.mode == "disabled" {
        "disabled"
    } else {
        "fallback"
    }
}

fn fallback_reason(is_policy_enabled: bool, profile: &SteelOrchestrationProfile) -> &'static str {
    if !is_policy_enabled {
        "policy_disabled"
    } else if profile.mode == "disabled" {
        "profile_disabled"
    } else {
        "steel_runtime_unavailable"
    }
}

fn fallback_profile() -> SteelOrchestrationProfile {
    SteelOrchestrationProfile {
        id: DEFAULT_PROFILE_ID.into(),
        mode: "disabled".into(),
        allowed_host_functions: Vec::new(),
        deterministic_budgets: Vec::new(),
        redaction_classes: vec!["safe_metadata".into()],
        fallback_modes: vec!["rust_nickel".into()],
        receipt_schema_version: STEEL_ORCHESTRATION_RECEIPT_SCHEMA_V1.into(),
    }
}

fn receipt_without_hash(receipt: &Value) -> Value {
    let mut material = receipt.clone();
    material["receipt_hash"] = json!("");
    material
}

fn steel_usage() -> &'static str {
    "usage: cairn steel-orchestration receipt [--root PATH] [--policy PATH] [--profile ID] [--script-hash HASH]"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_policy_selects_default_profile() {
        let profile = SteelOrchestrationProfile {
            id: DEFAULT_PROFILE_ID.into(),
            mode: "disabled".into(),
            allowed_host_functions: Vec::new(),
            deterministic_budgets: Vec::new(),
            redaction_classes: vec!["safe_metadata".into()],
            fallback_modes: vec!["rust_nickel".into()],
            receipt_schema_version: STEEL_ORCHESTRATION_RECEIPT_SCHEMA_V1.into(),
        };

        assert_eq!(
            select_profile(&[profile], None).unwrap().id,
            DEFAULT_PROFILE_ID
        );
    }

    #[test]
    fn enabled_profile_falls_back_until_runtime_is_wired() {
        let profile = SteelOrchestrationProfile {
            id: "comparison.lifecycle.closeout".into(),
            mode: "comparison".into(),
            allowed_host_functions: vec!["cairn.validate".into()],
            deterministic_budgets: vec!["no_fs_mutation".into()],
            redaction_classes: vec!["safe_metadata".into()],
            fallback_modes: vec!["rust_nickel".into()],
            receipt_schema_version: STEEL_ORCHESTRATION_RECEIPT_SCHEMA_V1.into(),
        };

        assert_eq!(receipt_outcome(true, &profile), "fallback");
        assert_eq!(fallback_reason(true, &profile), "steel_runtime_unavailable");
    }
}
