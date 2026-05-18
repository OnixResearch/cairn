use std::{
    env,
    error::Error,
    fs,
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
};

use cairn_core::{
    archive_dir, blake3_hex, build_archive_plan, build_change_detail, build_change_list,
    build_change_skeleton, build_doctor_report, build_ironspec_migration_plan,
    build_openspec_migration_plan, build_repo_status, build_spec_detail, build_spec_list,
    build_sync_plan, canonical_json_hash, changes_dir, design_template,
    evaluate_agent_run_completion, evaluate_gate_snapshot, init_artifacts, init_artifacts_for,
    layout_name, layout_root, merge_delta_spec, parse_policy_json, proposal_template, specs_dir,
    tasks_template, validate_changes, validate_spec_sources, AgentRunGateIssue, ArchivePlan,
    ChangeDetail, ChangeList, ChangeSource, ChangeStatus, DoctorCheckStatus, DoctorReport,
    DoctorSnapshot, GateArtifact, GateReport, GateSnapshot, GateStage, LifecycleArtifactKind,
    LifecycleLayout, MigrationInput, MigrationPlan, PlanAction, PlanActionKind, Policy, RepoStatus,
    SpecDetail, SpecList, SpecSource, SpecSummary, SyncPlan, TaskCounts,
};
use serde_json::{json, Value};

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let invoked_as = env::args().next().unwrap_or_default();
    if invoked_as.ends_with("ironspec") {
        eprintln!("warning: `ironspec` is a compatibility alias; use canonical `cairn`");
    }
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        print_usage();
        return Ok(());
    };

    match command.as_str() {
        "--help" | "-h" | "help" => print_usage(),
        "init" => run_init(args.collect())?,
        "change" => run_change(args.collect())?,
        "spec" => run_spec(args.collect())?,
        "status" => run_status(args.collect())?,
        "doctor" => run_doctor(args.collect())?,
        "validate" => run_validate(args.collect())?,
        "agent-receipt" => run_agent_receipt(args.collect())?,
        "policy" => run_policy(args.collect())?,
        "migrate" => run_migrate(args.collect())?,
        "mcp" => run_mcp(args.collect())?,
        "gate" => run_gate(args.collect())?,
        "gate-context" => {
            let Some(change) = args.next() else {
                return Err("gate-context requires a change name".into());
            };
            run_change_context(args.collect(), &change)?;
        }
        "sync" => {
            let Some(change) = args.next() else {
                return Err("sync requires a change name".into());
            };
            run_sync(args.collect(), &change)?;
        }
        "archive" => {
            let Some(change) = args.next() else {
                return Err("archive requires a change name".into());
            };
            run_archive(args.collect(), &change)?;
        }
        unknown => return Err(format!("unknown command: {unknown}\n\n{}", usage_text()).into()),
    }
    Ok(())
}

fn run_status(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args, "--root", ".");
    let selected = detect_layout(&root);
    let status = scan_repo(root, selected.layout)?;
    print_json(repo_status_json(&status, selected.layout))
}

fn run_doctor(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args.clone(), "--root", ".");
    let policy_path = read_option(args, "--policy", default_policy_path());
    let selected = detect_layout(&root);
    let status = scan_repo(root.clone(), selected.layout)?;
    let spec_sources = collect_validation_spec_sources(&root, selected.layout)?;
    let spec_issues = validate_spec_sources(&spec_sources);
    let policy_result = load_policy_file(policy_path.clone());
    let (policy_name, policy_error, change_issue_count) = match policy_result {
        Ok(policy) => {
            let change_issues = validate_changes(&status.changes, &policy);
            (Some(policy.name), None, change_issues.len())
        }
        Err(error) => (None, Some(error.to_string()), 0),
    };
    let snapshot = DoctorSnapshot {
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
    };
    let report = build_doctor_report(snapshot);
    print_json(doctor_report_json(&report))
}

fn run_change(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("change requires a subcommand: create, list, or show".into());
    };
    match subcommand.as_str() {
        "create" => {
            let Some(change) = iter.next() else {
                return Err("change create requires a change name".into());
            };
            run_change_create(iter.collect(), &change)
        }
        "list" => run_change_list(iter.collect()),
        "show" => {
            let Some(change) = iter.next() else {
                return Err("change show requires a change name".into());
            };
            run_change_show(iter.collect(), &change)
        }
        unknown => Err(format!("unknown change subcommand: {unknown}").into()),
    }
}

fn run_spec(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("spec requires a subcommand: list or show".into());
    };
    match subcommand.as_str() {
        "list" => run_spec_list(iter.collect()),
        "show" => {
            let Some(spec) = iter.next() else {
                return Err("spec show requires a spec id".into());
            };
            run_spec_show(iter.collect(), &spec)
        }
        unknown => Err(format!("unknown spec subcommand: {unknown}").into()),
    }
}

fn run_change_list(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args, "--root", ".");
    let selected = detect_layout(&root);
    let status = scan_repo(root.clone(), selected.layout)?;
    let list = build_change_list(status.changes);
    print_json(change_list_json(&path_string(root), selected.layout, &list))
}

fn run_change_show(args: Vec<String>, change: &str) -> Result<(), Box<dyn Error>> {
    let root = read_option(args, "--root", ".");
    let selected = detect_layout(&root);
    let change_status = find_change(root.clone(), selected.layout, change)?;
    let detail = build_change_detail(change_status);
    print_json(change_detail_json(
        &path_string(root),
        selected.layout,
        &detail,
    ))
}

fn run_spec_list(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args, "--root", ".");
    let selected = detect_layout(&root);
    let specs = collect_spec_sources(&root.join(specs_dir(selected.layout)))?;
    let list = build_spec_list(specs);
    print_json(spec_list_json(&path_string(root), selected.layout, &list))
}

fn run_spec_show(args: Vec<String>, spec: &str) -> Result<(), Box<dyn Error>> {
    let root = read_option(args, "--root", ".");
    let selected = detect_layout(&root);
    let source = find_spec_source(&root, selected.layout, spec)?;
    let detail = build_spec_detail(source);
    print_json(spec_detail_json(
        &path_string(root),
        selected.layout,
        &detail,
    ))
}

fn run_validate(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args.clone(), "--root", ".");
    let policy_path = read_option(args, "--policy", default_policy_path());
    let policy = load_policy_file(policy_path)?;
    let selected = detect_layout(&root);
    let status = scan_repo(root.clone(), selected.layout)?;
    let change_issues = validate_changes(&status.changes, &policy);
    let spec_sources = collect_validation_spec_sources(&root, selected.layout)?;
    let spec_issues = validate_spec_sources(&spec_sources);
    let mut issues = Vec::with_capacity(change_issues.len() + spec_issues.len());
    issues.extend(change_issues.iter().cloned());
    issues.extend(spec_issues.iter().cloned());
    print_json(json!({
        "valid": issues.is_empty(),
        "issues": issues,
        "change_issues": change_issues,
        "spec_issues": spec_issues,
        "changes": status.changes.len(),
        "specs_validated": spec_sources.len(),
        "policy": policy.name,
        "layout": layout_name(selected.layout),
    }))
}

fn run_policy(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let policy_path = read_option(args, "--policy", default_policy_path());
    let policy = load_policy_file(policy_path)?;
    print_json(policy.raw)
}

fn run_gate(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(stage_text) = iter.next() else {
        return Err("gate requires a stage: proposal, design, or tasks".into());
    };
    let stage = parse_gate_stage(&stage_text)?;
    let Some(change) = iter.next() else {
        return Err("gate requires a change name".into());
    };
    let rest = iter.collect::<Vec<_>>();
    let root = read_option(rest.clone(), "--root", ".");
    let policy_path = read_option(rest, "--policy", default_policy_path());
    let policy = load_policy_file(policy_path)?;
    let selected = detect_layout(&root);
    let change_status = find_change(root.clone(), selected.layout, &change)?;
    let snapshot = read_gate_snapshot(&root, selected.layout, change_status)?;
    let report = evaluate_gate_snapshot(&snapshot, stage, &policy);
    let input_hash =
        canonical_json_hash(&gate_input_json(selected.layout, stage, &policy, &snapshot));
    print_json(with_receipt_hash(gate_report_json(
        selected.layout,
        &policy,
        &report,
        &input_hash,
    )))
}

fn parse_gate_stage(stage: &str) -> Result<GateStage, Box<dyn Error>> {
    match stage {
        "proposal" => Ok(GateStage::Proposal),
        "design" => Ok(GateStage::Design),
        "tasks" => Ok(GateStage::Tasks),
        unknown => Err(format!("unknown gate stage: {unknown}").into()),
    }
}

fn run_change_context(args: Vec<String>, change: &str) -> Result<(), Box<dyn Error>> {
    let root = read_option(args, "--root", ".");
    let selected = detect_layout(&root);
    let change_status = find_change(root, selected.layout, change)?;
    print_json(change_status_json(&change_status))
}

fn run_sync(args: Vec<String>, change: &str) -> Result<(), Box<dyn Error>> {
    let root = read_option(args.clone(), "--root", ".");
    let is_execute = has_flag(&args, "--execute");
    let policy_path = read_option(args.clone(), "--policy", default_policy_path());
    let policy = load_policy_file(policy_path)?;
    let selected = detect_layout(&root);
    let change_status = find_change(root.clone(), selected.layout, change)?;
    let input_hash = canonical_json_hash(&change_status_json(&change_status));
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
    if is_execute {
        plan.dry_run = false;
        if !plan.blocked {
            execute_sync_plan(&root, selected.layout, change, &plan)?;
            plan.mutated = true;
        }
    }
    print_json(with_receipt_hash(sync_plan_json(
        selected.layout,
        &policy,
        &plan,
        &input_hash,
    )))
}

fn run_archive(args: Vec<String>, change: &str) -> Result<(), Box<dyn Error>> {
    let root = read_option(args.clone(), "--root", ".");
    let is_execute = has_flag(&args, "--execute");
    let policy_path = read_option(args.clone(), "--policy", default_policy_path());
    let policy = load_policy_file(policy_path)?;
    let selected = detect_layout(&root);
    let change_status = find_change(root.clone(), selected.layout, change)?;
    let input_hash = canonical_json_hash(&change_status_json(&change_status));
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
    if is_execute {
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
    print_json(with_receipt_hash(archive_plan_json(
        selected.layout,
        &policy,
        &plan,
        &input_hash,
    )))
}

fn run_init(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args, "--root", ".");
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
    print_json(json!({
        "command": "init",
        "root": path_string(root),
        "mutated": !created.is_empty(),
        "layout": layout_name(LifecycleLayout::Native),
        "created": created,
        "existing": existing,
    }))
}

fn run_change_create(args: Vec<String>, change: &str) -> Result<(), Box<dyn Error>> {
    let root = read_option(args, "--root", ".");
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
    print_json(json!({
        "command": "change_create",
        "root": path_string(root),
        "change": change,
        "mutated": !created.is_empty(),
        "layout": layout_name(LifecycleLayout::Native),
        "created": created,
        "existing": existing,
    }))
}

fn change_template(change: &str, artifact_path: &str) -> String {
    if artifact_path.ends_with("proposal.md") {
        return proposal_template(change);
    }
    if artifact_path.ends_with("design.md") {
        return design_template(change);
    }
    if artifact_path.ends_with("tasks.md") {
        return tasks_template();
    }
    String::new()
}

fn write_new_file(path: &Path, content: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}

fn run_migrate(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(source) = iter.next() else {
        return Err("migrate requires a source: openspec or ironspec".into());
    };
    let rest = iter.collect::<Vec<_>>();
    let root = read_option(rest.clone(), "--root", ".");
    let is_execute = has_flag(&rest, "--execute");
    let policy_path = read_option(rest.clone(), "--policy", default_policy_path());
    let policy = load_policy_file(policy_path)?;
    let source_layout = migration_source_layout(&source)?;
    let inputs = collect_layout_migration_inputs(&root, source_layout)?;
    let mut plan = match source_layout {
        LifecycleLayout::LegacyOpenSpec => build_openspec_migration_plan(inputs, is_execute),
        LifecycleLayout::LegacyIronspec => build_ironspec_migration_plan(inputs, is_execute),
        LifecycleLayout::Native => return Err("cannot migrate from canonical cairn layout".into()),
    };
    if is_execute && !plan.blocked {
        execute_migration_plan(&root, &plan)?;
        plan.mutated = !plan.actions.is_empty();
    }
    print_json(with_receipt_hash(migration_plan_json(
        &root, &policy, &plan,
    )))
}

const MCP_PROTOCOL_VERSION: &str = "2024-11-05";
const MCP_MAX_CONTENT_BYTES: usize = 64 * 1024;
const AGENT_RECEIPT_MAX_BYTES: u64 = 64 * 1024;

fn run_agent_receipt(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("agent-receipt requires a subcommand: validate".into());
    };
    match subcommand.as_str() {
        "validate" => {
            let Some(path) = iter.next() else {
                return Err("agent-receipt validate requires a JSON receipt path".into());
            };
            let path = PathBuf::from(path);
            let receipt = read_agent_receipt_file(&path)?;
            print_json(agent_receipt_validation_receipt(
                "agent-receipt validate",
                Some(path),
                &receipt,
                false,
            ))
        }
        unknown => Err(format!("unknown agent-receipt subcommand: {unknown}").into()),
    }
}

fn run_mcp(args: Vec<String>) -> Result<(), Box<dyn Error>> {
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

fn mcp_tool_catalog(is_allow_mutations: bool) -> Value {
    // r[impl cairn.supersede.mcp_server.boundary.core]
    json!({"protocol_version": MCP_PROTOCOL_VERSION, "allow_mutations": is_allow_mutations, "tools": mcp_tools(is_allow_mutations)})
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
    json!({"name": name, "description": description, "inputSchema": {"type": "object", "additionalProperties": true}, "annotations": {"readOnlyHint": read_only, "destructiveHint": destructive}})
}

fn mcp_call_tool(
    name: &str,
    input: &Value,
    is_mutation_allowed: bool,
) -> Result<Value, Box<dyn Error>> {
    let receipt = match name {
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
        "cairn_init"
        | "cairn_change_create"
        | "cairn_sync_execute"
        | "cairn_archive_execute"
        | "cairn_migrate_execute"
            if !is_mutation_allowed =>
        {
            mutation_denied(name)
        }
        "cairn_sync_execute" => mcp_sync(input, true)?,
        "cairn_archive_execute" => mcp_archive(input, true)?,
        "cairn_migrate_execute" => mcp_migrate(input, true)?,
        "cairn_init" => mcp_init(input)?,
        "cairn_change_create" => mcp_change_create(input)?,
        unknown => return Err(format!("unknown MCP tool: {unknown}").into()),
    };
    Ok(
        json!({"content": [{"type": "text", "text": mcp_summary(name, &receipt)}], "structuredContent": receipt, "isError": false}),
    )
}

fn mutation_denied(name: &str) -> Value {
    json!({"tool": name, "allowed": false, "error": "mutation capability required", "mutated": false})
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
    let mut value = repo_status_json(&scan_repo(root, selected.layout)?, selected.layout);
    value["mcp"] = json!({"read_only": true, "destructive": false});
    Ok(with_receipt_hash(value))
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
    let mut value = spec_detail_json(&path_string(root), selected.layout, &detail);
    let markdown = value["spec"]["markdown"].as_str().unwrap_or_default();
    if markdown.len() > MCP_MAX_CONTENT_BYTES {
        return Err("spec content exceeds MCP bounded read limit".into());
    }
    value["mcp"] = json!({"read_only": true, "destructive": false, "max_content_bytes": MCP_MAX_CONTENT_BYTES});
    Ok(with_receipt_hash(value))
}

fn mcp_change_show(input: &Value) -> Result<Value, Box<dyn Error>> {
    let root = mcp_root(input);
    let change = mcp_required_string(input, "change")?;
    reject_selector_escape(change)?;
    let selected = detect_layout(&root);
    let detail = build_change_detail(find_change(root.clone(), selected.layout, change)?);
    let mut value = change_detail_json(&path_string(root), selected.layout, &detail);
    value["mcp"] = json!({"read_only": true, "destructive": false});
    Ok(with_receipt_hash(value))
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
    Ok(with_receipt_hash(value))
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
    let input_hash =
        canonical_json_hash(&gate_input_json(selected.layout, stage, &policy, &snapshot));
    let mut value = gate_report_json(selected.layout, &policy, &report, &input_hash);
    value["mcp"] = json!({"read_only": true, "destructive": false});
    Ok(with_receipt_hash(value))
}

fn mcp_gate_context(input: &Value) -> Result<Value, Box<dyn Error>> {
    let root = mcp_root(input);
    let change = mcp_required_string(input, "change")?;
    reject_selector_escape(change)?;
    let selected = detect_layout(&root);
    let mut value = change_status_json(&find_change(root, selected.layout, change)?);
    value["mcp"] = json!({"read_only": true, "destructive": false});
    Ok(with_receipt_hash(value))
}

fn mcp_agent_receipt_validate(input: &Value) -> Result<Value, Box<dyn Error>> {
    // r[impl cairn.supersede.agent_receipt_validation_surface.mcp]
    // r[impl cairn.supersede.agent_receipt_validation_surface.mcp.read_only]
    let (path, receipt) = if input.get("receipt").is_some() {
        let receipt = input["receipt"].clone();
        ensure_inline_receipt_is_bounded(&receipt)?;
        (None, receipt)
    } else {
        let root = mcp_root(input);
        let selector = mcp_required_string(input, "path")?;
        reject_selector_escape(selector)?;
        let path = root.join(selector);
        let receipt = read_agent_receipt_file(&path)?;
        (Some(path), receipt)
    };
    let mut value =
        agent_receipt_validation_receipt("mcp cairn_agent_receipt_validate", path, &receipt, true);
    value["mcp"] = json!({"read_only": true, "destructive": false, "max_content_bytes": AGENT_RECEIPT_MAX_BYTES});
    Ok(with_receipt_hash(value))
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
    let input_hash = canonical_json_hash(&change_status_json(&change_status));
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
    let mut value = sync_plan_json(selected.layout, &policy, &plan, &input_hash);
    value["mcp"] = json!({"read_only": !execute, "destructive": execute});
    Ok(with_receipt_hash(value))
}

fn mcp_archive(input: &Value, execute: bool) -> Result<Value, Box<dyn Error>> {
    let root = mcp_root(input);
    let change = mcp_required_string(input, "change")?;
    reject_selector_escape(change)?;
    let policy = load_policy_file(mcp_policy(input))?;
    let selected = detect_layout(&root);
    let change_status = find_change(root.clone(), selected.layout, change)?;
    let input_hash = canonical_json_hash(&change_status_json(&change_status));
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
    let mut value = archive_plan_json(selected.layout, &policy, &plan, &input_hash);
    value["mcp"] = json!({"read_only": !execute, "destructive": execute});
    Ok(with_receipt_hash(value))
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
    let mut value = migration_plan_json(&root, &policy, &plan);
    value["mcp"] = json!({"read_only": !execute, "destructive": execute});
    Ok(with_receipt_hash(value))
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

#[derive(Debug, Clone, Copy)]
struct SelectedLayout {
    layout: LifecycleLayout,
}

fn detect_layout(_root: &Path) -> SelectedLayout {
    SelectedLayout {
        layout: LifecycleLayout::Native,
    }
}

fn load_policy_file(path: PathBuf) -> Result<Policy, Box<dyn Error>> {
    let text = fs::read_to_string(&path)
        .map_err(|source| format!("failed to read policy {}: {source}", path.display()))?;
    parse_policy_json(&text)
        .map_err(|source| format!("failed to parse policy {}: {source}", path.display()).into())
}

fn scan_repo(root: PathBuf, layout: LifecycleLayout) -> std::io::Result<RepoStatus> {
    let specs = collect_spec_files(&root.join(specs_dir(layout)))?;
    let changes = collect_change_sources(&root.join(changes_dir(layout)))?;
    Ok(build_repo_status(path_string(root), specs, changes))
}

fn collect_change_sources(changes_dir: &Path) -> std::io::Result<Vec<ChangeSource>> {
    if !changes_dir.exists() {
        return Ok(Vec::new());
    }
    let entries = fs::read_dir(changes_dir)?.collect::<Result<Vec<_>, _>>()?;
    let mut changes = Vec::with_capacity(entries.len());
    for entry in entries {
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if is_hidden_or_archive_change(&name) {
            continue;
        }
        changes.push(read_change_source(name, entry.path())?);
    }
    Ok(changes)
}

fn read_change_source(name: String, path: PathBuf) -> std::io::Result<ChangeSource> {
    let tasks_path = path.join("tasks.md");
    let tasks_markdown = if tasks_path.exists() {
        Some(fs::read_to_string(&tasks_path)?)
    } else {
        None
    };
    Ok(ChangeSource {
        name,
        path: path_string(&path),
        has_proposal: path.join("proposal.md").exists(),
        has_design: path.join("design.md").exists(),
        has_tasks: tasks_path.exists(),
        delta_specs: collect_spec_files(&path.join("specs"))?,
        tasks_markdown,
    })
}

fn collect_spec_files(dir: &Path) -> std::io::Result<Vec<String>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut specs = Vec::new();
    let mut dirs = Vec::with_capacity(1);
    dirs.push(dir.to_path_buf());
    while let Some(current_dir) = dirs.pop() {
        let entries = fs::read_dir(&current_dir)?.collect::<Result<Vec<_>, _>>()?;
        specs.reserve(entries.len());
        for entry in entries {
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                dirs.push(path);
            } else if path.file_name().is_some_and(|name| name == "spec.md") {
                specs.push(path_string(path));
            }
        }
    }
    specs.sort();
    Ok(specs)
}

fn collect_spec_sources(specs_dir: &Path) -> std::io::Result<Vec<SpecSource>> {
    let spec_paths = collect_spec_files(specs_dir)?;
    let mut specs = Vec::with_capacity(spec_paths.len());
    for spec_path in spec_paths {
        let path = PathBuf::from(&spec_path);
        specs.push(SpecSource {
            id: spec_id(specs_dir, &path),
            path: spec_path,
            markdown: fs::read_to_string(path)?,
        });
    }
    Ok(specs)
}

fn find_spec_source(
    root: &Path,
    layout: LifecycleLayout,
    spec: &str,
) -> Result<SpecSource, Box<dyn Error>> {
    let specs_dir = root.join(specs_dir(layout));
    let Some(source) = collect_spec_sources(&specs_dir)?
        .into_iter()
        .find(|candidate| candidate.id == spec)
    else {
        return Err(format!("spec not found: {spec}").into());
    };
    Ok(source)
}

fn collect_validation_spec_sources(
    root: &Path,
    layout: LifecycleLayout,
) -> std::io::Result<Vec<SpecSource>> {
    let mut sources = collect_spec_sources(&root.join(specs_dir(layout)))?;
    sources.extend(collect_delta_spec_sources(root, layout)?);
    sources.sort_by(|left, right| left.path.cmp(&right.path).then(left.id.cmp(&right.id)));
    Ok(sources)
}

fn collect_delta_spec_sources(
    root: &Path,
    layout: LifecycleLayout,
) -> std::io::Result<Vec<SpecSource>> {
    let changes_dir = root.join(changes_dir(layout));
    if !changes_dir.exists() {
        return Ok(Vec::new());
    }
    let entries = fs::read_dir(&changes_dir)?.collect::<Result<Vec<_>, _>>()?;
    let mut sources = Vec::with_capacity(entries.len());
    for entry in entries {
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let change = entry.file_name().to_string_lossy().to_string();
        if is_hidden_or_archive_change(&change) {
            continue;
        }
        let specs_dir = entry.path().join("specs");
        let mut delta_sources = collect_spec_sources(&specs_dir)?;
        for source in &mut delta_sources {
            source.id = format!("change/{change}/{}", source.id);
        }
        sources.reserve(delta_sources.len());
        sources.extend(delta_sources);
    }
    Ok(sources)
}

fn spec_id(specs_dir: &Path, path: &Path) -> String {
    path.strip_prefix(specs_dir)
        .ok()
        .and_then(|relative| relative.parent())
        .map(path_string)
        .unwrap_or_else(|| path_string(path))
}

fn is_hidden_or_archive_change(name: &str) -> bool {
    name == "archive" || name == "_done" || name.starts_with('.')
}

fn find_change(
    root: PathBuf,
    layout: LifecycleLayout,
    change: &str,
) -> Result<ChangeStatus, Box<dyn Error>> {
    let status = scan_repo(root, layout)?;
    let Some(change_status) = status
        .changes
        .into_iter()
        .find(|candidate| candidate.name == change)
    else {
        return Err(format!("change not found: {change}").into());
    };
    Ok(change_status)
}

fn read_gate_snapshot(
    root: &Path,
    layout: LifecycleLayout,
    change: ChangeStatus,
) -> Result<GateSnapshot, Box<dyn Error>> {
    // r[impl cairn.supersede.gate_policy.references]
    // r[impl cairn.supersede.gate_policy.task_links]
    // r[impl cairn.supersede.gate_policy.exemptions]
    let change_dir = root.join(changes_dir(layout)).join(&change.name);
    let proposal = read_gate_artifact(change_dir.join("proposal.md"))?;
    let design = read_gate_artifact(change_dir.join("design.md"))?;
    let tasks = read_gate_artifact(change_dir.join("tasks.md"))?;
    let mut delta_specs = Vec::with_capacity(change.delta_specs.len());
    for path in &change.delta_specs {
        if let Some(artifact) = read_gate_artifact(PathBuf::from(path))? {
            delta_specs.push(artifact);
        }
    }
    Ok(GateSnapshot {
        change,
        proposal,
        design,
        tasks,
        delta_specs,
    })
}

fn read_gate_artifact(path: PathBuf) -> Result<Option<GateArtifact>, Box<dyn Error>> {
    if !path.exists() {
        return Ok(None);
    }
    Ok(Some(GateArtifact {
        path: path_string(&path),
        markdown: fs::read_to_string(path)?,
    }))
}

fn mutation_gate_blockers(
    // r[impl cairn.supersede.deterministic_gates] r[impl cairn.supersede.deterministic_gates.pre_sync]
    // r[impl cairn.supersede.deterministic_gates.pre_archive] r[impl cairn.supersede.deterministic_gates.stage]
    // r[impl cairn.supersede.deterministic_gates.traceability]
    root: &Path,
    layout: LifecycleLayout,
    change_status: &ChangeStatus,
    stage: GateStage,
    policy: &Policy,
) -> Result<Vec<String>, Box<dyn Error>> {
    let snapshot = read_gate_snapshot(root, layout, change_status.clone())?;
    let report = evaluate_gate_snapshot(&snapshot, stage, policy);
    let mut blockers = report
        .issues
        .into_iter()
        .map(|issue| format!("gate {}: {}", stage.as_str(), issue.message))
        .collect::<Vec<_>>();
    let mut validation_sources = collect_spec_sources(&root.join(specs_dir(layout)))?;
    for source in collect_delta_spec_sources(root, layout)? {
        if source
            .path
            .contains(&format!("/changes/{}/", change_status.name))
        {
            validation_sources.push(source);
        }
    }
    blockers.extend(
        validate_spec_sources(&validation_sources)
            .into_iter()
            .map(|issue| format!("validation: {issue}")),
    );
    blockers.sort();
    blockers.dedup();
    Ok(blockers)
}

fn gate_input_json(
    layout: LifecycleLayout,
    stage: GateStage,
    policy: &Policy,
    snapshot: &GateSnapshot,
) -> Value {
    json!({
        "layout": layout_name(layout),
        "stage": stage.as_str(),
        "policy_hash": canonical_json_hash(&policy.raw),
        "change": change_status_json(&snapshot.change),
        "proposal": snapshot.proposal.as_ref().map(gate_artifact_json),
        "design": snapshot.design.as_ref().map(gate_artifact_json),
        "tasks": snapshot.tasks.as_ref().map(gate_artifact_json),
        "delta_specs": snapshot.delta_specs.iter().map(gate_artifact_json).collect::<Vec<_>>(),
    })
}

fn gate_artifact_json(artifact: &GateArtifact) -> Value {
    json!({
        "path": artifact.path,
        "markdown_hash": canonical_json_hash(&json!({"markdown": artifact.markdown})),
    })
}

fn execute_sync_plan(
    root: &Path,
    layout: LifecycleLayout,
    change: &str,
    plan: &SyncPlan,
) -> Result<(), Box<dyn Error>> {
    let delta_root = root.join(changes_dir(layout)).join(change).join("specs");
    let specs_root = root.join(specs_dir(layout));
    for action in &plan.actions {
        let source = PathBuf::from(&action.path);
        let relative = source.strip_prefix(&delta_root).map_err(|_| {
            format!(
                "delta spec {} is outside expected change specs root {}",
                source.display(),
                delta_root.display()
            )
        })?;
        let destination = specs_root.join(relative);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        let delta_markdown = fs::read_to_string(&source).map_err(|source_error| {
            format!(
                "failed to read delta spec {}: {source_error}",
                source.display()
            )
        })?;
        let accepted_markdown = if destination.exists() {
            Some(fs::read_to_string(&destination).map_err(|source_error| {
                format!(
                    "failed to read accepted spec {}: {source_error}",
                    destination.display()
                )
            })?)
        } else {
            None
        };
        let spec_id = relative
            .parent()
            .map(path_string)
            .unwrap_or_else(|| path_string(relative));
        let merged = merge_delta_spec(accepted_markdown.as_deref(), &delta_markdown, &spec_id);
        fs::write(&destination, merged.markdown).map_err(|source_error| {
            format!(
                "failed to sync {} to {}: {source_error}",
                source.display(),
                destination.display()
            )
        })?;
    }
    Ok(())
}

fn execute_archive_plan(
    root: &Path,
    layout: LifecycleLayout,
    change: &str,
    change_path: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let source = PathBuf::from(change_path);
    let destination = root
        .join(archive_dir(layout))
        .join(format!("{}-{change}", archive_date()));
    if destination.exists() {
        return Err(format!(
            "archive destination already exists: {}",
            destination.display()
        )
        .into());
    }
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::rename(&source, &destination).map_err(|source_error| {
        format!(
            "failed to archive {} to {}: {source_error}",
            source.display(),
            destination.display()
        )
    })?;
    Ok(destination)
}

fn collect_layout_migration_inputs(
    root: &Path,
    source_layout: LifecycleLayout,
) -> std::io::Result<Vec<MigrationInput>> {
    let source_root = root.join(layout_root(source_layout));
    if !source_root.exists() {
        return Ok(Vec::new());
    }
    let source_files = collect_files(&source_root)?;
    let mut inputs = Vec::with_capacity(source_files.len());
    for source in source_files {
        let relative = source.strip_prefix(&source_root).unwrap_or(&source);
        let destination = root.join(native_migration_destination(source_layout, relative));
        let source_hash = blake3_hex(&fs::read(&source)?);
        let destination_hash = if destination.exists() {
            Some(blake3_hex(&fs::read(&destination)?))
        } else {
            None
        };
        inputs.push(MigrationInput {
            source: path_string(&source),
            source_hash,
            destination: path_string(&destination),
            destination_hash,
            destination_exists: destination.exists(),
        });
    }
    Ok(inputs)
}

fn native_migration_destination(source_layout: LifecycleLayout, relative: &Path) -> PathBuf {
    let relative_text = path_string(relative);
    if source_layout == LifecycleLayout::LegacyOpenSpec {
        if let Some(rest) = relative_text.strip_prefix("changes/archive/") {
            return PathBuf::from(layout_root(LifecycleLayout::Native))
                .join("archive")
                .join(rest);
        }
    }
    PathBuf::from(layout_root(LifecycleLayout::Native)).join(relative)
}

fn migration_source_layout(source: &str) -> Result<LifecycleLayout, Box<dyn Error>> {
    match source {
        "openspec" => Ok(LifecycleLayout::LegacyOpenSpec),
        "ironspec" => Ok(LifecycleLayout::LegacyIronspec),
        _ => Err(format!("unsupported migration source: {source}").into()),
    }
}

fn collect_files(root: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut dirs = vec![root.to_path_buf()];
    while let Some(current_dir) = dirs.pop() {
        let entries = fs::read_dir(&current_dir)?.collect::<Result<Vec<_>, _>>()?;
        files.reserve(entries.len());
        dirs.reserve(entries.len());
        for entry in entries {
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                dirs.push(path);
            } else {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

fn execute_migration_plan(root: &Path, plan: &MigrationPlan) -> Result<(), Box<dyn Error>> {
    for artifact in init_artifacts_for(LifecycleLayout::Native) {
        let path = root.join(artifact.path);
        if matches!(artifact.kind, LifecycleArtifactKind::Directory) {
            fs::create_dir_all(path)?;
        }
    }
    for action in &plan.actions {
        let source = PathBuf::from(&action.source);
        let destination = PathBuf::from(&action.destination);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&source, &destination).map_err(|source_error| {
            format!(
                "failed to migrate {} to {}: {source_error}",
                source.display(),
                destination.display()
            )
        })?;
    }
    Ok(())
}

fn archive_date() -> String {
    env::var("CAIRN_ARCHIVE_DATE")
        .or_else(|_| env::var("IRONSPEC_ARCHIVE_DATE"))
        .unwrap_or_else(|_| "1970-01-01".to_string())
}

fn repo_status_json(status: &RepoStatus, layout: LifecycleLayout) -> Value {
    json!({
        "root": status.root,
        "layout": layout_name(layout),
        "specs": status.specs,
        "changes": status.changes.iter().map(change_status_json).collect::<Vec<_>>(),
    })
}

fn doctor_report_json(report: &DoctorReport) -> Value {
    json!({
        "valid": report.valid,
        "root": report.root,
        "layout": report.layout,
        "spec_count": report.spec_count,
        "change_count": report.change_count,
        "validation_issue_count": report.validation_issue_count,
        "checks": report.checks.iter().map(|check| {
            json!({
                "name": check.name,
                "status": doctor_check_status(check.status),
                "message": check.message,
            })
        }).collect::<Vec<_>>(),
    })
}

fn doctor_check_status(status: DoctorCheckStatus) -> &'static str {
    match status {
        DoctorCheckStatus::Ok => "ok",
        DoctorCheckStatus::Warn => "warn",
        DoctorCheckStatus::Fail => "fail",
    }
}

fn change_list_json(root: &str, layout: LifecycleLayout, list: &ChangeList) -> Value {
    json!({
        "root": root,
        "layout": layout_name(layout),
        "changes": list.changes.iter().map(change_status_json).collect::<Vec<_>>(),
    })
}

fn change_detail_json(root: &str, layout: LifecycleLayout, detail: &ChangeDetail) -> Value {
    json!({
        "root": root,
        "layout": layout_name(layout),
        "change": change_status_json(&detail.change),
    })
}

fn spec_list_json(root: &str, layout: LifecycleLayout, list: &SpecList) -> Value {
    json!({
        "root": root,
        "layout": layout_name(layout),
        "specs": list.specs.iter().map(spec_summary_json).collect::<Vec<_>>(),
    })
}

fn spec_detail_json(root: &str, layout: LifecycleLayout, detail: &SpecDetail) -> Value {
    json!({
        "root": root,
        "layout": layout_name(layout),
        "spec": {
            "id": detail.id,
            "path": detail.path,
            "title": detail.title,
            "markdown": detail.markdown,
        }
    })
}

fn spec_summary_json(summary: &SpecSummary) -> Value {
    json!({
        "id": summary.id,
        "path": summary.path,
        "title": summary.title,
    })
}

fn change_status_json(change: &ChangeStatus) -> Value {
    json!({
        "name": change.name,
        "path": change.path,
        "has_proposal": change.has_proposal,
        "has_design": change.has_design,
        "has_tasks": change.has_tasks,
        "delta_specs": change.delta_specs,
        "task_counts": change.task_counts.map(task_counts_json),
    })
}

fn task_counts_json(counts: TaskCounts) -> Value {
    json!({
        "total": counts.total,
        "done": counts.done,
        "in_progress": counts.in_progress,
        "todo": counts.todo,
        "unmarked": counts.unmarked,
    })
}

fn migration_plan_json(root: &Path, policy: &Policy, plan: &MigrationPlan) -> Value {
    let mut value = json!({
        "command": format!("migrate_{}", layout_name(plan.source_layout)),
        "root": path_string(root),
        "source_layout": layout_name(plan.source_layout),
        "destination_layout": layout_name(plan.destination_layout),
        "dry_run": plan.dry_run,
        "blocked": plan.blocked,
        "mutated": plan.mutated,
        "actions": plan.actions.iter().map(|action| {
            json!({
                "source": action.source,
                "source_hash": action.source_hash,
                "destination": action.destination,
                "destination_hash": action.destination_hash,
            })
        }).collect::<Vec<_>>(),
        "conflicts": plan.conflicts,
        "policy": policy.name,
        "policy_hash": canonical_json_hash(&policy.raw),
    });
    value["source_manifest_hash"] = json!(canonical_json_hash(&json!({
        "source_layout": layout_name(plan.source_layout),
        "sources": plan.actions.iter().map(|action| json!({
            "source": action.source,
            "source_hash": action.source_hash,
        })).collect::<Vec<_>>(),
    })));
    value["destination_manifest_hash"] = json!(canonical_json_hash(&json!({
        "destination_layout": layout_name(plan.destination_layout),
        "destinations": plan.actions.iter().map(|action| json!({
            "destination": action.destination,
            "destination_hash": action.destination_hash,
        })).collect::<Vec<_>>(),
    })));
    value["manifest_hash"] = json!(canonical_json_hash(&value));
    value
}

fn sync_plan_json(
    layout: LifecycleLayout,
    policy: &Policy,
    plan: &SyncPlan,
    input_hash: &str,
) -> Value {
    let mut value = json!({
        "change": plan.change,
        "layout": layout_name(layout),
        "dry_run": plan.dry_run,
        "mutated": plan.mutated,
        "actions": plan.actions.iter().map(plan_action_json).collect::<Vec<_>>(),
        "delta_specs": plan.delta_specs,
        "blocked": plan.blocked,
        "reasons": plan.reasons,
        "policy": policy.name,
        "policy_hash": canonical_json_hash(&policy.raw),
        "input_hash": input_hash,
    });
    let plan_hash = canonical_json_hash(&value);
    value["plan_hash"] = json!(plan_hash);
    value
}

fn archive_plan_json(
    layout: LifecycleLayout,
    policy: &Policy,
    plan: &ArchivePlan,
    input_hash: &str,
) -> Value {
    let mut value = json!({
        "change": plan.change,
        "layout": layout_name(layout),
        "dry_run": plan.dry_run,
        "mutated": plan.mutated,
        "actions": plan.actions.iter().map(plan_action_json).collect::<Vec<_>>(),
        "blocked": plan.blocked,
        "reasons": plan.reasons,
        "policy": policy.name,
        "policy_hash": canonical_json_hash(&policy.raw),
        "input_hash": input_hash,
    });
    let plan_hash = canonical_json_hash(&value);
    value["plan_hash"] = json!(plan_hash);
    value
}

fn gate_report_json(
    layout: LifecycleLayout,
    policy: &Policy,
    report: &GateReport,
    input_hash: &str,
) -> Value {
    json!({
        "change": report.change,
        "stage": report.stage,
        "verdict": report.verdict,
        "valid": report.issues.is_empty(),
        "issues": report.issues.iter().map(|issue| {
            json!({
                "message": issue.message,
                "scope": issue.scope,
                "promoted": issue.promoted,
            })
        }).collect::<Vec<_>>(),
        "policy": policy.name,
        "policy_hash": canonical_json_hash(&policy.raw),
        "input_hash": input_hash,
        "layout": layout_name(layout),
    })
}

fn plan_action_json(action: &PlanAction) -> Value {
    json!({
        "kind": plan_action_kind(action.kind),
        "path": action.path,
        "description": action.description,
    })
}

fn plan_action_kind(kind: PlanActionKind) -> &'static str {
    match kind {
        PlanActionKind::SyncDeltaSpec => "sync_delta_spec",
        PlanActionKind::ArchiveChange => "archive_change",
    }
}

fn read_agent_receipt_file(path: &Path) -> Result<Value, Box<dyn Error>> {
    // r[impl cairn.supersede.agent_receipt_validation_surface.cli]
    let metadata = fs::metadata(path)?;
    if metadata.len() > AGENT_RECEIPT_MAX_BYTES {
        return Err(format!(
            "agent receipt exceeds bounded read limit: {} bytes > {}",
            metadata.len(),
            AGENT_RECEIPT_MAX_BYTES
        )
        .into());
    }
    let bytes = fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn ensure_inline_receipt_is_bounded(receipt: &Value) -> Result<(), Box<dyn Error>> {
    let bytes = serde_json::to_vec(receipt)?;
    if bytes.len() as u64 > AGENT_RECEIPT_MAX_BYTES {
        return Err(format!(
            "inline agent receipt exceeds bounded read limit: {} bytes > {}",
            bytes.len(),
            AGENT_RECEIPT_MAX_BYTES
        )
        .into());
    }
    Ok(())
}

fn agent_receipt_validation_receipt(
    command: &str,
    path: Option<PathBuf>,
    receipt: &Value,
    is_mcp: bool,
) -> Value {
    // r[impl cairn.supersede.agent_receipt_validation_surface.cli]
    // r[impl cairn.supersede.agent_receipt_validation_surface.cli.schema]
    let report = evaluate_agent_run_completion(receipt);
    let input_kind = if is_mcp && path.is_none() {
        "inline"
    } else {
        "path"
    };
    with_receipt_hash(json!({
        "command": command,
        "path": path.map(path_string),
        "input_kind": input_kind,
        "valid": report.valid,
        "verdict": report.verdict,
        "issues": report.issues.iter().map(agent_gate_issue_json).collect::<Vec<_>>(),
        "agent_receipt_hash": report.receipt_hash,
        "input_hash": canonical_json_hash(receipt),
        "stated_agent_receipt_hash": receipt.get("receipt_hash").and_then(Value::as_str).unwrap_or_default(),
        "max_content_bytes": AGENT_RECEIPT_MAX_BYTES,
    }))
}

fn agent_gate_issue_json(issue: &AgentRunGateIssue) -> Value {
    json!({"code": issue.code, "message": issue.message})
}

fn path_string(path: impl Into<PathBuf>) -> String {
    path.into().display().to_string()
}

fn read_option(args: Vec<String>, name: &str, default: &str) -> PathBuf {
    args.windows(2)
        .find_map(|pair| (pair[0] == name).then(|| PathBuf::from(&pair[1])))
        .unwrap_or_else(|| PathBuf::from(default))
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|arg| arg == name)
}

fn print_json(value: Value) -> Result<(), Box<dyn Error>> {
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

fn with_receipt_hash(mut value: Value) -> Value {
    let receipt_hash = canonical_json_hash(&value);
    value["receipt_hash"] = json!(receipt_hash);
    value
}

fn default_policy_path() -> &'static str {
    "cairn-policy/generated/cairn-policy.json"
}

fn usage_text() -> &'static str {
    // r[impl cairn.supersede.hermes_skill] r[impl cairn.supersede.hermes_skill.discover]
    // r[impl cairn.supersede.hermes_skill.contracts] r[impl cairn.supersede.hermes_skill.audit]
    "usage: cairn <command> [args]\n\ncommands:\n  init [--root PATH]\n  change create <name> [--root PATH]\n  change list [--root PATH]\n  change show <name> [--root PATH]\n  spec list [--root PATH]\n  spec show <id> [--root PATH]\n  status [--root PATH]\n  doctor [--root PATH] [--policy PATH]\n  validate [--root PATH] [--policy PATH]\n  agent-receipt validate <path>\n  policy [--policy PATH]\n  migrate <openspec|ironspec> [--root PATH] [--policy PATH] [--execute]\n  mcp [--describe] [--call TOOL JSON] [--allow-mutations]\n  gate <proposal|design|tasks> <change> [--root PATH] [--policy PATH]\n  gate-context <change> [--root PATH]\n  sync <change> [--root PATH] [--policy PATH] [--execute]\n  archive <change> [--root PATH] [--policy PATH] [--execute]"
}

fn print_usage() {
    println!("{}", usage_text());
}
