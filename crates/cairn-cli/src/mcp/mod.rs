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

use cairn_core::verified::mcp_projection::{ResponsePolicy, ResponseProfile};
use cairn_core::{
    archive_dir, build_archive_plan_for_effect_with_dependency_blockers,
    build_cairn_migration_plan, build_change_dependency_graph, build_change_detail,
    build_doctor_report, build_ironspec_migration_plan, build_openspec_migration_plan,
    build_spec_detail, build_sync_plan_for_effect_with_dependency_blockers, canonical_json_hash,
    changes_dir, dependency_blockers_for_change, finalize_gate_report, init_artifacts, layout_name,
    layout_root, lifecycle_substance_issues, specs_dir, validate_changes_with_archives,
    validate_spec_source_with_policy, validate_spec_sources_with_policy, ChangeDependencyGraph,
    ChangeStatus, DoctorSnapshot, GateStage, LifecycleArtifactKind, LifecycleLayout, PlanAction,
    PlanActionKind,
};
use serde_json::{json, Value};

use crate::agent_receipt::{
    agent_completion_validation_receipt, agent_receipt_validation_receipt,
    ensure_inline_receipt_is_bounded, read_agent_receipt_file, AgentCompletionValidationInput,
    AGENT_RECEIPT_MAX_BYTES,
};
use crate::lifecycle::parse_gate_stage;
use crate::migration::{
    collect_layout_migration_inputs, execute_migration_plan, migration_source_layout,
};
use crate::native::{
    change_dependency_edge_json, change_dependency_graph_json, change_dependency_issue_json,
    change_dependency_node_json, change_detail_json, change_status_json, doctor_report_json,
    repo_status_json, spec_detail_json, write_new_file,
};
use crate::plan_receipts::{
    archive_plan_json, gate_input_json, gate_report_json, migration_plan_hash, migration_plan_json,
    sync_plan_json, task_ordering_graph_json,
};
use crate::support::{
    collect_archived_change_ids, collect_validation_spec_sources, default_policy_path,
    detect_layout, execute_archive_plan, execute_sync_snapshots, file_content_manifest,
    find_change, find_spec_source, load_policy_file, load_sync_merge_snapshots,
    mutation_gate_blockers, path_string, portable_json_hash, print_json,
    read_change_dependency_graph, read_gate_snapshot, reject_legacy_only_lifecycle_root, scan_repo,
    spec_substance_inventory_json, substance_inventory_json, sync_changed_paths,
    sync_merge_bindings, sync_merge_blockers, sync_mutation_manifest, validation_findings_json,
    with_portable_receipt_hash, with_receipt_hash,
};

const MCP_PROTOCOL_VERSION: &str = "2024-11-05";
const MCP_MAX_CONTENT_BYTES: usize = 64 * 1024;

mod common;
mod discovery;
mod mutation_tools;
mod read_tools;
mod schema;
mod transport;

use common::mcp_root;
use discovery::{catalog_json, load_catalog};
use mutation_tools::*;
use read_tools::*;
use schema::*;
use transport::*;

pub(crate) fn run_mcp(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.mcp_server.read_tools]
    // r[impl cairn.supersede.mcp_server.plan_tools]
    // r[impl cairn.supersede.mcp_server.mutation_guard]
    // r[impl cairn.supersede.mcp_server.boundary]
    let mut is_mutation_allowed = false;
    let mut describe = false;
    let mut call = None;
    let mut describe_tool = None;
    let mut tool_profile = None;
    let mut paged_tools = false;
    let mut response_profile = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--allow-mutations" => {
                if is_mutation_allowed {
                    return Err("duplicate option: --allow-mutations".into());
                }
                is_mutation_allowed = true;
                index += 1;
            }
            "--describe" => {
                if describe {
                    return Err("duplicate option: --describe".into());
                }
                describe = true;
                index += 1;
            }
            "--tool-profile" | "--describe-tool" => {
                let option = args[index].as_str();
                let value = args
                    .get(index + 1)
                    .filter(|value| !value.is_empty() && !value.starts_with("--"))
                    .ok_or_else(|| format!("{option} requires a value"))?;
                let target = if option == "--tool-profile" {
                    &mut tool_profile
                } else {
                    &mut describe_tool
                };
                if target.replace(value.clone()).is_some() {
                    return Err(format!("duplicate option: {option}").into());
                }
                index += 2;
            }
            "--paged-tools" => {
                if paged_tools {
                    return Err("duplicate option: --paged-tools".into());
                }
                paged_tools = true;
                index += 1;
            }
            "--response-profile" => {
                if response_profile.is_some() {
                    return Err("duplicate option: --response-profile".into());
                }
                let id = args
                    .get(index + 1)
                    .filter(|value| !value.is_empty() && !value.starts_with("--"))
                    .ok_or("mcp --response-profile requires a value")?;
                response_profile = Some(id.as_str());
                index += 2;
            }
            "--call" => {
                if call.is_some() {
                    return Err("duplicate option: --call".into());
                }
                let name = args
                    .get(index + 1)
                    .filter(|value| !value.is_empty() && !value.starts_with("--"))
                    .ok_or("mcp --call requires a tool name")?;
                let input = args.get(index + 2).filter(|value| !value.starts_with("--"));
                call = Some((name.clone(), input.cloned()));
                index += 2 + usize::from(input.is_some());
            }
            unknown => return Err(format!("unknown option: {unknown}").into()),
        }
    }
    if usize::from(describe) + usize::from(call.is_some()) + usize::from(describe_tool.is_some())
        > 1
    {
        return Err("mcp --describe, --describe-tool and --call cannot be combined".into());
    }
    let response_policy = ResponsePolicy::from_json(include_str!(
        "../../../../cairn-policy/generated/mcp-response-profile.json"
    ))?;
    let profile = match response_profile {
        Some(id) => response_policy.admit(id)?,
        None => response_policy.default_profile()?,
    };
    if describe && call.is_some() {
        return Err("mcp --describe and --call cannot be combined".into());
    }
    let catalog = load_catalog(
        tool_profile.as_deref().unwrap_or("full"),
        is_mutation_allowed,
    )?;
    if describe {
        return print_json(catalog_json(&catalog));
    }
    if let Some(name) = describe_tool {
        return print_json(catalog.describe(&name)?.clone());
    }
    if let Some((name, input)) = call {
        let input = input
            .map(|text| serde_json::from_str(&text))
            .transpose()?
            .unwrap_or_else(|| json!({}));
        return print_json(mcp_call_tool_in_catalog(
            &name,
            &input,
            &catalog,
            Some((profile, &response_policy)),
        )?);
    }
    run_mcp_json_rpc(&catalog, paged_tools, profile, &response_policy)
}

pub(crate) fn mcp_tool_catalog(is_allow_mutations: bool) -> Result<Value, Box<dyn Error>> {
    // r[impl cairn.supersede.mcp_server.boundary.core]
    Ok(catalog_json(&load_catalog("full", is_allow_mutations)?))
}

pub(crate) fn mcp_agent_smoke_receipt(root: PathBuf) -> Result<Value, Box<dyn Error>> {
    // r[impl cairn.supersede.mcp_agent_smoke_receipts]
    // r[impl cairn.supersede.mcp_agent_smoke_receipts.flow]
    // r[impl cairn.supersede.mcp_agent_smoke_receipts.evidence]
    // r[impl cairn.supersede.mcp_agent_smoke_receipts.read_only]
    // r[verify cairn.determinism.preserves_content_hash_binding_readiness.mcp_smoke]
    let fixture = "fixtures/agent-completion";
    let contract: Value =
        serde_json::from_slice(&fs::read(root.join(fixture).join("contract.json"))?)?;
    let results: Value =
        serde_json::from_slice(&fs::read(root.join(fixture).join("results.json"))?)?;
    let subject_hash = contract["final_subject_hash"]
        .as_str()
        .ok_or("agent completion fixture missing final_subject_hash")?
        .to_string();
    let root_string = path_string(root.clone());
    let catalog = mcp_tool_catalog(false)?;
    let tools = catalog["tools"]
        .as_array()
        .ok_or("MCP catalog missing tools")?;
    let receipt_tool = tools
        .iter()
        .find(|tool| tool["name"] == "cairn_agent_completion_validate")
        .ok_or("MCP catalog missing cairn_agent_completion_validate")?;
    let receipt_schema = receipt_tool["inputSchema"].clone();
    let receipt_call = mcp_call_tool(
        "cairn_agent_completion_validate",
        &json!({
            "root": root_string,
            "contract": contract,
            "results": results,
            "subject_hash": subject_hash,
        }),
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
        json!({"name": "receipt_tool_schema_is_strict", "passed": receipt_schema["additionalProperties"] == false && receipt_schema["required"].is_array()}),
        json!({"name": "receipt_validation_is_structured", "passed": validation["valid"] == true && validation["mcp"]["read_only"] == true && validation["checks_executed"] == false && validation["completion_receipt_hash"].is_string()}),
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
        "agent_receipt_hash": validation["completion_receipt_hash"].as_str().unwrap_or_default(),
        "invalid_argument_error": invalid_argument_error,
        "mutation_denial": {
            "allowed": mutation_receipt["allowed"].clone(),
            "mutated": mutation_receipt["mutated"].clone(),
            "error": mutation_receipt["error"].clone()
        },
        "mcp": {"read_only": true, "destructive": false}
    })))
}
fn mcp_tool_requires_native_lifecycle(name: &str) -> bool {
    matches!(
        name,
        "cairn_status"
            | "cairn_list_content"
            | "cairn_read_spec"
            | "cairn_change_show"
            | "cairn_context"
            | "cairn_context_read"
            | "cairn_workflow_status"
            | "cairn_workflow_instructions"
            | "cairn_doctor"
            | "cairn_validate"
            | "cairn_gate"
            | "cairn_gate_context"
            | "cairn_change_graph"
            | "cairn_drain_plan"
            | "cairn_implementation_ready"
            | "cairn_verification_impact"
            | "cairn_adoption_report"
            | "cairn_sync_plan"
            | "cairn_archive_plan"
            | "cairn_sync_execute"
            | "cairn_archive_execute"
            | "cairn_init"
            | "cairn_change_create"
    )
}

pub(crate) fn implementation_ready(input: &Value) -> Result<Value, Box<dyn Error>> {
    read_tools::mcp_implementation_ready(input)
}

pub(crate) fn drain_plan(input: &Value, command: &str) -> Result<Value, Box<dyn Error>> {
    read_tools::mcp_drain_plan(input, command)
}

pub(crate) fn mcp_call_tool(
    name: &str,
    input: &Value,
    is_mutation_allowed: bool,
) -> Result<Value, Box<dyn Error>> {
    let catalog = load_catalog("full", is_mutation_allowed)?;
    mcp_call_tool_in_catalog(name, input, &catalog, None)
}

fn mcp_call_tool_in_catalog(
    name: &str,
    input: &Value,
    catalog: &cairn_core::verified::mcp_discovery::ToolCatalog,
    projection: Option<(ResponseProfile, &ResponsePolicy)>,
) -> Result<Value, Box<dyn Error>> {
    let is_mutation_allowed = catalog.allow_mutations;
    // r[impl cairn.supersede.mcp_server.strict_schemas]
    // r[impl cairn.supersede.mcp_server.strict_schemas.validation]
    // r[impl cairn.supersede.mcp_server.strict_schemas.errors]
    let receipt = if is_mutating_mcp_tool(name) && !is_mutation_allowed {
        mutation_denied(name)
    } else {
        // r[impl cairn.mcp_discovery.authority]
        // Capability denial above deliberately precedes profile and argument admission.
        catalog.describe(name)?;
        validate_mcp_tool_input(name, input)?;
        if mcp_tool_requires_native_lifecycle(name) {
            reject_legacy_only_lifecycle_root(&mcp_root(input))?;
        }
        match name {
            "cairn_tool_describe" => catalog
                .describe(input["name"].as_str().ok_or("missing tool name")?)?
                .clone(),
            "cairn_status" => mcp_status(input)?,
            "cairn_list_content" => mcp_list_content(input)?,
            "cairn_read_spec" => mcp_read_spec(input)?,
            "cairn_change_show" => mcp_change_show(input)?,
            "cairn_context" => mcp_context(input)?,
            "cairn_context_read" => read_tools::mcp_context_read(input)?,
            "cairn_workflow_status" => mcp_workflow_status(input)?,
            "cairn_workflow_instructions" => mcp_workflow_instructions(input)?,
            "cairn_doctor" => mcp_doctor(input)?,
            "cairn_validate" => mcp_validate(input)?,
            "cairn_gate" => mcp_gate(input)?,
            "cairn_gate_context" => mcp_gate_context(input)?,
            "cairn_change_graph" => mcp_change_graph(input)?,
            "cairn_agent_receipt_validate" => mcp_agent_receipt_validate(input)?,
            "cairn_agent_completion_validate" => mcp_agent_completion_validate(input)?,
            "cairn_drain_plan" => mcp_drain_plan(input, "mcp cairn_drain_plan")?,
            "cairn_implementation_ready" => mcp_implementation_ready(input)?,
            "cairn_readiness" => crate::readiness::readiness_value(input)?,
            "cairn_selected_work" => crate::selected_work::selected_work_value(input)?,
            "cairn_observe" => crate::observations::mcp_observe(input)?,
            "cairn_verification_impact" => {
                crate::verification_impact::guidance_value_from_mcp(input)?
            }
            "cairn_adoption_report" => crate::adoption::adoption_report_value(input)?,
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
    let summary = match projection {
        Some((ResponseProfile::StructuredSummary, policy)) => policy.summarize(name, &receipt),
        _ => mcp_summary(name, &receipt),
    };
    Ok(
        json!({"content": [{"type": "text", "text": summary}], "structuredContent": receipt, "isError": false}),
    )
}
