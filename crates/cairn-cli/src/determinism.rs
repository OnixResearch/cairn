use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use cairn_core::{
    build_archive_plan, build_openspec_migration_plan, build_sync_plan, canonical_json_hash,
    evaluate_gate_snapshot, GateStage, LifecycleLayout, Policy,
};
use serde_json::{json, Value};

use crate::agent_receipt::{
    agent_receipt_validation_receipt, is_blake3_hex_text, read_agent_receipt_file,
};
use crate::lifecycle::parse_gate_stage;
use crate::mcp::{mcp_call_tool, mcp_tool_catalog};
use crate::native::{cairn_validation_receipt, change_status_json, repo_status_json};
use crate::{
    archive_plan_json, collect_files, collect_layout_migration_inputs, default_policy_path,
    detect_layout, file_content_manifest, find_change, gate_input_json, gate_report_json, has_flag,
    load_policy_file, migration_plan_json, mutation_gate_blockers, path_string, portable_json_hash,
    portable_text, print_json, read_gate_snapshot, read_option, read_string_option,
    release_readiness_receipt, scan_repo, sync_plan_json, with_portable_receipt_hash,
};

pub(crate) fn run_determinism(args: Vec<String>) -> Result<(), Box<dyn Error>> {
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
pub(crate) struct DeterminismSurface {
    pub(crate) id: String,
    pub(crate) command: String,
    pub(crate) surface_kind: String,
    pub(crate) receipt_command: String,
    pub(crate) required_replay_cases: Vec<String>,
    pub(crate) required_replay_groups: Vec<String>,
    pub(crate) determinism_claim: String,
}

pub(crate) fn determinism_coverage_audit_receipt(root: &Path, policy: &Policy) -> Value {
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

pub(crate) fn determinism_surfaces_from_policy(
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

pub(crate) fn determinism_verify_replay_receipt(root: &Path, path: &Path) -> Value {
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
pub(crate) struct ReplayGroup {
    pub(crate) id: String,
    pub(crate) cases: Vec<String>,
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
pub(crate) struct ReplayCase {
    pub(crate) id: String,
    command_kind: String,
    args: Vec<String>,
    fixture_root: String,
    comparison_modes: Vec<String>,
    kind: ReplayCaseKind,
    hash_fields: Vec<String>,
    display_only_paths: Vec<String>,
    normalization_rules: Vec<String>,
}

pub(crate) fn determinism_replay_receipt(
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

pub(crate) fn replay_cases_from_policy(policy: &Policy) -> Result<Vec<ReplayCase>, Box<dyn Error>> {
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

pub(crate) fn replay_groups_from_policy(
    policy: &Policy,
) -> Result<Vec<ReplayGroup>, Box<dyn Error>> {
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
