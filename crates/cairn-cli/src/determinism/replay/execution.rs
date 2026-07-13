use cairn_core::receipt_field_drift;
use serde_json::Value;

use super::support::*;
use super::types::*;

pub(super) fn replay_case_source_root(
    root: &Path,
    case: &ReplayCase,
) -> Result<PathBuf, Box<dyn Error>> {
    if case.fixture_root == "<root>" || case.fixture_root == "." {
        return Ok(root.to_path_buf());
    }
    let candidate = root.join(&case.fixture_root);
    if candidate.exists() {
        return Ok(candidate);
    }
    let candidate = PathBuf::from(&case.fixture_root);
    if candidate.exists() {
        return Ok(candidate);
    }
    Err(format!(
        "replay case {} fixture root does not exist: {}",
        case.id, case.fixture_root
    )
    .into())
}

pub(super) fn replay_case_root(
    source: &Path,
    case: &ReplayCase,
    label: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    if case.fixture_root == "." {
        return Ok(source.to_path_buf());
    }
    replay_temp_root(source, &format!("{}-{label}", case.id))
}

pub(super) fn replay_manifest_for_case(
    root: &Path,
    case: &ReplayCase,
) -> Result<Value, Box<dyn Error>> {
    if case.fixture_root == "." {
        return Ok(json!({"manifest_hash": "not-collected-policy-root", "paths": []}));
    }
    Ok(file_content_manifest(root, &collect_files(root)?))
}

pub(super) fn run_replay_case(case: &ReplayCase, root: &Path) -> Result<Value, Box<dyn Error>> {
    match &case.kind {
        ReplayCaseKind::AgentReceiptPreservesHash => {
            // r[impl cairn.determinism.preserves_hash_cli.replay_execution]
            let Some(path) = case.args.get(2) else {
                return Err("agent receipt Preserves hash replay case missing fixture path".into());
            };
            let receipt_path = replay_fixture_path(root, path);
            let receipt = read_agent_receipt_file(&receipt_path)?;
            let mut value = agent_preserves_hash_receipt(receipt_path, &receipt);
            value
                .as_object_mut()
                .map(|object| object.remove("receipt_hash"));
            Ok(with_portable_receipt_hash(root, value))
        }
        ReplayCaseKind::AgentReceiptValidate => {
            let Some(path) = case.args.get(2) else {
                return Err("agent receipt replay case missing fixture path".into());
            };
            let receipt_path = replay_fixture_path(root, path);
            let receipt = read_agent_receipt_file(&receipt_path)?;
            let policy = load_policy_file(default_policy_path().into())?;
            let mut value = agent_receipt_validation_receipt(
                "agent-receipt validate",
                Some(receipt_path),
                &receipt,
                false,
                &policy,
            );
            value
                .as_object_mut()
                .map(|object| object.remove("receipt_hash"));
            Ok(with_portable_receipt_hash(root, value))
        }
        ReplayCaseKind::ArchiveDryRun => {
            let selected = detect_layout(root);
            let change = replay_change_arg(case, "archive")?;
            let policy = load_policy_file(default_policy_path().into())?;
            let change_status = find_change(root.to_path_buf(), selected.layout, change)?;
            let input_hash = portable_json_hash(root, &change_status_json(&change_status));
            let mut plan = build_archive_plan(&change_status);
            if policy.archive_require_gate && !plan.blocked {
                plan.reasons.extend(mutation_gate_blockers(
                    root,
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
            Ok(with_portable_receipt_hash(
                root,
                archive_plan_json(
                    root,
                    selected.layout,
                    &policy,
                    &plan,
                    &input_hash,
                    Value::Null,
                ),
            ))
        }
        ReplayCaseKind::PreservesCorpusFreshness => {
            // r[impl cairn.determinism.preserves_corpus_policy_inventory.replay_case]
            let Some(path) = case.args.get(3) else {
                return Err("Preserves corpus freshness replay case missing corpus path".into());
            };
            let corpus_path = replay_fixture_path(root, path);
            let mut value = preserves_corpus_freshness_receipt(corpus_path);
            value
                .as_object_mut()
                .map(|object| object.remove("receipt_hash"));
            Ok(with_portable_receipt_hash(root, value))
        }
        ReplayCaseKind::Gate { stage, change } => {
            let policy = load_policy_file(default_policy_path().into())?;
            let selected = detect_layout(root);
            let change_status = find_change(root.to_path_buf(), selected.layout, change)?;
            let snapshot = read_gate_snapshot(root, selected.layout, change_status)?;
            let report = evaluate_gate_snapshot(&snapshot, *stage, &policy);
            let input_hash = portable_json_hash(
                root,
                &gate_input_json(selected.layout, *stage, &policy, &snapshot),
            );
            Ok(with_portable_receipt_hash(
                root,
                gate_report_json(selected.layout, &policy, &report, &input_hash),
            ))
        }
        ReplayCaseKind::MigrateDryRun => {
            let policy = load_policy_file(default_policy_path().into())?;
            let source_layout = LifecycleLayout::LegacyOpenSpec;
            let inputs = collect_layout_migration_inputs(root, source_layout)?;
            let plan = build_openspec_migration_plan(inputs, false);
            Ok(with_portable_receipt_hash(
                root,
                migration_plan_json(root, &policy, &plan, Value::Null),
            ))
        }
        ReplayCaseKind::McpDescribe => Ok(mcp_tool_catalog(false)),
        ReplayCaseKind::McpStatus => {
            mcp_call_tool("cairn_status", &json!({"root": path_string(root)}), false)
        }
        ReplayCaseKind::ReceiptPreservesDiff => {
            // r[impl cairn.determinism.preserves_receipt_field_drift.validation]
            // args: ["receipt-preserves-diff", "--baseline", "<path>", "--candidate", "<path>"]
            let Some(baseline_flag) = case.args.get(1) else {
                return Err("receipt-preserves-diff replay case missing --baseline flag".into());
            };
            if baseline_flag != "--baseline" {
                return Err(format!(
                    "receipt-preserves-diff replay case expected --baseline, got {baseline_flag}"
                )
                .into());
            }
            let Some(baseline_path) = case.args.get(2) else {
                return Err("receipt-preserves-diff replay case missing baseline path".into());
            };
            let Some(candidate_flag) = case.args.get(3) else {
                return Err("receipt-preserves-diff replay case missing --candidate flag".into());
            };
            if candidate_flag != "--candidate" {
                return Err(format!(
                    "receipt-preserves-diff replay case expected --candidate, got {candidate_flag}"
                )
                .into());
            }
            let Some(candidate_path) = case.args.get(4) else {
                return Err("receipt-preserves-diff replay case missing candidate path".into());
            };
            let baseline_full = replay_fixture_path(root, baseline_path);
            let candidate_full = replay_fixture_path(root, candidate_path);
            let baseline_text = fs::read_to_string(&baseline_full)?;
            let candidate_text = fs::read_to_string(&candidate_full)?;
            let baseline_val: Value =
                serde_json::from_str(&baseline_text).map_err(|e| format!("baseline parse: {e}"))?;
            let candidate_val: Value = serde_json::from_str(&candidate_text)
                .map_err(|e| format!("candidate parse: {e}"))?;
            let policy = load_policy_file(default_policy_path().into())?;
            let source_map = crate::receipt_preserves_diff::parse_source_id_map(&policy);
            let report = receipt_field_drift(&baseline_val, &candidate_val, &source_map);
            let mut value = crate::receipt_preserves_diff::diff_receipt_json(
                &baseline_full,
                &candidate_full,
                &report,
                &policy,
            );
            value
                .as_object_mut()
                .map(|object| object.remove("receipt_hash"));
            Ok(crate::support::with_portable_receipt_hash(root, value))
        }
        ReplayCaseKind::ReleaseReadiness => Ok(release_readiness_receipt(
            root.to_path_buf(),
            default_policy_path().into(),
        )),
        ReplayCaseKind::Status => {
            let selected = detect_layout(root);
            let status = scan_repo(root.to_path_buf(), selected.layout)?;
            Ok(repo_status_json(&status, selected.layout))
        }
        ReplayCaseKind::SyncDryRun => {
            let selected = detect_layout(root);
            let change = replay_change_arg(case, "sync")?;
            let policy = load_policy_file(default_policy_path().into())?;
            let change_status = find_change(root.to_path_buf(), selected.layout, change)?;
            let input_hash = portable_json_hash(root, &change_status_json(&change_status));
            let mut plan = build_sync_plan(&change_status);
            if policy.sync_require_gate && !plan.blocked {
                plan.reasons.extend(mutation_gate_blockers(
                    root,
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
            Ok(with_portable_receipt_hash(
                root,
                sync_plan_json(
                    root,
                    selected.layout,
                    &policy,
                    &plan,
                    &input_hash,
                    Value::Null,
                ),
            ))
        }
        ReplayCaseKind::Validate => {
            cairn_validation_receipt(root.to_path_buf(), default_policy_path().into())
        }
        ReplayCaseKind::View { view } => {
            // r[impl cairn.lifecycle_views.determinism]
            crate::view::lifecycle_view_receipt_for_replay(root, view)
        }
    }
}

fn replay_change_arg<'a>(case: &'a ReplayCase, command: &str) -> Result<&'a str, Box<dyn Error>> {
    case.args
        .get(1)
        .map(String::as_str)
        .ok_or_else(|| format!("{command} replay case missing change argument").into())
}

fn replay_fixture_path(root: &Path, path: &str) -> PathBuf {
    let rooted = root.join(path);
    if rooted.exists() {
        rooted
    } else {
        PathBuf::from(path)
    }
}

pub(super) fn replay_output_mutated(value: &Value) -> bool {
    value
        .get("mutated")
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

pub(super) fn replay_reproduce_command(
    root: &Path,
    case: Option<&str>,
    group: Option<&str>,
) -> String {
    let mut command = format!("cairn determinism replay --root {}", path_string(root));
    if let Some(case) = case {
        command.push_str(&format!(" --case {case}"));
    }
    if let Some(group) = group {
        command.push_str(&format!(" --group {group}"));
    }
    command
}

fn replay_temp_root(source: &Path, label: &str) -> Result<PathBuf, Box<dyn Error>> {
    let destination = env::temp_dir().join(format!("cairn-replay-{}-{label}", std::process::id()));
    if destination.exists() {
        fs::remove_dir_all(&destination)?;
    }
    copy_tree(source, &destination)?;
    Ok(destination)
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    let mut dirs = vec![(source.to_path_buf(), destination.to_path_buf())];
    while let Some((current_source, current_destination)) = dirs.pop() {
        fs::create_dir_all(&current_destination)?;
        let mut entries = fs::read_dir(current_source)?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(|entry| entry.path());
        for entry in entries {
            let source_path = entry.path();
            let destination_path = current_destination.join(entry.file_name());
            if entry.file_type()?.is_dir() {
                dirs.push((source_path, destination_path));
            } else {
                fs::copy(&source_path, &destination_path)?;
            }
        }
    }
    Ok(())
}
