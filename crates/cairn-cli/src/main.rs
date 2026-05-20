use std::{
    env,
    error::Error,
    fs,
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
    process::Command,
};

mod content_addressed_evidence;

use content_addressed_evidence::{
    run_evidence_downstream_pilot, run_evidence_export_bundle_receipt,
    run_evidence_function_object, run_evidence_name_pointer,
};

use cairn_core::{
    archive_dir, blake3_hex, build_archive_plan, build_change_detail, build_change_list,
    build_change_skeleton, build_doctor_report, build_ironspec_migration_plan,
    build_openspec_migration_plan, build_repo_status, build_spec_detail, build_spec_list,
    build_sync_plan, canonical_json_hash, changes_dir, design_template,
    evaluate_agent_run_completion, evaluate_gate_snapshot, init_artifacts, init_artifacts_for,
    layout_name, layout_root, merge_delta_spec, parse_policy_json, proposal_template, specs_dir,
    tasks_template, validate_changes, validate_policy_json, validate_spec_sources,
    AgentRunGateIssue, ArchivePlan, ChangeDetail, ChangeList, ChangeSource, ChangeStatus,
    DoctorCheckStatus, DoctorReport, DoctorSnapshot, GateArtifact, GateReport, GateSnapshot,
    GateStage, LifecycleArtifactKind, LifecycleLayout, MigrationInput, MigrationPlan, PlanAction,
    PlanActionKind, Policy, RepoStatus, SpecDetail, SpecList, SpecSource, SpecSummary, SyncPlan,
    TaskCounts,
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
        "evidence" => run_evidence(args.collect())?,
        "evidence-manifest" => run_evidence_manifest(args.collect())?,
        "policy" => run_policy(args.collect())?,
        "tracey" => run_tracey(args.collect())?,
        "rename-classification" => run_rename_classification(args.collect())?,
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
    // r[impl cairn.supersede.rust_nickel_only_gates.gates]
    let root = read_option(args.clone(), "--root", ".");
    let policy_path = read_option(args.clone(), "--policy", default_policy_path());
    let is_strict = has_flag(&args, "--strict");
    let receipt = cairn_validation_receipt(root, policy_path)?;
    print_json(receipt.clone())?;
    if is_strict && receipt["valid"].as_bool() != Some(true) {
        return Err("validation failed".into());
    }
    Ok(())
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
        "coverage-audit" => run_determinism_coverage_audit(iter.collect()),
        "replay" => run_determinism_replay(iter.collect()),
        "verify-receipt" => run_determinism_verify_receipt(iter.collect()),
        unknown => Err(format!("unknown determinism subcommand: {unknown}").into()),
    }
}

fn run_determinism_coverage_audit(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.determinism_surface_coverage_audit.cli]
    // r[impl cairn.supersede.determinism_surface_coverage_audit.inventory]
    // r[impl cairn.supersede.determinism_surface_coverage_audit.failures]
    let root = read_option(args, "--root", ".");
    let policy = load_policy_file(default_policy_path().into())?;
    let receipt = determinism_coverage_audit_receipt(&root, &policy);
    let valid = receipt
        .get("valid")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    print_json(receipt)?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

#[derive(Clone)]
struct DeterminismSurface {
    id: String,
    command: String,
    surface_kind: String,
    receipt_command: String,
    required_replay_cases: Vec<String>,
    required_replay_groups: Vec<String>,
    determinism_claim: String,
}

fn determinism_coverage_audit_receipt(root: &Path, policy: &Policy) -> Value {
    let current_policy_hash = canonical_json_hash(&policy.raw);
    let surfaces = determinism_surfaces_from_policy(policy).unwrap_or_default();
    let cases = replay_cases_from_policy(policy).unwrap_or_default();
    let groups = replay_groups_from_policy(policy).unwrap_or_default();
    let (covered_surfaces, missing_surfaces, mut issues) =
        audit_determinism_surface_coverage(&surfaces, &cases, &groups);
    issues.sort_by_key(canonical_json_hash);
    let surface_inventory = determinism_surfaces_json(&surfaces);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "determinism coverage-audit",
            "root": path_string(root),
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "current_replay_policy_hash": current_policy_hash,
            "surface_inventory_hash": canonical_json_hash(&json!(surface_inventory)),
            "surface_count": surfaces.len(),
            "covered_surfaces": covered_surfaces,
            "missing_surfaces": missing_surfaces,
        }),
    )
}

fn determinism_surfaces_from_policy(
    policy: &Policy,
) -> Result<Vec<DeterminismSurface>, Box<dyn Error>> {
    let surfaces = policy
        .raw
        .get("determinism_surface_inventory")
        .and_then(Value::as_array)
        .ok_or("policy missing determinism_surface_inventory")?;
    let mut parsed = surfaces
        .iter()
        .map(determinism_surface_from_policy_value)
        .collect::<Result<Vec<_>, _>>()?;
    parsed.sort_by_key(|surface| surface.id.clone());
    Ok(parsed)
}

fn determinism_surface_from_policy_value(
    value: &Value,
) -> Result<DeterminismSurface, Box<dyn Error>> {
    Ok(DeterminismSurface {
        id: required_string(value, "id")?.to_owned(),
        command: required_string(value, "command")?.to_owned(),
        surface_kind: required_string(value, "surface_kind")?.to_owned(),
        receipt_command: required_string(value, "receipt_command")?.to_owned(),
        required_replay_cases: required_string_array(value, "required_replay_cases")?,
        required_replay_groups: required_string_array(value, "required_replay_groups")?,
        determinism_claim: required_string(value, "determinism_claim")?.to_owned(),
    })
}

fn determinism_surfaces_json(surfaces: &[DeterminismSurface]) -> Vec<Value> {
    surfaces
        .iter()
        .map(|surface| {
            json!({
                "id": surface.id,
                "command": surface.command,
                "surface_kind": surface.surface_kind,
                "receipt_command": surface.receipt_command,
                "required_replay_cases": surface.required_replay_cases,
                "required_replay_groups": surface.required_replay_groups,
                "determinism_claim": surface.determinism_claim,
            })
        })
        .collect()
}

fn audit_determinism_surface_coverage(
    surfaces: &[DeterminismSurface],
    cases: &[ReplayCase],
    groups: &[ReplayGroup],
) -> (Vec<Value>, Vec<Value>, Vec<Value>) {
    let case_ids = cases
        .iter()
        .map(|case| case.id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let group_map = groups
        .iter()
        .map(|group| (group.id.as_str(), group))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut covered = Vec::new();
    let mut missing = Vec::new();
    let mut issues = Vec::new();
    for surface in surfaces {
        let mut surface_issues = Vec::new();
        for case_id in &surface.required_replay_cases {
            if !case_ids.contains(case_id.as_str()) {
                surface_issues.push(coverage_issue(
                    "coverage.required_case_missing",
                    &surface.id,
                    "required_replay_cases",
                    case_id,
                ));
            }
        }
        for group_id in &surface.required_replay_groups {
            match group_map.get(group_id.as_str()) {
                Some(group) => {
                    for case_id in &surface.required_replay_cases {
                        if !group.cases.iter().any(|candidate| candidate == case_id) {
                            surface_issues.push(coverage_issue(
                                "coverage.group_missing_case",
                                &surface.id,
                                group_id,
                                case_id,
                            ));
                        }
                    }
                }
                None => surface_issues.push(coverage_issue(
                    "coverage.required_group_missing",
                    &surface.id,
                    "required_replay_groups",
                    group_id,
                )),
            }
        }
        let surface_json = json!({
            "id": surface.id,
            "command": surface.command,
            "required_replay_cases": surface.required_replay_cases,
            "required_replay_groups": surface.required_replay_groups,
        });
        if surface_issues.is_empty() {
            covered.push(surface_json);
        } else {
            issues.extend(surface_issues);
            missing.push(surface_json);
        }
    }
    covered.sort_by(|left, right| left["id"].as_str().cmp(&right["id"].as_str()));
    missing.sort_by(|left, right| left["id"].as_str().cmp(&right["id"].as_str()));
    (covered, missing, issues)
}

fn coverage_issue(code: &str, surface_id: &str, field_path: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "surface_id": surface_id,
        "field_path": field_path,
        "expected": "policy replay coverage",
        "actual": actual,
    })
}

struct ReplayVerifyOptions {
    root: PathBuf,
    path: PathBuf,
}

fn run_determinism_verify_receipt(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = replay_verify_options(&args)?;
    let receipt = determinism_verify_replay_receipt(&options.root, &options.path);
    let valid = receipt
        .get("valid")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    print_json(receipt)?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

fn replay_verify_options(args: &[String]) -> Result<ReplayVerifyOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let path = args
        .iter()
        .find(|arg| !arg.starts_with("--") && *arg != &root)
        .ok_or("determinism verify-receipt requires <receipt.json>")?;
    Ok(ReplayVerifyOptions {
        root,
        path: PathBuf::from(path),
    })
}

fn determinism_verify_replay_receipt(root: &Path, path: &Path) -> Value {
    // r[impl cairn.supersede.replay_receipt_verification.contract]
    // r[impl cairn.supersede.replay_receipt_verification.cli]
    // r[impl cairn.supersede.replay_receipt_verification.validation]
    // r[impl cairn.supersede.replay_receipt_verification.receipt]
    let policy = match load_policy_file(default_policy_path().into()) {
        Ok(policy) => policy,
        Err(error) => {
            return replay_verification_receipt(
                root,
                path,
                vec![replay_verification_issue(
                    "policy.load_failed",
                    "policy",
                    "generated policy loads",
                    &error.to_string(),
                )],
                String::new(),
                String::new(),
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
            )
        }
    };
    let current_policy_hash = canonical_json_hash(&policy.raw);
    let all_cases = replay_cases_from_policy(&policy).unwrap_or_default();
    let groups = replay_groups_from_policy(&policy).unwrap_or_default();
    let entropy_inventory = replay_entropy_inventory_from_policy(&policy).unwrap_or_default();
    let environment_facts = replay_environment_facts_from_policy(&policy).unwrap_or_default();
    let expected_entropy_policy_hash = replay_entropy_policy_hash(&entropy_inventory);
    let expected_attestation_policy_hash = replay_attestation_policy_hash(&environment_facts);

    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            return replay_verification_receipt(
                root,
                path,
                vec![replay_verification_issue(
                    "receipt.read_failed",
                    "path",
                    "readable replay receipt",
                    &error.to_string(),
                )],
                current_policy_hash,
                String::new(),
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
            )
        }
    };
    let receipt: Value = match serde_json::from_str(&text) {
        Ok(value) => value,
        Err(error) => {
            return replay_verification_receipt(
                root,
                path,
                vec![replay_verification_issue(
                    "receipt.malformed_json",
                    "json",
                    "valid JSON replay receipt",
                    &error.to_string(),
                )],
                current_policy_hash,
                String::new(),
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
            )
        }
    };

    let mut issues = Vec::new();
    if receipt.get("command").and_then(Value::as_str) != Some("determinism replay") {
        issues.push(replay_verification_issue(
            "receipt.command_mismatch",
            "command",
            "determinism replay",
            receipt
                .get("command")
                .and_then(Value::as_str)
                .unwrap_or("<missing>"),
        ));
    }
    if receipt.get("valid").and_then(Value::as_bool) != Some(true)
        || receipt.get("verdict").and_then(Value::as_str) != Some("pass")
    {
        issues.push(replay_verification_issue(
            "receipt.replay_not_passing",
            "valid/verdict",
            "valid=true verdict=pass",
            "non-passing replay receipt",
        ));
    }

    let stated_receipt_hash = receipt
        .get("receipt_hash")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    if !is_blake3_hex_text(&stated_receipt_hash) {
        issues.push(replay_verification_issue(
            "receipt_hash.malformed",
            "receipt_hash",
            "BLAKE3 hex",
            stated_receipt_hash.as_str(),
        ));
    }
    let mut receipt_hash_input = receipt.clone();
    if let Some(object) = receipt_hash_input.as_object_mut() {
        object.remove("receipt_hash");
    }
    let replay_receipt_hash = portable_json_hash(root, &receipt_hash_input);
    if stated_receipt_hash != replay_receipt_hash {
        issues.push(replay_verification_issue(
            "receipt_hash.mismatch",
            "receipt_hash",
            &replay_receipt_hash,
            &stated_receipt_hash,
        ));
    }

    let selected_cases = receipt
        .get("selected_cases")
        .cloned()
        .unwrap_or(Value::Null);
    let selected_groups = receipt
        .get("selected_groups")
        .cloned()
        .unwrap_or(Value::Null);
    verify_replay_string_members(
        &mut issues,
        &selected_cases,
        "selected_cases",
        "selected_cases.invalid",
        all_cases.iter().map(|case| case.id.as_str()).collect(),
    );
    verify_replay_string_members(
        &mut issues,
        &selected_groups,
        "selected_groups",
        "selected_groups.invalid",
        groups.iter().map(|group| group.id.as_str()).collect(),
    );

    let entropy_policy_hash = receipt
        .get("entropy_policy_hash")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    if entropy_policy_hash != expected_entropy_policy_hash {
        issues.push(replay_verification_issue(
            "entropy_policy_hash.mismatch",
            "entropy_policy_hash",
            &expected_entropy_policy_hash,
            &entropy_policy_hash,
        ));
    }

    let attestation = receipt
        .get("environment_attestation")
        .cloned()
        .unwrap_or(Value::Null);
    let replay_policy_hash = attestation
        .get("generated_policy_hash")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    if replay_policy_hash != current_policy_hash {
        issues.push(replay_verification_issue(
            "replay_policy_hash.stale",
            "environment_attestation.generated_policy_hash",
            &current_policy_hash,
            &replay_policy_hash,
        ));
    }
    let environment_attestation_hash = attestation
        .get("attestation_hash")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    if !attestation.is_object() {
        issues.push(replay_verification_issue(
            "environment_attestation.missing",
            "environment_attestation",
            "object",
            "missing",
        ));
    } else {
        if !is_blake3_hex_text(&environment_attestation_hash) {
            issues.push(replay_verification_issue(
                "environment_attestation_hash.malformed",
                "environment_attestation.attestation_hash",
                "BLAKE3 hex",
                &environment_attestation_hash,
            ));
        }
        let mut attestation_hash_input = attestation.clone();
        if let Some(object) = attestation_hash_input.as_object_mut() {
            object.remove("attestation_hash");
        }
        let expected_attestation_hash = canonical_json_hash(&attestation_hash_input);
        if environment_attestation_hash != expected_attestation_hash {
            issues.push(replay_verification_issue(
                "environment_attestation_hash.mismatch",
                "environment_attestation.attestation_hash",
                &expected_attestation_hash,
                &environment_attestation_hash,
            ));
        }
        let stated_attestation_policy_hash = attestation
            .get("attestation_policy_hash")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if stated_attestation_policy_hash != expected_attestation_policy_hash {
            issues.push(replay_verification_issue(
                "attestation_policy_hash.mismatch",
                "environment_attestation.attestation_policy_hash",
                &expected_attestation_policy_hash,
                stated_attestation_policy_hash,
            ));
        }
        let stated_attestation_entropy_hash = attestation
            .get("entropy_policy_hash")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if stated_attestation_entropy_hash != expected_entropy_policy_hash {
            issues.push(replay_verification_issue(
                "attestation_entropy_policy_hash.mismatch",
                "environment_attestation.entropy_policy_hash",
                &expected_entropy_policy_hash,
                stated_attestation_entropy_hash,
            ));
        }
    }
    issues.sort_by_key(canonical_json_hash);

    replay_verification_receipt(
        root,
        path,
        issues,
        current_policy_hash,
        replay_receipt_hash,
        Value::String(stated_receipt_hash),
        Value::String(replay_policy_hash),
        selected_cases,
        selected_groups,
        Value::String(environment_attestation_hash),
    )
}

fn verify_replay_string_members(
    issues: &mut Vec<Value>,
    value: &Value,
    field_path: &str,
    code: &str,
    allowed: std::collections::BTreeSet<&str>,
) {
    let Some(items) = value.as_array() else {
        issues.push(replay_verification_issue(
            code,
            field_path,
            "array",
            "missing/non-array",
        ));
        return;
    };
    for item in items {
        let Some(text) = item.as_str() else {
            issues.push(replay_verification_issue(
                code,
                field_path,
                "string ids",
                "non-string",
            ));
            continue;
        };
        if !allowed.contains(text) {
            issues.push(replay_verification_issue(
                code,
                field_path,
                "known policy id",
                text,
            ));
        }
    }
}

fn replay_verification_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

#[allow(clippy::too_many_arguments)]
fn replay_verification_receipt(
    root: &Path,
    path: &Path,
    issues: Vec<Value>,
    current_replay_policy_hash: String,
    replay_receipt_hash: String,
    stated_replay_receipt_hash: Value,
    replay_policy_hash: Value,
    selected_cases: Value,
    selected_groups: Value,
    environment_attestation_hash: Value,
) -> Value {
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "determinism verify-receipt",
            "root": path_string(root),
            "path": portable_text(root, &path_string(path)),
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "current_replay_policy_hash": current_replay_policy_hash,
            "replay_policy_hash": replay_policy_hash,
            "replay_receipt_hash": replay_receipt_hash,
            "stated_replay_receipt_hash": stated_replay_receipt_hash,
            "selected_cases": selected_cases,
            "selected_groups": selected_groups,
            "environment_attestation_hash": environment_attestation_hash,
        }),
    )
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
            Value::Null,
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

#[derive(Clone)]
struct ReplayEnvironmentFact {
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
    let environment_facts = replay_environment_facts_from_policy(&policy)?;
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
    let selected_groups = selection.selected_groups.clone();
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
    let selected_cases = cases.iter().map(|case| case.id.clone()).collect::<Vec<_>>();
    let entropy_policy_hash = replay_entropy_policy_hash(&entropy_inventory);
    let environment_attestation = replay_environment_attestation(
        &policy,
        &environment_facts,
        &entropy_policy_hash,
        &selected_cases,
        &selected_groups,
        selected_case,
        selected_group,
    );
    let receipt = replay_receipt(
        root.clone(),
        case_receipts,
        mismatches,
        mutability_observations,
        skipped,
        nondeterminism_failures,
        available_cases,
        available_groups,
        selected_cases,
        selected_groups,
        replay_entropy_inventory_json(&entropy_inventory),
        replay_entropy_assumptions(&entropy_inventory),
        entropy_policy_hash,
        environment_attestation,
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

fn replay_environment_facts_from_policy(
    policy: &Policy,
) -> Result<Vec<ReplayEnvironmentFact>, Box<dyn Error>> {
    let entries = policy
        .raw
        .get("determinism_replay_policy")
        .and_then(|value| value.get("environment_attestation"))
        .and_then(Value::as_array)
        .ok_or("policy missing determinism_replay_policy.environment_attestation")?;
    let mut parsed = entries
        .iter()
        .map(replay_environment_fact_from_policy_value)
        .collect::<Result<Vec<_>, _>>()?;
    parsed.sort_by_key(|entry| entry.id.clone());
    Ok(parsed)
}

fn replay_environment_fact_from_policy_value(
    value: &Value,
) -> Result<ReplayEnvironmentFact, Box<dyn Error>> {
    Ok(ReplayEnvironmentFact {
        id: required_string(value, "id")?.to_owned(),
        class: required_string(value, "class")?.to_owned(),
        source: required_string(value, "source")?.to_owned(),
        handling: required_string(value, "handling")?.to_owned(),
        receipt_field: required_string(value, "receipt_field")?.to_owned(),
        description: required_string(value, "description")?.to_owned(),
    })
}

fn replay_environment_facts_json(entries: &[ReplayEnvironmentFact]) -> Vec<Value> {
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

fn replay_attestation_policy_hash(entries: &[ReplayEnvironmentFact]) -> String {
    canonical_json_hash(&json!(replay_environment_facts_json(entries)))
}

fn replay_environment_attestation(
    policy: &Policy,
    entries: &[ReplayEnvironmentFact],
    entropy_policy_hash: &str,
    selected_cases: &[String],
    selected_groups: &[String],
    selected_case: Option<&str>,
    selected_group: Option<&str>,
) -> Value {
    // r[impl cairn.supersede.replay_environment_attestation.policy]
    // r[impl cairn.supersede.replay_environment_attestation.receipt]
    let policy_facts = replay_environment_facts_json(entries);
    let attestation_policy_hash = replay_attestation_policy_hash(entries);
    let mut scope = json!({
        "selector": "default",
        "selected_case": Value::Null,
        "selected_group": Value::Null,
    });
    if let Some(case_id) = selected_case {
        scope["selector"] = Value::String("case".to_string());
        scope["selected_case"] = Value::String(case_id.to_string());
    }
    if let Some(group_id) = selected_group {
        scope["selector"] = Value::String("group".to_string());
        scope["selected_group"] = Value::String(group_id.to_string());
    }
    if selected_case.is_some() && selected_group.is_some() {
        scope["selector"] = Value::String("conflict".to_string());
    }
    let mut attestation = json!({
        "cairn_package": "cairn",
        "cairn_version": env!("CARGO_PKG_VERSION"),
        "generated_policy_hash": canonical_json_hash(&policy.raw),
        "entropy_policy_hash": entropy_policy_hash,
        "attestation_policy_hash": attestation_policy_hash,
        "selected_cases": selected_cases,
        "selected_groups": selected_groups,
        "replay_scope": scope,
        "policy_facts": policy_facts,
        "normalization_rules": ["canonical_json", "no_host_paths", "no_raw_environment_values"],
    });
    let attestation_hash = canonical_json_hash(&attestation);
    attestation["attestation_hash"] = Value::String(attestation_hash);
    attestation
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

#[allow(clippy::too_many_arguments)]
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
    environment_attestation: Value,
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
        "environment_attestation": environment_attestation,
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
    let coverage_audit = readiness_component("determinism_coverage_audit", || {
        let policy = load_policy_file(policy_path.clone())?;
        Ok(determinism_coverage_audit_receipt(&root, &policy))
    });
    let repository_gate_summary = repository_gate_summary();
    let replay_preflight = json!({
        "command": "determinism replay --root <root>",
        "included": false,
        "required": true,
        "reason": "workspace replay may copy fixtures and is an explicit operator/CI preflight",
    });
    let coverage_audit_valid = coverage_audit.check["passed"].clone();
    let mut checks = vec![
        validation.check,
        tracey.check,
        mcp_smoke.check,
        coverage_audit.check,
    ];
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
            "coverage_audit_valid": coverage_audit_valid,
            "coverage_audit_receipt_hash": coverage_audit.receipt_hash,
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
        "tracey_coverage" => "cairn tracey coverage --root <root>",
        "mcp_agent_smoke" => "cairn agent-receipt smoke-mcp --root <root>",
        "determinism_coverage_audit" => "cairn determinism coverage-audit --root <root>",
        _ => "unknown",
    }
}

fn tracey_coverage_receipt(root: PathBuf) -> Result<Value, Box<dyn Error>> {
    let report = tracey_coverage_report(&root)?;
    let stdout = if report["valid"].as_bool() == Some(true) {
        format!(
            "tracey bootstrap coverage ok: {}/{} referenced",
            report["referenced"].as_u64().unwrap_or(0),
            report["requirements"].as_u64().unwrap_or(0)
        )
    } else {
        serde_json::to_string_pretty(&report)?
    };
    let is_active_change_only = tracey_failure_is_active_release_readiness_change(&stdout);
    let is_valid = report["valid"].as_bool() == Some(true) || is_active_change_only;
    Ok(with_receipt_hash(json!({
        "command": "tracey coverage",
        "root": path_string(root),
        "valid": is_valid,
        "verdict": if is_valid { "pass" } else { "fail" },
        "stdout": stdout,
        "stderr": "",
        "exit_code": if report["valid"].as_bool() == Some(true) { 0 } else { 1 },
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
                    || id.starts_with("cairn.supersede.determinism_coverage_release_evidence")
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
    if args.first().map(String::as_str) == Some("export") {
        return run_policy_export(args.into_iter().skip(1).collect());
    }
    let policy_path = read_option(args, "--policy", default_policy_path());
    let policy = load_policy_file(policy_path)?;
    print_json(policy.raw)
}

fn run_policy_export(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.rust_nickel_only_gates.policy_export]
    let is_check = has_flag(&args, "--check");
    let default_source = default_policy_source_path();
    let source = read_option(
        args.clone(),
        "--source",
        default_source
            .to_str()
            .unwrap_or("cairn-policy/default.ncl"),
    );
    let output = read_option(args, "--output", default_policy_path());
    let rendered = export_policy_from_nickel(&source)?;
    check_invalid_policy_fixtures(source.parent().unwrap_or(Path::new(".")).join("fixtures"))?;
    if is_check {
        let existing = fs::read_to_string(&output).unwrap_or_default();
        if existing != rendered {
            return Err(format!("generated policy is stale: {}", output.display()).into());
        }
        println!("policy fresh: {}", output.display());
        return Ok(());
    }
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, rendered)?;
    println!("wrote {}", output.display());
    Ok(())
}

fn default_policy_source_path() -> PathBuf {
    PathBuf::from("cairn-policy/default.ncl")
}

fn export_policy_from_nickel(source: &Path) -> Result<String, Box<dyn Error>> {
    // r[verify cairn.supersede.rust_nickel_only_gates.no_python_scripts]
    let output = Command::new("nickel")
        .arg("export")
        .arg("--format")
        .arg("json")
        .arg(source)
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "nickel export failed for {}: {}{}",
            source.display(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    let value: Value = serde_json::from_slice(&output.stdout)?;
    validate_exported_policy(&value)?;
    Ok(serde_json::to_string_pretty(&value)?
        + "
")
}

fn validate_exported_policy(value: &Value) -> Result<(), Box<dyn Error>> {
    let policy = validate_policy_json(value.clone())?;
    require_policy_condition(
        value["receipt_hash_policy"]["algorithm"].as_str() == Some("BLAKE3"),
        "receipt hash algorithm must be BLAKE3",
    )?;
    require_policy_condition(
        value["receipt_hash_policy"]["canonicalization"].as_str()
            == Some("serde_json_compact_sorted_keys"),
        "receipt hash canonicalization must be serde_json_compact_sorted_keys",
    )?;
    for field in ["schemas", "lifecycle_stages", "receipt_schemas"] {
        require_policy_condition(
            value[field]
                .as_array()
                .is_some_and(|items| !items.is_empty()),
            &format!("{field} must be non-empty"),
        )?;
    }
    require_unique_policy_strings(
        json_string_array(&value["lifecycle_stages"]),
        "lifecycle stages",
    )?;
    let marker_policy = &value["task_marker_policy"];
    require_unique_policy_strings(
        json_string_array(&marker_policy["stages"]),
        "task marker stages",
    )?;
    let markers = marker_policy["markers"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    require_unique_policy_strings(
        markers
            .iter()
            .filter_map(|v| v["id"].as_str().map(ToOwned::to_owned))
            .collect(),
        "marker ids",
    )?;
    require_unique_policy_strings(
        markers
            .iter()
            .filter_map(|v| v["token"].as_str().map(ToOwned::to_owned))
            .collect(),
        "marker tokens",
    )?;
    for schema in &policy.receipt_schemas {
        require_unique_policy_strings(
            schema.top_level_keys.clone(),
            &format!("{} top-level receipt keys", schema.command),
        )?;
        require_unique_policy_strings(
            schema.action_keys.clone(),
            &format!("{} action keys", schema.command),
        )?;
        require_unique_policy_strings(
            schema.issue_keys.clone(),
            &format!("{} issue keys", schema.command),
        )?;
        for hash_field in &schema.hash_fields {
            require_policy_condition(
                schema.top_level_keys.iter().any(|key| key == hash_field),
                &format!(
                    "{} hash field {hash_field} is not a top-level key",
                    schema.command
                ),
            )?;
        }
    }
    validate_artifact_dependencies(value)?;
    validate_determinism_surface_inventory(&policy)?;
    Ok(())
}

fn check_invalid_policy_fixtures(fixtures: PathBuf) -> Result<(), Box<dyn Error>> {
    if !fixtures.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(fixtures)?.collect::<Result<Vec<_>, _>>()? {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        if !name.starts_with("invalid-")
            || path.extension().and_then(|value| value.to_str()) != Some("ncl")
        {
            continue;
        }
        let output = Command::new("nickel")
            .arg("export")
            .arg("--format")
            .arg("json")
            .arg(&path)
            .output()?;
        if !output.status.success() {
            continue;
        }
        let Ok(value) = serde_json::from_slice::<Value>(&output.stdout) else {
            continue;
        };
        if validate_exported_policy(&value).is_ok() {
            return Err(format!(
                "expected invalid policy fixture to fail: {}",
                path.display()
            )
            .into());
        }
    }
    Ok(())
}

fn require_policy_condition(condition: bool, message: &str) -> Result<(), Box<dyn Error>> {
    if condition {
        Ok(())
    } else {
        Err(format!("policy semantic check failed: {message}").into())
    }
}

fn require_unique_policy_strings(values: Vec<String>, label: &str) -> Result<(), Box<dyn Error>> {
    let mut seen = std::collections::BTreeSet::new();
    let mut duplicates = std::collections::BTreeSet::new();
    for value in values {
        if !seen.insert(value.clone()) {
            duplicates.insert(value);
        }
    }
    require_policy_condition(
        duplicates.is_empty(),
        &format!(
            "duplicate {label}: {}",
            duplicates.into_iter().collect::<Vec<_>>().join(", ")
        ),
    )
}

fn json_string_array(value: &Value) -> Vec<String> {
    value
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|v| v.as_str().map(ToOwned::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

fn validate_artifact_dependencies(value: &Value) -> Result<(), Box<dyn Error>> {
    for schema in value["schemas"].as_array().cloned().unwrap_or_default() {
        let artifacts = schema["artifacts"].as_array().cloned().unwrap_or_default();
        let artifact_ids = artifacts
            .iter()
            .filter_map(|artifact| artifact["id"].as_str().map(ToOwned::to_owned))
            .collect::<Vec<_>>();
        require_unique_policy_strings(
            artifact_ids.clone(),
            &format!(
                "artifact ids in schema {}",
                schema["id"].as_str().unwrap_or("<unknown>")
            ),
        )?;
        let known = artifact_ids
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>();
        for artifact in artifacts {
            for required in json_string_array(&artifact["requires"]) {
                require_policy_condition(
                    known.contains(&required),
                    &format!(
                        "artifact {} requires unknown artifact {required}",
                        artifact["id"].as_str().unwrap_or("<unknown>")
                    ),
                )?;
            }
        }
    }
    Ok(())
}

fn validate_determinism_surface_inventory(policy: &Policy) -> Result<(), Box<dyn Error>> {
    let cases = replay_cases_from_policy(policy)?
        .into_iter()
        .map(|case| case.id)
        .collect::<std::collections::BTreeSet<_>>();
    let groups = replay_groups_from_policy(policy)?
        .into_iter()
        .map(|group| {
            (
                group.id,
                group
                    .cases
                    .into_iter()
                    .collect::<std::collections::BTreeSet<_>>(),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let surfaces = determinism_surfaces_from_policy(policy)?;
    require_policy_condition(
        !surfaces.is_empty(),
        "determinism_surface_inventory must be non-empty",
    )?;
    require_unique_policy_strings(
        surfaces.iter().map(|surface| surface.id.clone()).collect(),
        "determinism surface ids",
    )?;
    for surface in surfaces {
        require_policy_condition(
            !surface.required_replay_cases.is_empty(),
            &format!(
                "determinism surface {} requires at least one replay case",
                surface.id
            ),
        )?;
        require_policy_condition(
            !surface.required_replay_groups.is_empty(),
            &format!(
                "determinism surface {} requires at least one replay group",
                surface.id
            ),
        )?;
        for case_id in &surface.required_replay_cases {
            require_policy_condition(
                cases.contains(case_id),
                &format!(
                    "determinism surface {} references unknown replay case {case_id}",
                    surface.id
                ),
            )?;
        }
        for group_id in &surface.required_replay_groups {
            let Some(group_cases) = groups.get(group_id) else {
                return Err(format!("policy semantic check failed: determinism surface {} references unknown replay group {group_id}", surface.id).into());
            };
            let missing = surface
                .required_replay_cases
                .iter()
                .filter(|case_id| !group_cases.contains(*case_id))
                .cloned()
                .collect::<Vec<_>>();
            require_policy_condition(
                missing.is_empty(),
                &format!(
                    "determinism surface {} group {group_id} missing cases: {}",
                    surface.id,
                    missing.join(", ")
                ),
            )?;
        }
    }
    Ok(())
}

fn run_tracey(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.rust_nickel_only_gates.tracey_coverage]
    let Some(command) = args.first().map(String::as_str) else {
        return Err("tracey requires a command: coverage".into());
    };
    if command != "coverage" {
        return Err(format!("unknown tracey command: {command}").into());
    }
    let rest = args.into_iter().skip(1).collect::<Vec<_>>();
    let root = read_option(rest.clone(), "--root", ".");
    let is_json = has_flag(&rest, "--json");
    let report = tracey_coverage_report(&root)?;
    if is_json || report["valid"].as_bool() != Some(true) {
        print_json(report.clone())?;
    } else {
        println!(
            "tracey bootstrap coverage ok: {}/{} referenced",
            report["referenced"].as_u64().unwrap_or(0),
            report["requirements"].as_u64().unwrap_or(0)
        );
    }
    if report["valid"].as_bool() == Some(true) {
        Ok(())
    } else {
        Err("tracey coverage failed".into())
    }
}

fn tracey_coverage_report(root: &Path) -> Result<Value, Box<dyn Error>> {
    let requirements = collect_tracey_requirements(root)?;
    let references = collect_tracey_references(root)?;
    let missing = requirements
        .keys()
        .filter(|req_id| !references.contains_key(*req_id))
        .cloned()
        .collect::<Vec<_>>();
    let dangling = references
        .keys()
        .filter(|req_id| !requirements.contains_key(*req_id))
        .cloned()
        .collect::<Vec<_>>();
    Ok(json!({
        "requirements": requirements.len(),
        "referenced": requirements.keys().filter(|req_id| references.contains_key(*req_id)).count(),
        "missing": missing,
        "dangling": dangling,
        "valid": missing.is_empty() && dangling.is_empty(),
    }))
}

fn collect_tracey_requirements(
    root: &Path,
) -> Result<std::collections::BTreeMap<String, Vec<String>>, Box<dyn Error>> {
    let mut requirements = std::collections::BTreeMap::new();
    for path in collect_files(root)? {
        let relative = safe_relative_path(root, &path);
        if !(relative.starts_with("openspec/specs/") || relative.starts_with("cairn/specs/"))
            || !relative.ends_with(".md")
        {
            continue;
        }
        let text = fs::read_to_string(&path)?;
        for (line_index, line) in text.lines().enumerate() {
            if let Some(req_id) = tracey_spec_marker(line.trim()) {
                requirements
                    .entry(req_id)
                    .or_insert_with(Vec::new)
                    .push(format!("{relative}:{}", line_index + 1));
            }
        }
    }
    Ok(requirements)
}

fn collect_tracey_references(
    root: &Path,
) -> Result<std::collections::BTreeMap<String, Vec<Value>>, Box<dyn Error>> {
    let mut references = std::collections::BTreeMap::new();
    for path in collect_files(root)? {
        let relative = safe_relative_path(root, &path);
        if !(relative.starts_with("crates/") || relative.starts_with("tools/"))
            || !(relative.ends_with(".rs") || relative.ends_with(".ncl"))
        {
            continue;
        }
        let text = fs::read_to_string(&path)?;
        for (line_index, line) in text.lines().enumerate() {
            for (verb, req_id) in tracey_ref_markers(line) {
                if req_id == "..." {
                    continue;
                }
                references
                    .entry(req_id)
                    .or_insert_with(Vec::new)
                    .push(json!({
                        "verb": verb,
                        "path": relative,
                        "line": (line_index + 1).to_string(),
                    }));
            }
        }
    }
    Ok(references)
}

fn tracey_spec_marker(line: &str) -> Option<String> {
    let rest = line.strip_prefix("r[")?;
    let end = rest.find(']')?;
    let marker = &rest[..end];
    let id = marker.split('+').next().unwrap_or(marker);
    if is_tracey_id(id) {
        Some(id.to_string())
    } else {
        None
    }
}

fn tracey_ref_markers(line: &str) -> Vec<(String, String)> {
    let mut markers = Vec::new();
    let mut start = 0usize;
    while let Some(offset) = line[start..].find("r[") {
        let marker_start = start + offset + 2;
        let Some(end_offset) = line[marker_start..].find(']') else {
            break;
        };
        let marker = &line[marker_start..marker_start + end_offset];
        let mut parts = marker.split_whitespace();
        let verb = parts.next().unwrap_or_default();
        let raw_id = parts.next().unwrap_or_default();
        let req_id = raw_id.split('+').next().unwrap_or(raw_id);
        if matches!(verb, "impl" | "verify" | "depends" | "related") && is_tracey_id(req_id) {
            markers.push((verb.to_string(), req_id.to_string()));
        }
        start = marker_start + end_offset + 1;
    }
    markers
}

fn is_tracey_id(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'-'))
}

fn run_rename_classification(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[verify cairn.supersede.rename_cairn.classification]
    // r[impl cairn.supersede.rust_nickel_only_gates.rename_classification]
    let output = read_string_option(&args, "--output").map(PathBuf::from);
    let root = PathBuf::from(".");
    let occurrences = rename_classification_occurrences(&root)?;
    let mut buckets = std::collections::BTreeMap::<String, usize>::new();
    for occurrence in &occurrences {
        if let Some(bucket) = occurrence["bucket"].as_str() {
            *buckets.entry(bucket.to_string()).or_insert(0) += 1;
        }
    }
    let report = json!({
        "legacy": "ironspec",
        "canonical": "cairn",
        "total": occurrences.len(),
        "buckets": buckets,
        "occurrences": occurrences,
        "valid": buckets.get("needs-fix").copied().unwrap_or(0) == 0,
    });
    let rendered = serde_json::to_string_pretty(&report)? + "\n";
    if let Some(path) = output {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, &rendered)?;
    }
    print!("{rendered}");
    if report["valid"].as_bool() == Some(true) {
        Ok(())
    } else {
        Err("rename classification found needs-fix occurrences".into())
    }
}

fn rename_classification_occurrences(root: &Path) -> Result<Vec<Value>, Box<dyn Error>> {
    let mut occurrences = Vec::new();
    for path in collect_files(root)? {
        let relative = safe_relative_path(root, &path);
        if !rename_classification_is_text_file(&relative)
            || rename_classification_is_skipped(&relative)
        {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let mut in_rename_classifier_impl = false;
        for (line_index, line) in text.lines().enumerate() {
            if relative == "crates/cairn-cli/src/main.rs"
                && line.starts_with("fn run_rename_classification")
            {
                in_rename_classifier_impl = true;
            }
            if relative == "crates/cairn-cli/src/main.rs"
                && line.starts_with("fn safe_relative_path")
            {
                in_rename_classifier_impl = false;
            }
            if in_rename_classifier_impl {
                continue;
            }
            if line.to_ascii_lowercase().contains("ironspec") {
                let bucket = rename_classification_bucket(&relative, line);
                occurrences.push(json!({
                    "path": relative,
                    "line": line_index + 1,
                    "bucket": bucket,
                    "text": line.trim(),
                }));
            }
        }
        if relative.to_ascii_lowercase().contains("ironspec") {
            let bucket = rename_classification_bucket(&relative, &relative);
            occurrences.push(json!({
                "path": relative,
                "line": 0,
                "bucket": bucket,
                "text": "<path>",
            }));
        }
    }
    occurrences.sort_by_key(|value| {
        format!(
            "{}:{:09}:{}:{}",
            value["path"].as_str().unwrap_or_default(),
            value["line"].as_u64().unwrap_or(0),
            value["bucket"].as_str().unwrap_or_default(),
            value["text"].as_str().unwrap_or_default()
        )
    });
    Ok(occurrences)
}

fn rename_classification_is_skipped(relative: &str) -> bool {
    relative
        .split('/')
        .any(|part| matches!(part, ".git" | "target" | "artifacts" | ".direnv"))
        || relative.ends_with("evidence/rename-classification.json")
}

fn rename_classification_is_text_file(relative: &str) -> bool {
    let suffixes = [
        ".md", ".rs", ".toml", ".nix", ".json", ".ncl", ".lock", ".txt", ".styx",
    ];
    relative == "Cargo.lock"
        || relative == "README.md"
        || suffixes.iter().any(|suffix| relative.ends_with(suffix))
}

fn rename_classification_bucket(path: &str, line: &str) -> &'static str {
    let text = line.to_ascii_lowercase();
    let path_text = path.to_ascii_lowercase();
    if path_text.contains("/archive/") || path_text.contains("openspec/changes/archive/") {
        return "historical-archive";
    }
    if path_text.starts_with("openspec/specs/") {
        return "requirement-id-history";
    }
    if path_text.contains("rename-ironspec-to-cairn")
        || path_text == "tools/check-rename-classification.rs"
    {
        return "intentional-reference";
    }
    if path_text.contains("ironspec")
        && (path_text.contains("docs/hermes")
            || path_text.contains("tool")
            || path_text.contains("src/ironspec.rs")
            || path_text.contains("ironspec-supersession"))
    {
        return "intentional-reference";
    }
    if text.contains("r[") && text.contains("ironspec.") {
        return "requirement-id-history";
    }
    if text.contains("ironspec") && text.contains("cairn") {
        return "intentional-reference";
    }
    let compatibility_tokens = [
        "legacy",
        "alias",
        "compat",
        "migration source",
        "migrate ironspec",
        "migrate_ironspec",
        "legacyironspec",
        "build_ironspec_migration_plan",
        "cargo_bin_exe_ironspec",
        "ironspec_archive_date",
        "name = \"ironspec\"",
        "path = \"src/ironspec.rs\"",
        "bin/ironspec",
        "invoked_as.ends_with(\"ironspec\")",
        "source_layout\"] == \"ironspec\"",
        "dry_run[\"source_layout\"], \"ironspec\"",
        "source_layout\", \"ironspec\"",
        "migrate-ironspec",
        "ironspec = {",
        "\"ironspec\" =>",
        "\"ironspec\",",
        "[\"openspec\", \"ironspec\"]",
        "openspec or ironspec",
        "<openspec|ironspec>",
    ];
    if text.contains("ironspec")
        && compatibility_tokens
            .iter()
            .any(|token| text.contains(token))
    {
        return "compatibility-alias";
    }
    "needs-fix"
}

fn safe_relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
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
        "verify-bundle" => run_agent_receipt_verify_bundle(iter.collect()),
        "smoke-mcp" => {
            let rest = iter.collect::<Vec<_>>();
            let root = read_option(rest, "--root", ".");
            print_json(mcp_agent_smoke_receipt(root)?)
        }
        unknown => Err(format!("unknown agent-receipt subcommand: {unknown}").into()),
    }
}

struct AgentEvidenceBundleOptions {
    root: PathBuf,
    path: PathBuf,
}

fn run_agent_receipt_verify_bundle(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = agent_evidence_bundle_options(&args)?;
    let receipt = agent_evidence_bundle_verification_receipt(&options.root, &options.path);
    let valid = receipt
        .get("valid")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    print_json(receipt)?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

fn agent_evidence_bundle_options(
    args: &[String],
) -> Result<AgentEvidenceBundleOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let path = args
        .iter()
        .find(|arg| !arg.starts_with("--") && *arg != &root)
        .ok_or("agent-receipt verify-bundle requires <bundle.json>")?;
    Ok(AgentEvidenceBundleOptions {
        root,
        path: PathBuf::from(path),
    })
}

fn agent_evidence_bundle_verification_receipt(root: &Path, bundle_path: &Path) -> Value {
    // r[impl cairn.supersede.agent_evidence_bundle_verification.cli]
    // r[impl cairn.supersede.agent_evidence_bundle_verification.validation]
    // r[impl cairn.supersede.agent_evidence_bundle_verification.receipt]
    // r[impl cairn.supersede.agent_evidence_bundle_verification.secret_safe]
    let policy = match load_policy_file(default_policy_path().into()) {
        Ok(policy) => policy,
        Err(_) => {
            return agent_bundle_receipt(
                root,
                vec![agent_bundle_issue(
                    "policy.load_failed",
                    "policy",
                    "generated policy loads",
                    "failed",
                )],
                String::new(),
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
            )
        }
    };
    let current_policy_hash = canonical_json_hash(&policy.raw);
    let bundle = match read_json_artifact(bundle_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            return agent_bundle_receipt(
                root,
                vec![agent_bundle_issue(
                    "bundle.read_failed",
                    "bundle",
                    "readable JSON object",
                    "unreadable",
                )],
                current_policy_hash,
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
            )
        }
    };
    let mut issues = Vec::new();
    if !bundle.is_object() {
        issues.push(agent_bundle_issue(
            "bundle.not_object",
            "bundle",
            "object",
            "non-object",
        ));
    }
    let bundle_base = bundle_path.parent().unwrap_or_else(|| Path::new("."));
    let agent_path = bundle_artifact_path(&bundle, bundle_base, "agent_receipt_path", &mut issues);
    let replay_path =
        bundle_artifact_path(&bundle, bundle_base, "replay_receipt_path", &mut issues);
    let replay_verify_path = bundle_artifact_path(
        &bundle,
        bundle_base,
        "replay_verification_path",
        &mut issues,
    );
    let release_path =
        optional_bundle_artifact_path(&bundle, bundle_base, "release_readiness_path", &mut issues);
    let coverage_audit_path =
        optional_bundle_artifact_path(&bundle, bundle_base, "coverage_audit_path", &mut issues);

    let mut agent_receipt_hash = Value::Null;
    let mut replay_receipt_hash = Value::Null;
    let mut replay_verification_receipt_hash = Value::Null;
    let mut environment_attestation_hash = Value::Null;
    let mut release_readiness_receipt_hash = Value::Null;
    let mut coverage_audit_receipt_hash = Value::Null;

    let mut agent_receipt = Value::Null;
    if let Some(path) = agent_path.as_deref() {
        match read_agent_receipt_file(path) {
            Ok(receipt) => {
                agent_receipt = receipt;
                let validation = agent_receipt_validation_receipt(
                    "agent-receipt validate",
                    None,
                    &agent_receipt,
                    false,
                    &policy,
                );
                agent_receipt_hash = validation["agent_receipt_hash"].clone();
                if validation["valid"].as_bool() != Some(true) {
                    issues.push(agent_bundle_issue(
                        "agent_receipt.invalid",
                        "agent_receipt",
                        "valid agent receipt",
                        "invalid",
                    ));
                }
            }
            Err(_) => issues.push(agent_bundle_issue(
                "agent_receipt.read_failed",
                "agent_receipt_path",
                "readable agent receipt",
                "unreadable",
            )),
        }
    }

    let mut replay_verifier = Value::Null;
    if let Some(path) = replay_path.as_deref() {
        replay_verifier = determinism_verify_replay_receipt(root, path);
        replay_receipt_hash = replay_verifier["replay_receipt_hash"].clone();
        environment_attestation_hash = replay_verifier["environment_attestation_hash"].clone();
        if replay_verifier["valid"].as_bool() != Some(true) {
            issues.push(agent_bundle_issue(
                "replay_receipt.invalid",
                "replay_receipt",
                "valid saved replay receipt",
                "invalid",
            ));
        }
    }

    let mut saved_replay_verification = Value::Null;
    if let Some(path) = replay_verify_path.as_deref() {
        match read_json_artifact(path, AGENT_RECEIPT_MAX_BYTES) {
            Ok(value) => {
                saved_replay_verification = value;
                replay_verification_receipt_hash =
                    receipt_stated_or_computed_hash(root, &saved_replay_verification);
                if saved_replay_verification["valid"].as_bool() != Some(true) {
                    issues.push(agent_bundle_issue(
                        "replay_verification.invalid",
                        "replay_verification",
                        "valid replay verification receipt",
                        "invalid",
                    ));
                }
            }
            Err(_) => issues.push(agent_bundle_issue(
                "replay_verification.read_failed",
                "replay_verification_path",
                "readable replay verification receipt",
                "unreadable",
            )),
        }
    }

    verify_optional_bundle_hash(
        &mut issues,
        &bundle,
        "agent_receipt_hash",
        &agent_receipt_hash,
    );
    verify_optional_bundle_hash(
        &mut issues,
        &bundle,
        "replay_receipt_hash",
        &replay_receipt_hash,
    );
    verify_optional_bundle_hash(
        &mut issues,
        &bundle,
        "replay_verification_receipt_hash",
        &replay_verification_receipt_hash,
    );

    if saved_replay_verification.is_object() && replay_verifier.is_object() {
        compare_bundle_field(
            &mut issues,
            "replay_verification.replay_receipt_hash_mismatch",
            "replay_verification.replay_receipt_hash",
            &replay_verifier["replay_receipt_hash"],
            &saved_replay_verification["replay_receipt_hash"],
        );
        compare_bundle_field(
            &mut issues,
            "replay_verification.policy_hash_mismatch",
            "replay_verification.replay_policy_hash",
            &replay_verifier["replay_policy_hash"],
            &saved_replay_verification["replay_policy_hash"],
        );
        compare_bundle_field(
            &mut issues,
            "replay_verification.attestation_hash_mismatch",
            "replay_verification.environment_attestation_hash",
            &replay_verifier["environment_attestation_hash"],
            &saved_replay_verification["environment_attestation_hash"],
        );
    }

    if agent_receipt.is_object() && replay_verifier.is_object() {
        let evidence = agent_receipt.get("replay_evidence").unwrap_or(&Value::Null);
        compare_bundle_field(
            &mut issues,
            "agent_replay_evidence.replay_hash_mismatch",
            "agent_receipt.replay_evidence.replay_receipt_hash",
            &replay_verifier["replay_receipt_hash"],
            evidence.get("replay_receipt_hash").unwrap_or(&Value::Null),
        );
        compare_bundle_field(
            &mut issues,
            "agent_replay_evidence.policy_hash_mismatch",
            "agent_receipt.replay_evidence.replay_policy_hash",
            &Value::String(current_policy_hash.clone()),
            evidence.get("replay_policy_hash").unwrap_or(&Value::Null),
        );
        compare_bundle_field(
            &mut issues,
            "agent_replay_evidence.attestation_hash_mismatch",
            "agent_receipt.replay_evidence.replay_environment_attestation_hash",
            &replay_verifier["environment_attestation_hash"],
            evidence
                .get("replay_environment_attestation_hash")
                .unwrap_or(&Value::Null),
        );
        let groups = evidence
            .get("replay_selected_groups")
            .and_then(Value::as_array)
            .map(|groups| groups.iter().filter_map(Value::as_str).collect::<Vec<_>>())
            .unwrap_or_default();
        if !groups.contains(&REQUIRED_AGENT_REPLAY_GROUP) {
            issues.push(agent_bundle_issue(
                "agent_replay_evidence.missing_group_coverage",
                "agent_receipt.replay_evidence.replay_selected_groups",
                REQUIRED_AGENT_REPLAY_GROUP,
                "missing",
            ));
        }
    }

    if let Some(path) = release_path.as_deref() {
        match read_json_artifact(path, AGENT_RECEIPT_MAX_BYTES) {
            Ok(value) => {
                release_readiness_receipt_hash = receipt_stated_or_computed_hash(root, &value);
                if value["valid"].as_bool() != Some(true) {
                    issues.push(agent_bundle_issue(
                        "release_readiness.invalid",
                        "release_readiness",
                        "valid release-readiness receipt",
                        "invalid",
                    ));
                }
            }
            Err(_) => issues.push(agent_bundle_issue(
                "release_readiness.read_failed",
                "release_readiness_path",
                "readable release-readiness receipt",
                "unreadable",
            )),
        }
        verify_optional_bundle_hash(
            &mut issues,
            &bundle,
            "release_readiness_receipt_hash",
            &release_readiness_receipt_hash,
        );
    }

    if let Some(path) = coverage_audit_path.as_deref() {
        match read_json_artifact(path, AGENT_RECEIPT_MAX_BYTES) {
            Ok(value) => {
                coverage_audit_receipt_hash = receipt_stated_or_computed_hash(root, &value);
                if value["valid"].as_bool() != Some(true) {
                    issues.push(agent_bundle_issue(
                        "coverage_audit.invalid",
                        "coverage_audit",
                        "valid coverage-audit receipt",
                        "invalid",
                    ));
                }
            }
            Err(_) => issues.push(agent_bundle_issue(
                "coverage_audit.read_failed",
                "coverage_audit_path",
                "readable coverage-audit receipt",
                "unreadable",
            )),
        }
        verify_optional_bundle_hash(
            &mut issues,
            &bundle,
            "coverage_audit_receipt_hash",
            &coverage_audit_receipt_hash,
        );
    }

    issues.sort_by_key(canonical_json_hash);
    agent_bundle_receipt(
        root,
        issues,
        current_policy_hash,
        agent_receipt_hash,
        replay_receipt_hash,
        replay_verification_receipt_hash,
        environment_attestation_hash,
        release_readiness_receipt_hash,
        coverage_audit_receipt_hash,
    )
}

fn read_json_artifact(path: &Path, max_bytes: u64) -> Result<Value, Box<dyn Error>> {
    let metadata = fs::metadata(path)?;
    if metadata.len() > max_bytes {
        return Err("artifact exceeds bounded read limit".into());
    }
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn bundle_artifact_path(
    bundle: &Value,
    base: &Path,
    field: &str,
    issues: &mut Vec<Value>,
) -> Option<PathBuf> {
    match bundle.get(field).and_then(Value::as_str) {
        Some(text) if !text.is_empty() => {
            let path = PathBuf::from(text);
            Some(if path.is_absolute() {
                path
            } else {
                base.join(path)
            })
        }
        _ => {
            issues.push(agent_bundle_issue(
                "bundle.missing_field",
                field,
                "non-empty string",
                "missing",
            ));
            None
        }
    }
}

fn optional_bundle_artifact_path(
    bundle: &Value,
    base: &Path,
    field: &str,
    issues: &mut Vec<Value>,
) -> Option<PathBuf> {
    match bundle.get(field) {
        None | Some(Value::Null) => None,
        Some(value) => match value.as_str() {
            Some(text) if !text.is_empty() => {
                let path = PathBuf::from(text);
                Some(if path.is_absolute() {
                    path
                } else {
                    base.join(path)
                })
            }
            _ => {
                issues.push(agent_bundle_issue(
                    "bundle.invalid_field",
                    field,
                    "non-empty string",
                    "invalid",
                ));
                None
            }
        },
    }
}

fn receipt_stated_or_computed_hash(root: &Path, receipt: &Value) -> Value {
    if let Some(hash) = receipt.get("receipt_hash").and_then(Value::as_str) {
        return Value::String(hash.to_string());
    }
    let mut input = receipt.clone();
    if let Some(object) = input.as_object_mut() {
        object.remove("receipt_hash");
    }
    Value::String(portable_json_hash(root, &input))
}

fn verify_optional_bundle_hash(
    issues: &mut Vec<Value>,
    bundle: &Value,
    field: &str,
    actual: &Value,
) {
    let Some(expected) = bundle.get(field) else {
        return;
    };
    if expected != actual {
        issues.push(agent_bundle_issue(
            "bundle.expected_hash_mismatch",
            field,
            expected.as_str().unwrap_or("hash"),
            actual.as_str().unwrap_or("missing"),
        ));
    }
}

fn compare_bundle_field(
    issues: &mut Vec<Value>,
    code: &str,
    field_path: &str,
    expected: &Value,
    actual: &Value,
) {
    if expected != actual {
        issues.push(agent_bundle_issue(
            code,
            field_path,
            expected.as_str().unwrap_or("value"),
            actual.as_str().unwrap_or("missing"),
        ));
    }
}

fn agent_bundle_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

#[allow(clippy::too_many_arguments)]
fn agent_bundle_receipt(
    root: &Path,
    issues: Vec<Value>,
    current_replay_policy_hash: String,
    agent_receipt_hash: Value,
    replay_receipt_hash: Value,
    replay_verification_receipt_hash: Value,
    environment_attestation_hash: Value,
    release_readiness_receipt_hash: Value,
    coverage_audit_receipt_hash: Value,
) -> Value {
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "agent-receipt verify-bundle",
            "root": path_string(root),
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "current_replay_policy_hash": current_replay_policy_hash,
            "required_replay_group": REQUIRED_AGENT_REPLAY_GROUP,
            "agent_receipt_hash": agent_receipt_hash,
            "replay_receipt_hash": replay_receipt_hash,
            "replay_verification_receipt_hash": replay_verification_receipt_hash,
            "environment_attestation_hash": environment_attestation_hash,
            "release_readiness_receipt_hash": release_readiness_receipt_hash,
            "coverage_audit_receipt_hash": coverage_audit_receipt_hash,
        }),
    )
}

struct EvidenceManifestOptions {
    root: PathBuf,
    path: PathBuf,
}

#[derive(Default)]
struct EvidenceExportOptions {
    root: PathBuf,
    out: Option<PathBuf>,
    agent_receipt: Option<PathBuf>,
}

#[derive(Default)]
struct EvidenceVerifyOptions {
    root: PathBuf,
    bundle: Option<PathBuf>,
}

#[derive(Default)]
struct EvidenceImportOptions {
    root: PathBuf,
    bundle: Option<PathBuf>,
    into: Option<PathBuf>,
}

#[derive(Default)]
struct EvidenceRepublishOptions {
    root: PathBuf,
    imported: Option<PathBuf>,
    out: Option<PathBuf>,
}

#[derive(Default)]
struct EvidenceIndexOptions {
    root: PathBuf,
    bundle: Option<PathBuf>,
    out: Option<PathBuf>,
}

#[derive(Default)]
struct EvidenceIndexVerifyOptions {
    root: PathBuf,
    index: Option<PathBuf>,
}

#[derive(Default)]
struct EvidenceRegistryOptions {
    root: PathBuf,
    indexes: Vec<PathBuf>,
    out: Option<PathBuf>,
    labels: EvidenceRegistryLabels,
}

#[derive(Default)]
struct EvidenceRegistryVerifyOptions {
    root: PathBuf,
    registry: Option<PathBuf>,
}

#[derive(Default)]
struct EvidenceRegistryMirrorVerifyOptions {
    root: PathBuf,
    registry: Option<PathBuf>,
    mirror: Option<PathBuf>,
}

#[derive(Default, Clone)]
struct EvidenceRegistryLabels {
    channel: Option<String>,
    release: Option<String>,
    platform: Option<String>,
    compatibility_class: Option<String>,
}

#[derive(Default, Clone)]
struct EvidenceRegistryConsumeSelectors {
    bundle_kind: Option<String>,
    policy_hash: Option<String>,
    channel: Option<String>,
    release: Option<String>,
    platform: Option<String>,
    compatibility_class: Option<String>,
    index_receipt_hash: Option<String>,
    index_verification_receipt_hash: Option<String>,
    bundle_receipt_hash: Option<String>,
    artifact_role: Option<String>,
    artifact_hash: Option<String>,
    coverage_audit_receipt_hash: Option<String>,
    release_readiness_receipt_hash: Option<String>,
}

#[derive(Default)]
struct EvidenceRegistrySelectOptions {
    root: PathBuf,
    registry: Option<PathBuf>,
    out: Option<PathBuf>,
    selectors: EvidenceRegistryConsumeSelectors,
}

#[derive(Default)]
struct EvidenceRegistryFetchPlanOptions {
    root: PathBuf,
    registry: Option<PathBuf>,
    out: Option<PathBuf>,
    selectors: EvidenceRegistryConsumeSelectors,
}

fn run_evidence(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("evidence requires a subcommand: export".into());
    };
    match subcommand.as_str() {
        "export" => run_evidence_export(iter.collect()),
        "verify" => run_evidence_verify(iter.collect()),
        "import" => run_evidence_import(iter.collect()),
        "republish" => run_evidence_republish(iter.collect()),
        "index" => run_evidence_index(iter.collect()),
        "registry" => run_evidence_registry(iter.collect()),
        "function-object" => run_evidence_function_object(iter.collect()),
        "name-pointer" => run_evidence_name_pointer(iter.collect()),
        "export-bundle" => run_evidence_export_bundle_receipt(iter.collect()),
        "downstream-pilot" => run_evidence_downstream_pilot(iter.collect()),
        unknown => Err(format!("unknown evidence subcommand: {unknown}").into()),
    }
}

fn run_evidence_export(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.evidence_export_bundle.command]
    // r[impl cairn.supersede.evidence_export_bundle.artifacts]
    // r[impl cairn.supersede.evidence_export_bundle.failures]
    let options = evidence_export_options(&args)?;
    let receipt = evidence_export_receipt(&options);
    let valid = receipt
        .get("valid")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    print_json(receipt)?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

fn run_evidence_verify(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.evidence_bundle_verify_command.command]
    // r[impl cairn.supersede.evidence_bundle_verify_command.artifacts]
    // r[impl cairn.supersede.evidence_bundle_verify_command.failures]
    let options = evidence_verify_options(&args)?;
    let receipt = evidence_verify_receipt(&options);
    let valid = receipt
        .get("valid")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    print_json(receipt)?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

fn run_evidence_import(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.evidence_import_consume_rail.command]
    // r[impl cairn.supersede.evidence_import_consume_rail.artifacts]
    // r[impl cairn.supersede.evidence_import_consume_rail.failures]
    let options = evidence_import_options(&args)?;
    let receipt = evidence_import_receipt(&options);
    let valid = receipt
        .get("valid")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    print_json(receipt)?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

fn run_evidence_index(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    if let Some(first) = iter.next() {
        if first == "verify" {
            return run_evidence_index_verify(iter.collect());
        }
        let mut rest = vec![first];
        rest.extend(iter);
        let options = evidence_index_options(&rest)?;
        let receipt = evidence_index_receipt(&options);
        let valid = receipt
            .get("valid")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        print_json(receipt)?;
        if !valid {
            std::process::exit(1);
        }
        return Ok(());
    }
    Err("evidence index requires --bundle <dir> --out <file> or verify".into())
}

fn run_evidence_index_verify(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = evidence_index_verify_options(&args)?;
    let receipt = evidence_index_verify_receipt(&options);
    let valid = receipt
        .get("valid")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    print_json(receipt)?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

fn evidence_index_options(args: &[String]) -> Result<EvidenceIndexOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let bundle = read_string_option(args, "--bundle").map(PathBuf::from);
    let out = read_string_option(args, "--out").map(PathBuf::from);
    if bundle.is_none() {
        return Err("evidence index requires --bundle <dir>".into());
    }
    if out.is_none() {
        return Err("evidence index requires --out <file>".into());
    }
    Ok(EvidenceIndexOptions { root, bundle, out })
}

fn evidence_index_verify_options(
    args: &[String],
) -> Result<EvidenceIndexVerifyOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let index = read_string_option(args, "--index").map(PathBuf::from);
    if index.is_none() {
        return Err("evidence index verify requires --index <file>".into());
    }
    Ok(EvidenceIndexVerifyOptions { root, index })
}

fn evidence_index_receipt(options: &EvidenceIndexOptions) -> Value {
    // r[impl cairn.supersede.evidence_publish_index_rail.command]
    // r[impl cairn.supersede.evidence_publish_index_rail.failures]
    let root = options.root.clone();
    let bundle = options.bundle.clone().unwrap_or_else(|| PathBuf::from("."));
    let out = options.out.clone().unwrap_or_else(|| PathBuf::from("."));
    let mut issues = Vec::new();
    let bundle_summary =
        evidence_index_bundle_summary(&root, &bundle, "evidence_index", &mut issues);
    let bundle_relative = evidence_index_bundle_relative(&bundle, &out);
    let mut receipt = evidence_index_final_receipt(
        &root,
        "evidence index",
        issues,
        bundle_summary,
        bundle_relative,
        json!({}),
    );
    if receipt.get("valid").and_then(Value::as_bool) == Some(true) {
        if let Some(parent) = out.parent() {
            if fs::create_dir_all(parent).is_err() {
                receipt = evidence_index_final_receipt(
                    &root,
                    "evidence index",
                    vec![evidence_index_issue(
                        "evidence_index.output_create_failed",
                        "out",
                        "writable output parent",
                        "failed",
                    )],
                    None,
                    Value::Null,
                    json!({}),
                );
            }
        }
        if receipt.get("valid").and_then(Value::as_bool) == Some(true)
            && fs::write(
                &out,
                serde_json::to_vec_pretty(&receipt).unwrap_or_default(),
            )
            .is_err()
        {
            receipt = evidence_index_final_receipt(
                &root,
                "evidence index",
                vec![evidence_index_issue(
                    "evidence_index.receipt_write_failed",
                    "out",
                    "writable index receipt",
                    "failed",
                )],
                None,
                Value::Null,
                json!({}),
            );
        }
    }
    receipt
}

fn evidence_index_verify_receipt(options: &EvidenceIndexVerifyOptions) -> Value {
    // r[impl cairn.supersede.evidence_publish_index_rail.verify]
    // r[impl cairn.supersede.evidence_publish_index_rail.failures]
    let root = options.root.clone();
    let index_path = options.index.clone().unwrap_or_else(|| PathBuf::from("."));
    let mut issues = Vec::new();
    let index = match read_json_artifact(&index_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            return evidence_index_verify_final_receipt(
                &root,
                issues_with_index_missing(),
                Value::Null,
                None,
            );
        }
    };
    let index_receipt_hash = index.get("receipt_hash").cloned().unwrap_or(Value::Null);
    if let Some(stated_hash) = index.get("receipt_hash").and_then(Value::as_str) {
        let mut hash_input = index.clone();
        if let Some(object) = hash_input.as_object_mut() {
            object.remove("receipt_hash");
        }
        if portable_json_hash(&root, &hash_input) != stated_hash {
            issues.push(evidence_index_issue(
                "evidence_index_verify.index_hash_mismatch",
                "receipt_hash",
                "portable receipt hash",
                "mismatch",
            ));
        }
    } else {
        issues.push(evidence_index_issue(
            "evidence_index_verify.index_hash_missing",
            "receipt_hash",
            "BLAKE3 hex",
            "missing",
        ));
    }
    let bundle = evidence_index_verify_bundle_path(&index_path, &index);
    let current_summary =
        evidence_index_bundle_summary(&root, &bundle, "evidence_index_verify", &mut issues);
    if let Some(summary) = current_summary.as_ref() {
        evidence_index_compare_summary(&mut issues, &index, summary);
    }
    evidence_index_verify_final_receipt(&root, issues, index_receipt_hash, current_summary)
}

fn issues_with_index_missing() -> Vec<Value> {
    vec![evidence_index_issue(
        "evidence_index_verify.index_unreadable",
        "index",
        "readable index receipt",
        "unreadable",
    )]
}

fn evidence_index_bundle_relative(bundle: &Path, out: &Path) -> Value {
    let Some(out_parent) = out.parent() else {
        return Value::Null;
    };
    if out_parent == bundle {
        return Value::Null;
    }
    if let (Some(bundle_parent), Some(bundle_name)) = (bundle.parent(), bundle.file_name()) {
        if bundle_parent == out_parent {
            return json!(bundle_name.to_string_lossy().to_string());
        }
    }
    Value::Null
}

fn evidence_index_verify_bundle_path(index_path: &Path, index: &Value) -> PathBuf {
    let parent = index_path.parent().unwrap_or_else(|| Path::new("."));
    if parent.join("evidence-republish-receipt.json").exists()
        || parent.join("evidence-import-receipt.json").exists()
        || parent.join("evidence-export-receipt.json").exists()
    {
        return parent.to_path_buf();
    }
    if let Some(relative) = index.get("bundle_relative").and_then(Value::as_str) {
        if !relative.is_empty() && !relative.starts_with('/') && !relative.contains("..") {
            return parent.join(relative);
        }
    }
    parent.to_path_buf()
}

fn evidence_index_bundle_summary(
    root: &Path,
    bundle: &Path,
    prefix: &'static str,
    issues: &mut Vec<Value>,
) -> Option<Value> {
    let receipt_files = [
        ("republished", "evidence-republish-receipt.json"),
        ("imported", "evidence-import-receipt.json"),
        ("exported", "evidence-export-receipt.json"),
    ];
    let mut present = Vec::new();
    for (kind, file) in receipt_files {
        if bundle.join(file).exists() {
            present.push((kind, file));
        }
    }
    if present.is_empty() {
        issues.push(evidence_index_issue(
            &format!("{prefix}.bundle_receipt_missing"),
            "bundle",
            "one canonical evidence receipt",
            "missing",
        ));
        return None;
    }
    let (kind, receipt_file) = present[0];
    let receipt_path = bundle.join(receipt_file);
    let receipt = match read_json_artifact(&receipt_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            issues.push(evidence_index_issue(
                &format!("{prefix}.bundle_receipt_unreadable"),
                receipt_file,
                "readable bundle receipt",
                "unreadable",
            ));
            return None;
        }
    };
    if receipt.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_index_issue(
            &format!("{prefix}.bundle_receipt_invalid"),
            receipt_file,
            "valid bundle receipt",
            "invalid",
        ));
    }
    let mut artifacts = receipt
        .get("artifacts")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    artifacts.sort_by_key(canonical_json_hash);
    for artifact in artifacts.iter() {
        let role = artifact
            .get("role")
            .and_then(Value::as_str)
            .unwrap_or("<missing>");
        let relative = artifact.get("path").and_then(Value::as_str).unwrap_or("");
        let declared_hash = artifact.get("hash").and_then(Value::as_str).unwrap_or("");
        let Some(path) = safe_bundle_artifact_path(bundle, relative) else {
            issues.push(evidence_index_issue(
                &format!("{prefix}.artifact_path_unsafe"),
                role,
                "safe relative artifact path",
                "unsafe",
            ));
            continue;
        };
        match fs::read(&path) {
            Ok(bytes) => {
                if blake3_hex(&bytes) != declared_hash {
                    issues.push(evidence_index_issue(
                        &format!("{prefix}.artifact_hash_mismatch"),
                        role,
                        "declared artifact hash",
                        "mismatch",
                    ));
                }
            }
            Err(_) => issues.push(evidence_index_issue(
                &format!("{prefix}.artifact_missing"),
                role,
                "declared artifact file",
                "missing",
            )),
        }
    }
    Some(json!({
        "bundle_kind": kind,
        "bundle_receipt": receipt_file,
        "bundle_receipt_hash": receipt.get("receipt_hash").cloned().unwrap_or(Value::Null),
        "receipt_chain": evidence_index_receipt_chain(&receipt),
        "artifacts": artifacts,
        "policy_hash": evidence_index_policy_hash(&receipt),
        "root_hash": portable_json_hash(root, &receipt),
    }))
}

fn evidence_index_receipt_chain(receipt: &Value) -> Value {
    let keys = [
        "receipt_hash",
        "export_receipt_hash",
        "verification_receipt_hash",
        "import_receipt_hash",
        "coverage_audit_receipt_hash",
        "release_readiness_receipt_hash",
    ];
    let mut object = serde_json::Map::new();
    for key in keys {
        if let Some(value) = receipt.get(key) {
            object.insert(key.to_string(), value.clone());
        }
    }
    Value::Object(object)
}

fn evidence_index_policy_hash(receipt: &Value) -> Value {
    receipt
        .get("current_replay_policy_hash")
        .or_else(|| receipt.get("replay_policy_hash"))
        .cloned()
        .unwrap_or(Value::Null)
}

fn evidence_index_compare_summary(issues: &mut Vec<Value>, index: &Value, summary: &Value) {
    for key in [
        "bundle_kind",
        "bundle_receipt_hash",
        "receipt_chain",
        "artifacts",
    ] {
        if index.get(key) != summary.get(key) {
            issues.push(evidence_index_issue(
                "evidence_index_verify.index_bundle_mismatch",
                key,
                "indexed bundle value",
                "mismatch",
            ));
        }
    }
}

fn evidence_index_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

fn evidence_index_final_receipt(
    root: &Path,
    command: &str,
    mut issues: Vec<Value>,
    bundle_summary: Option<Value>,
    bundle_relative: Value,
    extra: Value,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    let empty = json!({});
    let summary = bundle_summary.as_ref().unwrap_or(&empty);
    let mut receipt = json!({
        "command": command,
        "root": path_string(root),
        "bundle": "<bundle>",
        "bundle_relative": bundle_relative,
        "out": "<out>",
        "valid": valid,
        "verdict": if valid { "pass" } else { "fail" },
        "issues": issues,
        "bundle_kind": summary.get("bundle_kind").cloned().unwrap_or(Value::Null),
        "bundle_receipt": summary.get("bundle_receipt").cloned().unwrap_or(Value::Null),
        "bundle_receipt_hash": summary.get("bundle_receipt_hash").cloned().unwrap_or(Value::Null),
        "receipt_chain": summary.get("receipt_chain").cloned().unwrap_or_else(|| json!({})),
        "policy_hash": summary.get("policy_hash").cloned().unwrap_or(Value::Null),
        "artifacts": summary.get("artifacts").cloned().unwrap_or_else(|| json!([])),
    });
    if let Some(object) = receipt.as_object_mut() {
        if let Some(extra_object) = extra.as_object() {
            for (key, value) in extra_object {
                object.insert(key.clone(), value.clone());
            }
        }
    }
    with_portable_receipt_hash(root, receipt)
}

fn evidence_index_verify_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    index_receipt_hash: Value,
    current_summary: Option<Value>,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    let empty = json!({});
    let summary = current_summary.as_ref().unwrap_or(&empty);
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence index verify",
            "root": path_string(root),
            "index": "<index>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "index_receipt_hash": index_receipt_hash,
            "bundle_kind": summary.get("bundle_kind").cloned().unwrap_or(Value::Null),
            "bundle_receipt_hash": summary.get("bundle_receipt_hash").cloned().unwrap_or(Value::Null),
            "receipt_chain": summary.get("receipt_chain").cloned().unwrap_or_else(|| json!({})),
            "artifacts": summary.get("artifacts").cloned().unwrap_or_else(|| json!([])),
        }),
    )
}

fn run_evidence_registry(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("evidence registry requires build or verify".into());
    };
    let receipt = match subcommand.as_str() {
        "build" => {
            let options = evidence_registry_options(&iter.collect::<Vec<_>>())?;
            evidence_registry_receipt(&options)
        }
        "verify" => {
            let options = evidence_registry_verify_options(&iter.collect::<Vec<_>>())?;
            evidence_registry_verify_receipt(&options)
        }
        "select" => {
            let options = evidence_registry_select_options(&iter.collect::<Vec<_>>())?;
            evidence_registry_select_receipt(&options)
        }
        "fetch-plan" => {
            let options = evidence_registry_fetch_plan_options(&iter.collect::<Vec<_>>())?;
            evidence_registry_fetch_plan_receipt(&options)
        }
        "mirror-verify" => {
            let options = evidence_registry_mirror_verify_options(&iter.collect::<Vec<_>>())?;
            evidence_registry_mirror_verify_receipt(&options)
        }
        unknown => return Err(format!("unknown evidence registry subcommand: {unknown}").into()),
    };
    let valid = receipt
        .get("valid")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    print_json(receipt)?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

fn evidence_registry_options(args: &[String]) -> Result<EvidenceRegistryOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let indexes = read_repeated_path_options(args, "--index");
    let out = read_string_option(args, "--out").map(PathBuf::from);
    let labels = evidence_registry_labels_from_args(args);
    if indexes.is_empty() {
        return Err("evidence registry build requires at least one --index <file>".into());
    }
    if out.is_none() {
        return Err("evidence registry build requires --out <file>".into());
    }
    Ok(EvidenceRegistryOptions {
        root,
        indexes,
        out,
        labels,
    })
}

fn evidence_registry_verify_options(
    args: &[String],
) -> Result<EvidenceRegistryVerifyOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let registry = read_string_option(args, "--registry").map(PathBuf::from);
    if registry.is_none() {
        return Err("evidence registry verify requires --registry <file>".into());
    }
    Ok(EvidenceRegistryVerifyOptions { root, registry })
}

fn evidence_registry_mirror_verify_options(
    args: &[String],
) -> Result<EvidenceRegistryMirrorVerifyOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let registry = read_string_option(args, "--registry").map(PathBuf::from);
    let mirror = read_string_option(args, "--mirror").map(PathBuf::from);
    if registry.is_none() {
        return Err("evidence registry mirror-verify requires --registry <file>".into());
    }
    if mirror.is_none() {
        return Err("evidence registry mirror-verify requires --mirror <dir>".into());
    }
    Ok(EvidenceRegistryMirrorVerifyOptions {
        root,
        registry,
        mirror,
    })
}

fn evidence_registry_select_options(
    args: &[String],
) -> Result<EvidenceRegistrySelectOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let registry = read_string_option(args, "--registry").map(PathBuf::from);
    let out = read_string_option(args, "--out").map(PathBuf::from);
    if registry.is_none() {
        return Err("evidence registry select requires --registry <file>".into());
    }
    if out.is_none() {
        return Err("evidence registry select requires --out <file>".into());
    }
    Ok(EvidenceRegistrySelectOptions {
        root,
        registry,
        out,
        selectors: evidence_registry_consume_selectors(args),
    })
}

fn evidence_registry_fetch_plan_options(
    args: &[String],
) -> Result<EvidenceRegistryFetchPlanOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let registry = read_string_option(args, "--registry").map(PathBuf::from);
    let out = read_string_option(args, "--out").map(PathBuf::from);
    if registry.is_none() {
        return Err("evidence registry fetch-plan requires --registry <file>".into());
    }
    if out.is_none() {
        return Err("evidence registry fetch-plan requires --out <file>".into());
    }
    Ok(EvidenceRegistryFetchPlanOptions {
        root,
        registry,
        out,
        selectors: evidence_registry_consume_selectors(args),
    })
}

fn evidence_registry_consume_selectors(args: &[String]) -> EvidenceRegistryConsumeSelectors {
    EvidenceRegistryConsumeSelectors {
        bundle_kind: read_string_option(args, "--bundle-kind"),
        policy_hash: read_string_option(args, "--policy-hash"),
        channel: read_string_option(args, "--channel"),
        release: read_string_option(args, "--release"),
        platform: read_string_option(args, "--platform"),
        compatibility_class: read_string_option(args, "--compatibility-class"),
        index_receipt_hash: read_string_option(args, "--index-receipt-hash"),
        index_verification_receipt_hash: read_string_option(
            args,
            "--index-verification-receipt-hash",
        ),
        bundle_receipt_hash: read_string_option(args, "--bundle-receipt-hash"),
        artifact_role: read_string_option(args, "--artifact-role"),
        artifact_hash: read_string_option(args, "--artifact-hash"),
        coverage_audit_receipt_hash: read_string_option(args, "--coverage-audit-receipt-hash"),
        release_readiness_receipt_hash: read_string_option(
            args,
            "--release-readiness-receipt-hash",
        ),
    }
}

fn evidence_registry_labels_from_args(args: &[String]) -> EvidenceRegistryLabels {
    EvidenceRegistryLabels {
        channel: read_string_option(args, "--channel"),
        release: read_string_option(args, "--release"),
        platform: read_string_option(args, "--platform"),
        compatibility_class: read_string_option(args, "--compatibility-class"),
    }
}

fn evidence_registry_labels_json(labels: &EvidenceRegistryLabels) -> Value {
    json!({
        "channel": labels.channel.clone().unwrap_or_else(|| "unspecified".to_string()),
        "release": labels.release.clone().unwrap_or_else(|| "unspecified".to_string()),
        "platform": labels.platform.clone().unwrap_or_else(|| "unspecified".to_string()),
        "compatibility_class": labels
            .compatibility_class
            .clone()
            .unwrap_or_else(|| "unspecified".to_string()),
    })
}

fn evidence_registry_validate_labels(
    root: &Path,
    labels: &EvidenceRegistryLabels,
    prefix: &'static str,
    issues: &mut Vec<Value>,
) {
    let Ok(policy) = load_policy_file(default_policy_path().into()) else {
        issues.push(evidence_registry_issue(
            &format!("{prefix}.label_policy_unreadable"),
            "registry_label_policy",
            "readable generated policy",
            "unreadable",
        ));
        return;
    };
    let policy_hash = canonical_json_hash(&policy.raw);
    if policy_hash.is_empty() || root.as_os_str().is_empty() {
        issues.push(evidence_registry_issue(
            &format!("{prefix}.label_policy_hash_invalid"),
            "registry_label_policy",
            "BLAKE3 policy hash",
            "invalid",
        ));
    }
    for (field, value, policy_key) in [
        ("channel", labels.channel.as_deref(), "channels"),
        ("platform", labels.platform.as_deref(), "platforms"),
        (
            "compatibility_class",
            labels.compatibility_class.as_deref(),
            "compatibility_classes",
        ),
    ] {
        if let Some(value) = value {
            if evidence_registry_label_value_unsafe(value) {
                issues.push(evidence_registry_issue(
                    &format!("{prefix}.label_unsafe"),
                    field,
                    "stable non-path label value",
                    "unsafe",
                ));
                continue;
            }
            let allowed = registry_label_policy_values(&policy.raw, policy_key);
            if !allowed.iter().any(|allowed| allowed == value) {
                issues.push(evidence_registry_issue(
                    &format!("{prefix}.label_not_allowed"),
                    field,
                    "policy-owned allowed label",
                    value,
                ));
            }
        }
    }
    if let Some(release) = labels.release.as_deref() {
        if evidence_registry_label_value_unsafe(release) {
            issues.push(evidence_registry_issue(
                &format!("{prefix}.label_unsafe"),
                "release",
                "stable non-path label value",
                "unsafe",
            ));
        } else if !evidence_registry_release_label_valid(release) {
            issues.push(evidence_registry_issue(
                &format!("{prefix}.release_label_invalid"),
                "release",
                "YYYY-MM-DD release label",
                release,
            ));
        }
    }
    let label_count = [
        labels.channel.as_ref(),
        labels.release.as_ref(),
        labels.platform.as_ref(),
        labels.compatibility_class.as_ref(),
    ]
    .iter()
    .flatten()
    .count();
    if label_count > 0 && labels.channel.is_none() {
        issues.push(evidence_registry_issue(
            &format!("{prefix}.channel_required_for_labeled_registry"),
            "channel",
            "channel label when registry labels are present",
            "missing",
        ));
    }
    // r[impl cairn.supersede.registry_channel_metadata.policy]
}

fn registry_label_policy_values(policy: &Value, key: &str) -> Vec<String> {
    policy
        .get("registry_label_policy")
        .and_then(|policy| policy.get(key))
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn evidence_registry_label_value_unsafe(value: &str) -> bool {
    value.is_empty()
        || value.starts_with('/')
        || value.contains("..")
        || value.contains('\\')
        || value.contains(':')
        || value.chars().any(char::is_whitespace)
}

fn evidence_registry_release_label_valid(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(idx, byte)| idx == 4 || idx == 7 || byte.is_ascii_digit())
}

fn entry_labels_from_registry_entry(entry: &Value) -> Option<EvidenceRegistryLabels> {
    let labels = entry.get("labels")?;
    Some(EvidenceRegistryLabels {
        channel: registry_entry_label(labels, "channel"),
        release: registry_entry_label(labels, "release"),
        platform: registry_entry_label(labels, "platform"),
        compatibility_class: registry_entry_label(labels, "compatibility_class"),
    })
}

fn registry_entry_label(labels: &Value, key: &str) -> Option<String> {
    labels
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| *value != "unspecified")
        .map(str::to_string)
}

fn evidence_registry_label_key(entry: &Value) -> Option<String> {
    let labels = entry.get("labels")?;
    let channel = labels
        .get("channel")
        .and_then(Value::as_str)
        .unwrap_or("unspecified");
    if channel == "unspecified" {
        return None;
    }
    let release = labels
        .get("release")
        .and_then(Value::as_str)
        .unwrap_or("unspecified");
    let platform = labels
        .get("platform")
        .and_then(Value::as_str)
        .unwrap_or("unspecified");
    let compatibility = labels
        .get("compatibility_class")
        .and_then(Value::as_str)
        .unwrap_or("unspecified");
    Some(format!("{channel}|{release}|{platform}|{compatibility}"))
}

fn evidence_registry_reject_label_conflicts(
    entries: &[Value],
    prefix: &'static str,
    issues: &mut Vec<Value>,
) {
    let mut seen = std::collections::BTreeMap::<String, String>::new();
    for entry in entries {
        let Some(key) = evidence_registry_label_key(entry) else {
            continue;
        };
        let hash = entry
            .get("index_receipt_hash")
            .and_then(Value::as_str)
            .unwrap_or("<missing>")
            .to_string();
        if let Some(previous) = seen.insert(key, hash.clone()) {
            if previous != hash {
                issues.push(evidence_registry_issue(
                    &format!("{prefix}.label_conflict"),
                    "labels",
                    "unique label tuple per index hash",
                    "conflict",
                ));
            }
        }
    }
    // r[impl cairn.supersede.registry_channel_metadata.build]
}

fn read_repeated_path_options(args: &[String], flag: &str) -> Vec<PathBuf> {
    let mut values = Vec::new();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == flag {
            if let Some(value) = iter.next() {
                values.push(PathBuf::from(value));
            }
        }
    }
    values
}

fn evidence_registry_receipt(options: &EvidenceRegistryOptions) -> Value {
    // r[impl cairn.supersede.evidence_registry_rail.build]
    // r[impl cairn.supersede.evidence_registry_rail.failures]
    let root = options.root.clone();
    let out = options.out.clone().unwrap_or_else(|| PathBuf::from("."));
    let mut issues = Vec::new();
    let mut entries = Vec::new();
    let mut hashes = std::collections::BTreeSet::new();
    evidence_registry_validate_labels(&root, &options.labels, "evidence_registry", &mut issues);
    for (position, index_path) in options.indexes.iter().enumerate() {
        if let Some(entry) = evidence_registry_entry_from_index(
            &root,
            index_path,
            out.parent().unwrap_or_else(|| Path::new(".")),
            position,
            "evidence_registry",
            Some(&options.labels),
            &mut issues,
        ) {
            if let Some(hash) = entry.get("index_receipt_hash").and_then(Value::as_str) {
                if !hashes.insert(hash.to_string()) {
                    issues.push(evidence_registry_issue(
                        "evidence_registry.duplicate_index_hash",
                        "indexes",
                        "unique index receipt hashes",
                        "duplicate",
                    ));
                }
            }
            entries.push(entry);
        }
    }
    entries.sort_by_key(canonical_json_hash);
    evidence_registry_reject_label_conflicts(&entries, "evidence_registry", &mut issues);
    let mut receipt = evidence_registry_final_receipt(
        &root,
        "evidence registry build",
        issues,
        entries,
        "<out>",
        Value::Null,
    );
    if receipt.get("valid").and_then(Value::as_bool) == Some(true) {
        if let Some(parent) = out.parent() {
            if fs::create_dir_all(parent).is_err() {
                receipt = evidence_registry_final_receipt(
                    &root,
                    "evidence registry build",
                    vec![evidence_registry_issue(
                        "evidence_registry.output_create_failed",
                        "out",
                        "writable output parent",
                        "failed",
                    )],
                    Vec::new(),
                    "<out>",
                    Value::Null,
                );
            }
        }
        if receipt.get("valid").and_then(Value::as_bool) == Some(true)
            && fs::write(
                &out,
                serde_json::to_vec_pretty(&receipt).unwrap_or_default(),
            )
            .is_err()
        {
            receipt = evidence_registry_final_receipt(
                &root,
                "evidence registry build",
                vec![evidence_registry_issue(
                    "evidence_registry.receipt_write_failed",
                    "out",
                    "writable registry receipt",
                    "failed",
                )],
                Vec::new(),
                "<out>",
                Value::Null,
            );
        }
    }
    receipt
}

fn evidence_registry_verify_receipt(options: &EvidenceRegistryVerifyOptions) -> Value {
    // r[impl cairn.supersede.evidence_registry_rail.verify]
    // r[impl cairn.supersede.evidence_registry_rail.failures]
    let root = options.root.clone();
    let registry_path = options
        .registry
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));
    let registry = match read_json_artifact(&registry_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            return evidence_registry_verify_final_receipt(
                &root,
                vec![evidence_registry_issue(
                    "evidence_registry_verify.registry_unreadable",
                    "registry",
                    "readable registry",
                    "unreadable",
                )],
                Value::Null,
                Vec::new(),
            )
        }
    };
    let mut issues = Vec::new();
    let stated_hash = registry.get("receipt_hash").cloned().unwrap_or(Value::Null);
    if let Some(hash) = stated_hash.as_str() {
        let mut hash_input = registry.clone();
        if let Some(object) = hash_input.as_object_mut() {
            object.remove("receipt_hash");
        }
        if portable_json_hash(&root, &hash_input) != hash {
            issues.push(evidence_registry_issue(
                "evidence_registry_verify.registry_hash_mismatch",
                "receipt_hash",
                "portable receipt hash",
                "mismatch",
            ));
        }
    } else {
        issues.push(evidence_registry_issue(
            "evidence_registry_verify.registry_hash_missing",
            "receipt_hash",
            "BLAKE3 hex",
            "missing",
        ));
    }
    let mut entries = Vec::new();
    let base = registry_path.parent().unwrap_or_else(|| Path::new("."));
    for (position, entry) in registry
        .get("entries")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .iter()
        .enumerate()
    {
        let Some(index_path) = evidence_registry_resolve_index_path(base, entry) else {
            issues.push(evidence_registry_issue(
                "evidence_registry_verify.index_path_unsafe",
                "entries.index_relative",
                "safe relative index path",
                "unsafe",
            ));
            continue;
        };
        let entry_labels = entry_labels_from_registry_entry(entry);
        if let Some(labels) = entry_labels.as_ref() {
            evidence_registry_validate_labels(
                &root,
                labels,
                "evidence_registry_verify",
                &mut issues,
            );
        }
        if let Some(current) = evidence_registry_entry_from_index(
            &root,
            &index_path,
            base,
            position,
            "evidence_registry_verify",
            entry_labels.as_ref(),
            &mut issues,
        ) {
            if !evidence_registry_entries_equal(entry, &current) {
                issues.push(evidence_registry_issue(
                    "evidence_registry_verify.entry_mismatch",
                    "entries",
                    "current index entry",
                    "mismatch",
                ));
            }
            entries.push(current);
        }
    }
    entries.sort_by_key(canonical_json_hash);
    evidence_registry_reject_label_conflicts(&entries, "evidence_registry_verify", &mut issues);
    evidence_registry_verify_final_receipt(&root, issues, stated_hash, entries)
}

fn evidence_registry_mirror_verify_receipt(options: &EvidenceRegistryMirrorVerifyOptions) -> Value {
    // r[impl cairn.supersede.registry_mirror_verification.command]
    // r[impl cairn.supersede.registry_mirror_verification.receipt]
    // r[impl cairn.supersede.registry_mirror_verification.failures]
    let root = options.root.clone();
    let registry_path = options
        .registry
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));
    let mirror = options.mirror.clone().unwrap_or_else(|| PathBuf::from("."));
    let registry = match read_json_artifact(&registry_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            return evidence_registry_mirror_verify_final_receipt(
                &root,
                vec![evidence_registry_issue(
                    "evidence_registry_mirror_verify.registry_unreadable",
                    "registry",
                    "readable registry",
                    "unreadable",
                )],
                Value::Null,
                Value::Null,
                Vec::new(),
            )
        }
    };
    let mut issues = Vec::new();
    let registry_hash = registry.get("receipt_hash").cloned().unwrap_or(Value::Null);
    if let Some(hash) = registry_hash.as_str() {
        let mut hash_input = registry.clone();
        if let Some(object) = hash_input.as_object_mut() {
            object.remove("receipt_hash");
        }
        if portable_json_hash(&root, &hash_input) != hash {
            issues.push(evidence_registry_issue(
                "evidence_registry_mirror_verify.registry_hash_mismatch",
                "receipt_hash",
                "portable receipt hash",
                "mismatch",
            ));
        }
    } else {
        issues.push(evidence_registry_issue(
            "evidence_registry_mirror_verify.registry_hash_missing",
            "receipt_hash",
            "BLAKE3 hex",
            "missing",
        ));
    }
    let registry_verification = evidence_registry_verify_receipt(&EvidenceRegistryVerifyOptions {
        root: root.clone(),
        registry: Some(registry_path.clone()),
    });
    if registry_verification.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_registry_issue(
            "evidence_registry_mirror_verify.registry_verify_failed",
            "registry",
            "valid registry verification",
            "invalid",
        ));
    }
    let mut entries = Vec::new();
    for (position, entry) in registry
        .get("entries")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .iter()
        .enumerate()
    {
        let Some(index_path) = evidence_registry_resolve_index_path(&mirror, entry) else {
            issues.push(evidence_registry_issue(
                "evidence_registry_mirror_verify.index_path_unsafe",
                "entries.index_relative",
                "safe relative index path",
                "unsafe",
            ));
            continue;
        };
        if !index_path.exists() {
            issues.push(evidence_registry_issue(
                "evidence_registry_mirror_verify.index_missing",
                "entries.index_relative",
                "mirrored index file",
                "missing",
            ));
            continue;
        }
        let entry_labels = entry_labels_from_registry_entry(entry);
        if let Some(labels) = entry_labels.as_ref() {
            evidence_registry_validate_labels(
                &root,
                labels,
                "evidence_registry_mirror_verify",
                &mut issues,
            );
        }
        if let Some(current) = evidence_registry_entry_from_index(
            &root,
            &index_path,
            &mirror,
            position,
            "evidence_registry_mirror_verify",
            entry_labels.as_ref(),
            &mut issues,
        ) {
            if entry.get("index_receipt_hash") != current.get("index_receipt_hash") {
                issues.push(evidence_registry_issue(
                    "evidence_registry_mirror_verify.index_hash_mismatch",
                    "entries.index_receipt_hash",
                    "declared registry index hash",
                    "mismatch",
                ));
            } else if !evidence_registry_entries_equal(entry, &current) {
                issues.push(evidence_registry_issue(
                    "evidence_registry_mirror_verify.entry_mismatch",
                    "entries",
                    "current mirrored index entry",
                    "mismatch",
                ));
            }
            entries.push(current);
        }
    }
    entries.sort_by_key(canonical_json_hash);
    evidence_registry_reject_label_conflicts(
        &entries,
        "evidence_registry_mirror_verify",
        &mut issues,
    );
    evidence_registry_mirror_verify_final_receipt(
        &root,
        issues,
        registry_hash,
        registry_verification
            .get("receipt_hash")
            .cloned()
            .unwrap_or(Value::Null),
        entries,
    )
}

fn evidence_registry_mirror_verify_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    registry_hash: Value,
    registry_verification_receipt_hash: Value,
    entries: Vec<Value>,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence registry mirror-verify",
            "root": path_string(root),
            "registry": "<registry>",
            "mirror": "<mirror>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "registry_hash": registry_hash,
            "registry_verification_receipt_hash": registry_verification_receipt_hash,
            "entry_count": entries.len(),
            "entries": entries,
        }),
    )
}

fn evidence_registry_entry_from_index(
    root: &Path,
    index_path: &Path,
    base: &Path,
    position: usize,
    prefix: &'static str,
    labels: Option<&EvidenceRegistryLabels>,
    issues: &mut Vec<Value>,
) -> Option<Value> {
    let index = match read_json_artifact(index_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            issues.push(evidence_registry_issue(
                &format!("{prefix}.index_unreadable"),
                "index",
                "readable evidence index",
                "unreadable",
            ));
            return None;
        }
    };
    if index.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_registry_issue(
            &format!("{prefix}.index_invalid"),
            "index",
            "valid evidence index",
            "invalid",
        ));
    }
    let verify_options = EvidenceIndexVerifyOptions {
        root: root.to_path_buf(),
        index: Some(index_path.to_path_buf()),
    };
    let verify = evidence_index_verify_receipt(&verify_options);
    if verify.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_registry_issue(
            &format!("{prefix}.index_verify_failed"),
            "index",
            "valid index verification",
            "invalid",
        ));
    }
    let index_relative = evidence_registry_index_relative(base, index_path);
    let labels = labels
        .map(evidence_registry_labels_json)
        .unwrap_or_else(|| evidence_registry_labels_json(&EvidenceRegistryLabels::default()));
    Some(json!({
        "index": format!("<index:{position}>"),
        "index_relative": index_relative,
        "index_receipt_hash": index.get("receipt_hash").cloned().unwrap_or(Value::Null),
        "index_verification_receipt_hash": verify.get("receipt_hash").cloned().unwrap_or(Value::Null),
        "labels": labels,
        "bundle_kind": index.get("bundle_kind").cloned().unwrap_or(Value::Null),
        "bundle_receipt_hash": index.get("bundle_receipt_hash").cloned().unwrap_or(Value::Null),
        "receipt_chain": index.get("receipt_chain").cloned().unwrap_or_else(|| json!({})),
        "policy_hash": index.get("policy_hash").cloned().unwrap_or(Value::Null),
        "artifacts": index.get("artifacts").cloned().unwrap_or_else(|| json!([])),
    }))
}

fn evidence_registry_entries_equal(expected: &Value, current: &Value) -> bool {
    for key in [
        "index_receipt_hash",
        "labels",
        "bundle_kind",
        "bundle_receipt_hash",
        "receipt_chain",
        "policy_hash",
        "artifacts",
    ] {
        if expected.get(key) != current.get(key) {
            return false;
        }
    }
    true
}

fn evidence_registry_index_relative(base: &Path, index_path: &Path) -> Value {
    if let Ok(relative) = index_path.strip_prefix(base) {
        if let Some(text) = relative.to_str() {
            if !text.is_empty() && !text.starts_with('/') && !text.contains("..") {
                return json!(text);
            }
        }
    }
    if let (Some(parent), Some(name)) = (index_path.parent(), index_path.file_name()) {
        if parent == base {
            return json!(name.to_string_lossy().to_string());
        }
    }
    Value::Null
}

fn evidence_registry_resolve_index_path(base: &Path, entry: &Value) -> Option<PathBuf> {
    let relative = entry.get("index_relative").and_then(Value::as_str)?;
    if relative.is_empty() || relative.starts_with('/') || relative.contains("..") {
        return None;
    }
    Some(base.join(relative))
}

fn evidence_registry_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

fn evidence_registry_final_receipt(
    root: &Path,
    command: &str,
    mut issues: Vec<Value>,
    entries: Vec<Value>,
    out: &str,
    registry_hash: Value,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": command,
            "root": path_string(root),
            "out": out,
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "entry_count": entries.len(),
            "entries": entries,
            "registry_hash": registry_hash,
        }),
    )
}

fn evidence_registry_verify_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    registry_hash: Value,
    entries: Vec<Value>,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence registry verify",
            "root": path_string(root),
            "registry": "<registry>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "registry_hash": registry_hash,
            "entry_count": entries.len(),
            "entries": entries,
        }),
    )
}

fn evidence_registry_select_receipt(options: &EvidenceRegistrySelectOptions) -> Value {
    // r[impl cairn.supersede.evidence_registry_consume.select]
    let root = options.root.clone();
    let registry_path = options
        .registry
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));
    let out = options.out.clone().unwrap_or_else(|| PathBuf::from("."));
    let (registry, verification, mut issues) =
        evidence_registry_consume_registry(&root, &registry_path, "evidence_registry_select");
    let entries = registry
        .get("entries")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let matched_entries =
        evidence_registry_matching_entries(&entries, &options.selectors, &mut issues);
    let mut selected_entry = Value::Null;
    if matched_entries.is_empty() {
        issues.push(evidence_registry_issue(
            "evidence_registry_select.no_match",
            "selectors",
            "exactly one matching registry entry",
            "none",
        ));
    } else if matched_entries.len() > 1 {
        issues.push(evidence_registry_issue(
            "evidence_registry_select.ambiguous_match",
            "selectors",
            "exactly one matching registry entry",
            "multiple",
        ));
    } else {
        selected_entry = matched_entries[0].clone();
    }

    if issues.is_empty() {
        let base = registry_path.parent().unwrap_or_else(|| Path::new("."));
        let Some(index_path) = evidence_registry_resolve_index_path(base, &selected_entry) else {
            return evidence_registry_select_final_receipt(
                &root,
                vec![evidence_registry_issue(
                    "evidence_registry_select.index_path_unsafe",
                    "selected.index_relative",
                    "safe relative index path",
                    "unsafe",
                )],
                &options.selectors,
                verification,
                Value::Null,
            );
        };
        match fs::read(&index_path) {
            Ok(bytes) => {
                if let Some(parent) = out.parent() {
                    if fs::create_dir_all(parent).is_err() {
                        issues.push(evidence_registry_issue(
                            "evidence_registry_select.output_create_failed",
                            "out",
                            "writable output parent",
                            "failed",
                        ));
                    }
                }
                if issues.is_empty() && fs::write(&out, bytes).is_err() {
                    issues.push(evidence_registry_issue(
                        "evidence_registry_select.output_write_failed",
                        "out",
                        "writable selected index artifact",
                        "failed",
                    ));
                }
            }
            Err(_) => issues.push(evidence_registry_issue(
                "evidence_registry_select.index_unreadable",
                "selected.index_relative",
                "readable selected index",
                "unreadable",
            )),
        }
    }

    evidence_registry_select_final_receipt(
        &root,
        issues,
        &options.selectors,
        verification,
        selected_entry,
    )
}

fn evidence_registry_fetch_plan_receipt(options: &EvidenceRegistryFetchPlanOptions) -> Value {
    // r[impl cairn.supersede.evidence_registry_consume.fetch_plan]
    let root = options.root.clone();
    let registry_path = options
        .registry
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));
    let out = options.out.clone().unwrap_or_else(|| PathBuf::from("."));
    let (registry, verification, mut issues) =
        evidence_registry_consume_registry(&root, &registry_path, "evidence_registry_fetch_plan");
    let entries = registry
        .get("entries")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut plan_entries =
        evidence_registry_matching_entries(&entries, &options.selectors, &mut issues);
    plan_entries.sort_by_key(canonical_json_hash);
    let mut receipt = evidence_registry_fetch_plan_final_receipt(
        &root,
        issues,
        &options.selectors,
        verification,
        plan_entries,
    );
    if receipt.get("valid").and_then(Value::as_bool) == Some(true) {
        if let Some(parent) = out.parent() {
            if fs::create_dir_all(parent).is_err() {
                receipt = evidence_registry_fetch_plan_final_receipt(
                    &root,
                    vec![evidence_registry_issue(
                        "evidence_registry_fetch_plan.output_create_failed",
                        "out",
                        "writable output parent",
                        "failed",
                    )],
                    &options.selectors,
                    Value::Null,
                    Vec::new(),
                );
            }
        }
        if receipt.get("valid").and_then(Value::as_bool) == Some(true)
            && fs::write(
                &out,
                serde_json::to_vec_pretty(&receipt).unwrap_or_default(),
            )
            .is_err()
        {
            receipt = evidence_registry_fetch_plan_final_receipt(
                &root,
                vec![evidence_registry_issue(
                    "evidence_registry_fetch_plan.output_write_failed",
                    "out",
                    "writable fetch plan receipt",
                    "failed",
                )],
                &options.selectors,
                Value::Null,
                Vec::new(),
            );
        }
    }
    receipt
}

fn evidence_registry_consume_registry(
    root: &Path,
    registry_path: &Path,
    prefix: &'static str,
) -> (Value, Value, Vec<Value>) {
    let mut issues = Vec::new();
    let registry = match read_json_artifact(registry_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            issues.push(evidence_registry_issue(
                &format!("{prefix}.registry_unreadable"),
                "registry",
                "readable registry",
                "unreadable",
            ));
            return (Value::Null, Value::Null, issues);
        }
    };
    let verification = evidence_registry_verify_receipt(&EvidenceRegistryVerifyOptions {
        root: root.to_path_buf(),
        registry: Some(registry_path.to_path_buf()),
    });
    if verification.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_registry_issue(
            &format!("{prefix}.registry_verify_failed"),
            "registry",
            "valid registry verification",
            "invalid",
        ));
    }
    (registry, verification, issues)
}

fn evidence_registry_matching_entries(
    entries: &[Value],
    selectors: &EvidenceRegistryConsumeSelectors,
    issues: &mut Vec<Value>,
) -> Vec<Value> {
    if let Some(issue) = evidence_registry_selector_issue(selectors) {
        issues.push(issue);
        return Vec::new();
    }
    entries
        .iter()
        .filter(|entry| evidence_registry_entry_matches(entry, selectors))
        .cloned()
        .collect()
}

fn evidence_registry_entry_matches(
    entry: &Value,
    selectors: &EvidenceRegistryConsumeSelectors,
) -> bool {
    if !evidence_registry_match_string(entry, "bundle_kind", selectors.bundle_kind.as_deref()) {
        return false;
    }
    if !evidence_registry_match_string(entry, "policy_hash", selectors.policy_hash.as_deref()) {
        return false;
    }
    if !evidence_registry_match_label(entry, "channel", selectors.channel.as_deref()) {
        return false;
    }
    if !evidence_registry_match_label(entry, "release", selectors.release.as_deref()) {
        return false;
    }
    if !evidence_registry_match_label(entry, "platform", selectors.platform.as_deref()) {
        return false;
    }
    if !evidence_registry_match_label(
        entry,
        "compatibility_class",
        selectors.compatibility_class.as_deref(),
    ) {
        return false;
    }
    if !evidence_registry_match_string(
        entry,
        "index_receipt_hash",
        selectors.index_receipt_hash.as_deref(),
    ) {
        return false;
    }
    if !evidence_registry_match_string(
        entry,
        "index_verification_receipt_hash",
        selectors.index_verification_receipt_hash.as_deref(),
    ) {
        return false;
    }
    if !evidence_registry_match_string(
        entry,
        "bundle_receipt_hash",
        selectors.bundle_receipt_hash.as_deref(),
    ) {
        return false;
    }
    if !evidence_registry_match_receipt_chain(
        entry,
        "coverage_audit_receipt_hash",
        selectors.coverage_audit_receipt_hash.as_deref(),
    ) {
        return false;
    }
    if !evidence_registry_match_receipt_chain(
        entry,
        "release_readiness_receipt_hash",
        selectors.release_readiness_receipt_hash.as_deref(),
    ) {
        return false;
    }
    if !evidence_registry_match_artifact(entry, selectors) {
        return false;
    }
    true
}

fn evidence_registry_match_string(entry: &Value, key: &str, selector: Option<&str>) -> bool {
    match selector {
        Some(expected) => entry.get(key).and_then(Value::as_str) == Some(expected),
        None => true,
    }
}

fn evidence_registry_match_label(entry: &Value, key: &str, selector: Option<&str>) -> bool {
    match selector {
        Some(expected) => {
            entry
                .get("labels")
                .and_then(|labels| labels.get(key))
                .and_then(Value::as_str)
                == Some(expected)
        }
        None => true,
    }
}

fn evidence_registry_match_receipt_chain(entry: &Value, key: &str, selector: Option<&str>) -> bool {
    match selector {
        Some(expected) => {
            entry
                .get("receipt_chain")
                .and_then(|chain| chain.get(key))
                .and_then(Value::as_str)
                == Some(expected)
        }
        None => true,
    }
}

fn evidence_registry_match_artifact(
    entry: &Value,
    selectors: &EvidenceRegistryConsumeSelectors,
) -> bool {
    if selectors.artifact_role.is_none() && selectors.artifact_hash.is_none() {
        return true;
    }
    let Some(artifacts) = entry.get("artifacts").and_then(Value::as_array) else {
        return false;
    };
    artifacts.iter().any(|artifact| {
        let role_ok = selectors
            .artifact_role
            .as_deref()
            .is_none_or(|role| artifact.get("role").and_then(Value::as_str) == Some(role));
        let hash_ok = selectors
            .artifact_hash
            .as_deref()
            .is_none_or(|hash| artifact.get("hash").and_then(Value::as_str) == Some(hash));
        role_ok && hash_ok
    })
}

fn evidence_registry_selector_issue(selectors: &EvidenceRegistryConsumeSelectors) -> Option<Value> {
    let selector_values = [
        selectors.bundle_kind.as_deref(),
        selectors.policy_hash.as_deref(),
        selectors.channel.as_deref(),
        selectors.release.as_deref(),
        selectors.platform.as_deref(),
        selectors.compatibility_class.as_deref(),
        selectors.index_receipt_hash.as_deref(),
        selectors.index_verification_receipt_hash.as_deref(),
        selectors.bundle_receipt_hash.as_deref(),
        selectors.artifact_role.as_deref(),
        selectors.artifact_hash.as_deref(),
        selectors.coverage_audit_receipt_hash.as_deref(),
        selectors.release_readiness_receipt_hash.as_deref(),
    ];
    if selector_values.iter().flatten().any(|value| {
        value.is_empty() || value.starts_with('/') || value.contains("..") || value.contains('\\')
    }) {
        return Some(evidence_registry_issue(
            "evidence_registry_consume.selector_unsafe",
            "selectors",
            "stable non-path selector values",
            "unsafe",
        ));
    }
    None
}

fn evidence_registry_selectors_json(selectors: &EvidenceRegistryConsumeSelectors) -> Value {
    let mut object = serde_json::Map::new();
    if let Some(value) = selectors.bundle_kind.as_ref() {
        object.insert("bundle_kind".to_string(), json!(value));
    }
    if let Some(value) = selectors.policy_hash.as_ref() {
        object.insert("policy_hash".to_string(), json!(value));
    }
    if let Some(value) = selectors.channel.as_ref() {
        object.insert("channel".to_string(), json!(value));
    }
    if let Some(value) = selectors.release.as_ref() {
        object.insert("release".to_string(), json!(value));
    }
    if let Some(value) = selectors.platform.as_ref() {
        object.insert("platform".to_string(), json!(value));
    }
    if let Some(value) = selectors.compatibility_class.as_ref() {
        object.insert("compatibility_class".to_string(), json!(value));
    }
    if let Some(value) = selectors.index_receipt_hash.as_ref() {
        object.insert("index_receipt_hash".to_string(), json!(value));
    }
    if let Some(value) = selectors.index_verification_receipt_hash.as_ref() {
        object.insert("index_verification_receipt_hash".to_string(), json!(value));
    }
    if let Some(value) = selectors.bundle_receipt_hash.as_ref() {
        object.insert("bundle_receipt_hash".to_string(), json!(value));
    }
    if let Some(value) = selectors.artifact_role.as_ref() {
        object.insert("artifact_role".to_string(), json!(value));
    }
    if let Some(value) = selectors.artifact_hash.as_ref() {
        object.insert("artifact_hash".to_string(), json!(value));
    }
    if let Some(value) = selectors.coverage_audit_receipt_hash.as_ref() {
        object.insert("coverage_audit_receipt_hash".to_string(), json!(value));
    }
    if let Some(value) = selectors.release_readiness_receipt_hash.as_ref() {
        object.insert("release_readiness_receipt_hash".to_string(), json!(value));
    }
    Value::Object(object)
}

fn evidence_registry_select_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    selectors: &EvidenceRegistryConsumeSelectors,
    verification: Value,
    selected_entry: Value,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence registry select",
            "root": path_string(root),
            "registry": "<registry>",
            "out": "<out>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "selectors": evidence_registry_selectors_json(selectors),
            "registry_receipt_hash": verification.get("registry_hash").cloned().unwrap_or(Value::Null),
            "registry_verification_receipt_hash": verification.get("receipt_hash").cloned().unwrap_or(Value::Null),
            "selected_entry": selected_entry,
        }),
    )
}

fn evidence_registry_fetch_plan_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    selectors: &EvidenceRegistryConsumeSelectors,
    verification: Value,
    entries: Vec<Value>,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence registry fetch-plan",
            "root": path_string(root),
            "registry": "<registry>",
            "out": "<out>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "selectors": evidence_registry_selectors_json(selectors),
            "registry_receipt_hash": verification.get("registry_hash").cloned().unwrap_or(Value::Null),
            "registry_verification_receipt_hash": verification.get("receipt_hash").cloned().unwrap_or(Value::Null),
            "entry_count": entries.len(),
            "entries": entries,
        }),
    )
}

fn run_evidence_republish(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.evidence_import_republish_rail.command]
    // r[impl cairn.supersede.evidence_import_republish_rail.artifacts]
    // r[impl cairn.supersede.evidence_import_republish_rail.failures]
    let options = evidence_republish_options(&args)?;
    let receipt = evidence_republish_receipt(&options);
    let valid = receipt
        .get("valid")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    print_json(receipt)?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

fn evidence_republish_options(args: &[String]) -> Result<EvidenceRepublishOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let imported = read_string_option(args, "--imported").map(PathBuf::from);
    let out = read_string_option(args, "--out").map(PathBuf::from);
    if imported.is_none() {
        return Err("evidence republish requires --imported <dir>".into());
    }
    if out.is_none() {
        return Err("evidence republish requires --out <dir>".into());
    }
    Ok(EvidenceRepublishOptions {
        root,
        imported,
        out,
    })
}

fn evidence_republish_receipt(options: &EvidenceRepublishOptions) -> Value {
    let root = options.root.clone();
    let imported_dir = options
        .imported
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));
    let out_dir = options.out.clone().unwrap_or_else(|| PathBuf::from("."));
    let mut issues = Vec::new();
    let import_receipt_path = imported_dir.join("evidence-import-receipt.json");
    let import_receipt = match read_json_artifact(&import_receipt_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(receipt) => receipt,
        Err(_) => {
            issues.push(evidence_republish_issue(
                "evidence_republish.import_receipt_missing",
                "evidence-import-receipt.json",
                "readable import receipt",
                "missing",
            ));
            return evidence_republish_final_receipt(&root, issues, Vec::new(), Value::Null);
        }
    };

    let import_receipt_hash = import_receipt
        .get("receipt_hash")
        .cloned()
        .unwrap_or(Value::Null);
    if import_receipt.get("command").and_then(Value::as_str) != Some("evidence import") {
        issues.push(evidence_republish_issue(
            "evidence_republish.import_receipt_command_invalid",
            "command",
            "evidence import",
            import_receipt
                .get("command")
                .and_then(Value::as_str)
                .unwrap_or("<missing>"),
        ));
    }
    if import_receipt.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_republish_issue(
            "evidence_republish.import_receipt_invalid",
            "valid",
            "true",
            "false",
        ));
    }
    if let Some(stated_hash) = import_receipt.get("receipt_hash").and_then(Value::as_str) {
        let mut receipt_hash_input = import_receipt.clone();
        if let Some(object) = receipt_hash_input.as_object_mut() {
            object.remove("receipt_hash");
        }
        let expected_hash = portable_json_hash(&root, &receipt_hash_input);
        if stated_hash != expected_hash {
            issues.push(evidence_republish_issue(
                "evidence_republish.import_receipt_hash_mismatch",
                "receipt_hash",
                "portable receipt hash",
                "mismatch",
            ));
        }
    } else {
        issues.push(evidence_republish_issue(
            "evidence_republish.import_receipt_hash_missing",
            "receipt_hash",
            "BLAKE3 hex",
            "missing",
        ));
    }

    let Some(artifacts) = import_receipt.get("artifacts").and_then(Value::as_array) else {
        issues.push(evidence_republish_issue(
            "evidence_republish.artifacts_missing",
            "artifacts",
            "artifact array",
            "missing",
        ));
        return evidence_republish_final_receipt(&root, issues, Vec::new(), import_receipt_hash);
    };

    if out_dir.exists() {
        match bundle_regular_files_with_prefix(&out_dir, "evidence_republish") {
            Ok(files) if !files.is_empty() => issues.push(evidence_republish_issue(
                "evidence_republish.output_not_empty",
                "out",
                "empty output directory",
                "non_empty",
            )),
            Err(code) => issues.push(evidence_republish_issue(
                code,
                "out",
                "bounded readable output files",
                "failed",
            )),
            _ => {}
        }
    }

    let mut declared_files = std::collections::BTreeSet::new();
    declared_files.insert("evidence-import-receipt.json".to_string());
    let mut copied = Vec::new();
    let mut seen_paths = std::collections::BTreeSet::new();
    match bundle_regular_files_with_prefix(&imported_dir, "evidence_republish") {
        Ok(files) => {
            for relative in files {
                if !declared_files.contains(&relative)
                    && !artifacts.iter().any(|artifact| {
                        artifact.get("path").and_then(Value::as_str) == Some(relative.as_str())
                    })
                {
                    issues.push(evidence_republish_issue(
                        "evidence_republish.undeclared_imported_file",
                        &relative,
                        "declared imported artifact",
                        "extra",
                    ));
                }
            }
        }
        Err(code) => issues.push(evidence_republish_issue(
            code,
            "imported",
            "bounded readable imported files",
            "failed",
        )),
    }
    for artifact in artifacts {
        let role = artifact
            .get("role")
            .and_then(Value::as_str)
            .unwrap_or("<missing>");
        let relative = artifact.get("path").and_then(Value::as_str).unwrap_or("");
        if !seen_paths.insert(relative.to_string()) {
            issues.push(evidence_republish_issue(
                "evidence_republish.duplicate_artifact_path",
                role,
                "unique artifact path",
                "duplicate",
            ));
            continue;
        }
        let Some(source_path) = safe_bundle_artifact_path(&imported_dir, relative) else {
            issues.push(evidence_republish_issue(
                "evidence_republish.artifact_path_unsafe",
                role,
                "safe relative artifact path",
                "unsafe",
            ));
            continue;
        };
        let Some(destination_path) = safe_bundle_artifact_path(&out_dir, relative) else {
            issues.push(evidence_republish_issue(
                "evidence_republish.destination_path_unsafe",
                role,
                "safe relative destination path",
                "unsafe",
            ));
            continue;
        };
        declared_files.insert(relative.to_string());
        let declared_hash = artifact.get("hash").and_then(Value::as_str).unwrap_or("");
        let source_bytes = match fs::read(&source_path) {
            Ok(bytes) => bytes,
            Err(_) => {
                issues.push(evidence_republish_issue(
                    "evidence_republish.artifact_missing",
                    role,
                    "declared artifact file",
                    "missing",
                ));
                continue;
            }
        };
        let source_hash = blake3_hex(&source_bytes);
        if source_hash != declared_hash {
            issues.push(evidence_republish_issue(
                "evidence_republish.artifact_hash_mismatch",
                role,
                "declared artifact hash",
                "mismatch",
            ));
            continue;
        }
        if issues.is_empty() {
            if let Some(parent) = destination_path.parent() {
                if fs::create_dir_all(parent).is_err() {
                    issues.push(evidence_republish_issue(
                        "evidence_republish.destination_create_failed",
                        role,
                        "writable destination parent",
                        "failed",
                    ));
                    continue;
                }
            }
            if destination_path.exists() {
                issues.push(evidence_republish_issue(
                    "evidence_republish.destination_collision",
                    role,
                    "absent destination artifact",
                    "exists",
                ));
                continue;
            }
            if fs::write(&destination_path, &source_bytes).is_err() {
                issues.push(evidence_republish_issue(
                    "evidence_republish.artifact_copy_failed",
                    role,
                    "copied artifact file",
                    "failed",
                ));
                continue;
            }
        }
        copied.push(json!({
            "role": role,
            "path": relative,
            "hash": source_hash,
        }));
    }

    let mut receipt = evidence_republish_final_receipt(&root, issues, copied, import_receipt_hash);
    if receipt.get("valid").and_then(Value::as_bool) == Some(true) {
        if fs::create_dir_all(&out_dir).is_err() {
            receipt = evidence_republish_final_receipt(
                &root,
                vec![evidence_republish_issue(
                    "evidence_republish.output_create_failed",
                    "out",
                    "writable output directory",
                    "failed",
                )],
                Vec::new(),
                receipt
                    .get("import_receipt_hash")
                    .cloned()
                    .unwrap_or(Value::Null),
            );
        } else {
            let import_receipt_bytes =
                serde_json::to_vec_pretty(&import_receipt).unwrap_or_default();
            if fs::write(
                out_dir.join("evidence-import-receipt.json"),
                import_receipt_bytes,
            )
            .is_err()
            {
                receipt = evidence_republish_final_receipt(
                    &root,
                    vec![evidence_republish_issue(
                        "evidence_republish.import_receipt_copy_failed",
                        "evidence-import-receipt.json",
                        "copied import receipt",
                        "failed",
                    )],
                    Vec::new(),
                    receipt
                        .get("import_receipt_hash")
                        .cloned()
                        .unwrap_or(Value::Null),
                );
            } else if fs::write(
                out_dir.join("evidence-republish-receipt.json"),
                serde_json::to_vec_pretty(&receipt).unwrap_or_default(),
            )
            .is_err()
            {
                receipt = evidence_republish_final_receipt(
                    &root,
                    vec![evidence_republish_issue(
                        "evidence_republish.receipt_write_failed",
                        "evidence-republish-receipt.json",
                        "writable republish receipt",
                        "failed",
                    )],
                    Vec::new(),
                    receipt
                        .get("import_receipt_hash")
                        .cloned()
                        .unwrap_or(Value::Null),
                );
            }
        }
    }
    receipt
}

fn bundle_regular_files_with_prefix(
    bundle_dir: &Path,
    prefix: &'static str,
) -> Result<std::collections::BTreeSet<String>, &'static str> {
    let mut out = std::collections::BTreeSet::new();
    collect_bundle_regular_files_with_prefix(bundle_dir, bundle_dir, &mut out, prefix)?;
    Ok(out)
}

fn collect_bundle_regular_files_with_prefix(
    root: &Path,
    dir: &Path,
    out: &mut std::collections::BTreeSet<String>,
    prefix: &'static str,
) -> Result<(), &'static str> {
    let entries = fs::read_dir(dir).map_err(|_| match prefix {
        "evidence_republish" => "evidence_republish.bundle_read_failed",
        _ => "evidence_import.bundle_read_failed",
    })?;
    let mut paths = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|_| match prefix {
            "evidence_republish" => "evidence_republish.bundle_read_failed",
            _ => "evidence_import.bundle_read_failed",
        })?;
        paths.push(entry.path());
    }
    paths.sort();
    for path in paths {
        let metadata = fs::metadata(&path).map_err(|_| match prefix {
            "evidence_republish" => "evidence_republish.bundle_read_failed",
            _ => "evidence_import.bundle_read_failed",
        })?;
        if metadata.is_dir() {
            collect_bundle_regular_files_with_prefix(root, &path, out, prefix)?;
        } else if metadata.is_file() {
            if out.len() >= EVIDENCE_IMPORT_MAX_FILES {
                return Err(match prefix {
                    "evidence_republish" => "evidence_republish.bundle_file_limit_exceeded",
                    _ => "evidence_import.bundle_file_limit_exceeded",
                });
            }
            let relative = path
                .strip_prefix(root)
                .map_err(|_| match prefix {
                    "evidence_republish" => "evidence_republish.bundle_read_failed",
                    _ => "evidence_import.bundle_read_failed",
                })?
                .to_string_lossy()
                .replace('\\', "/");
            out.insert(relative);
        }
    }
    Ok(())
}

fn evidence_republish_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

fn evidence_republish_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    mut artifacts: Vec<Value>,
    import_receipt_hash: Value,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    artifacts.sort_by_key(|artifact| {
        format!(
            "{}:{}",
            artifact.get("role").and_then(Value::as_str).unwrap_or(""),
            artifact.get("path").and_then(Value::as_str).unwrap_or("")
        )
    });
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence republish",
            "root": path_string(root),
            "imported": "<imported>",
            "out": "<out>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "artifacts": artifacts,
            "import_receipt_hash": import_receipt_hash,
        }),
    )
}

fn evidence_import_options(args: &[String]) -> Result<EvidenceImportOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let bundle = read_string_option(args, "--bundle").map(PathBuf::from);
    let into = read_string_option(args, "--into").map(PathBuf::from);
    if bundle.is_none() {
        return Err("evidence import requires --bundle <dir>".into());
    }
    if into.is_none() {
        return Err("evidence import requires --into <dir>".into());
    }
    Ok(EvidenceImportOptions { root, bundle, into })
}

fn evidence_import_receipt(options: &EvidenceImportOptions) -> Value {
    let root = options.root.clone();
    let bundle_dir = options.bundle.clone().unwrap_or_else(|| PathBuf::from("."));
    let into_dir = options.into.clone().unwrap_or_else(|| PathBuf::from("."));
    let mut issues = Vec::new();
    let verify_options = EvidenceVerifyOptions {
        root: root.clone(),
        bundle: Some(bundle_dir.clone()),
    };
    let verify_receipt = evidence_verify_receipt(&verify_options);
    let verification_receipt_hash = verify_receipt
        .get("receipt_hash")
        .cloned()
        .unwrap_or(Value::Null);
    if verify_receipt.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_import_issue(
            "evidence_import.bundle_verification_failed",
            "bundle",
            "valid evidence verify receipt",
            "invalid",
        ));
        return evidence_import_final_receipt(&root, issues, Vec::new(), verification_receipt_hash);
    }

    let export_receipt_path = bundle_dir.join("evidence-export-receipt.json");
    let export_receipt = match read_json_artifact(&export_receipt_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(receipt) => receipt,
        Err(_) => {
            issues.push(evidence_import_issue(
                "evidence_import.export_receipt_unreadable",
                "evidence-export-receipt.json",
                "readable export receipt",
                "unreadable",
            ));
            return evidence_import_final_receipt(
                &root,
                issues,
                Vec::new(),
                verification_receipt_hash,
            );
        }
    };

    let Some(artifacts) = export_receipt.get("artifacts").and_then(Value::as_array) else {
        issues.push(evidence_import_issue(
            "evidence_import.artifacts_missing",
            "artifacts",
            "artifact array",
            "missing",
        ));
        return evidence_import_final_receipt(&root, issues, Vec::new(), verification_receipt_hash);
    };

    let mut declared_files = std::collections::BTreeSet::new();
    declared_files.insert("evidence-export-receipt.json".to_string());
    let mut copied = Vec::new();
    for artifact in artifacts {
        let role = artifact
            .get("role")
            .and_then(Value::as_str)
            .unwrap_or("<missing>");
        let relative = artifact.get("path").and_then(Value::as_str).unwrap_or("");
        let Some(source_path) = safe_bundle_artifact_path(&bundle_dir, relative) else {
            issues.push(evidence_import_issue(
                "evidence_import.artifact_path_unsafe",
                role,
                "safe relative artifact path",
                "unsafe",
            ));
            continue;
        };
        let Some(destination_path) = safe_bundle_artifact_path(&into_dir, relative) else {
            issues.push(evidence_import_issue(
                "evidence_import.destination_path_unsafe",
                role,
                "safe relative destination path",
                "unsafe",
            ));
            continue;
        };
        declared_files.insert(relative.to_string());
        let declared_hash = artifact.get("hash").and_then(Value::as_str).unwrap_or("");
        let source_bytes = match fs::read(&source_path) {
            Ok(bytes) => bytes,
            Err(_) => {
                issues.push(evidence_import_issue(
                    "evidence_import.artifact_missing",
                    role,
                    "declared artifact file",
                    "missing",
                ));
                continue;
            }
        };
        let source_hash = blake3_hex(&source_bytes);
        if source_hash != declared_hash {
            issues.push(evidence_import_issue(
                "evidence_import.artifact_hash_mismatch",
                role,
                "declared artifact hash",
                "mismatch",
            ));
            continue;
        }
        if let Some(parent) = destination_path.parent() {
            if fs::create_dir_all(parent).is_err() {
                issues.push(evidence_import_issue(
                    "evidence_import.destination_create_failed",
                    role,
                    "writable destination parent",
                    "failed",
                ));
                continue;
            }
        }
        if fs::write(&destination_path, &source_bytes).is_err() {
            issues.push(evidence_import_issue(
                "evidence_import.artifact_copy_failed",
                role,
                "copied artifact file",
                "failed",
            ));
            continue;
        }
        let destination_hash = match fs::read(&destination_path) {
            Ok(bytes) => blake3_hex(&bytes),
            Err(_) => String::new(),
        };
        if destination_hash != source_hash {
            issues.push(evidence_import_issue(
                "evidence_import.destination_hash_mismatch",
                role,
                "copied artifact hash",
                "mismatch",
            ));
            continue;
        }
        copied.push(json!({
            "role": role,
            "path": relative,
            "hash": destination_hash,
        }));
    }

    match bundle_regular_files(&bundle_dir) {
        Ok(files) => {
            for relative in files {
                if !declared_files.contains(&relative) {
                    issues.push(evidence_import_issue(
                        "evidence_import.undeclared_bundle_file",
                        &relative,
                        "declared export artifact",
                        "extra",
                    ));
                }
            }
        }
        Err(code) => issues.push(evidence_import_issue(
            code,
            "bundle",
            "bounded readable bundle files",
            "failed",
        )),
    }

    let mut receipt =
        evidence_import_final_receipt(&root, issues, copied, verification_receipt_hash);
    if receipt.get("valid").and_then(Value::as_bool) == Some(true) {
        if fs::create_dir_all(&into_dir).is_err() {
            let mut issues = vec![evidence_import_issue(
                "evidence_import.destination_create_failed",
                "into",
                "writable import directory",
                "failed",
            )];
            receipt = evidence_import_final_receipt(
                &root,
                std::mem::take(&mut issues),
                Vec::new(),
                receipt
                    .get("verification_receipt_hash")
                    .cloned()
                    .unwrap_or(Value::Null),
            );
        } else if fs::write(
            into_dir.join("evidence-import-receipt.json"),
            serde_json::to_vec_pretty(&receipt).unwrap_or_default(),
        )
        .is_err()
        {
            receipt = evidence_import_final_receipt(
                &root,
                vec![evidence_import_issue(
                    "evidence_import.receipt_write_failed",
                    "evidence-import-receipt.json",
                    "writable import receipt",
                    "failed",
                )],
                Vec::new(),
                receipt
                    .get("verification_receipt_hash")
                    .cloned()
                    .unwrap_or(Value::Null),
            );
        }
    }
    receipt
}

const EVIDENCE_IMPORT_MAX_FILES: usize = 64;

fn bundle_regular_files(
    bundle_dir: &Path,
) -> Result<std::collections::BTreeSet<String>, &'static str> {
    let mut out = std::collections::BTreeSet::new();
    collect_bundle_regular_files(bundle_dir, bundle_dir, &mut out)?;
    Ok(out)
}

fn collect_bundle_regular_files(
    root: &Path,
    dir: &Path,
    out: &mut std::collections::BTreeSet<String>,
) -> Result<(), &'static str> {
    let entries = fs::read_dir(dir).map_err(|_| "evidence_import.bundle_read_failed")?;
    let mut paths = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|_| "evidence_import.bundle_read_failed")?;
        paths.push(entry.path());
    }
    paths.sort();
    for path in paths {
        let metadata = fs::metadata(&path).map_err(|_| "evidence_import.bundle_read_failed")?;
        if metadata.is_dir() {
            collect_bundle_regular_files(root, &path, out)?;
        } else if metadata.is_file() {
            if out.len() >= EVIDENCE_IMPORT_MAX_FILES {
                return Err("evidence_import.bundle_file_limit_exceeded");
            }
            let relative = path
                .strip_prefix(root)
                .map_err(|_| "evidence_import.bundle_read_failed")?
                .to_string_lossy()
                .replace('\\', "/");
            out.insert(relative);
        }
    }
    Ok(())
}

fn evidence_import_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

fn evidence_import_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    mut artifacts: Vec<Value>,
    verification_receipt_hash: Value,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    artifacts.sort_by_key(|artifact| {
        format!(
            "{}:{}",
            artifact.get("role").and_then(Value::as_str).unwrap_or(""),
            artifact.get("path").and_then(Value::as_str).unwrap_or("")
        )
    });
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence import",
            "root": path_string(root),
            "bundle": "<bundle>",
            "into": "<into>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "artifacts": artifacts,
            "verification_receipt_hash": verification_receipt_hash,
        }),
    )
}

fn evidence_verify_options(args: &[String]) -> Result<EvidenceVerifyOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let bundle = read_string_option(args, "--bundle").map(PathBuf::from);
    if bundle.is_none() {
        return Err("evidence verify requires --bundle <dir>".into());
    }
    Ok(EvidenceVerifyOptions { root, bundle })
}

fn evidence_verify_receipt(options: &EvidenceVerifyOptions) -> Value {
    let root = options.root.clone();
    let bundle_dir = options.bundle.clone().unwrap_or_else(|| PathBuf::from("."));
    let mut issues = Vec::new();
    let export_receipt_path = bundle_dir.join("evidence-export-receipt.json");
    let export_receipt = match read_json_artifact(&export_receipt_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(receipt) => receipt,
        Err(_) => {
            issues.push(evidence_verify_issue(
                "evidence_verify.export_receipt_missing",
                "evidence-export-receipt.json",
                "readable export receipt",
                "missing",
            ));
            return evidence_verify_final_receipt(
                &root,
                issues,
                Vec::new(),
                Value::Null,
                Value::Null,
                Value::Null,
            );
        }
    };

    if export_receipt.get("command").and_then(Value::as_str) != Some("evidence export") {
        issues.push(evidence_verify_issue(
            "evidence_verify.export_receipt_command_invalid",
            "evidence-export-receipt.command",
            "evidence export",
            "invalid",
        ));
    }
    let stated_export_hash = export_receipt
        .get("receipt_hash")
        .and_then(Value::as_str)
        .unwrap_or("");
    let computed_export_hash = receipt_stated_or_computed_hash(&root, &export_receipt);
    if computed_export_hash.as_str() != Some(stated_export_hash) {
        issues.push(evidence_verify_issue(
            "evidence_verify.export_receipt_hash_mismatch",
            "evidence-export-receipt.receipt_hash",
            "self-consistent receipt hash",
            "mismatch",
        ));
    }

    let mut verified_artifacts = Vec::new();
    let mut seen_roles = std::collections::BTreeSet::new();
    if let Some(artifacts) = export_receipt.get("artifacts").and_then(Value::as_array) {
        for artifact in artifacts {
            let role = artifact
                .get("role")
                .and_then(Value::as_str)
                .unwrap_or("<missing>");
            if !seen_roles.insert(role.to_string()) {
                issues.push(evidence_verify_issue(
                    "evidence_verify.role_duplicate",
                    role,
                    "unique artifact role",
                    "duplicate",
                ));
            }
            let relative = artifact.get("path").and_then(Value::as_str).unwrap_or("");
            let Some(path) = safe_bundle_artifact_path(&bundle_dir, relative) else {
                issues.push(evidence_verify_issue(
                    "evidence_verify.artifact_path_unsafe",
                    role,
                    "safe relative artifact path",
                    "unsafe",
                ));
                continue;
            };
            let declared_hash = artifact.get("hash").and_then(Value::as_str).unwrap_or("");
            let actual_hash = match fs::read(&path) {
                Ok(bytes) => blake3_hex(&bytes),
                Err(_) => {
                    issues.push(evidence_verify_issue(
                        "evidence_verify.artifact_missing",
                        role,
                        "declared artifact file",
                        "missing",
                    ));
                    String::new()
                }
            };
            if !actual_hash.is_empty() && actual_hash != declared_hash {
                issues.push(evidence_verify_issue(
                    "evidence_verify.artifact_hash_mismatch",
                    role,
                    "declared artifact hash",
                    "mismatch",
                ));
            }
            verified_artifacts.push(json!({
                "role": role,
                "path": relative,
                "hash": if actual_hash.is_empty() { Value::Null } else { json!(actual_hash) },
                "required": artifact.get("required").and_then(Value::as_bool).unwrap_or(true),
            }));
        }
    } else {
        issues.push(evidence_verify_issue(
            "evidence_verify.artifacts_missing",
            "artifacts",
            "artifact array",
            "missing",
        ));
    }

    for role in EVIDENCE_EXPORT_REQUIRED_ROLES {
        if !seen_roles.contains(*role) {
            issues.push(evidence_verify_issue(
                "evidence_verify.required_role_missing",
                role,
                "required artifact role",
                "missing",
            ));
        }
    }

    verify_saved_receipt_valid(
        &bundle_dir,
        "coverage_audit",
        "coverage-audit.json",
        "evidence_verify.coverage_audit_invalid",
        &mut issues,
    );
    verify_saved_receipt_valid(
        &bundle_dir,
        "replay_verification",
        "replay-verification.json",
        "evidence_verify.replay_verification_invalid",
        &mut issues,
    );
    verify_saved_receipt_valid(
        &bundle_dir,
        "release_readiness",
        "release-readiness.json",
        "evidence_verify.release_readiness_invalid",
        &mut issues,
    );
    verify_saved_receipt_valid(
        &bundle_dir,
        "agent_evidence_bundle_verification",
        "agent-evidence-bundle-verification.json",
        "evidence_verify.bundle_verification_invalid",
        &mut issues,
    );
    verify_saved_receipt_valid(
        &bundle_dir,
        "evidence_manifest_verification",
        "evidence-manifest-verification.json",
        "evidence_verify.manifest_verification_invalid",
        &mut issues,
    );

    let rerun_bundle = agent_evidence_bundle_verification_receipt(
        &root,
        &bundle_dir.join("agent-evidence-bundle.json"),
    );
    if rerun_bundle.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_verify_issue(
            "evidence_verify.bundle_rerun_invalid",
            "agent-evidence-bundle.json",
            "valid rerun bundle verification",
            "invalid",
        ));
    }
    let rerun_manifest =
        evidence_manifest_verification_receipt(&root, &bundle_dir.join("evidence-manifest.json"));
    if rerun_manifest.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_verify_issue(
            "evidence_verify.manifest_rerun_invalid",
            "evidence-manifest.json",
            "valid rerun manifest verification",
            "invalid",
        ));
    }

    let manifest_hash = export_receipt
        .get("manifest_hash")
        .cloned()
        .unwrap_or(Value::Null);
    let manifest_verification_receipt_hash = export_receipt
        .get("manifest_verification_receipt_hash")
        .cloned()
        .unwrap_or(Value::Null);
    evidence_verify_final_receipt(
        &root,
        issues,
        verified_artifacts,
        computed_export_hash,
        manifest_hash,
        manifest_verification_receipt_hash,
    )
}

const EVIDENCE_EXPORT_REQUIRED_ROLES: &[&str] = &[
    "agent_evidence_bundle",
    "agent_evidence_bundle_verification",
    "agent_receipt",
    "coverage_audit",
    "evidence_manifest",
    "evidence_manifest_verification",
    "release_readiness",
    "replay_receipt",
    "replay_verification",
];

fn safe_bundle_artifact_path(bundle_dir: &Path, relative: &str) -> Option<PathBuf> {
    let path = PathBuf::from(relative);
    if relative.is_empty() || path.is_absolute() {
        return None;
    }
    if path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return None;
    }
    Some(bundle_dir.join(path))
}

fn verify_saved_receipt_valid(
    bundle_dir: &Path,
    role: &str,
    relative: &str,
    code: &str,
    issues: &mut Vec<Value>,
) {
    let path = bundle_dir.join(relative);
    match read_json_artifact(&path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(receipt) => {
            if receipt.get("valid").and_then(Value::as_bool) != Some(true) {
                issues.push(evidence_verify_issue(
                    code,
                    role,
                    "persisted valid receipt",
                    "invalid",
                ));
            }
        }
        Err(_) => issues.push(evidence_verify_issue(
            "evidence_verify.receipt_unreadable",
            role,
            "readable JSON receipt",
            "unreadable",
        )),
    }
}

fn evidence_verify_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

fn evidence_verify_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    mut artifacts: Vec<Value>,
    export_receipt_hash: Value,
    manifest_hash: Value,
    manifest_verification_receipt_hash: Value,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    artifacts.sort_by_key(|artifact| {
        artifact
            .get("role")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string()
    });
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence verify",
            "root": path_string(root),
            "bundle": "<bundle>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "artifacts": artifacts,
            "export_receipt_hash": export_receipt_hash,
            "manifest_hash": manifest_hash,
            "manifest_verification_receipt_hash": manifest_verification_receipt_hash,
        }),
    )
}

fn evidence_export_options(args: &[String]) -> Result<EvidenceExportOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let out = read_string_option(args, "--out").map(PathBuf::from);
    let agent_receipt = read_string_option(args, "--agent-receipt").map(PathBuf::from);
    if out.is_none() {
        return Err("evidence export requires --out <dir>".into());
    }
    Ok(EvidenceExportOptions {
        root,
        out,
        agent_receipt,
    })
}

fn evidence_export_receipt(options: &EvidenceExportOptions) -> Value {
    let root = options.root.clone();
    let out_dir = options.out.clone().unwrap_or_else(|| PathBuf::from("."));
    let mut issues = Vec::new();
    if fs::create_dir_all(&out_dir).is_err() {
        issues.push(evidence_export_issue(
            "export.out_dir_create_failed",
            "out_dir",
            "writable output directory",
            "failed",
        ));
        return evidence_export_final_receipt(
            &root,
            &out_dir,
            issues,
            Vec::new(),
            Value::Null,
            Value::Null,
        );
    }

    let policy = match load_policy_file(default_policy_path().into()) {
        Ok(policy) => policy,
        Err(_) => {
            issues.push(evidence_export_issue(
                "export.policy_load_failed",
                "policy",
                "generated policy loads",
                "failed",
            ));
            return evidence_export_final_receipt(
                &root,
                &out_dir,
                issues,
                Vec::new(),
                Value::Null,
                Value::Null,
            );
        }
    };

    let agent_path = options
        .agent_receipt
        .clone()
        .unwrap_or_else(|| root.join("fixtures/agent-receipts/positive.json"));
    let coverage_path = out_dir.join("coverage-audit.json");
    let replay_path = out_dir.join("replay-agent.json");
    let replay_verification_path = out_dir.join("replay-verification.json");
    let release_path = out_dir.join("release-readiness.json");
    let exported_agent_path = out_dir.join("agent-receipt.json");
    let bundle_path = out_dir.join("agent-evidence-bundle.json");
    let bundle_verification_path = out_dir.join("agent-evidence-bundle-verification.json");
    let manifest_path = out_dir.join("evidence-manifest.json");
    let manifest_verification_path = out_dir.join("evidence-manifest-verification.json");

    let coverage = determinism_coverage_audit_receipt(&root, &policy);
    write_evidence_json(&coverage_path, &coverage, &mut issues, "coverage_audit");

    let replay =
        match determinism_replay_receipt(root.clone(), None, Some(REQUIRED_AGENT_REPLAY_GROUP)) {
            Ok(receipt) => receipt,
            Err(_) => {
                issues.push(evidence_export_issue(
                    "export.replay_failed",
                    "replay_receipt",
                    "valid replay receipt",
                    "failed",
                ));
                Value::Null
            }
        };
    if replay.is_object() {
        write_evidence_json(&replay_path, &replay, &mut issues, "replay_receipt");
    }

    let replay_verification = if replay.is_object() {
        write_evidence_json(&replay_path, &replay, &mut issues, "replay_receipt");
        determinism_verify_replay_receipt(&root, &replay_path)
    } else {
        Value::Null
    };
    if replay_verification.is_object() {
        write_evidence_json(
            &replay_verification_path,
            &replay_verification,
            &mut issues,
            "replay_verification",
        );
    }

    let release = release_readiness_receipt(root.clone(), default_policy_path().into());
    write_evidence_json(&release_path, &release, &mut issues, "release_readiness");

    let exported_agent_receipt = if options.agent_receipt.is_some() {
        match fs::copy(&agent_path, &exported_agent_path) {
            Ok(_) => read_json_artifact(&exported_agent_path, AGENT_RECEIPT_MAX_BYTES)
                .unwrap_or(Value::Null),
            Err(_) => {
                issues.push(evidence_export_issue(
                    "export.agent_receipt_copy_failed",
                    "agent_receipt",
                    "readable agent receipt fixture",
                    "failed",
                ));
                Value::Null
            }
        }
    } else {
        match export_agent_receipt_bound_to_replay(
            &agent_path,
            &exported_agent_path,
            &replay_verification,
            &policy,
        ) {
            Ok(receipt) => receipt,
            Err(_) => {
                issues.push(evidence_export_issue(
                    "export.agent_receipt_generate_failed",
                    "agent_receipt",
                    "agent receipt bound to exported replay evidence",
                    "failed",
                ));
                Value::Null
            }
        }
    };

    let bundle = json!({
        "agent_receipt_path": "agent-receipt.json",
        "replay_receipt_path": "replay-agent.json",
        "replay_verification_path": "replay-verification.json",
        "release_readiness_path": "release-readiness.json",
        "coverage_audit_path": "coverage-audit.json",
        "agent_receipt_hash": receipt_stated_or_computed_hash(&root, &exported_agent_receipt),
        "replay_receipt_hash": receipt_stated_or_computed_hash(&root, &replay),
        "replay_verification_receipt_hash": receipt_stated_or_computed_hash(&root, &replay_verification),
        "release_readiness_receipt_hash": receipt_stated_or_computed_hash(&root, &release),
        "coverage_audit_receipt_hash": receipt_stated_or_computed_hash(&root, &coverage),
    });
    write_evidence_json(&bundle_path, &bundle, &mut issues, "agent_evidence_bundle");

    let bundle_verification = agent_evidence_bundle_verification_receipt(&root, &bundle_path);
    write_evidence_json(
        &bundle_verification_path,
        &bundle_verification,
        &mut issues,
        "agent_evidence_bundle_verification",
    );

    let manifest_options = EvidenceManifestGenerateOptions {
        root: root.clone(),
        bundle: Some(bundle_path.clone()),
        agent_receipt: Some(exported_agent_path.clone()),
        replay: Some(replay_path.clone()),
        replay_verification: Some(replay_verification_path.clone()),
        release_readiness: Some(release_path.clone()),
        coverage_audit: Some(coverage_path.clone()),
        registry: None,
        registry_verification: None,
        registry_selection: None,
        registry_fetch_plan: None,
        signer_metadata: None,
        anchor_metadata: None,
        out: None,
    };
    let (manifest, manifest_issues) = evidence_manifest_generate_manifest(&manifest_options);
    issues.extend(
        manifest_issues
            .into_iter()
            .map(|issue| evidence_export_nested_issue("export.manifest_generate_failed", issue)),
    );
    let manifest = manifest.unwrap_or(Value::Null);
    if manifest.is_object() {
        let relative_manifest = evidence_manifest_with_relative_paths(&manifest, &out_dir);
        write_evidence_json(
            &manifest_path,
            &relative_manifest,
            &mut issues,
            "evidence_manifest",
        );
    }

    let manifest_verification = evidence_manifest_verification_receipt(&root, &manifest_path);
    write_evidence_json(
        &manifest_verification_path,
        &manifest_verification,
        &mut issues,
        "evidence_manifest_verification",
    );

    if bundle_verification.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_export_issue(
            "export.bundle_verification_failed",
            "agent_evidence_bundle_verification",
            "valid bundle verification receipt",
            "invalid",
        ));
    }
    if manifest_verification.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_export_issue(
            "export.manifest_verification_failed",
            "evidence_manifest_verification",
            "valid manifest verification receipt",
            "invalid",
        ));
    }

    let mut artifacts = evidence_export_artifacts(&out_dir, &mut issues);
    artifacts.sort_by_key(|artifact| {
        artifact
            .get("role")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string()
    });
    let manifest_hash = manifest_verification
        .get("manifest_hash")
        .cloned()
        .unwrap_or(Value::Null);
    let manifest_verification_hash = receipt_stated_or_computed_hash(&root, &manifest_verification);
    let receipt = evidence_export_final_receipt(
        &root,
        &out_dir,
        issues,
        artifacts,
        manifest_hash,
        manifest_verification_hash,
    );
    write_evidence_json(
        &out_dir.join("evidence-export-receipt.json"),
        &receipt,
        &mut Vec::new(),
        "evidence_export_receipt",
    );
    receipt
}

fn export_agent_receipt_bound_to_replay(
    template_path: &Path,
    output_path: &Path,
    replay_verification: &Value,
    policy: &Policy,
) -> Result<Value, Box<dyn Error>> {
    let mut receipt = read_json_artifact(template_path, AGENT_RECEIPT_MAX_BYTES)?;
    let evidence = json!({
        "replay_artifact_path": "replay-agent.json",
        "replay_receipt_hash": replay_verification
            .get("replay_receipt_hash")
            .cloned()
            .unwrap_or(Value::Null),
        "replay_policy_hash": replay_verification
            .get("replay_policy_hash")
            .cloned()
            .unwrap_or(Value::Null),
        "replay_environment_attestation_hash": replay_verification
            .get("environment_attestation_hash")
            .cloned()
            .unwrap_or(Value::Null),
        "replay_selected_cases": replay_verification
            .get("selected_cases")
            .cloned()
            .unwrap_or_else(|| json!([])),
        "replay_selected_groups": replay_verification
            .get("selected_groups")
            .cloned()
            .unwrap_or_else(|| json!([])),
        "replay_verdict": if replay_verification.get("valid").and_then(Value::as_bool) == Some(true) { "pass" } else { "fail" },
    });
    receipt["replay_evidence"] = evidence;
    if let Some(object) = receipt.as_object_mut() {
        object.remove("receipt_hash");
    }
    let validation = agent_receipt_validation_receipt(
        "agent-receipt validate",
        Some(output_path.to_path_buf()),
        &receipt,
        false,
        policy,
    );
    receipt["receipt_hash"] = validation
        .get("agent_receipt_hash")
        .cloned()
        .unwrap_or(Value::Null);
    let final_validation = agent_receipt_validation_receipt(
        "agent-receipt validate",
        Some(output_path.to_path_buf()),
        &receipt,
        false,
        policy,
    );
    if final_validation.get("valid").and_then(Value::as_bool) != Some(true) {
        return Err("generated agent receipt failed validation".into());
    }
    write_evidence_json(output_path, &receipt, &mut Vec::new(), "agent_receipt");
    Ok(receipt)
}

fn evidence_manifest_with_relative_paths(manifest: &Value, out_dir: &Path) -> Value {
    let mut value = manifest.clone();
    if let Some(entries) = value.get_mut("artifacts").and_then(Value::as_array_mut) {
        for entry in entries {
            if let Some(path_text) = entry.get("path").and_then(Value::as_str) {
                let path = PathBuf::from(path_text);
                if let Ok(relative) = path.strip_prefix(out_dir) {
                    entry["path"] = json!(path_string(relative));
                }
            }
        }
    }
    value
}

fn evidence_export_artifacts(out_dir: &Path, issues: &mut Vec<Value>) -> Vec<Value> {
    let roles = [
        ("agent_evidence_bundle", "agent-evidence-bundle.json", true),
        (
            "agent_evidence_bundle_verification",
            "agent-evidence-bundle-verification.json",
            true,
        ),
        ("agent_receipt", "agent-receipt.json", true),
        ("coverage_audit", "coverage-audit.json", true),
        ("evidence_manifest", "evidence-manifest.json", true),
        (
            "evidence_manifest_verification",
            "evidence-manifest-verification.json",
            true,
        ),
        ("release_readiness", "release-readiness.json", true),
        ("replay_receipt", "replay-agent.json", true),
        ("replay_verification", "replay-verification.json", true),
    ];
    roles
        .iter()
        .map(|(role, relative, required)| {
            let path = out_dir.join(relative);
            let hash = match fs::read(&path) {
                Ok(bytes) => Value::String(blake3_hex(&bytes)),
                Err(_) => {
                    issues.push(evidence_export_issue(
                        "export.artifact_missing",
                        role,
                        "generated artifact file",
                        "missing",
                    ));
                    Value::Null
                }
            };
            json!({
                "role": role,
                "path": relative,
                "hash": hash,
                "required": required,
            })
        })
        .collect()
}

fn write_evidence_json(path: &Path, value: &Value, issues: &mut Vec<Value>, role: &str) {
    if let Some(parent) = path.parent() {
        if fs::create_dir_all(parent).is_err() {
            issues.push(evidence_export_issue(
                "export.artifact_parent_create_failed",
                role,
                "writable parent directory",
                "failed",
            ));
            return;
        }
    }
    if fs::write(
        path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(value).unwrap_or_default()
        ),
    )
    .is_err()
    {
        issues.push(evidence_export_issue(
            "export.artifact_write_failed",
            role,
            "writable JSON artifact",
            "failed",
        ));
    }
}

fn evidence_export_nested_issue(code: &str, issue: Value) -> Value {
    json!({
        "code": code,
        "field_path": issue.get("field_path").and_then(Value::as_str).unwrap_or("nested"),
        "expected": issue.get("expected").and_then(Value::as_str).unwrap_or("valid nested receipt"),
        "actual": issue.get("actual").and_then(Value::as_str).unwrap_or("invalid"),
    })
}

fn evidence_export_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

fn evidence_export_final_receipt(
    root: &Path,
    _out_dir: &Path,
    mut issues: Vec<Value>,
    artifacts: Vec<Value>,
    manifest_hash: Value,
    manifest_verification_receipt_hash: Value,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence export",
            "root": path_string(root),
            "output": "<out>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "artifacts": artifacts,
            "manifest_hash": manifest_hash,
            "manifest_verification_receipt_hash": manifest_verification_receipt_hash,
        }),
    )
}

fn run_evidence_manifest(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut iter = args.into_iter();
    let Some(subcommand) = iter.next() else {
        return Err("evidence-manifest requires a subcommand: verify".into());
    };
    match subcommand.as_str() {
        "generate" => run_evidence_manifest_generate(iter.collect()),
        "verify" => run_evidence_manifest_verify(iter.collect()),
        unknown => Err(format!("unknown evidence-manifest subcommand: {unknown}").into()),
    }
}

#[derive(Default)]
struct EvidenceManifestGenerateOptions {
    root: PathBuf,
    bundle: Option<PathBuf>,
    agent_receipt: Option<PathBuf>,
    replay: Option<PathBuf>,
    replay_verification: Option<PathBuf>,
    release_readiness: Option<PathBuf>,
    coverage_audit: Option<PathBuf>,
    registry: Option<PathBuf>,
    registry_verification: Option<PathBuf>,
    registry_selection: Option<PathBuf>,
    registry_fetch_plan: Option<PathBuf>,
    signer_metadata: Option<PathBuf>,
    anchor_metadata: Option<PathBuf>,
    out: Option<String>,
}

fn run_evidence_manifest_generate(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.evidence_manifest_generate.cli]
    // r[impl cairn.supersede.evidence_manifest_generate.canonical]
    // r[impl cairn.supersede.evidence_manifest_generate.metadata]
    // r[impl cairn.supersede.evidence_manifest_generate.failures]
    // r[impl cairn.supersede.registry_manifest_binding.generate]
    // r[impl cairn.supersede.registry_manifest_binding.failure]
    // r[impl cairn.supersede.registry_manifest_binding.policy]
    let options = evidence_manifest_generate_options(&args);
    let (manifest, issues) = evidence_manifest_generate_manifest(&options);
    let valid = issues.is_empty();
    if !valid {
        print_json(evidence_manifest_generate_receipt(
            &options.root,
            Value::Null,
            false,
            Value::Null,
            issues,
        ))?;
        std::process::exit(1);
    }
    let manifest = manifest.expect("manifest exists when generation issues are empty");
    let manifest_hash = Value::String(portable_json_hash(&options.root, &manifest));
    match options.out.as_deref() {
        Some("-") | None => print_json(manifest)?,
        Some(out) => {
            let path = Path::new(out);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(
                path,
                format!("{}\n", serde_json::to_string_pretty(&manifest)?),
            )?;
            print_json(evidence_manifest_generate_receipt(
                &options.root,
                manifest_hash,
                true,
                Value::String("written".to_string()),
                Vec::new(),
            ))?;
        }
    }
    Ok(())
}

fn evidence_manifest_generate_options(args: &[String]) -> EvidenceManifestGenerateOptions {
    let mut options = EvidenceManifestGenerateOptions {
        root: read_option(args.to_vec(), "--root", "."),
        ..EvidenceManifestGenerateOptions::default()
    };
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        let value = match arg.as_str() {
            "--bundle"
            | "--agent-receipt"
            | "--replay"
            | "--replay-verification"
            | "--release-readiness"
            | "--coverage-audit"
            | "--registry"
            | "--registry-verification"
            | "--registry-selection"
            | "--registry-fetch-plan"
            | "--signer-metadata"
            | "--anchor-metadata"
            | "--out"
            | "--root" => iter.next(),
            _ => None,
        };
        match (arg.as_str(), value) {
            ("--bundle", Some(value)) => options.bundle = Some(PathBuf::from(value)),
            ("--agent-receipt", Some(value)) => options.agent_receipt = Some(PathBuf::from(value)),
            ("--replay", Some(value)) => options.replay = Some(PathBuf::from(value)),
            ("--replay-verification", Some(value)) => {
                options.replay_verification = Some(PathBuf::from(value));
            }
            ("--release-readiness", Some(value)) => {
                options.release_readiness = Some(PathBuf::from(value));
            }
            ("--coverage-audit", Some(value)) => {
                options.coverage_audit = Some(PathBuf::from(value));
            }
            ("--registry", Some(value)) => {
                options.registry = Some(PathBuf::from(value));
            }
            ("--registry-verification", Some(value)) => {
                options.registry_verification = Some(PathBuf::from(value));
            }
            ("--registry-selection", Some(value)) => {
                options.registry_selection = Some(PathBuf::from(value));
            }
            ("--registry-fetch-plan", Some(value)) => {
                options.registry_fetch_plan = Some(PathBuf::from(value));
            }
            ("--signer-metadata", Some(value)) => {
                options.signer_metadata = Some(PathBuf::from(value));
            }
            ("--anchor-metadata", Some(value)) => {
                options.anchor_metadata = Some(PathBuf::from(value));
            }
            ("--out", Some(value)) => options.out = Some(value.to_string()),
            ("--root", Some(_)) => {}
            _ => {}
        }
    }
    options
}

fn evidence_manifest_generate_manifest(
    options: &EvidenceManifestGenerateOptions,
) -> (Option<Value>, Vec<Value>) {
    let mut issues = Vec::new();
    let mut artifacts = Vec::new();
    for (role, path) in [
        ("agent_evidence_bundle", options.bundle.as_deref()),
        ("agent_receipt", options.agent_receipt.as_deref()),
        ("replay_receipt", options.replay.as_deref()),
        (
            "replay_verification",
            options.replay_verification.as_deref(),
        ),
        ("release_readiness", options.release_readiness.as_deref()),
        ("coverage_audit", options.coverage_audit.as_deref()),
        ("evidence_registry", options.registry.as_deref()),
        (
            "evidence_registry_verification",
            options.registry_verification.as_deref(),
        ),
        (
            "evidence_registry_selection",
            options.registry_selection.as_deref(),
        ),
        (
            "evidence_registry_fetch_plan",
            options.registry_fetch_plan.as_deref(),
        ),
    ] {
        if let Some(path) = path {
            match read_json_artifact(path, AGENT_RECEIPT_MAX_BYTES) {
                Ok(value) => artifacts.push(json!({
                    "role": role,
                    "path": path_string(path),
                    "hash": receipt_stated_or_computed_hash(&options.root, &value),
                })),
                Err(_) => issues.push(evidence_manifest_issue(
                    "generate.artifact_read_failed",
                    role,
                    "readable JSON artifact",
                    "unreadable",
                )),
            }
        } else if ![
            "release_readiness",
            "coverage_audit",
            "evidence_registry",
            "evidence_registry_verification",
            "evidence_registry_selection",
            "evidence_registry_fetch_plan",
        ]
        .contains(&role)
        {
            issues.push(evidence_manifest_issue(
                "generate.required_artifact_missing",
                role,
                "artifact path flag",
                "missing",
            ));
        }
    }
    let signer =
        read_optional_manifest_metadata(options.signer_metadata.as_deref(), "signer", &mut issues);
    let anchor =
        read_optional_manifest_metadata(options.anchor_metadata.as_deref(), "anchor", &mut issues);
    issues.sort_by_key(canonical_json_hash);
    if !issues.is_empty() {
        return (None, issues);
    }
    let mut object = serde_json::Map::new();
    object.insert("artifacts".to_string(), Value::Array(artifacts));
    if let Some(value) = signer {
        object.insert("signer".to_string(), value);
    }
    if let Some(value) = anchor {
        object.insert("anchor".to_string(), value);
    }
    (Some(Value::Object(object)), issues)
}

fn read_optional_manifest_metadata(
    path: Option<&Path>,
    field: &str,
    issues: &mut Vec<Value>,
) -> Option<Value> {
    let path = path?;
    match read_json_artifact(path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) if value.is_object() => Some(value),
        Ok(_) => {
            issues.push(evidence_manifest_issue(
                "generate.metadata_not_object",
                field,
                "JSON object",
                "non-object",
            ));
            None
        }
        Err(_) => {
            issues.push(evidence_manifest_issue(
                "generate.metadata_read_failed",
                field,
                "readable JSON object",
                "unreadable",
            ));
            None
        }
    }
}

fn evidence_manifest_generate_receipt(
    root: &Path,
    manifest_hash: Value,
    valid: bool,
    output: Value,
    issues: Vec<Value>,
) -> Value {
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence-manifest generate",
            "root": path_string(root),
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "manifest_hash": manifest_hash,
            "output": output,
        }),
    )
}

fn run_evidence_manifest_verify(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = evidence_manifest_options(&args)?;
    let receipt = evidence_manifest_verification_receipt(&options.root, &options.path);
    let valid = receipt
        .get("valid")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    print_json(receipt)?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

fn evidence_manifest_options(args: &[String]) -> Result<EvidenceManifestOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let path = args
        .iter()
        .find(|arg| !arg.starts_with("--") && *arg != &root)
        .ok_or("evidence-manifest verify requires <manifest.json>")?;
    Ok(EvidenceManifestOptions {
        root,
        path: PathBuf::from(path),
    })
}

fn evidence_manifest_verification_receipt(root: &Path, manifest_path: &Path) -> Value {
    // r[impl cairn.supersede.signed_evidence_manifest.verify]
    // r[impl cairn.supersede.signed_evidence_manifest.chain]
    // r[impl cairn.supersede.signed_evidence_manifest.metadata]
    let manifest = match read_json_artifact(manifest_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            return evidence_manifest_receipt(
                root,
                Value::Null,
                Vec::new(),
                Value::Null,
                Value::Null,
                Value::Null,
                vec![evidence_manifest_issue(
                    "manifest.read_failed",
                    "manifest",
                    "readable JSON object",
                    "unreadable",
                )],
            )
        }
    };
    let manifest_hash = Value::String(portable_json_hash(root, &manifest));
    let mut issues = Vec::new();
    if !manifest.is_object() {
        issues.push(evidence_manifest_issue(
            "manifest.not_object",
            "manifest",
            "object",
            "non-object",
        ));
    }
    let signer_metadata_hash = optional_metadata_hash(&manifest, "signer", &mut issues);
    let anchor_metadata_hash = optional_metadata_hash(&manifest, "anchor", &mut issues);
    let base = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    let mut artifacts = Vec::new();
    match manifest.get("artifacts").and_then(Value::as_array) {
        Some(entries) if !entries.is_empty() => {
            for entry in entries {
                artifacts.push(verify_evidence_manifest_artifact(
                    root,
                    base,
                    entry,
                    &mut issues,
                ));
            }
        }
        _ => issues.push(evidence_manifest_issue(
            "manifest.artifacts_missing",
            "artifacts",
            "non-empty array",
            "missing",
        )),
    }
    let coverage_audit_receipt_hash = artifacts
        .iter()
        .find(|artifact| artifact.get("role").and_then(Value::as_str) == Some("coverage_audit"))
        .and_then(|artifact| artifact.get("hash"))
        .cloned()
        .unwrap_or(Value::Null);
    let has_bundle = artifacts.iter().any(|artifact| {
        artifact.get("role").and_then(Value::as_str) == Some("agent_evidence_bundle")
    });
    if !has_bundle {
        issues.push(evidence_manifest_issue(
            "manifest.bundle_missing",
            "artifacts.role",
            "agent_evidence_bundle",
            "missing",
        ));
    }
    issues.sort_by_key(canonical_json_hash);
    evidence_manifest_receipt(
        root,
        manifest_hash,
        artifacts,
        signer_metadata_hash,
        anchor_metadata_hash,
        coverage_audit_receipt_hash,
        issues,
    )
}

fn verify_evidence_manifest_artifact(
    root: &Path,
    base: &Path,
    entry: &Value,
    issues: &mut Vec<Value>,
) -> Value {
    let role = entry
        .get("role")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let schema = entry.get("schema").cloned().unwrap_or(Value::Null);
    let mut artifact_valid = true;
    if !entry.is_object() {
        artifact_valid = false;
        issues.push(evidence_manifest_issue(
            "artifact.not_object",
            "artifacts[]",
            "object",
            "non-object",
        ));
    }
    let path = match entry.get("path").and_then(Value::as_str) {
        Some(text) if !text.is_empty() => {
            let path = PathBuf::from(text);
            Some(if path.is_absolute() {
                path
            } else {
                base.join(path)
            })
        }
        _ => {
            artifact_valid = false;
            issues.push(evidence_manifest_issue(
                "artifact.path_missing",
                role,
                "non-empty string path",
                "missing",
            ));
            None
        }
    };
    let expected_hash = entry.get("hash").and_then(Value::as_str);
    if expected_hash.is_none() {
        artifact_valid = false;
        issues.push(evidence_manifest_issue(
            "artifact.hash_missing",
            role,
            "declared artifact hash",
            "missing",
        ));
    }
    let mut actual_hash = Value::Null;
    if let Some(path) = path.as_deref() {
        match read_json_artifact(path, AGENT_RECEIPT_MAX_BYTES) {
            Ok(value) => {
                actual_hash = receipt_stated_or_computed_hash(root, &value);
                if let Some(expected) = expected_hash {
                    if actual_hash.as_str() != Some(expected) {
                        artifact_valid = false;
                        issues.push(evidence_manifest_issue(
                            "artifact.hash_mismatch",
                            role,
                            "declared hash matches artifact",
                            "mismatch",
                        ));
                    }
                }
                if !validate_manifest_artifact_role(root, path, role, &value, issues) {
                    artifact_valid = false;
                }
            }
            Err(_) => {
                artifact_valid = false;
                issues.push(evidence_manifest_issue(
                    "artifact.read_failed",
                    role,
                    "readable JSON artifact",
                    "unreadable",
                ));
            }
        }
    }
    json!({
        "role": role,
        "hash": actual_hash,
        "schema": schema,
        "valid": artifact_valid,
    })
}

fn validate_manifest_artifact_role(
    root: &Path,
    path: &Path,
    role: &str,
    value: &Value,
    issues: &mut Vec<Value>,
) -> bool {
    // r[impl cairn.supersede.registry_manifest_binding.roles]
    // r[impl cairn.supersede.registry_manifest_binding.verify]
    match role {
        "agent_evidence_bundle" => {
            let receipt = agent_evidence_bundle_verification_receipt(root, path);
            if receipt["valid"].as_bool() == Some(true) {
                true
            } else {
                issues.push(evidence_manifest_issue(
                    "artifact.bundle_invalid",
                    role,
                    "valid agent evidence bundle",
                    "invalid",
                ));
                false
            }
        }
        "replay_receipt" => {
            let receipt = determinism_verify_replay_receipt(root, path);
            if receipt["valid"].as_bool() == Some(true) {
                true
            } else {
                issues.push(evidence_manifest_issue(
                    "artifact.replay_invalid",
                    role,
                    "valid replay receipt",
                    "invalid",
                ));
                false
            }
        }
        "replay_verification" => receipt_valid_or_issue(
            value,
            issues,
            role,
            "artifact.replay_verification_invalid",
            "valid replay verification receipt",
        ),
        "agent_receipt" => match load_policy_file(default_policy_path().into()) {
            Ok(policy) => {
                let receipt = agent_receipt_validation_receipt(
                    "agent-receipt validate",
                    None,
                    value,
                    false,
                    &policy,
                );
                if receipt["valid"].as_bool() == Some(true) {
                    true
                } else {
                    issues.push(evidence_manifest_issue(
                        "artifact.agent_receipt_invalid",
                        role,
                        "valid agent receipt",
                        "invalid",
                    ));
                    false
                }
            }
            Err(_) => {
                issues.push(evidence_manifest_issue(
                    "artifact.policy_load_failed",
                    role,
                    "generated policy loads",
                    "failed",
                ));
                false
            }
        },
        "release_readiness" => receipt_valid_or_issue(
            value,
            issues,
            role,
            "artifact.release_readiness_invalid",
            "valid release-readiness receipt",
        ),
        "coverage_audit" => receipt_valid_or_issue(
            value,
            issues,
            role,
            "artifact.coverage_audit_invalid",
            "valid coverage-audit receipt",
        ),
        "evidence_registry" => {
            let receipt = evidence_registry_verify_receipt(&EvidenceRegistryVerifyOptions {
                root: root.to_path_buf(),
                registry: Some(path.to_path_buf()),
            });
            if receipt["valid"].as_bool() == Some(true) {
                true
            } else {
                issues.push(evidence_manifest_issue(
                    "artifact.registry_invalid",
                    role,
                    "valid evidence registry",
                    "invalid",
                ));
                false
            }
        }
        "evidence_registry_verification" => receipt_valid_or_issue(
            value,
            issues,
            role,
            "artifact.registry_verification_invalid",
            "valid evidence registry verification receipt",
        ),
        "evidence_registry_selection" => receipt_valid_or_issue(
            value,
            issues,
            role,
            "artifact.registry_selection_invalid",
            "valid evidence registry selection receipt",
        ),
        "evidence_registry_fetch_plan" => receipt_valid_or_issue(
            value,
            issues,
            role,
            "artifact.registry_fetch_plan_invalid",
            "valid evidence registry fetch-plan receipt",
        ),
        _ => {
            issues.push(evidence_manifest_issue(
                "artifact.unknown_role",
                role,
                "known evidence role",
                "unknown",
            ));
            false
        }
    }
}

fn receipt_valid_or_issue(
    value: &Value,
    issues: &mut Vec<Value>,
    role: &str,
    code: &str,
    expected: &str,
) -> bool {
    if value.get("valid").and_then(Value::as_bool) == Some(true) {
        true
    } else {
        issues.push(evidence_manifest_issue(code, role, expected, "invalid"));
        false
    }
}

fn optional_metadata_hash(manifest: &Value, field: &str, issues: &mut Vec<Value>) -> Value {
    match manifest.get(field) {
        None | Some(Value::Null) => Value::Null,
        Some(value) if value.is_object() => Value::String(canonical_json_hash(value)),
        Some(_) => {
            issues.push(evidence_manifest_issue(
                "manifest.metadata_invalid",
                field,
                "object",
                "invalid",
            ));
            Value::Null
        }
    }
}

fn evidence_manifest_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

fn evidence_manifest_receipt(
    root: &Path,
    manifest_hash: Value,
    artifacts: Vec<Value>,
    signer_metadata_hash: Value,
    anchor_metadata_hash: Value,
    coverage_audit_receipt_hash: Value,
    issues: Vec<Value>,
) -> Value {
    // r[impl cairn.supersede.signed_evidence_manifest.schema]
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence-manifest verify",
            "root": path_string(root),
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "manifest_hash": manifest_hash,
            "artifacts": artifacts,
            "signer_metadata_hash": signer_metadata_hash,
            "anchor_metadata_hash": anchor_metadata_hash,
            "coverage_audit_receipt_hash": coverage_audit_receipt_hash,
        }),
    )
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
    if let Some(attestation_hash) = evidence.get("replay_environment_attestation_hash") {
        if !attestation_hash.as_str().is_some_and(is_blake3_hex_text) {
            issues.push(AgentRunGateIssue {
                code: "replay_evidence.invalid_environment_attestation_hash".to_string(),
                message:
                    "replay_evidence.replay_environment_attestation_hash must be a BLAKE3 hex hash"
                        .to_string(),
            });
        }
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
        "replay_environment_attestation_hash": evidence.get("replay_environment_attestation_hash").cloned().unwrap_or(Value::Null),
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
    "usage: cairn <command> [args]\n\ncommands:\n  init [--root PATH]\n  change create <name> [--root PATH]\n  change list [--root PATH]\n  change show <name> [--root PATH]\n  spec list [--root PATH]\n  spec show <id> [--root PATH]\n  status [--root PATH]\n  doctor [--root PATH] [--policy PATH]\n  validate [--root PATH] [--policy PATH] [--strict]\n  release-readiness [--root PATH] [--policy PATH]\n  determinism coverage-audit [--root PATH]\n  determinism replay [--root PATH] [--case ID] [--group ID] [--format json|jsonl|summary] [--out PATH]\n  determinism verify-receipt <receipt.json> [--root PATH]\n  agent-receipt validate <path>\n  agent-receipt verify-bundle <bundle.json> [--root PATH]\n  agent-receipt smoke-mcp [--root PATH]\n  evidence export --root PATH --out DIR [--agent-receipt PATH]\n  evidence verify --root PATH --bundle DIR\n  evidence import --root PATH --bundle DIR --into DIR\n  evidence republish --root PATH --imported DIR --out DIR\n  evidence index --root PATH --bundle DIR --out FILE\n  evidence index verify --root PATH --index FILE\n  evidence registry build --root PATH --index FILE [--index FILE ...] --out FILE [--channel CHANNEL] [--release YYYY-MM-DD] [--platform PLATFORM] [--compatibility-class CLASS]\n  evidence registry verify --root PATH --registry FILE\n  evidence registry mirror-verify --root PATH --registry FILE --mirror DIR\n  evidence registry select --root PATH --registry FILE --out FILE [--bundle-kind KIND] [--policy-hash HASH] [--channel CHANNEL] [--release YYYY-MM-DD] [--platform PLATFORM] [--compatibility-class CLASS] [--index-receipt-hash HASH]\n  evidence registry fetch-plan --root PATH --registry FILE --out FILE [--bundle-kind KIND] [--policy-hash HASH] [--channel CHANNEL] [--release YYYY-MM-DD] [--platform PLATFORM] [--compatibility-class CLASS]\n  evidence-manifest generate --bundle PATH --agent-receipt PATH --replay PATH --replay-verification PATH [--release-readiness PATH] [--coverage-audit PATH] [--registry PATH] [--registry-verification PATH] [--registry-selection PATH] [--registry-fetch-plan PATH] [--signer-metadata PATH] [--anchor-metadata PATH] [--out PATH|-] [--root PATH]\n  evidence-manifest verify <manifest.json> [--root PATH]\n  policy [--policy PATH]\n  policy export [--source PATH] [--output PATH] [--check]\n  tracey coverage [--root PATH] [--json]\n  rename-classification [--output PATH]\n  migrate <openspec|ironspec> [--root PATH] [--policy PATH] [--execute]\n  mcp [--describe] [--call TOOL JSON] [--allow-mutations]\n  gate <proposal|design|tasks> <change> [--root PATH] [--policy PATH]\n  gate-context <change> [--root PATH]\n  sync <change> [--root PATH] [--policy PATH] [--execute]\n  archive <change> [--root PATH] [--policy PATH] [--execute]"
}

fn print_usage() {
    println!("{}", usage_text());
}
