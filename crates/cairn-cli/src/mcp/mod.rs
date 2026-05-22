// r[impl cli_maintainability.mcp_cli_source.focused_modules]
// r[impl cli_maintainability.mcp_cli_source.focused_modules.visible_boundaries]
// r[impl cli_maintainability.mcp_cli_source.focused_modules.narrow_bridges]
// r[impl cli_maintainability.mcp_cli_source.behavior_preserved]
// r[verify cli_maintainability.mcp_cli_source.behavior_preserved.focused_checks]
// r[verify cli_maintainability.mcp_cli_source.behavior_preserved.repository_checks]
use std::{
    error::Error,
    fs,
    io::{self, BufRead, Write},
    path::PathBuf,
};

use cairn_core::{
    archive_dir, build_archive_plan, build_change_detail, build_change_skeleton,
    build_doctor_report, build_ironspec_migration_plan, build_openspec_migration_plan,
    build_spec_detail, build_sync_plan, canonical_json_hash, changes_dir, evaluate_gate_snapshot,
    init_artifacts, layout_name, layout_root, specs_dir, validate_changes, validate_spec_sources,
    DoctorSnapshot, GateStage, LifecycleArtifactKind, LifecycleLayout, PlanAction, PlanActionKind,
};
use serde_json::{json, Value};

use crate::agent_receipt::{
    agent_receipt_validation_receipt, ensure_inline_receipt_is_bounded, read_agent_receipt_file,
    AGENT_RECEIPT_MAX_BYTES,
};
use crate::lifecycle::parse_gate_stage;
use crate::migration::{
    collect_layout_migration_inputs, execute_migration_plan, migration_source_layout,
};
use crate::native::{
    change_detail_json, change_status_json, change_template, doctor_report_json, repo_status_json,
    spec_detail_json, write_new_file,
};
use crate::plan_receipts::{
    archive_plan_json, gate_input_json, gate_report_json, migration_plan_json, sync_plan_json,
};
use crate::support::{
    collect_validation_spec_sources, default_policy_path, detect_layout, execute_archive_plan,
    execute_sync_plan, find_change, find_spec_source, has_flag, load_policy_file,
    mutation_gate_blockers, path_string, portable_json_hash, print_json, read_gate_snapshot,
    scan_repo, with_portable_receipt_hash, with_receipt_hash,
};

const MCP_PROTOCOL_VERSION: &str = "2024-11-05";
const MCP_MAX_CONTENT_BYTES: usize = 64 * 1024;

mod common;
mod mutation_tools;
mod read_tools;
mod schema;
mod transport;

use mutation_tools::*;
use read_tools::*;
use schema::*;
use transport::*;

pub(crate) fn run_mcp(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.mcp_server.read_tools]
    // r[impl cairn.supersede.mcp_server.plan_tools]
    // r[impl cairn.supersede.mcp_server.mutation_guard]
    // r[impl cairn.supersede.mcp_server.boundary]
    let is_mutation_allowed = has_flag(&args, "--allow-mutations");
    if has_flag(&args, "--describe") {
        return print_json(mcp_tool_catalog(is_mutation_allowed));
    }
    if let Some(index) = args.iter().position(|arg| arg == "--call") {
        let name = args
            .get(index + 1)
            .ok_or("mcp --call requires a tool name")?;
        let input = args
            .get(index + 2)
            .map(|text| serde_json::from_str(text))
            .transpose()?
            .unwrap_or_else(|| json!({}));
        return print_json(mcp_call_tool(name, &input, is_mutation_allowed)?);
    }
    run_mcp_json_rpc(is_mutation_allowed)
}
pub(crate) fn mcp_tool_catalog(is_allow_mutations: bool) -> Value {
    // r[impl cairn.supersede.mcp_server.boundary.core]
    json!({"protocol_version": MCP_PROTOCOL_VERSION, "allow_mutations": is_allow_mutations, "tools": mcp_tools(is_allow_mutations)})
}

pub(crate) fn mcp_agent_smoke_receipt(root: PathBuf) -> Result<Value, Box<dyn Error>> {
    // r[impl cairn.supersede.mcp_agent_smoke_receipts]
    // r[impl cairn.supersede.mcp_agent_smoke_receipts.flow]
    // r[impl cairn.supersede.mcp_agent_smoke_receipts.evidence]
    // r[impl cairn.supersede.mcp_agent_smoke_receipts.read_only]
    let fixture = "fixtures/agent-receipts/positive.json";
    let root_string = path_string(root.clone());
    let catalog = mcp_tool_catalog(false);
    let tools = catalog["tools"]
        .as_array()
        .ok_or("MCP catalog missing tools")?;
    let receipt_tool = tools
        .iter()
        .find(|tool| tool["name"] == "cairn_agent_receipt_validate")
        .ok_or("MCP catalog missing cairn_agent_receipt_validate")?;
    let receipt_schema = receipt_tool["inputSchema"].clone();
    let receipt_call = mcp_call_tool(
        "cairn_agent_receipt_validate",
        &json!({"root": root_string, "path": fixture}),
        false,
    )?;
    let validation = &receipt_call["structuredContent"];
    let invalid_argument_error = match mcp_call_tool(
        "cairn_gate",
        &json!({"root": path_string(root.clone()), "change": "demo", "stage": "archive"}),
        false,
    ) {
        Ok(_) => return Err("MCP smoke expected invalid cairn_gate arguments to fail".into()),
        Err(error) => error.to_string(),
    };
    let mutation_denial = mcp_call_tool(
        "cairn_sync_execute",
        &json!({"root": path_string(root.clone()), "change": "demo"}),
        false,
    )?;
    let mutation_receipt = &mutation_denial["structuredContent"];
    let checks = vec![
        json!({"name": "catalog_lists_receipt_tool", "passed": true}),
        json!({"name": "receipt_tool_schema_is_strict", "passed": receipt_schema["additionalProperties"] == false && receipt_schema["anyOf"].is_array()}),
        json!({"name": "receipt_validation_is_structured", "passed": validation["valid"] == true && validation["mcp"]["read_only"] == true && validation["receipt_hash"].is_string()}),
        json!({"name": "invalid_arguments_rejected_before_dispatch", "passed": invalid_argument_error.contains("invalid arguments for cairn_gate")}),
        json!({"name": "default_mutation_denied_without_mutation", "passed": mutation_receipt["allowed"] == false && mutation_receipt["mutated"] == false}),
    ];
    let is_valid = checks
        .iter()
        .all(|check| check["passed"].as_bool().unwrap_or(false));
    Ok(with_receipt_hash(json!({
        "command": "agent-receipt smoke-mcp",
        "root": path_string(root),
        "fixture": fixture,
        "valid": is_valid,
        "verdict": if is_valid { "pass" } else { "fail" },
        "checks": checks,
        "catalog_tool_count": tools.len(),
        "receipt_tool_schema_hash": canonical_json_hash(&receipt_schema),
        "agent_receipt_validation_hash": validation["receipt_hash"].as_str().unwrap_or_default(),
        "agent_receipt_hash": validation["agent_receipt_hash"].as_str().unwrap_or_default(),
        "invalid_argument_error": invalid_argument_error,
        "mutation_denial": {
            "allowed": mutation_receipt["allowed"].clone(),
            "mutated": mutation_receipt["mutated"].clone(),
            "error": mutation_receipt["error"].clone()
        },
        "mcp": {"read_only": true, "destructive": false}
    })))
}
pub(crate) fn mcp_call_tool(
    name: &str,
    input: &Value,
    is_mutation_allowed: bool,
) -> Result<Value, Box<dyn Error>> {
    // r[impl cairn.supersede.mcp_server.strict_schemas]
    // r[impl cairn.supersede.mcp_server.strict_schemas.validation]
    // r[impl cairn.supersede.mcp_server.strict_schemas.errors]
    let receipt = if is_mutating_mcp_tool(name) && !is_mutation_allowed {
        mutation_denied(name)
    } else {
        validate_mcp_tool_input(name, input)?;
        match name {
            "cairn_status" => mcp_status(input)?,
            "cairn_list_content" => mcp_list_content(input)?,
            "cairn_read_spec" => mcp_read_spec(input)?,
            "cairn_change_show" => mcp_change_show(input)?,
            "cairn_doctor" => mcp_doctor(input)?,
            "cairn_validate" => mcp_validate(input)?,
            "cairn_gate" => mcp_gate(input)?,
            "cairn_gate_context" => mcp_gate_context(input)?,
            "cairn_agent_receipt_validate" => mcp_agent_receipt_validate(input)?,
            "cairn_drain_plan" => mcp_drain_plan(input)?,
            "cairn_sync_plan" => mcp_sync(input, false)?,
            "cairn_archive_plan" => mcp_archive(input, false)?,
            "cairn_migrate_plan" => mcp_migrate(input, false)?,
            "cairn_sync_execute" => mcp_sync(input, true)?,
            "cairn_archive_execute" => mcp_archive(input, true)?,
            "cairn_migrate_execute" => mcp_migrate(input, true)?,
            "cairn_init" => mcp_init(input)?,
            "cairn_change_create" => mcp_change_create(input)?,
            unknown => return Err(format!("unknown MCP tool: {unknown}").into()),
        }
    };
    Ok(
        json!({"content": [{"type": "text", "text": mcp_summary(name, &receipt)}], "structuredContent": receipt, "isError": false}),
    )
}
