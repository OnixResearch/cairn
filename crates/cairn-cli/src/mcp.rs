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
use crate::native::{
    change_detail_json, change_status_json, change_template, doctor_report_json, repo_status_json,
    spec_detail_json, write_new_file,
};
use crate::{
    archive_plan_json, collect_layout_migration_inputs, collect_validation_spec_sources,
    default_policy_path, detect_layout, execute_archive_plan, execute_migration_plan,
    execute_sync_plan, find_change, find_spec_source, gate_input_json, gate_report_json, has_flag,
    load_policy_file, migration_plan_json, migration_source_layout, mutation_gate_blockers,
    path_string, portable_json_hash, print_json, read_gate_snapshot, scan_repo, sync_plan_json,
    with_portable_receipt_hash, with_receipt_hash,
};

const MCP_PROTOCOL_VERSION: &str = "2024-11-05";
const MCP_MAX_CONTENT_BYTES: usize = 64 * 1024;

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

fn run_mcp_json_rpc(is_mutation_allowed: bool) -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let request: Value = serde_json::from_str(&line)?;
        let id = request.get("id").cloned().unwrap_or(Value::Null);
        let method = request["method"].as_str().unwrap_or_default();
        let response = match method {
            "initialize" => json!({
                "jsonrpc": "2.0", "id": id,
                "result": {"protocolVersion": MCP_PROTOCOL_VERSION, "serverInfo": {"name": "cairn", "version": env!("CARGO_PKG_VERSION")}, "capabilities": {"tools": {}}}
            }),
            "tools/list" => {
                json!({"jsonrpc": "2.0", "id": id, "result": {"tools": mcp_tools(is_mutation_allowed)}})
            }
            "tools/call" => {
                let params = &request["params"];
                let name = params["name"].as_str().unwrap_or_default();
                let arguments = params.get("arguments").unwrap_or(&Value::Null);
                match mcp_call_tool(name, arguments, is_mutation_allowed) {
                    Ok(result) => json!({"jsonrpc": "2.0", "id": id, "result": result}),
                    Err(error) => {
                        json!({"jsonrpc": "2.0", "id": id, "error": {"code": -32000, "message": error.to_string()}})
                    }
                }
            }
            _ => {
                json!({"jsonrpc": "2.0", "id": id, "error": {"code": -32601, "message": format!("unknown method: {method}")}})
            }
        };
        writeln!(stdout, "{}", serde_json::to_string(&response)?)?;
        stdout.flush()?;
    }
    Ok(())
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

fn mcp_tools(is_mutation_allowed: bool) -> Vec<Value> {
    let mut tools = mcp_read_and_plan_tools();
    if is_mutation_allowed {
        tools.extend(mcp_mutation_tools());
    }
    tools
}

fn mcp_read_and_plan_tools() -> Vec<Value> {
    let mut tools = mcp_read_tools();
    tools.extend(mcp_plan_tools());
    tools
}

fn mcp_read_tools() -> Vec<Value> {
    vec![
        mcp_tool(
            "cairn_status",
            true,
            false,
            "List Cairn lifecycle state for a bounded root",
        ),
        mcp_tool(
            "cairn_list_content",
            true,
            false,
            "List accepted specs, active changes, and archived changes",
        ),
        mcp_tool(
            "cairn_read_spec",
            true,
            false,
            "Read a bounded accepted spec by id",
        ),
        mcp_tool(
            "cairn_change_show",
            true,
            false,
            "Read bounded active change metadata by id",
        ),
        mcp_tool(
            "cairn_doctor",
            true,
            false,
            "Run Cairn diagnostics and return a structured receipt",
        ),
        mcp_tool(
            "cairn_validate",
            true,
            false,
            "Run Cairn validation and return a structured receipt",
        ),
        mcp_tool(
            "cairn_gate",
            true,
            false,
            "Evaluate a Cairn gate for proposal, design, or tasks",
        ),
        mcp_tool(
            "cairn_gate_context",
            true,
            false,
            "Return bounded gate context for a change",
        ),
        mcp_tool(
            "cairn_agent_receipt_validate",
            true,
            false,
            "Validate an agent-run receipt without mutation",
        ),
    ]
}

fn mcp_plan_tools() -> Vec<Value> {
    vec![
        mcp_tool("cairn_sync_plan", true, false, "Plan sync without mutation"),
        mcp_tool(
            "cairn_archive_plan",
            true,
            false,
            "Plan archive without mutation",
        ),
        mcp_tool(
            "cairn_migrate_plan",
            true,
            false,
            "Plan legacy migration without mutation",
        ),
    ]
}

fn mcp_mutation_tools() -> Vec<Value> {
    vec![
        mcp_tool(
            "cairn_init",
            false,
            true,
            "Initialize Cairn lifecycle files",
        ),
        mcp_tool(
            "cairn_change_create",
            false,
            true,
            "Create a Cairn change skeleton",
        ),
        mcp_tool(
            "cairn_sync_execute",
            false,
            true,
            "Execute a gated sync plan",
        ),
        mcp_tool(
            "cairn_archive_execute",
            false,
            true,
            "Execute a gated archive plan",
        ),
        mcp_tool(
            "cairn_migrate_execute",
            false,
            true,
            "Execute a legacy migration plan",
        ),
    ]
}

fn mcp_tool(name: &str, read_only: bool, destructive: bool, description: &str) -> Value {
    // r[impl cairn.supersede.mcp_server.strict_schemas.catalog]
    json!({"name": name, "description": description, "inputSchema": mcp_input_schema(name), "annotations": {"readOnlyHint": read_only, "destructiveHint": destructive}})
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

fn mutation_denied(name: &str) -> Value {
    json!({"tool": name, "allowed": false, "error": "mutation capability required", "mutated": false})
}

fn is_mutating_mcp_tool(name: &str) -> bool {
    matches!(
        name,
        "cairn_init"
            | "cairn_change_create"
            | "cairn_sync_execute"
            | "cairn_archive_execute"
            | "cairn_migrate_execute"
    )
}

fn mcp_input_schema(name: &str) -> Value {
    match name {
        "cairn_status" | "cairn_list_content" => mcp_object_schema(&[], &["root"]),
        "cairn_doctor" | "cairn_validate" | "cairn_init" => {
            mcp_object_schema(&[], &["root", "policy"])
        }
        "cairn_read_spec" => mcp_object_schema(&["spec"], &["root"]),
        "cairn_change_show" | "cairn_gate_context" | "cairn_change_create" => {
            mcp_object_schema(&["change"], &["root"])
        }
        "cairn_gate" => mcp_schema_with_enum(
            &["change", "stage"],
            &["root", "policy"],
            "stage",
            &["proposal", "design", "tasks"],
        ),
        "cairn_sync_plan"
        | "cairn_archive_plan"
        | "cairn_sync_execute"
        | "cairn_archive_execute" => mcp_object_schema(&["change"], &["root", "policy"]),
        "cairn_migrate_plan" | "cairn_migrate_execute" => mcp_schema_with_enum(
            &["source"],
            &["root", "policy"],
            "source",
            &["openspec", "ironspec"],
        ),
        "cairn_agent_receipt_validate" => json!({
            "type": "object",
            "additionalProperties": false,
            "properties": {
                "root": {"type": "string", "description": "Repository root for path input"},
                "path": {"type": "string", "description": "Repository-relative bounded receipt path"},
                "receipt": {"type": "object", "description": "Inline bounded agent-run receipt"}
            },
            "anyOf": [{"required": ["path"]}, {"required": ["receipt"]}]
        }),
        _ => json!({"type": "object", "additionalProperties": false, "properties": {}}),
    }
}

fn mcp_object_schema(required: &[&str], optional: &[&str]) -> Value {
    let mut properties = serde_json::Map::new();
    for key in required.iter().chain(optional.iter()) {
        properties.insert(
            (*key).to_string(),
            json!({"type": "string", "description": mcp_argument_description(key)}),
        );
    }
    json!({"type": "object", "additionalProperties": false, "properties": properties, "required": required})
}

fn mcp_schema_with_enum(
    required: &[&str],
    optional: &[&str],
    enum_key: &str,
    allowed: &[&str],
) -> Value {
    let mut schema = mcp_object_schema(required, optional);
    schema["properties"][enum_key]["enum"] = json!(allowed);
    schema
}

fn mcp_argument_description(key: &str) -> &'static str {
    match key {
        "root" => "Repository root; defaults to the current directory",
        "policy" => "Policy JSON path; defaults to Cairn's checked-in policy",
        "spec" => "Accepted spec lifecycle identifier",
        "change" => "Active change lifecycle identifier",
        "stage" => "Gate stage: proposal, design, or tasks",
        "source" => "Legacy migration source: openspec or ironspec",
        _ => "String argument",
    }
}

fn validate_mcp_tool_input(name: &str, input: &Value) -> Result<(), Box<dyn Error>> {
    let Some(object) = input.as_object() else {
        return Err(format!("invalid arguments for {name}: expected object").into());
    };
    match name {
        "cairn_status" | "cairn_list_content" => validate_string_args(name, object, &[], &["root"]),
        "cairn_doctor" | "cairn_validate" | "cairn_init" => {
            validate_string_args(name, object, &[], &["root", "policy"])
        }
        "cairn_read_spec" => validate_string_args(name, object, &["spec"], &["root"]),
        "cairn_change_show" | "cairn_gate_context" | "cairn_change_create" => {
            validate_string_args(name, object, &["change"], &["root"])
        }
        "cairn_gate" => {
            validate_string_args(name, object, &["change", "stage"], &["root", "policy"])?;
            validate_enum_arg(name, object, "stage", &["proposal", "design", "tasks"])
        }
        "cairn_sync_plan"
        | "cairn_archive_plan"
        | "cairn_sync_execute"
        | "cairn_archive_execute" => {
            validate_string_args(name, object, &["change"], &["root", "policy"])
        }
        "cairn_migrate_plan" | "cairn_migrate_execute" => {
            validate_string_args(name, object, &["source"], &["root", "policy"])?;
            validate_enum_arg(name, object, "source", &["openspec", "ironspec"])
        }
        "cairn_agent_receipt_validate" => validate_agent_receipt_args(name, object),
        unknown => Err(format!("unknown MCP tool: {unknown}").into()),
    }
}

fn validate_string_args(
    tool: &str,
    object: &serde_json::Map<String, Value>,
    required: &[&str],
    optional: &[&str],
) -> Result<(), Box<dyn Error>> {
    for key in object.keys() {
        if !required.contains(&key.as_str()) && !optional.contains(&key.as_str()) {
            return Err(format!("invalid arguments for {tool}: unknown argument {key}").into());
        }
    }
    for key in required {
        if !object.contains_key(*key) {
            return Err(
                format!("invalid arguments for {tool}: missing required argument {key}").into(),
            );
        }
    }
    for key in required.iter().chain(optional.iter()) {
        if let Some(value) = object.get(*key) {
            if !value.is_string() {
                return Err(format!(
                    "invalid arguments for {tool}: argument {key} must be a string"
                )
                .into());
            }
        }
    }
    Ok(())
}

fn validate_enum_arg(
    tool: &str,
    object: &serde_json::Map<String, Value>,
    key: &str,
    allowed: &[&str],
) -> Result<(), Box<dyn Error>> {
    let value = object.get(key).and_then(Value::as_str).unwrap_or_default();
    if allowed.contains(&value) {
        return Ok(());
    }
    Err(format!(
        "invalid arguments for {tool}: argument {key} must be one of {}",
        allowed.join(", ")
    )
    .into())
}

fn validate_agent_receipt_args(
    tool: &str,
    object: &serde_json::Map<String, Value>,
) -> Result<(), Box<dyn Error>> {
    for key in object.keys() {
        if !matches!(key.as_str(), "root" | "path" | "receipt") {
            return Err(format!("invalid arguments for {tool}: unknown argument {key}").into());
        }
    }
    let has_path = object.contains_key("path");
    let has_receipt = object.contains_key("receipt");
    if has_path == has_receipt {
        return Err(format!(
            "invalid arguments for {tool}: provide exactly one of path or receipt"
        )
        .into());
    }
    if let Some(root) = object.get("root") {
        if !root.is_string() {
            return Err(
                format!("invalid arguments for {tool}: argument root must be a string").into(),
            );
        }
    }
    if let Some(path) = object.get("path") {
        if !path.is_string() {
            return Err(
                format!("invalid arguments for {tool}: argument path must be a string").into(),
            );
        }
    }
    if let Some(receipt) = object.get("receipt") {
        if !receipt.is_object() {
            return Err(format!(
                "invalid arguments for {tool}: argument receipt must be an object"
            )
            .into());
        }
    }
    Ok(())
}

fn mcp_summary(name: &str, receipt: &Value) -> String {
    format!(
        "{name}: {}",
        serde_json::to_string(receipt).unwrap_or_else(|_| "{}".to_string())
    )
}

fn mcp_root(input: &Value) -> PathBuf {
    input["root"]
        .as_str()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn mcp_policy(input: &Value) -> PathBuf {
    input["policy"]
        .as_str()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(default_policy_path()))
}

fn mcp_required_string<'a>(input: &'a Value, key: &str) -> Result<&'a str, Box<dyn Error>> {
    input[key]
        .as_str()
        .ok_or_else(|| format!("missing string argument: {key}").into())
}

fn reject_selector_escape(selector: &str) -> Result<(), Box<dyn Error>> {
    if selector.starts_with('/')
        || selector.contains("..")
        || selector.contains('\\')
        || selector.starts_with('.')
    {
        return Err(format!("selector escapes lifecycle bounds: {selector}").into());
    }
    Ok(())
}

fn mcp_status(input: &Value) -> Result<Value, Box<dyn Error>> {
    let root = mcp_root(input);
    let selected = detect_layout(&root);
    let mut value = repo_status_json(&scan_repo(root.clone(), selected.layout)?, selected.layout);
    value["mcp"] = json!({"read_only": true, "destructive": false});
    Ok(with_portable_receipt_hash(&root, value))
}

fn mcp_list_content(input: &Value) -> Result<Value, Box<dyn Error>> {
    let root = mcp_root(input);
    let selected = detect_layout(&root);
    let status = scan_repo(root.clone(), selected.layout)?;
    let archive_root = root.join(archive_dir(selected.layout));
    let archived_changes = if archive_root.exists() {
        let mut changes = fs::read_dir(archive_root)?
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .filter(|entry| entry.file_type().map(|kind| kind.is_dir()).unwrap_or(false))
            .map(|entry| entry.file_name().to_string_lossy().to_string())
            .collect::<Vec<_>>();
        changes.sort();
        changes
    } else {
        Vec::new()
    };
    Ok(with_receipt_hash(
        json!({"root": path_string(root), "layout": layout_name(selected.layout), "accepted_specs": status.specs, "active_changes": status.changes.iter().map(change_status_json).collect::<Vec<_>>(), "archived_changes": archived_changes, "mcp": {"read_only": true, "destructive": false}}),
    ))
}

fn mcp_read_spec(input: &Value) -> Result<Value, Box<dyn Error>> {
    // r[impl cairn.supersede.mcp_server.read_tools.artifacts]
    let root = mcp_root(input);
    let spec = mcp_required_string(input, "spec")?;
    reject_selector_escape(spec)?;
    let selected = detect_layout(&root);
    let detail = build_spec_detail(find_spec_source(&root, selected.layout, spec)?);
    let mut value = spec_detail_json(&path_string(&root), selected.layout, &detail);
    let markdown = value["spec"]["markdown"].as_str().unwrap_or_default();
    if markdown.len() > MCP_MAX_CONTENT_BYTES {
        return Err("spec content exceeds MCP bounded read limit".into());
    }
    value["mcp"] = json!({"read_only": true, "destructive": false, "max_content_bytes": MCP_MAX_CONTENT_BYTES});
    Ok(with_portable_receipt_hash(&root, value))
}

fn mcp_change_show(input: &Value) -> Result<Value, Box<dyn Error>> {
    let root = mcp_root(input);
    let change = mcp_required_string(input, "change")?;
    reject_selector_escape(change)?;
    let selected = detect_layout(&root);
    let detail = build_change_detail(find_change(root.clone(), selected.layout, change)?);
    let mut value = change_detail_json(&path_string(&root), selected.layout, &detail);
    value["mcp"] = json!({"read_only": true, "destructive": false});
    Ok(with_portable_receipt_hash(&root, value))
}

fn mcp_doctor(input: &Value) -> Result<Value, Box<dyn Error>> {
    let root = mcp_root(input);
    let policy_path = mcp_policy(input);
    let selected = detect_layout(&root);
    let status = scan_repo(root.clone(), selected.layout)?;
    let spec_issues =
        validate_spec_sources(&collect_validation_spec_sources(&root, selected.layout)?);
    let policy_result = load_policy_file(policy_path.clone());
    let (policy_name, policy_error, change_issue_count) = match policy_result {
        Ok(policy) => (
            Some(policy.name.clone()),
            None,
            validate_changes(&status.changes, &policy).len(),
        ),
        Err(error) => (None, Some(error.to_string()), 0),
    };
    let report = build_doctor_report(DoctorSnapshot {
        root: path_string(&root),
        selected_layout: selected.layout,
        native_layout_present: root.join(layout_root(LifecycleLayout::Native)).exists(),
        legacy_layout_present: root
            .join(layout_root(LifecycleLayout::LegacyOpenSpec))
            .exists(),
        specs_dir_present: root.join(specs_dir(selected.layout)).exists(),
        changes_dir_present: root.join(changes_dir(selected.layout)).exists(),
        archive_dir_present: root.join(archive_dir(selected.layout)).exists(),
        policy_path: path_string(policy_path),
        policy_name,
        policy_error,
        spec_count: status.specs.len(),
        change_count: status.changes.len(),
        validation_issue_count: spec_issues.len() + change_issue_count,
    });
    let mut value = doctor_report_json(&report);
    value["mcp"] = json!({"read_only": true, "destructive": false});
    Ok(with_portable_receipt_hash(&root, value))
}

fn mcp_validate(input: &Value) -> Result<Value, Box<dyn Error>> {
    let root = mcp_root(input);
    let policy = load_policy_file(mcp_policy(input))?;
    let selected = detect_layout(&root);
    let status = scan_repo(root.clone(), selected.layout)?;
    let change_issues = validate_changes(&status.changes, &policy);
    let spec_sources = collect_validation_spec_sources(&root, selected.layout)?;
    let spec_issues = validate_spec_sources(&spec_sources);
    let mut issues = Vec::with_capacity(change_issues.len() + spec_issues.len());
    issues.extend(change_issues.iter().cloned());
    issues.extend(spec_issues.iter().cloned());
    Ok(with_receipt_hash(
        json!({"valid": issues.is_empty(), "issues": issues, "change_issues": change_issues, "spec_issues": spec_issues, "changes": status.changes.len(), "specs_validated": spec_sources.len(), "policy": policy.name, "layout": layout_name(selected.layout), "mcp": {"read_only": true, "destructive": false}}),
    ))
}

fn mcp_gate(input: &Value) -> Result<Value, Box<dyn Error>> {
    let root = mcp_root(input);
    let change = mcp_required_string(input, "change")?;
    reject_selector_escape(change)?;
    let stage = parse_gate_stage(mcp_required_string(input, "stage")?)?;
    let policy = load_policy_file(mcp_policy(input))?;
    let selected = detect_layout(&root);
    let snapshot = read_gate_snapshot(
        &root,
        selected.layout,
        find_change(root.clone(), selected.layout, change)?,
    )?;
    let report = evaluate_gate_snapshot(&snapshot, stage, &policy);
    let input_hash = portable_json_hash(
        &root,
        &gate_input_json(selected.layout, stage, &policy, &snapshot),
    );
    let mut value = gate_report_json(selected.layout, &policy, &report, &input_hash);
    value["mcp"] = json!({"read_only": true, "destructive": false});
    Ok(with_portable_receipt_hash(&root, value))
}

fn mcp_gate_context(input: &Value) -> Result<Value, Box<dyn Error>> {
    let root = mcp_root(input);
    let change = mcp_required_string(input, "change")?;
    reject_selector_escape(change)?;
    let selected = detect_layout(&root);
    let mut value = change_status_json(&find_change(root.clone(), selected.layout, change)?);
    value["mcp"] = json!({"read_only": true, "destructive": false});
    Ok(with_portable_receipt_hash(&root, value))
}

fn mcp_agent_receipt_validate(input: &Value) -> Result<Value, Box<dyn Error>> {
    // r[impl cairn.supersede.agent_receipt_validation_surface.mcp]
    // r[impl cairn.supersede.agent_receipt_validation_surface.mcp.read_only]
    let root = mcp_root(input);
    let (path, receipt) = if input.get("receipt").is_some() {
        let receipt = input["receipt"].clone();
        ensure_inline_receipt_is_bounded(&receipt)?;
        (None, receipt)
    } else {
        let selector = mcp_required_string(input, "path")?;
        reject_selector_escape(selector)?;
        let path = root.join(selector);
        let receipt = read_agent_receipt_file(&path)?;
        (Some(path), receipt)
    };
    let policy = load_policy_file(mcp_policy(input))?;
    let mut value = agent_receipt_validation_receipt(
        "mcp cairn_agent_receipt_validate",
        path,
        &receipt,
        true,
        &policy,
    );
    value["mcp"] = json!({"read_only": true, "destructive": false, "max_content_bytes": AGENT_RECEIPT_MAX_BYTES});
    Ok(with_portable_receipt_hash(&root, value))
}

fn mcp_sync(input: &Value, execute: bool) -> Result<Value, Box<dyn Error>> {
    // r[impl cairn.supersede.mcp_server.plan_tools.lifecycle]
    // r[impl cairn.supersede.mcp_server.mutation_guard.enabled]
    let root = mcp_root(input);
    let change = mcp_required_string(input, "change")?;
    reject_selector_escape(change)?;
    let policy = load_policy_file(mcp_policy(input))?;
    let selected = detect_layout(&root);
    let change_status = find_change(root.clone(), selected.layout, change)?;
    let input_hash = portable_json_hash(&root, &change_status_json(&change_status));
    let mut plan = build_sync_plan(&change_status);
    if policy.sync_require_gate && !plan.blocked {
        plan.reasons.extend(mutation_gate_blockers(
            &root,
            selected.layout,
            &change_status,
            GateStage::Tasks,
            &policy,
        )?);
        plan.reasons.sort();
        plan.reasons.dedup();
        plan.blocked = !plan.reasons.is_empty();
        if plan.blocked {
            plan.actions.clear();
        }
    }
    if execute {
        plan.dry_run = false;
        if !plan.blocked {
            execute_sync_plan(&root, selected.layout, change, &plan)?;
            plan.mutated = true;
        }
    }
    let mut value = sync_plan_json(
        &root,
        selected.layout,
        &policy,
        &plan,
        &input_hash,
        Value::Null,
    );
    value["mcp"] = json!({"read_only": !execute, "destructive": execute});
    Ok(with_portable_receipt_hash(&root, value))
}

fn mcp_archive(input: &Value, execute: bool) -> Result<Value, Box<dyn Error>> {
    let root = mcp_root(input);
    let change = mcp_required_string(input, "change")?;
    reject_selector_escape(change)?;
    let policy = load_policy_file(mcp_policy(input))?;
    let selected = detect_layout(&root);
    let change_status = find_change(root.clone(), selected.layout, change)?;
    let input_hash = portable_json_hash(&root, &change_status_json(&change_status));
    let mut plan = build_archive_plan(&change_status);
    if policy.archive_require_gate && !plan.blocked {
        plan.reasons.extend(mutation_gate_blockers(
            &root,
            selected.layout,
            &change_status,
            GateStage::Tasks,
            &policy,
        )?);
        plan.reasons.sort();
        plan.reasons.dedup();
        plan.blocked = !plan.reasons.is_empty();
        if plan.blocked {
            plan.actions.clear();
        }
    }
    if execute {
        plan.dry_run = false;
        if !plan.blocked {
            let archive_path =
                execute_archive_plan(&root, selected.layout, change, &change_status.path)?;
            plan.actions = vec![PlanAction {
                kind: PlanActionKind::ArchiveChange,
                path: path_string(archive_path),
                description: format!("move active change to archive: {change}"),
            }];
            plan.mutated = true;
        }
    }
    let mut value = archive_plan_json(
        &root,
        selected.layout,
        &policy,
        &plan,
        &input_hash,
        Value::Null,
    );
    value["mcp"] = json!({"read_only": !execute, "destructive": execute});
    Ok(with_portable_receipt_hash(&root, value))
}

fn mcp_migrate(input: &Value, execute: bool) -> Result<Value, Box<dyn Error>> {
    // r[impl cairn.supersede.mcp_server.plan_tools.migration]
    let root = mcp_root(input);
    let source = mcp_required_string(input, "source")?;
    let policy = load_policy_file(mcp_policy(input))?;
    let source_layout = migration_source_layout(source)?;
    let inputs = collect_layout_migration_inputs(&root, source_layout)?;
    let mut plan = match source_layout {
        LifecycleLayout::LegacyOpenSpec => build_openspec_migration_plan(inputs, execute),
        LifecycleLayout::LegacyIronspec => build_ironspec_migration_plan(inputs, execute),
        LifecycleLayout::Native => return Err("cannot migrate from canonical cairn layout".into()),
    };
    if execute && !plan.blocked {
        execute_migration_plan(&root, &plan)?;
        plan.mutated = !plan.actions.is_empty();
    }
    let mut value = migration_plan_json(&root, &policy, &plan, Value::Null);
    value["mcp"] = json!({"read_only": !execute, "destructive": execute});
    Ok(with_portable_receipt_hash(&root, value))
}

fn mcp_init(input: &Value) -> Result<Value, Box<dyn Error>> {
    let root = mcp_root(input);
    let artifacts = init_artifacts();
    let mut created = Vec::with_capacity(artifacts.len());
    let mut existing = Vec::with_capacity(artifacts.len());
    for artifact in artifacts {
        let path = root.join(&artifact.path);
        if path.exists() {
            existing.push(artifact.path);
        } else {
            match artifact.kind {
                LifecycleArtifactKind::Directory => fs::create_dir_all(&path)?,
                LifecycleArtifactKind::File => write_new_file(&path, "")?,
            }
            created.push(artifact.path);
        }
    }
    Ok(with_receipt_hash(
        json!({"command": "init", "root": path_string(root), "mutated": !created.is_empty(), "layout": layout_name(LifecycleLayout::Native), "created": created, "existing": existing, "mcp": {"read_only": false, "destructive": true}}),
    ))
}

fn mcp_change_create(input: &Value) -> Result<Value, Box<dyn Error>> {
    let root = mcp_root(input);
    let change = mcp_required_string(input, "change")?;
    reject_selector_escape(change)?;
    let skeleton = build_change_skeleton(change)?;
    let mut created = Vec::with_capacity(skeleton.artifacts.len());
    let mut existing = Vec::with_capacity(skeleton.artifacts.len());
    for artifact in skeleton.artifacts {
        let path = root.join(&artifact.path);
        if path.exists() {
            existing.push(artifact.path);
            continue;
        }
        match artifact.kind {
            LifecycleArtifactKind::Directory => fs::create_dir_all(&path)?,
            LifecycleArtifactKind::File => {
                write_new_file(&path, &change_template(change, &artifact.path))?
            }
        }
        created.push(artifact.path);
    }
    Ok(with_receipt_hash(
        json!({"command": "change_create", "root": path_string(root), "change": change, "mutated": !created.is_empty(), "layout": layout_name(LifecycleLayout::Native), "created": created, "existing": existing, "mcp": {"read_only": false, "destructive": true}}),
    ))
}
