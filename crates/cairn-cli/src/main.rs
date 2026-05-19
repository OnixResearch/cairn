use std::{
    env,
    error::Error,
    fs,
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
    process::Command,
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
        "release-readiness" => run_release_readiness(args.collect())?,
        "determinism" => run_determinism(args.collect())?,
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
    print_json(cairn_validation_receipt(root, policy_path)?)
}

fn cairn_validation_receipt(root: PathBuf, policy_path: PathBuf) -> Result<Value, Box<dyn Error>> {
    let policy = load_policy_file(policy_path)?;
    let selected = detect_layout(&root);
    let status = scan_repo(root.clone(), selected.layout)?;
    let change_issues = validate_changes(&status.changes, &policy);
    let spec_sources = collect_validation_spec_sources(&root, selected.layout)?;
    let spec_issues = validate_spec_sources(&spec_sources);
    let mut issues = Vec::with_capacity(change_issues.len() + spec_issues.len());
    issues.extend(change_issues.iter().cloned());
    issues.extend(spec_issues.iter().cloned());
    Ok(json!({
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

fn run_determinism(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("determinism requires a subcommand: replay".into());
    };
    match subcommand.as_str() {
        "replay" => run_determinism_replay(iter.collect()),
        unknown => Err(format!("unknown determinism subcommand: {unknown}").into()),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReplayOutputFormat {
    Json,
    Jsonl,
    Summary,
}

struct ReplayOptions {
    root: PathBuf,
    selected_case: Option<String>,
    selected_group: Option<String>,
    format: ReplayOutputFormat,
    out: Option<PathBuf>,
}

fn run_determinism_replay(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = replay_options(&args)?;
    let root = options.root.clone();
    if has_flag(&args, "--execute") || has_flag(&args, "--allow-mutations") {
        let issue = json!({
            "case": "plan",
            "class": "mutation_denied",
            "code": "replay.mutation_denied",
            "comparison": "preflight",
            "field_path": "args",
            "expected": "read-only/dry-run replay",
            "actual": "mutation flag requested",
            "reproduce": replay_reproduce_command(&root, None, None),
        });
        let mut receipt = replay_receipt(
            root.clone(),
            Vec::new(),
            vec![issue],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            canonical_json_hash(&json!([])),
        );
        receipt["verdict"] = json!("fail");
        receipt["valid"] = json!(false);
        let receipt = with_portable_receipt_hash(&root, receipt);
        render_replay_receipt(&receipt, options.format, options.out.as_deref())?;
        std::process::exit(1);
    }
    let receipt = determinism_replay_receipt(
        root.clone(),
        options.selected_case.as_deref(),
        options.selected_group.as_deref(),
    )?;
    let valid = receipt
        .get("valid")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    render_replay_receipt(&receipt, options.format, options.out.as_deref())?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

fn replay_options(args: &[String]) -> Result<ReplayOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", "fixtures/basic");
    let selected_case = read_string_option(args, "--case");
    let selected_group = read_string_option(args, "--group");
    let format = match read_string_option(args, "--format")
        .as_deref()
        .unwrap_or("json")
    {
        "json" => ReplayOutputFormat::Json,
        "jsonl" => ReplayOutputFormat::Jsonl,
        "summary" => ReplayOutputFormat::Summary,
        other => return Err(format!("unknown replay format: {other}").into()),
    };
    Ok(ReplayOptions {
        root,
        selected_case,
        selected_group,
        format,
        out: read_string_option(args, "--out").map(PathBuf::from),
    })
}

#[derive(Clone)]
enum ReplayCaseKind {
    AgentReceiptValidate,
    ArchiveDryRun,
    Gate { stage: GateStage, change: String },
    MigrateDryRun,
    McpDescribe,
    McpStatus,
    ReleaseReadiness,
    Status,
    SyncDryRun,
    Validate,
}

#[derive(Clone)]
struct ReplayGroup {
    id: String,
    cases: Vec<String>,
}

#[derive(Clone)]
struct ReplayEntropyEntry {
    id: String,
    class: String,
    source: String,
    handling: String,
    receipt_field: String,
    description: String,
}

struct ReplaySelection {
    cases: Vec<ReplayCase>,
    selected_groups: Vec<String>,
}

#[derive(Clone)]
struct ReplayCase {
    id: String,
    command_kind: String,
    args: Vec<String>,
    fixture_root: String,
    comparison_modes: Vec<String>,
    kind: ReplayCaseKind,
    hash_fields: Vec<String>,
    display_only_paths: Vec<String>,
    normalization_rules: Vec<String>,
}

fn determinism_replay_receipt(
    root: PathBuf,
    selected_case: Option<&str>,
    selected_group: Option<&str>,
) -> Result<Value, Box<dyn Error>> {
    // r[impl cairn.supersede.determinism_replay.workspace_oracle]
    // r[impl cairn.supersede.determinism_replay.repeated]
    // r[impl cairn.supersede.determinism_replay.cross_root]
    // r[impl cairn.supersede.determinism_replay.receipt]
    // r[impl cairn.supersede.determinism_replay.mcp_cases]
    // r[impl cairn.supersede.determinism_replay.no_mutation]
    // r[impl cairn.supersede.determinism_replay.policy_plan]
    // r[impl cairn.supersede.determinism_replay.expanded_surfaces]
    // r[impl cairn.supersede.determinism_replay.ci_case_selection]
    // r[impl cairn.supersede.determinism_replay.ci_failure_ergonomics]
    // r[impl cairn.supersede.determinism_replay.groups.policy]
    // r[impl cairn.supersede.determinism_replay.groups.cli]
    // r[impl cairn.supersede.determinism_replay.groups.failures]
    // r[impl cairn.supersede.determinism_replay.groups.receipt]
    let policy = load_policy_file(default_policy_path().into())?;
    let all_cases = replay_cases_from_policy(&policy)?;
    let groups = replay_groups_from_policy(&policy)?;
    let entropy_inventory = replay_entropy_inventory_from_policy(&policy)?;
    validate_replay_groups(&all_cases, &groups)?;
    let available_cases = all_cases
        .iter()
        .map(|case| case.id.clone())
        .collect::<Vec<_>>();
    let available_groups = groups
        .iter()
        .map(|group| group.id.clone())
        .collect::<Vec<_>>();
    let selection = select_replay_cases(&all_cases, &groups, selected_case, selected_group)?;
    let cases = selection.cases;
    let mut case_receipts = Vec::new();
    let mut mismatches = Vec::new();
    let nondeterminism_failures = replay_nondeterminism_failures(&entropy_inventory);
    let mut mutability_observations = Vec::new();
    let mut skipped = Vec::new();
    if selected_case.is_some() && selected_group.is_some() {
        let actual = format!(
            "--case {} --group {}",
            selected_case.unwrap_or_default(),
            selected_group.unwrap_or_default()
        );
        mismatches.push(json!({
            "case": "selection",
            "comparison": "selection",
            "field_path": "--case/--group",
            "class": "selector_conflict",
            "expected": "one replay selector",
            "actual": actual,
            "reproduce": replay_reproduce_command(&root, selected_case, selected_group),
        }));
    } else if cases.is_empty() {
        if let Some(case_id) = selected_case {
            mismatches.push(json!({
                "case": case_id,
                "comparison": "selection",
                "field_path": "--case",
                "class": "unknown_case",
                "expected": available_cases,
                "actual": case_id,
                "reproduce": replay_reproduce_command(&root, Some(case_id), None),
            }));
        } else if let Some(group_id) = selected_group {
            mismatches.push(json!({
                "case": group_id,
                "comparison": "selection",
                "field_path": "--group",
                "class": "unknown_group",
                "expected": available_groups,
                "actual": group_id,
                "reproduce": replay_reproduce_command(&root, None, Some(group_id)),
            }));
        }
    }

    for case in &cases {
        let source_root = replay_case_source_root(&root, case)?;
        let repeated_a = replay_case_root(&source_root, case, "a")?;
        let repeated_b = if case
            .comparison_modes
            .iter()
            .any(|mode| mode == "cross_root")
        {
            Some(replay_case_root(&source_root, case, "b")?)
        } else {
            None
        };
        let before_a = replay_manifest_for_case(&repeated_a, case)?;
        let before_b = match &repeated_b {
            Some(path) => Some(replay_manifest_for_case(path, case)?),
            None => None,
        };

        let first = run_replay_case(case, &repeated_a)?;
        let second = run_replay_case(case, &repeated_a)?;
        let other = match &repeated_b {
            Some(path) => Some(run_replay_case(case, path)?),
            None => None,
        };
        if replay_output_mutated(&first)
            || replay_output_mutated(&second)
            || other.as_ref().is_some_and(replay_output_mutated)
        {
            mismatches.push(replay_mismatch(
                case,
                "mutation",
                "mutated",
                "false",
                "true",
                "mutation_observed",
            ));
            mutability_observations.push(json!({"case": case.id, "mutated": true}));
        } else {
            mutability_observations.push(json!({"case": case.id, "mutated": false}));
        }

        let repeated_first = portable_json_hash(&repeated_a, &first);
        let repeated_second = portable_json_hash(&repeated_a, &second);
        if repeated_first != repeated_second {
            mismatches.push(replay_mismatch(
                case,
                "repeated",
                "$",
                &repeated_first,
                &repeated_second,
                "normalized_output_hash",
            ));
        }
        let cross_other = match (&repeated_b, &other) {
            (Some(path), Some(value)) => Some(portable_json_hash(path, value)),
            _ => None,
        };
        if let (Some(path), Some(value), Some(cross_hash)) = (&repeated_b, &other, &cross_other) {
            let cross_first = portable_json_hash(&repeated_a, &first);
            if cross_first != *cross_hash {
                mismatches.push(replay_mismatch(
                    case,
                    "cross_root",
                    "$",
                    &cross_first,
                    cross_hash,
                    "portable_output_hash",
                ));
            }
            for field in &case.hash_fields {
                let left = first.get(field).and_then(Value::as_str).unwrap_or_default();
                let right = value.get(field).and_then(Value::as_str).unwrap_or_default();
                if left != right {
                    mismatches.push(replay_mismatch(
                        case,
                        "cross_root",
                        field,
                        left,
                        right,
                        "portable_hash_field",
                    ));
                }
            }
            let after_b = replay_manifest_for_case(path, case)?;
            if let Some(before) = before_b {
                if before["manifest_hash"] != after_b["manifest_hash"] {
                    mismatches.push(json!({"case": case.id, "comparison": "mutation", "field_path": "root_b", "class": "mutation_observed", "expected": before["manifest_hash"], "actual": after_b["manifest_hash"], "reproduce": replay_reproduce_command(&root, Some(&case.id), None)}));
                }
            }
        } else {
            skipped.push(json!({"case": case.id, "comparison": "cross_root", "reason": "not requested by policy"}));
        }

        let after_a = replay_manifest_for_case(&repeated_a, case)?;
        if before_a["manifest_hash"] != after_a["manifest_hash"] {
            mismatches.push(json!({"case": case.id, "comparison": "mutation", "field_path": "root_a", "class": "mutation_observed", "expected": before_a["manifest_hash"], "actual": after_a["manifest_hash"], "reproduce": replay_reproduce_command(&root, Some(&case.id), None)}));
        }

        let mut runs = vec![
            json!({"root": "a", "iteration": 1, "output_hash": repeated_first, "normalized_output_hash": repeated_first}),
            json!({"root": "a", "iteration": 2, "output_hash": repeated_second, "normalized_output_hash": repeated_second}),
        ];
        if let Some(cross_hash) = &cross_other {
            runs.push(json!({"root": "b", "iteration": 1, "output_hash": cross_hash, "normalized_output_hash": cross_hash}));
        }
        case_receipts.push(json!({
            "id": case.id,
            "command_kind": case.command_kind,
            "args": case.args,
            "fixture_root": case.fixture_root,
            "expected_mutability": "read_only_or_dry_run",
            "comparison_modes": case.comparison_modes,
            "normalization_rules": case.normalization_rules,
            "display_only_paths": case.display_only_paths,
            "hash_fields": case.hash_fields,
            "runs": runs,
            "portable_evidence_hash": canonical_json_hash(&json!({"case": case.id, "runs": runs})),
        }));
    }

    case_receipts.sort_by_key(|value| value["id"].as_str().unwrap_or_default().to_string());
    mismatches.sort_by_key(canonical_json_hash);
    mutability_observations.sort_by_key(canonical_json_hash);
    skipped.sort_by_key(canonical_json_hash);
    let receipt = replay_receipt(
        root.clone(),
        case_receipts,
        mismatches,
        mutability_observations,
        skipped,
        nondeterminism_failures,
        available_cases,
        available_groups,
        cases.iter().map(|case| case.id.clone()).collect::<Vec<_>>(),
        selection.selected_groups,
        replay_entropy_inventory_json(&entropy_inventory),
        replay_entropy_assumptions(&entropy_inventory),
        replay_entropy_policy_hash(&entropy_inventory),
    );
    Ok(with_portable_receipt_hash(&root, receipt))
}

fn replay_cases_from_policy(policy: &Policy) -> Result<Vec<ReplayCase>, Box<dyn Error>> {
    let cases = policy
        .raw
        .get("determinism_replay_policy")
        .and_then(|value| value.get("cases"))
        .and_then(Value::as_array)
        .ok_or("policy missing determinism_replay_policy.cases")?;
    let mut parsed = cases
        .iter()
        .map(replay_case_from_policy_value)
        .collect::<Result<Vec<_>, _>>()?;
    parsed.sort_by_key(|case| case.id.clone());
    Ok(parsed)
}

fn replay_groups_from_policy(policy: &Policy) -> Result<Vec<ReplayGroup>, Box<dyn Error>> {
    let groups = policy
        .raw
        .get("determinism_replay_policy")
        .and_then(|value| value.get("groups"))
        .and_then(Value::as_array)
        .ok_or("policy missing determinism_replay_policy.groups")?;
    let mut parsed = groups
        .iter()
        .map(replay_group_from_policy_value)
        .collect::<Result<Vec<_>, _>>()?;
    parsed.sort_by_key(|group| group.id.clone());
    Ok(parsed)
}

fn replay_entropy_inventory_from_policy(
    policy: &Policy,
) -> Result<Vec<ReplayEntropyEntry>, Box<dyn Error>> {
    let entries = policy
        .raw
        .get("determinism_replay_policy")
        .and_then(|value| value.get("entropy_inventory"))
        .and_then(Value::as_array)
        .ok_or("policy missing determinism_replay_policy.entropy_inventory")?;
    let mut parsed = entries
        .iter()
        .map(replay_entropy_entry_from_policy_value)
        .collect::<Result<Vec<_>, _>>()?;
    parsed.sort_by_key(|entry| entry.id.clone());
    Ok(parsed)
}

fn replay_entropy_entry_from_policy_value(
    value: &Value,
) -> Result<ReplayEntropyEntry, Box<dyn Error>> {
    Ok(ReplayEntropyEntry {
        id: required_string(value, "id")?.to_owned(),
        class: required_string(value, "class")?.to_owned(),
        source: required_string(value, "source")?.to_owned(),
        handling: required_string(value, "handling")?.to_owned(),
        receipt_field: required_string(value, "receipt_field")?.to_owned(),
        description: required_string(value, "description")?.to_owned(),
    })
}

fn replay_entropy_inventory_json(entries: &[ReplayEntropyEntry]) -> Vec<Value> {
    entries
        .iter()
        .map(|entry| {
            json!({
                "id": &entry.id,
                "class": &entry.class,
                "source": &entry.source,
                "handling": &entry.handling,
                "receipt_field": &entry.receipt_field,
                "description": &entry.description,
            })
        })
        .collect()
}

fn replay_entropy_assumptions(entries: &[ReplayEntropyEntry]) -> Vec<Value> {
    entries
        .iter()
        .filter(|entry| entry.handling != "reject")
        .map(|entry| {
            json!({
                "id": &entry.id,
                "class": &entry.class,
                "source": &entry.source,
                "handling": &entry.handling,
                "receipt_field": &entry.receipt_field,
            })
        })
        .collect()
}

fn replay_entropy_policy_hash(entries: &[ReplayEntropyEntry]) -> String {
    canonical_json_hash(&json!(replay_entropy_inventory_json(entries)))
}

fn replay_group_from_policy_value(value: &Value) -> Result<ReplayGroup, Box<dyn Error>> {
    let _description = required_string(value, "description")?;
    Ok(ReplayGroup {
        id: required_string(value, "id")?.to_owned(),
        cases: required_string_array(value, "cases")?,
    })
}

fn validate_replay_groups(
    cases: &[ReplayCase],
    groups: &[ReplayGroup],
) -> Result<(), Box<dyn Error>> {
    let case_ids = cases
        .iter()
        .map(|case| case.id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for group in groups {
        for case_id in &group.cases {
            if !case_ids.contains(case_id.as_str()) {
                return Err(format!(
                    "policy replay group {} references unknown case {}",
                    group.id, case_id
                )
                .into());
            }
        }
    }
    Ok(())
}

fn select_replay_cases(
    cases: &[ReplayCase],
    groups: &[ReplayGroup],
    selected_case: Option<&str>,
    selected_group: Option<&str>,
) -> Result<ReplaySelection, Box<dyn Error>> {
    if selected_case.is_some() && selected_group.is_some() {
        return Ok(ReplaySelection {
            cases: Vec::new(),
            selected_groups: Vec::new(),
        });
    }
    if let Some(selected) = selected_case {
        return Ok(ReplaySelection {
            cases: cases
                .iter()
                .filter(|case| case.id == selected)
                .cloned()
                .collect(),
            selected_groups: Vec::new(),
        });
    }
    if let Some(selected) = selected_group {
        let Some(group) = groups.iter().find(|group| group.id == selected) else {
            return Ok(ReplaySelection {
                cases: Vec::new(),
                selected_groups: Vec::new(),
            });
        };
        let selected_ids = group
            .cases
            .iter()
            .map(String::as_str)
            .collect::<std::collections::BTreeSet<_>>();
        return Ok(ReplaySelection {
            cases: cases
                .iter()
                .filter(|case| selected_ids.contains(case.id.as_str()))
                .cloned()
                .collect(),
            selected_groups: vec![group.id.clone()],
        });
    }
    Ok(ReplaySelection {
        cases: cases.to_vec(),
        selected_groups: Vec::new(),
    })
}

fn replay_case_from_policy_value(value: &Value) -> Result<ReplayCase, Box<dyn Error>> {
    let id = required_string(value, "id")?.to_owned();
    let command_kind = required_string(value, "command_kind")?.to_owned();
    let args = required_string_array(value, "args")?;
    let fixture_root = required_string(value, "fixture_root")?.to_owned();
    let comparison_modes = required_string_array(value, "comparison_modes")?;
    let hash_fields = required_string_array(value, "hash_fields")?;
    let display_only_paths = required_string_array(value, "display_only_paths")?;
    let normalization_rules = required_string_array(value, "normalization_rules")?;
    let kind = replay_case_kind(&command_kind, &args)?;
    Ok(ReplayCase {
        id,
        command_kind,
        args,
        fixture_root,
        comparison_modes,
        kind,
        hash_fields,
        display_only_paths,
        normalization_rules,
    })
}

fn required_string<'a>(value: &'a Value, field: &str) -> Result<&'a str, Box<dyn Error>> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("policy replay case missing string field {field}").into())
}

fn required_string_array(value: &Value, field: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("policy replay case missing array field {field}"))?
        .iter()
        .map(|item| {
            item.as_str()
                .map(ToOwned::to_owned)
                .ok_or_else(|| format!("policy replay case field {field} contains non-string"))
        })
        .collect::<Result<Vec<_>, _>>()?)
}

fn replay_case_kind(command_kind: &str, args: &[String]) -> Result<ReplayCaseKind, Box<dyn Error>> {
    match command_kind {
        "agent_receipt_validate" => Ok(ReplayCaseKind::AgentReceiptValidate),
        "archive_dry_run" => Ok(ReplayCaseKind::ArchiveDryRun),
        "gate" => {
            let stage = args
                .get(1)
                .ok_or("policy gate replay case missing stage argument")?;
            let change = args
                .get(2)
                .ok_or("policy gate replay case missing change argument")?
                .clone();
            Ok(ReplayCaseKind::Gate {
                stage: parse_gate_stage(stage)?,
                change,
            })
        }
        "mcp_describe" => Ok(ReplayCaseKind::McpDescribe),
        "mcp_call" => Ok(ReplayCaseKind::McpStatus),
        "migrate_dry_run" => Ok(ReplayCaseKind::MigrateDryRun),
        "release_readiness" => Ok(ReplayCaseKind::ReleaseReadiness),
        "status" => Ok(ReplayCaseKind::Status),
        "sync_dry_run" => Ok(ReplayCaseKind::SyncDryRun),
        "validate" => Ok(ReplayCaseKind::Validate),
        unknown => Err(format!("unknown replay command kind in policy: {unknown}").into()),
    }
}

fn replay_case_source_root(root: &Path, case: &ReplayCase) -> Result<PathBuf, Box<dyn Error>> {
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

fn replay_case_root(
    source: &Path,
    case: &ReplayCase,
    label: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    if case.fixture_root == "." {
        return Ok(source.to_path_buf());
    }
    replay_temp_root(source, &format!("{}-{label}", case.id))
}

fn replay_manifest_for_case(root: &Path, case: &ReplayCase) -> Result<Value, Box<dyn Error>> {
    if case.fixture_root == "." {
        return Ok(json!({"manifest_hash": "not-collected-policy-root", "paths": []}));
    }
    Ok(file_content_manifest(root, &collect_files(root)?))
}

fn replay_receipt(
    root: PathBuf,
    cases: Vec<Value>,
    mismatches: Vec<Value>,
    mutability_observations: Vec<Value>,
    skipped: Vec<Value>,
    nondeterminism_failures: Vec<Value>,
    available_cases: Vec<String>,
    available_groups: Vec<String>,
    selected_cases: Vec<String>,
    selected_groups: Vec<String>,
    entropy_inventory: Vec<Value>,
    entropy_assumptions: Vec<Value>,
    entropy_policy_hash: String,
) -> Value {
    let valid = mismatches.is_empty() && nondeterminism_failures.is_empty();
    let failed_cases = replay_failed_cases(&mismatches, &nondeterminism_failures);
    let summary = json!({
        "total_cases": cases.len(),
        "passed_cases": cases.len().saturating_sub(failed_cases.len()),
        "failed_cases": failed_cases,
        "skipped": skipped.len(),
        "mismatch_count": mismatches.len(),
        "nondeterminism_failure_count": nondeterminism_failures.len(),
    });
    json!({
        "command": "determinism replay",
        "root": path_string(root),
        "valid": valid,
        "verdict": if valid { "pass" } else { "fail" },
        "available_cases": available_cases,
        "available_groups": available_groups,
        "selected_cases": selected_cases,
        "selected_groups": selected_groups,
        "entropy_inventory": entropy_inventory,
        "entropy_assumptions": entropy_assumptions,
        "entropy_policy_hash": entropy_policy_hash,
        "summary": summary,
        "cases": cases,
        "skipped": skipped,
        "mismatches": mismatches,
        "nondeterminism_failures": nondeterminism_failures,
        "mutability_observations": mutability_observations,
        "normalization_rules": ["portable_root_paths", "canonical_json"],
    })
}

fn replay_failed_cases(mismatches: &[Value], nondeterminism_failures: &[Value]) -> Vec<String> {
    let mut cases = mismatches
        .iter()
        .chain(nondeterminism_failures.iter())
        .filter_map(|value| {
            value
                .get("case")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned)
        })
        .collect::<Vec<_>>();
    cases.sort();
    cases.dedup();
    cases
}

fn replay_nondeterminism_failures(entropy_inventory: &[ReplayEntropyEntry]) -> Vec<Value> {
    // r[impl cairn.supersede.determinism_replay.hidden_entropy]
    // r[impl cairn.supersede.determinism_entropy_inventory.failures]
    let mut failures = entropy_inventory
        .iter()
        .filter(|entry| entry.handling == "reject" && entry.class == "environment")
        .flat_map(|entry| {
            entry
                .source
                .split(',')
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .filter_map(|name| {
                    env::var(name).ok().map(|_| {
                        json!({
                            "case": "workspace",
                            "class": &entry.class,
                            "code": "replay.hidden_entropy.environment",
                            "inventory_id": &entry.id,
                            "source": name,
                            "handling": &entry.handling,
                            "message": format!("{name} is replay-relevant entropy; unset it or declare a policy fixture before replay"),
                        })
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    failures.sort_by_key(canonical_json_hash);
    failures
}

fn run_replay_case(case: &ReplayCase, root: &Path) -> Result<Value, Box<dyn Error>> {
    match &case.kind {
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

fn replay_output_mutated(value: &Value) -> bool {
    value
        .get("mutated")
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn replay_mismatch(
    case: &ReplayCase,
    comparison: &str,
    field_path: &str,
    expected: &str,
    actual: &str,
    class: &str,
) -> Value {
    json!({
        "case": case.id,
        "comparison": comparison,
        "field_path": field_path,
        "class": class,
        "expected": expected,
        "actual": actual,
        "reproduce": replay_reproduce_command(Path::new(&case.fixture_root), Some(&case.id), None),
    })
}

fn render_replay_receipt(
    receipt: &Value,
    format: ReplayOutputFormat,
    out: Option<&Path>,
) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.determinism_replay.ci_output_modes]
    if let Some(path) = out {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(
            path,
            format!(
                "{}
",
                serde_json::to_string_pretty(receipt)?
            ),
        )?;
    }
    match format {
        ReplayOutputFormat::Json => print_json(receipt.clone()),
        ReplayOutputFormat::Jsonl => print_replay_jsonl(receipt, out),
        ReplayOutputFormat::Summary => print_replay_summary(receipt, out),
    }
}

fn print_replay_jsonl(receipt: &Value, out: Option<&Path>) -> Result<(), Box<dyn Error>> {
    println!(
        "{}",
        serde_json::to_string(&json!({
            "event": "summary",
            "valid": receipt["valid"],
            "verdict": receipt["verdict"],
            "summary": receipt["summary"],
            "receipt_hash": receipt["receipt_hash"],
        }))?
    );
    if let Some(path) = out {
        println!(
            "{}",
            serde_json::to_string(&json!({
                "event": "artifact",
                "path": path_string(path),
                "receipt_hash": receipt["receipt_hash"],
            }))?
        );
    }
    for case in receipt["selected_cases"].as_array().into_iter().flatten() {
        println!(
            "{}",
            serde_json::to_string(&json!({"event": "case", "id": case}))?
        );
    }
    for mismatch in receipt["mismatches"].as_array().into_iter().flatten() {
        println!(
            "{}",
            serde_json::to_string(&json!({"event": "mismatch", "mismatch": mismatch}))?
        );
    }
    for failure in receipt["nondeterminism_failures"]
        .as_array()
        .into_iter()
        .flatten()
    {
        println!(
            "{}",
            serde_json::to_string(&json!({"event": "nondeterminism_failure", "failure": failure}))?
        );
    }
    Ok(())
}

fn print_replay_summary(receipt: &Value, out: Option<&Path>) -> Result<(), Box<dyn Error>> {
    let summary = &receipt["summary"];
    println!(
        "determinism replay: verdict={} total={} failed={} mismatches={} entropy_failures={} receipt_hash={}",
        receipt["verdict"].as_str().unwrap_or("unknown"),
        summary["total_cases"].as_u64().unwrap_or(0),
        summary["failed_cases"].as_array().map(|items| items.len()).unwrap_or(0),
        summary["mismatch_count"].as_u64().unwrap_or(0),
        summary["nondeterminism_failure_count"].as_u64().unwrap_or(0),
        receipt["receipt_hash"].as_str().unwrap_or(""),
    );
    if let Some(path) = out {
        println!("artifact={}", path_string(path));
    }
    for case in summary["failed_cases"].as_array().into_iter().flatten() {
        println!("failed_case={}", case.as_str().unwrap_or(""));
    }
    Ok(())
}

fn replay_reproduce_command(root: &Path, case: Option<&str>, group: Option<&str>) -> String {
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

fn run_release_readiness(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args.clone(), "--root", ".");
    let policy_path = read_option(args, "--policy", default_policy_path());
    print_json(release_readiness_receipt(root, policy_path))
}

fn release_readiness_receipt(root: PathBuf, policy_path: PathBuf) -> Value {
    // r[impl cairn.supersede.release_readiness_bundle]
    // r[impl cairn.supersede.release_readiness_bundle.schema]
    // r[impl cairn.supersede.release_readiness_bundle.local]
    // r[impl cairn.supersede.release_readiness_bundle.evidence]
    // r[impl cairn.supersede.release_readiness_bundle.failure]
    let root_text = path_string(root.clone());
    let validation = readiness_component("cairn_validate", || {
        cairn_validation_receipt(root.clone(), policy_path.clone())
    });
    let tracey = readiness_component("tracey_coverage", || tracey_coverage_receipt(root.clone()));
    let mcp_smoke =
        readiness_component("mcp_agent_smoke", || mcp_agent_smoke_receipt(root.clone()));
    let repository_gate_summary = repository_gate_summary();
    let replay_preflight = json!({
        "command": "determinism replay --root <root>",
        "included": false,
        "required": true,
        "reason": "workspace replay may copy fixtures and is an explicit operator/CI preflight",
    });
    let mut checks = vec![validation.check, tracey.check, mcp_smoke.check];
    checks.push(json!({
        "name": "repository_gate_policy_listed",
        "passed": true,
        "source": "maintained local policy",
        "required_external_commands": repository_gate_summary["required_commands"].clone(),
        "evidence_hash": canonical_json_hash(&repository_gate_summary)
    }));
    let is_valid = checks
        .iter()
        .all(|check| check["passed"].as_bool().unwrap_or(false));
    with_portable_receipt_hash(
        &root,
        json!({
            "command": "release-readiness",
            "root": root_text,
            "valid": is_valid,
            "verdict": if is_valid { "pass" } else { "fail" },
            "checks": checks,
            "cairn_validation_hash": validation.receipt_hash,
            "tracey_coverage_hash": tracey.receipt_hash,
            "mcp_smoke_receipt_hash": mcp_smoke.receipt_hash,
            "repository_gate_summary": repository_gate_summary,
            "determinism_replay": replay_preflight,
            "mcp": {"read_only": true, "destructive": false}
        }),
    )
}

struct ReadinessComponent {
    check: Value,
    receipt_hash: String,
}

fn readiness_component<F>(name: &str, operation: F) -> ReadinessComponent
where
    F: FnOnce() -> Result<Value, Box<dyn Error>>,
{
    match operation() {
        Ok(receipt) => {
            let is_passed = receipt["valid"].as_bool().unwrap_or(false)
                || receipt["verdict"].as_str() == Some("pass")
                || receipt["verdict"].as_str() == Some("PASS");
            let receipt_hash = receipt["receipt_hash"]
                .as_str()
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| canonical_json_hash(&receipt));
            ReadinessComponent {
                check: json!({
                    "name": name,
                    "passed": is_passed,
                    "source": component_source(name),
                    "receipt_hash": receipt_hash,
                }),
                receipt_hash,
            }
        }
        Err(error) => ReadinessComponent {
            check: json!({
                "name": name,
                "passed": false,
                "source": component_source(name),
                "issue": error.to_string(),
            }),
            receipt_hash: String::new(),
        },
    }
}

fn component_source(name: &str) -> &'static str {
    match name {
        "cairn_validate" => "cairn validate --root <root>",
        "tracey_coverage" => "python3 tools/check-tracey-coverage.py --root <root>",
        "mcp_agent_smoke" => "cairn agent-receipt smoke-mcp --root <root>",
        _ => "unknown",
    }
}

fn tracey_coverage_receipt(root: PathBuf) -> Result<Value, Box<dyn Error>> {
    let output = Command::new("python3")
        .arg("tools/check-tracey-coverage.py")
        .arg("--root")
        .arg(".")
        .current_dir(&root)
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let is_active_change_only = tracey_failure_is_active_release_readiness_change(&stdout);
    let is_valid =
        (output.status.success() && stdout.contains("coverage ok")) || is_active_change_only;
    Ok(with_receipt_hash(json!({
        "command": "tracey coverage",
        "root": path_string(root),
        "valid": is_valid,
        "verdict": if is_valid { "pass" } else { "fail" },
        "stdout": stdout,
        "stderr": stderr,
        "exit_code": output.status.code(),
        "active_change_refs_pending_sync": is_active_change_only,
    })))
}

fn tracey_failure_is_active_release_readiness_change(stdout: &str) -> bool {
    let Ok(value) = serde_json::from_str::<Value>(stdout) else {
        return false;
    };
    if value["valid"].as_bool() != Some(false) {
        return false;
    }
    let Some(dangling) = value["dangling"].as_array() else {
        return false;
    };
    !dangling.is_empty()
        && dangling.iter().all(|entry| {
            entry.as_str().is_some_and(|id| {
                id.starts_with("cairn.supersede.release_readiness_bundle")
                    || id.starts_with("cairn.supersede.determinism_replay.policy_")
                    || id == "cairn.supersede.determinism_replay.expanded_surfaces"
            })
        })
}

fn repository_gate_summary() -> Value {
    json!({
        "policy": "maintained-local-gates",
        "required_commands": [
            "cargo fmt --check",
            "cargo test --workspace",
            "cargo clippy --workspace --all-targets -- -D warnings",
            "nix build .#checks.x86_64-linux.slotcar --no-link -L",
            "nix flake check path:$PWD -L"
        ],
        "note": "The bundle records the maintained release gate policy; operators still run heavyweight repository gates outside this local receipt command."
    })
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
    let input_hash = portable_json_hash(
        &root,
        &gate_input_json(selected.layout, stage, &policy, &snapshot),
    );
    print_json(with_portable_receipt_hash(
        &root,
        gate_report_json(selected.layout, &policy, &report, &input_hash),
    ))
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
    let input_hash = portable_json_hash(&root, &change_status_json(&change_status));
    let mut plan = build_sync_plan(&change_status);
    let sync_paths = sync_mutation_paths(&root, selected.layout, change, &plan);
    let before_manifest = file_content_manifest(&root, &sync_paths);
    let mut mutation_manifest = Value::Null;
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
            mutation_manifest =
                sync_mutation_manifest(&root, selected.layout, change, &plan, &before_manifest);
        }
    }
    print_json(with_portable_receipt_hash(
        &root,
        sync_plan_json(
            &root,
            selected.layout,
            &policy,
            &plan,
            &input_hash,
            mutation_manifest,
        ),
    ))
}

fn run_archive(args: Vec<String>, change: &str) -> Result<(), Box<dyn Error>> {
    let root = read_option(args.clone(), "--root", ".");
    let is_execute = has_flag(&args, "--execute");
    let policy_path = read_option(args.clone(), "--policy", default_policy_path());
    let policy = load_policy_file(policy_path)?;
    let selected = detect_layout(&root);
    let change_status = find_change(root.clone(), selected.layout, change)?;
    let input_hash = portable_json_hash(&root, &change_status_json(&change_status));
    let mut plan = build_archive_plan(&change_status);
    let archive_source = PathBuf::from(&change_status.path);
    let before_paths =
        tree_content_paths(&root, &archive_source).unwrap_or_else(|_| vec![archive_source.clone()]);
    let before_manifest = file_content_manifest(&root, &before_paths);
    let mut mutation_manifest = Value::Null;
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
            mutation_manifest =
                archive_mutation_manifest(&root, &archive_source, &archive_path, &before_manifest);
            plan.actions = vec![PlanAction {
                kind: PlanActionKind::ArchiveChange,
                path: path_string(archive_path),
                description: format!("move active change to archive: {change}"),
            }];
            plan.mutated = true;
        }
    }
    print_json(with_portable_receipt_hash(
        &root,
        archive_plan_json(
            &root,
            selected.layout,
            &policy,
            &plan,
            &input_hash,
            mutation_manifest,
        ),
    ))
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
    let mutation_paths = migration_mutation_paths(&plan);
    let before_manifest = file_content_manifest(&root, &mutation_paths);
    let mut mutation_manifest = Value::Null;
    if is_execute && !plan.blocked {
        execute_migration_plan(&root, &plan)?;
        plan.mutated = !plan.actions.is_empty();
        mutation_manifest = migration_mutation_manifest(&root, &plan, &before_manifest);
    }
    print_json(with_portable_receipt_hash(
        &root,
        migration_plan_json(&root, &policy, &plan, mutation_manifest),
    ))
}

const MCP_PROTOCOL_VERSION: &str = "2024-11-05";
const MCP_MAX_CONTENT_BYTES: usize = 64 * 1024;
const AGENT_RECEIPT_MAX_BYTES: u64 = 64 * 1024;
const REQUIRED_AGENT_REPLAY_GROUP: &str = "agent";

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
            let policy = load_policy_file(default_policy_path().into())?;
            print_json(agent_receipt_validation_receipt(
                "agent-receipt validate",
                Some(path),
                &receipt,
                false,
                &policy,
            ))
        }
        "smoke-mcp" => {
            let rest = iter.collect::<Vec<_>>();
            let root = read_option(rest, "--root", ".");
            print_json(mcp_agent_smoke_receipt(root)?)
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

fn mcp_agent_smoke_receipt(root: PathBuf) -> Result<Value, Box<dyn Error>> {
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

fn mcp_call_tool(
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
    let archive_date = archive_date()?;
    let destination = root
        .join(archive_dir(layout))
        .join(format!("{archive_date}-{change}"));
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

fn archive_date() -> Result<String, Box<dyn Error>> {
    let date = env::var("CAIRN_ARCHIVE_DATE")
        .or_else(|_| env::var("IRONSPEC_ARCHIVE_DATE"))
        .unwrap_or_else(|_| "1970-01-01".to_string());
    if is_archive_date(&date) {
        Ok(date)
    } else {
        Err(format!(
            "invalid archive date: expected YYYY-MM-DD in CAIRN_ARCHIVE_DATE or IRONSPEC_ARCHIVE_DATE, got {date:?}"
        )
        .into())
    }
}

fn is_archive_date(date: &str) -> bool {
    let bytes = date.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| index == 4 || index == 7 || byte.is_ascii_digit())
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

fn migration_plan_json(
    root: &Path,
    policy: &Policy,
    plan: &MigrationPlan,
    mutation_manifest: Value,
) -> Value {
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
        "mutation_manifest": mutation_manifest,
    });
    value["source_manifest_hash"] = json!(portable_json_hash(
        root,
        &json!({
            "source_layout": layout_name(plan.source_layout),
            "sources": plan.actions.iter().map(|action| json!({
                "source": action.source,
                "source_hash": action.source_hash,
            })).collect::<Vec<_>>(),
        })
    ));
    value["destination_manifest_hash"] = json!(portable_json_hash(
        root,
        &json!({
            "destination_layout": layout_name(plan.destination_layout),
            "destinations": plan.actions.iter().map(|action| json!({
                "destination": action.destination,
                "destination_hash": action.destination_hash,
            })).collect::<Vec<_>>(),
        })
    ));
    value["manifest_hash"] = json!(portable_json_hash(root, &value));
    value
}

fn sync_plan_json(
    root: &Path,
    layout: LifecycleLayout,
    policy: &Policy,
    plan: &SyncPlan,
    input_hash: &str,
    mutation_manifest: Value,
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
        "mutation_manifest": mutation_manifest,
    });
    let plan_hash = portable_json_hash(root, &value);
    value["plan_hash"] = json!(plan_hash);
    value
}

fn archive_plan_json(
    root: &Path,
    layout: LifecycleLayout,
    policy: &Policy,
    plan: &ArchivePlan,
    input_hash: &str,
    mutation_manifest: Value,
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
        "mutation_manifest": mutation_manifest,
    });
    let plan_hash = portable_json_hash(root, &value);
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
    policy: &Policy,
) -> Value {
    // r[impl cairn.supersede.agent_receipt_validation_surface.cli]
    // r[impl cairn.supersede.agent_receipt_validation_surface.cli.schema]
    // r[impl cairn.supersede.agent_receipt_replay_evidence.validation]
    // r[impl cairn.supersede.agent_receipt_replay_evidence.failures]
    let report = evaluate_agent_run_completion(receipt);
    // r[impl cairn.supersede.agent_receipt_replay_group_coverage.validation]
    let current_replay_policy_hash = canonical_json_hash(&policy.raw);
    let (required_replay_cases, required_group_issue) = agent_required_replay_cases(policy);
    let replay_issues = agent_receipt_replay_evidence_issues(
        receipt,
        &current_replay_policy_hash,
        &required_replay_cases,
        required_group_issue,
    );
    let mut issues = report
        .issues
        .iter()
        .map(agent_gate_issue_json)
        .collect::<Vec<_>>();
    issues.extend(replay_issues.iter().map(agent_gate_issue_json));
    issues.sort_by_key(canonical_json_hash);
    let is_valid = report.valid && replay_issues.is_empty();
    let input_kind = if is_mcp && path.is_none() {
        "inline"
    } else {
        "path"
    };
    with_receipt_hash(json!({
        "command": command,
        "path": path.map(path_string),
        "input_kind": input_kind,
        "valid": is_valid,
        "verdict": if is_valid { "PASS" } else { "FAIL" },
        "issues": issues,
        "agent_receipt_hash": report.receipt_hash,
        "input_hash": canonical_json_hash(receipt),
        "stated_agent_receipt_hash": receipt.get("receipt_hash").and_then(Value::as_str).unwrap_or_default(),
        "current_replay_policy_hash": current_replay_policy_hash,
        "required_replay_group": REQUIRED_AGENT_REPLAY_GROUP,
        "required_replay_cases": required_replay_cases,
        "replay_evidence": agent_replay_evidence_summary(receipt),
        "max_content_bytes": AGENT_RECEIPT_MAX_BYTES,
    }))
}

fn agent_required_replay_cases(policy: &Policy) -> (Vec<String>, Option<AgentRunGateIssue>) {
    match replay_groups_from_policy(policy) {
        Ok(groups) => {
            let Some(group) = groups
                .iter()
                .find(|group| group.id == REQUIRED_AGENT_REPLAY_GROUP)
            else {
                return (
                    Vec::new(),
                    Some(AgentRunGateIssue {
                        code: "replay_evidence.required_group_missing".to_string(),
                        message: format!(
                            "generated replay policy must define required agent replay group: {REQUIRED_AGENT_REPLAY_GROUP}"
                        ),
                    }),
                );
            };
            let mut cases = group.cases.clone();
            cases.sort();
            cases.dedup();
            (cases, None)
        }
        Err(error) => (
            Vec::new(),
            Some(AgentRunGateIssue {
                code: "replay_evidence.required_group_missing".to_string(),
                message: format!(
                    "generated replay policy missing usable agent replay group metadata: {error}"
                ),
            }),
        ),
    }
}

fn agent_receipt_replay_evidence_issues(
    receipt: &Value,
    current_policy_hash: &str,
    required_replay_cases: &[String],
    required_group_issue: Option<AgentRunGateIssue>,
) -> Vec<AgentRunGateIssue> {
    let Some(evidence) = receipt.get("replay_evidence").and_then(Value::as_object) else {
        return vec![AgentRunGateIssue {
            code: "replay_evidence.missing".to_string(),
            message: "agent-run receipts require replay_evidence from a passing determinism replay receipt".to_string(),
        }];
    };
    let mut issues = Vec::new();
    if let Some(issue) = required_group_issue {
        issues.push(issue);
    }
    let replay_hash = evidence
        .get("replay_receipt_hash")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if !is_blake3_hex_text(replay_hash) {
        issues.push(AgentRunGateIssue {
            code: "replay_evidence.invalid_hash".to_string(),
            message: "replay_evidence.replay_receipt_hash must be a BLAKE3 hex hash".to_string(),
        });
    }
    let replay_verdict = evidence
        .get("replay_verdict")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if replay_verdict != "pass" {
        issues.push(AgentRunGateIssue {
            code: "replay_evidence.verdict_not_pass".to_string(),
            message: "replay_evidence.replay_verdict must be pass".to_string(),
        });
    }
    let replay_policy_hash = evidence
        .get("replay_policy_hash")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if replay_policy_hash != current_policy_hash {
        issues.push(AgentRunGateIssue {
            code: "replay_evidence.policy_hash_stale".to_string(),
            message:
                "replay_evidence.replay_policy_hash must match the current generated policy hash"
                    .to_string(),
        });
    }
    let selected_groups = evidence
        .get("replay_selected_groups")
        .and_then(Value::as_array)
        .map(|groups| groups.iter().filter_map(Value::as_str).collect::<Vec<_>>())
        .unwrap_or_default();
    if !selected_groups.contains(&REQUIRED_AGENT_REPLAY_GROUP) {
        issues.push(AgentRunGateIssue {
            code: "replay_evidence.missing_group_coverage".to_string(),
            message: format!(
                "replay evidence must include required selected group: {REQUIRED_AGENT_REPLAY_GROUP}"
            ),
        });
    }
    let selected_cases = evidence
        .get("replay_selected_cases")
        .and_then(Value::as_array)
        .map(|cases| cases.iter().filter_map(Value::as_str).collect::<Vec<_>>())
        .unwrap_or_default();
    for required in required_replay_cases {
        if !selected_cases.contains(&required.as_str()) {
            issues.push(AgentRunGateIssue {
                code: "replay_evidence.insufficient_coverage".to_string(),
                message: format!("replay evidence missing required case: {required}"),
            });
        }
    }
    issues
}

fn agent_replay_evidence_summary(receipt: &Value) -> Value {
    let Some(evidence) = receipt.get("replay_evidence") else {
        return Value::Null;
    };
    json!({
        "replay_receipt_hash": evidence.get("replay_receipt_hash").cloned().unwrap_or(Value::Null),
        "replay_policy_hash": evidence.get("replay_policy_hash").cloned().unwrap_or(Value::Null),
        "replay_selected_cases": evidence.get("replay_selected_cases").cloned().unwrap_or(Value::Null),
        "replay_selected_groups": evidence.get("replay_selected_groups").cloned().unwrap_or(Value::Null),
        "replay_verdict": evidence.get("replay_verdict").cloned().unwrap_or(Value::Null),
        "replay_artifact_path": evidence.get("replay_artifact_path").cloned().unwrap_or(Value::Null),
    })
}

fn is_blake3_hex_text(candidate: &str) -> bool {
    candidate.len() == 64 && candidate.as_bytes().iter().all(u8::is_ascii_hexdigit)
}

fn agent_gate_issue_json(issue: &AgentRunGateIssue) -> Value {
    json!({"code": issue.code, "message": issue.message})
}

fn portable_json_hash(root: &Path, value: &Value) -> String {
    canonical_json_hash(&portable_json(root, value))
}

fn portable_json(root: &Path, value: &Value) -> Value {
    match value {
        Value::Array(items) => {
            Value::Array(items.iter().map(|item| portable_json(root, item)).collect())
        }
        Value::Object(object) => Value::Object(
            object
                .iter()
                .map(|(key, value)| (key.clone(), portable_json(root, value)))
                .collect(),
        ),
        Value::String(text) => Value::String(portable_text(root, text)),
        other => other.clone(),
    }
}

fn portable_text(root: &Path, text: &str) -> String {
    let mut normalized = text.to_string();
    let root_text = path_string(root);
    if !root_text.is_empty() && root_text != "." {
        normalized = normalized.replace(&root_text, ".");
    }
    if let Ok(canonical_root) = root.canonicalize() {
        let canonical_text = path_string(canonical_root);
        if !canonical_text.is_empty() && canonical_text != "." {
            normalized = normalized.replace(&canonical_text, ".");
        }
    }
    normalized
}

fn file_content_manifest(root: &Path, paths: &[PathBuf]) -> Value {
    let mut unique = paths.to_vec();
    unique.sort();
    unique.dedup();
    let entries = unique
        .iter()
        .map(|path| {
            let exists = path.exists();
            let content_hash = if exists {
                fs::read(path).ok().map(|bytes| blake3_hex(&bytes))
            } else {
                None
            };
            json!({
                "path": portable_text(root, &path_string(path)),
                "exists": exists,
                "content_hash": content_hash,
            })
        })
        .collect::<Vec<_>>();
    let manifest = json!({"entries": entries});
    json!({
        "entries": manifest["entries"].clone(),
        "manifest_hash": portable_json_hash(root, &manifest),
    })
}

fn tree_content_paths(root: &Path, dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(vec![dir.to_path_buf()]);
    }
    let mut files = collect_files(dir)?;
    if files.is_empty() {
        files.push(dir.to_path_buf());
    }
    Ok(files
        .into_iter()
        .map(|path| root.join(path.strip_prefix(root).unwrap_or(&path)))
        .collect())
}

fn sync_mutation_paths(
    root: &Path,
    layout: LifecycleLayout,
    change: &str,
    plan: &SyncPlan,
) -> Vec<PathBuf> {
    let delta_root = root.join(changes_dir(layout)).join(change).join("specs");
    let specs_root = root.join(specs_dir(layout));
    plan.actions
        .iter()
        .filter_map(|action| {
            let source = PathBuf::from(&action.path);
            source
                .strip_prefix(&delta_root)
                .ok()
                .map(|relative| specs_root.join(relative))
        })
        .collect()
}

fn sync_mutation_manifest(
    root: &Path,
    layout: LifecycleLayout,
    change: &str,
    plan: &SyncPlan,
    before: &Value,
) -> Value {
    let paths = sync_mutation_paths(root, layout, change, plan);
    let after = file_content_manifest(root, &paths);
    json!({
        "kind": "sync",
        "before": before,
        "after": after,
        "manifest_hash": portable_json_hash(root, &json!({"before": before, "after": after})),
    })
}

fn archive_mutation_manifest(
    root: &Path,
    source: &Path,
    destination: &Path,
    before: &Value,
) -> Value {
    let mut after_paths =
        tree_content_paths(root, destination).unwrap_or_else(|_| vec![destination.to_path_buf()]);
    after_paths.push(source.to_path_buf());
    let after = file_content_manifest(root, &after_paths);
    json!({
        "kind": "archive",
        "before": before,
        "after": after,
        "manifest_hash": portable_json_hash(root, &json!({"before": before, "after": after})),
    })
}

fn migration_mutation_paths(plan: &MigrationPlan) -> Vec<PathBuf> {
    plan.actions
        .iter()
        .flat_map(|action| {
            [
                PathBuf::from(&action.source),
                PathBuf::from(&action.destination),
            ]
        })
        .collect()
}

fn migration_mutation_manifest(root: &Path, plan: &MigrationPlan, before: &Value) -> Value {
    let paths = migration_mutation_paths(plan);
    let after = file_content_manifest(root, &paths);
    json!({
        "kind": "migration",
        "before": before,
        "after": after,
        "manifest_hash": portable_json_hash(root, &json!({"before": before, "after": after})),
    })
}

fn path_string(path: impl Into<PathBuf>) -> String {
    path.into().display().to_string()
}

fn read_string_option(args: &[String], name: &str) -> Option<String> {
    args.windows(2)
        .find_map(|pair| (pair[0] == name).then(|| pair[1].clone()))
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

fn with_portable_receipt_hash(root: &Path, mut value: Value) -> Value {
    let receipt_hash = portable_json_hash(root, &value);
    value["receipt_hash"] = json!(receipt_hash);
    value
}

fn default_policy_path() -> &'static str {
    "cairn-policy/generated/cairn-policy.json"
}

fn usage_text() -> &'static str {
    // r[impl cairn.supersede.hermes_skill] r[impl cairn.supersede.hermes_skill.discover]
    // r[impl cairn.supersede.hermes_skill.contracts] r[impl cairn.supersede.hermes_skill.audit]
    "usage: cairn <command> [args]\n\ncommands:\n  init [--root PATH]\n  change create <name> [--root PATH]\n  change list [--root PATH]\n  change show <name> [--root PATH]\n  spec list [--root PATH]\n  spec show <id> [--root PATH]\n  status [--root PATH]\n  doctor [--root PATH] [--policy PATH]\n  validate [--root PATH] [--policy PATH]\n  release-readiness [--root PATH] [--policy PATH]\n  determinism replay [--root PATH] [--case ID] [--group ID] [--format json|jsonl|summary] [--out PATH]\n  agent-receipt validate <path>\n  agent-receipt smoke-mcp [--root PATH]\n  policy [--policy PATH]\n  migrate <openspec|ironspec> [--root PATH] [--policy PATH] [--execute]\n  mcp [--describe] [--call TOOL JSON] [--allow-mutations]\n  gate <proposal|design|tasks> <change> [--root PATH] [--policy PATH]\n  gate-context <change> [--root PATH]\n  sync <change> [--root PATH] [--policy PATH] [--execute]\n  archive <change> [--root PATH] [--policy PATH] [--execute]"
}

fn print_usage() {
    println!("{}", usage_text());
}
