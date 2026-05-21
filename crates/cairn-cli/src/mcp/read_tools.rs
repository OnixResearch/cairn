use super::common::*;
use super::*;

pub(super) fn mcp_status(input: &Value) -> Result<Value, Box<dyn Error>> {
    let root = mcp_root(input);
    let selected = detect_layout(&root);
    let mut value = repo_status_json(&scan_repo(root.clone(), selected.layout)?, selected.layout);
    value["mcp"] = json!({"read_only": true, "destructive": false});
    Ok(with_portable_receipt_hash(&root, value))
}

pub(super) fn mcp_list_content(input: &Value) -> Result<Value, Box<dyn Error>> {
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

pub(super) fn mcp_read_spec(input: &Value) -> Result<Value, Box<dyn Error>> {
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

pub(super) fn mcp_change_show(input: &Value) -> Result<Value, Box<dyn Error>> {
    let root = mcp_root(input);
    let change = mcp_required_string(input, "change")?;
    reject_selector_escape(change)?;
    let selected = detect_layout(&root);
    let detail = build_change_detail(find_change(root.clone(), selected.layout, change)?);
    let mut value = change_detail_json(&path_string(&root), selected.layout, &detail);
    value["mcp"] = json!({"read_only": true, "destructive": false});
    Ok(with_portable_receipt_hash(&root, value))
}

pub(super) fn mcp_doctor(input: &Value) -> Result<Value, Box<dyn Error>> {
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

pub(super) fn mcp_validate(input: &Value) -> Result<Value, Box<dyn Error>> {
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

pub(super) fn mcp_gate(input: &Value) -> Result<Value, Box<dyn Error>> {
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

pub(super) fn mcp_gate_context(input: &Value) -> Result<Value, Box<dyn Error>> {
    let root = mcp_root(input);
    let change = mcp_required_string(input, "change")?;
    reject_selector_escape(change)?;
    let selected = detect_layout(&root);
    let mut value = change_status_json(&find_change(root.clone(), selected.layout, change)?);
    value["mcp"] = json!({"read_only": true, "destructive": false});
    Ok(with_portable_receipt_hash(&root, value))
}

pub(super) fn mcp_agent_receipt_validate(input: &Value) -> Result<Value, Box<dyn Error>> {
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
