use std::{error::Error, fs, path::PathBuf};

use cairn_core::{
    blake3_hex, canonical_json_hash, validate_steel_action_envelope, SteelActionIssue,
    SteelOrchestrationProfile, STEEL_ORCHESTRATION_RECEIPT_SCHEMA_V1,
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
    let requested_script_hash = read_string_option(&args, "--script-hash");
    let script_path = read_string_option(&args, "--script").map(PathBuf::from);

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
    let evaluation = evaluate_script_if_enabled(
        policy.steel_orchestration_policy.enabled,
        &profile,
        script_path.as_ref(),
        requested_script_hash.as_deref(),
    );
    issues.extend(evaluation.issues.clone());
    let outcome = receipt_outcome(
        policy.steel_orchestration_policy.enabled,
        &profile,
        &evaluation,
    );
    let fallback_reason = fallback_reason(
        policy.steel_orchestration_policy.enabled,
        &profile,
        &evaluation,
    );

    let mut receipt = json!({
        "schema_version": STEEL_ORCHESTRATION_RECEIPT_SCHEMA_V1,
        "command": "steel-orchestration receipt",
        "valid": issues.iter().all(|issue| issue.get("class").and_then(Value::as_str) != Some("denied")),
        "verdict": if issues.iter().any(|issue| issue.get("class").and_then(Value::as_str) == Some("denied")) { "FAIL" } else { "PASS" },
        "outcome": outcome,
        "mode": profile.mode,
        "profile_id": profile.id,
        "steel_enabled": policy.steel_orchestration_policy.enabled,
        "steel_runtime_available": evaluation.runtime_available,
        "script_hash": evaluation.script_hash,
        "authorized_actions": evaluation.authorized_actions,
        "denied_actions": evaluation.denied_actions,
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

fn receipt_outcome(
    is_policy_enabled: bool,
    profile: &SteelOrchestrationProfile,
    evaluation: &ScriptEvaluation,
) -> &'static str {
    if !is_policy_enabled || profile.mode == "disabled" {
        "disabled"
    } else if evaluation.runtime_available && !evaluation.has_denial() {
        "planned"
    } else if evaluation.runtime_available {
        "denied"
    } else {
        "fallback"
    }
}

fn fallback_reason(
    is_policy_enabled: bool,
    profile: &SteelOrchestrationProfile,
    evaluation: &ScriptEvaluation,
) -> &'static str {
    if !is_policy_enabled {
        "policy_disabled"
    } else if profile.mode == "disabled" {
        "profile_disabled"
    } else if evaluation.runtime_available {
        "none"
    } else if evaluation.script_requested {
        "steel_script_unavailable"
    } else {
        "steel_runtime_unavailable"
    }
}

struct ScriptEvaluation {
    runtime_available: bool,
    script_requested: bool,
    script_hash: String,
    authorized_actions: Vec<Value>,
    denied_actions: Vec<Value>,
    issues: Vec<Value>,
}

impl ScriptEvaluation {
    fn disabled(requested_hash: Option<&str>) -> Self {
        Self {
            runtime_available: false,
            script_requested: false,
            script_hash: requested_hash.unwrap_or(NO_SCRIPT_HASH).to_string(),
            authorized_actions: Vec::new(),
            denied_actions: Vec::new(),
            issues: Vec::new(),
        }
    }

    fn has_denial(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.get("class").and_then(Value::as_str) == Some("denied"))
    }
}

fn evaluate_script_if_enabled(
    is_policy_enabled: bool,
    profile: &SteelOrchestrationProfile,
    script_path: Option<&PathBuf>,
    requested_script_hash: Option<&str>,
) -> ScriptEvaluation {
    if !is_policy_enabled || profile.mode == "disabled" {
        return ScriptEvaluation::disabled(requested_script_hash);
    }

    let Some(script_path) = script_path else {
        let mut evaluation = ScriptEvaluation::disabled(requested_script_hash);
        evaluation.issues.push(json!({
            "code": "steel_runtime_unavailable",
            "class": "fallback",
            "message": "no reviewed Steel script envelope was provided"
        }));
        return evaluation;
    };

    let script_bytes = match fs::read(script_path) {
        Ok(bytes) => bytes,
        Err(_) => {
            let mut evaluation = ScriptEvaluation::disabled(requested_script_hash);
            evaluation.script_requested = true;
            evaluation.issues.push(json!({
                "code": "steel_script_unavailable",
                "class": "fallback",
                "message": "reviewed Steel script envelope could not be read"
            }));
            return evaluation;
        }
    };
    let script_hash = blake3_hex(&script_bytes);
    let mut issues = Vec::new();
    if let Some(expected_hash) = requested_script_hash {
        if expected_hash != script_hash {
            issues.push(json!({
                "code": "script_hash_mismatch",
                "class": "denied",
                "message": "reviewed Steel script hash does not match the requested hash"
            }));
        }
    }

    let envelope: Value = match serde_json::from_slice(&script_bytes) {
        Ok(value) => value,
        Err(_) => {
            issues.push(json!({
                "code": "invalid_script_envelope_json",
                "class": "denied",
                "message": "reviewed Steel script envelope is not valid JSON"
            }));
            return ScriptEvaluation {
                runtime_available: true,
                script_requested: true,
                script_hash,
                authorized_actions: Vec::new(),
                denied_actions: Vec::new(),
                issues,
            };
        }
    };

    if envelope.get("profile_id").and_then(Value::as_str) != Some(profile.id.as_str()) {
        issues.push(json!({
            "code": "profile_mismatch",
            "class": "denied",
            "message": "reviewed Steel script envelope targets a different profile"
        }));
    }
    let validation = validate_steel_action_envelope(&envelope, &profile.allowed_host_functions);
    let denied_actions = validation
        .issues
        .iter()
        .map(steel_issue_json)
        .collect::<Vec<_>>();
    issues.extend(denied_actions.iter().cloned());
    let authorized_actions = if validation.valid && issues.is_empty() {
        envelope
            .get("actions")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(redacted_action)
            .collect()
    } else {
        Vec::new()
    };

    ScriptEvaluation {
        runtime_available: true,
        script_requested: true,
        script_hash,
        authorized_actions,
        denied_actions,
        issues,
    }
}

fn steel_issue_json(issue: &SteelActionIssue) -> Value {
    json!({
        "code": issue.code,
        "class": "denied",
        "field_path": issue.field_path,
        "message": issue.message,
    })
}

fn redacted_action(action: Value) -> Value {
    json!({
        "id": action.get("id").and_then(Value::as_str).unwrap_or(""),
        "host_function": action.get("host_function").and_then(Value::as_str).unwrap_or(""),
        "request_hash": canonical_json_hash(&action),
    })
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
    "usage: cairn steel-orchestration receipt [--root PATH] [--policy PATH] [--profile ID] [--script PATH] [--script-hash HASH]"
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

        let evaluation = ScriptEvaluation::disabled(None);
        assert_eq!(receipt_outcome(true, &profile, &evaluation), "fallback");
        assert_eq!(
            fallback_reason(true, &profile, &evaluation),
            "steel_runtime_unavailable"
        );
    }
}
