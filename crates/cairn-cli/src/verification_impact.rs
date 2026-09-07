use std::{collections::BTreeMap, error::Error, path::Path};

use cairn_core::verified::verification_impact::adapters::{
    admit_verification_structural_evidence, VerificationStructuralEvidence,
};
use cairn_core::{canonical_json_hash, derive_verification_impact, VerificationImpactInput};
use serde_json::{json, Value};

use crate::support::{print_json, CapabilityRoot, CapabilityRootKind};

const MAX_INPUT_BYTES: u64 = 1_048_576;
const MAX_ARGUMENTS: usize = 10;

pub(crate) fn run_verification_impact(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    match args.first().map(String::as_str) {
        Some("guidance") => {
            let value = guidance_value(&args[1..])?;
            print_json(value)
        }
        _ => Err("verification-impact requires guidance".into()),
    }
}

pub(crate) fn guidance_value_from_mcp(input: &Value) -> Result<Value, Box<dyn Error>> {
    let source = input.get("input").ok_or("missing input")?.clone();
    let request = GuidanceRequest {
        root: input["root"].as_str().unwrap_or(".").to_string(),
        change: input["change"].as_str().map(ToString::to_string),
        contract: input["contract"].as_str().map(ToString::to_string),
        profiles: input["profiles"].as_str().map(ToString::to_string),
        applicability: input["applicability"].as_str().map(ToString::to_string),
    };
    derive_value(source, input.get("evidence").cloned(), &request)
}

fn guidance_value(args: &[String]) -> Result<Value, Box<dyn Error>> {
    let options = named_options(args)?;
    let root_path = options.get("--root").copied().unwrap_or(".");
    let input_path = options.get("--input").ok_or("missing --input")?;
    let root = CapabilityRoot::open(CapabilityRootKind::Repository, Path::new(root_path))?;
    let bytes = root.read_bounded(Path::new(input_path), MAX_INPUT_BYTES)?;
    let input = serde_json::from_slice(&bytes)?;
    let evidence = options
        .get("--evidence")
        .map(|path| {
            let bytes = root.read_bounded(Path::new(path), MAX_INPUT_BYTES)?;
            Ok::<Value, Box<dyn Error>>(serde_json::from_slice(&bytes)?)
        })
        .transpose()?;
    let request = GuidanceRequest {
        root: root_path.to_string(),
        change: options.get("--change").map(|value| (*value).to_string()),
        contract: options.get("--contract").map(|value| (*value).to_string()),
        profiles: options.get("--profiles").map(|value| (*value).to_string()),
        applicability: options
            .get("--applicability")
            .map(|value| (*value).to_string()),
    };
    derive_value(input, evidence, &request)
}

fn derive_value(
    input: Value,
    evidence: Option<Value>,
    request: &GuidanceRequest,
) -> Result<Value, Box<dyn Error>> {
    // r[impl cairn.verification_impact.interfaces]
    let plan_parts = [&request.contract, &request.profiles, &request.applicability];
    if plan_parts.iter().any(|part| part.is_some()) && plan_parts.iter().any(|part| part.is_none())
    {
        return Err(
            "producer-plan request requires contract, profiles, and applicability together".into(),
        );
    }
    let input_hash = canonical_json_hash(&json!({"input":input,"evidence":evidence}));
    let impact: VerificationImpactInput = serde_json::from_value(input)?;
    let evidence: Option<VerificationStructuralEvidence> =
        evidence.map(serde_json::from_value).transpose()?;
    let admission = admit_verification_structural_evidence(&impact, evidence.as_ref());
    let report = derive_verification_impact(&impact);
    let valid = report.valid && admission.valid;
    let report_value = json!(report);
    Ok(json!({
        "schema": "cairn.verification-impact-guidance.v1",
        "valid": valid,
        "checks_executed": false,
        "mutation_authorized": false,
        "read_only": true,
        "input_hash": input_hash,
        "report": report_value,
        "structural_evidence": admission,
        "actions": if valid { guidance_actions(request) } else { Vec::new() },
    }))
}

fn guidance_actions(request: &GuidanceRequest) -> Vec<Value> {
    let mut actions = Vec::new();
    if let Some(change) = &request.change {
        actions.push(json!({
            "kind": "unified_readiness",
            "command": format!("cairn readiness {} --root {}", change, request.root),
            "command_is_display_only": true,
            "argv": ["cairn", "readiness", change, "--root", &request.root],
            "tool": "cairn_readiness",
            "arguments": {"change": change, "root": request.root},
            "mutation_authorized": false,
            "full_gate_authoritative": true,
        }));
    }
    if let (Some(contract), Some(profiles), Some(applicability)) =
        (&request.contract, &request.profiles, &request.applicability)
    {
        actions.push(json!({
            "kind": "check_evidence_plan_request",
            "command": format!(
                "cairn check-evidence plan --root {} --contract {} --profiles {} --applicability {}",
                request.root, contract, profiles, applicability
            ),
            "command_is_display_only": true,
            "argv": ["cairn", "check-evidence", "plan", "--root", &request.root,
                "--contract", contract, "--profiles", profiles, "--applicability", applicability],
            "arguments": {"root": request.root, "contract": contract,
                "profiles": profiles, "applicability": applicability},
            "checks_executed": false,
            "publication_performed": false,
            "requires_operator_review": true,
        }));
    }
    actions
}

struct GuidanceRequest {
    root: String,
    change: Option<String>,
    contract: Option<String>,
    profiles: Option<String>,
    applicability: Option<String>,
}

fn named_options(args: &[String]) -> Result<BTreeMap<&str, &str>, Box<dyn Error>> {
    if args.len() > MAX_ARGUMENTS * 2 || !args.len().is_multiple_of(2) {
        return Err("verification-impact guidance requires named input pairs".into());
    }
    let allowed = [
        "--root",
        "--input",
        "--change",
        "--contract",
        "--profiles",
        "--applicability",
        "--evidence",
    ];
    let mut options = BTreeMap::new();
    for pair in args.chunks_exact(2) {
        if !allowed.contains(&pair[0].as_str())
            || pair[1].is_empty()
            || pair[1].starts_with("--")
            || options.insert(pair[0].as_str(), pair[1].as_str()).is_some()
        {
            return Err(
                "verification-impact guidance rejected an unknown, empty, or repeated option"
                    .into(),
            );
        }
    }
    Ok(options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // r[verify cairn.verification_impact.interfaces]
    fn actions_are_read_only_and_request_check_evidence_plan_without_execution() {
        let request = GuidanceRequest {
            root: ".".to_string(),
            change: Some("demo".to_string()),
            contract: Some("contract.json".to_string()),
            profiles: Some("profiles.json".to_string()),
            applicability: Some("applicability.json".to_string()),
        };
        let actions = guidance_actions(&request);
        assert_eq!(actions.len(), 2);
        assert_eq!(actions[0]["kind"], "unified_readiness");
        assert_eq!(actions[0]["mutation_authorized"], false);
        assert_eq!(actions[1]["kind"], "check_evidence_plan_request");
        assert_eq!(actions[1]["checks_executed"], false);
        assert!(actions[1]["command"]
            .as_str()
            .expect("command")
            .contains("cairn check-evidence plan"));
    }
}
