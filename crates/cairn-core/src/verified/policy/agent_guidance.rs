use super::PolicyError;

pub(super) fn agent_guidance_policy(
    value: &serde_json::Value,
) -> Result<crate::verified::agent_guidance::AgentGuidancePolicy, PolicyError> {
    // r[impl cairn.agent_guidance_context.project_config]
    let policy = value
        .get("agent_guidance_policy")
        .cloned()
        .ok_or(PolicyError::MissingField("agent_guidance_policy"))?;
    let parsed =
        serde_json::from_value::<crate::verified::agent_guidance::AgentGuidancePolicy>(policy)
            .map_err(|_| PolicyError::InvalidField("agent_guidance_policy"))?;
    crate::verified::agent_guidance::validate_agent_guidance_policy(&parsed)
        .map_err(PolicyError::InvalidField)?;
    Ok(parsed)
}
