use super::common::*;
use super::*;

pub(super) fn mcp_sync(input: &Value, execute: bool) -> Result<Value, Box<dyn Error>> {
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

pub(super) fn mcp_archive(input: &Value, execute: bool) -> Result<Value, Box<dyn Error>> {
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

pub(super) fn mcp_migrate(input: &Value, execute: bool) -> Result<Value, Box<dyn Error>> {
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

pub(super) fn mcp_init(input: &Value) -> Result<Value, Box<dyn Error>> {
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

pub(super) fn mcp_change_create(input: &Value) -> Result<Value, Box<dyn Error>> {
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
