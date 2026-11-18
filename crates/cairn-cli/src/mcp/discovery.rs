use cairn_core::verified::mcp_discovery::{DiscoveryPolicy, ToolCatalog, ToolDescriptor};

use super::*;

const BUILTIN: &str = include_str!("../../../../cairn-policy/generated/mcp-discovery.json");

pub(super) fn load_catalog(
    profile: &str,
    allow_mutations: bool,
) -> Result<ToolCatalog, Box<dyn Error>> {
    let policy = DiscoveryPolicy::parse(BUILTIN)?;
    let authority = mcp_tools(true)
        .into_iter()
        .map(|descriptor| ToolDescriptor {
            requires_mutations: is_mutating_mcp_tool(
                descriptor["name"].as_str().unwrap_or_default(),
            ),
            descriptor,
        })
        .collect::<Vec<_>>();
    Ok(policy.admit(&authority, profile, allow_mutations)?)
}

pub(super) fn catalog_json(catalog: &ToolCatalog) -> Value {
    json!({
        "protocol_version": MCP_PROTOCOL_VERSION,
        "allow_mutations": catalog.allow_mutations,
        "tool_profile": catalog.profile,
        "catalog_identity": catalog.identity,
        "tools": catalog.tools(),
    })
}
