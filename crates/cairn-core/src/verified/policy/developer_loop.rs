use super::PolicyError;

pub(super) fn developer_loop_policy(
    value: &serde_json::Value,
) -> Result<crate::verified::developer_loop::DeveloperLoopPolicy, PolicyError> {
    let field = value
        .get("developer_loop_policy")
        .ok_or(PolicyError::MissingField("developer_loop_policy"))?;
    let policy: crate::verified::developer_loop::DeveloperLoopPolicy =
        serde_json::from_value(field.clone())
            .map_err(|_| PolicyError::InvalidField("developer_loop_policy"))?;
    crate::verified::developer_loop::admit_developer_loop_policy(policy.clone()).map_err(
        |first| {
            let first = first.first().expect("admission returns at least one issue");
            PolicyError::InvalidWorkflow {
                code: "developer_loop.policy.invalid",
                path: first.field_path.clone(),
                message: format!("{}: {}", first.code, first.message),
            }
        },
    )?;
    Ok(policy)
}
