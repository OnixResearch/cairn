use super::*;

pub(super) fn export_agent_receipt_bound_to_replay(
    template_path: &Path,
    output_path: &Path,
    replay_verification: &Value,
    policy: &Policy,
) -> Result<Value, Box<dyn Error>> {
    let mut receipt = read_json_artifact(template_path, AGENT_RECEIPT_MAX_BYTES)?;
    let evidence = json!({
        "replay_artifact_path": "replay-agent.json",
        "replay_receipt_hash": replay_verification
            .get("replay_receipt_hash")
            .cloned()
            .unwrap_or(Value::Null),
        "replay_policy_hash": replay_verification
            .get("replay_policy_hash")
            .cloned()
            .unwrap_or(Value::Null),
        "replay_environment_attestation_hash": replay_verification
            .get("environment_attestation_hash")
            .cloned()
            .unwrap_or(Value::Null),
        "replay_selected_cases": replay_verification
            .get("selected_cases")
            .cloned()
            .unwrap_or_else(|| json!([])),
        "replay_selected_groups": replay_verification
            .get("selected_groups")
            .cloned()
            .unwrap_or_else(|| json!([])),
        "replay_verdict": if replay_verification.get("valid").and_then(Value::as_bool) == Some(true) { "pass" } else { "fail" },
    });
    receipt["replay_evidence"] = evidence;
    if let Some(object) = receipt.as_object_mut() {
        object.remove("receipt_hash");
    }
    let validation = agent_receipt_validation_receipt(
        "agent-receipt validate",
        Some(output_path.to_path_buf()),
        &receipt,
        false,
        policy,
    );
    receipt["receipt_hash"] = validation
        .get("agent_receipt_hash")
        .cloned()
        .unwrap_or(Value::Null);
    let final_validation = agent_receipt_validation_receipt(
        "agent-receipt validate",
        Some(output_path.to_path_buf()),
        &receipt,
        false,
        policy,
    );
    if final_validation.get("valid").and_then(Value::as_bool) != Some(true) {
        return Err("generated agent receipt failed validation".into());
    }
    write_evidence_json(output_path, &receipt, &mut Vec::new(), "agent_receipt");
    Ok(receipt)
}
