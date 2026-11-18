use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use cairn_core::{
    canonical_json, content_bound_requirement_registry_issues_json,
    evaluate_content_bound_trace_index, parse_content_bound_requirement_registry_value,
    parse_content_bound_trace_index_value, query_content_bound_trace_index,
    verify_content_bound_requirement_registry, verify_content_bound_trace_index,
    ContentBoundRequirementRegistry, ContentBoundRequirementRegistryIssue, ContentBoundTraceIndex,
    ContentBoundTraceIndexIssue, CONTENT_BOUND_TRACE_INDEX_BOUNDARY,
    CONTENT_BOUND_TRACE_INDEX_SCHEMA_V1,
};
use cairn_core::verified::traceability_comparison::{
    buoy_comparison_fixture_tree_blake3, buoy_comparison_manifest_blake3,
    evaluate_buoy_comparison, parse_buoy_comparison_manifest_value, BuoyComparisonInputs,
    BuoyComparisonManifest, BuoyComparisonObservation, BUOY_COMPARISON_INDEX_MUTATION_SCHEMA_VERSION,
    BUOY_COMPARISON_MODE_EXPORT_AND_VERIFY, BUOY_COMPARISON_MODE_NOT_EXECUTED,
    BUOY_COMPARISON_MODE_REJECT_BASE_INDEX, BUOY_COMPARISON_MODE_VERIFY_BASE_INDEX,
};
use serde_json::{json, Value};

use crate::{
    requirement_id_registry::read_accepted_specifications,
    support::{
        default_policy_path, path_string, print_json, read_option, read_string_option,
        reject_positional_arguments, reject_unknown_options, with_portable_receipt_hash,
        CapabilityRoot, CapabilityRootKind,
    },
    tracey::{collect_traceability_files, selected_traceability_profile},
};

const TRACE_INDEX_EXPORT_RECEIPT_SCHEMA_V1: &str = "cairn.trace-index.export-receipt.v1";
pub(crate) const TRACE_INDEX_VERIFY_RECEIPT_SCHEMA_V1: &str = "cairn.trace-index.verify-receipt.v1";
const TRACE_INDEX_SHOW_RECEIPT_SCHEMA_V1: &str = "cairn.trace-index.show-receipt.v1";
const BYTES_PER_KIBIBYTE: u64 = 1_024;
const KIBIBYTES_PER_MEBIBYTE: u64 = 1_024;
const MAX_POLICY_MEBIBYTES: u64 = 16;
const MAX_REGISTRY_MEBIBYTES: u64 = 64;
const MAX_TRACE_INDEX_MEBIBYTES: u64 = 128;
const MAX_POLICY_BYTES: u64 = MAX_POLICY_MEBIBYTES * KIBIBYTES_PER_MEBIBYTE * BYTES_PER_KIBIBYTE;
const MAX_REGISTRY_BYTES: u64 =
    MAX_REGISTRY_MEBIBYTES * KIBIBYTES_PER_MEBIBYTE * BYTES_PER_KIBIBYTE;
const MAX_TRACE_INDEX_BYTES: u64 =
    MAX_TRACE_INDEX_MEBIBYTES * KIBIBYTES_PER_MEBIBYTE * BYTES_PER_KIBIBYTE;
const MAX_COMPARISON_MANIFEST_BYTES: u64 = 1_048_576;
const MAX_COMPARISON_FIXTURE_FILES: usize = 128;
const MAX_COMPARISON_FIXTURE_FILE_BYTES: u64 = 1_048_576;
const MAX_COMPARISON_FIXTURE_TOTAL_BYTES: u64 = 8_388_608;
const COMPARISON_MANIFEST_FILE: &str = "manifest.json";
const COMPARISON_POLICY_RELATIVE: &str = "cairn-policy/generated/cairn-policy.json";

pub(crate) fn run_trace_index(subcommand: &str, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    match subcommand {
        "export" => run_export(args),
        "verify" => run_verify(args),
        "show" => run_show(args),
        "compare" => run_compare(args),
        other => Err(format!("unknown traceability command: {other}").into()),
    }
}

fn run_export(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.trace_index.cli]
    reject_unknown_options(
        &args,
        &["--root", "--policy", "--profile", "--registry", "--out"],
    )?;
    reject_positional_arguments(&args)?;
    let root = read_option(args.clone(), "--root", ".");
    let policy_path = read_option(args.clone(), "--policy", default_policy_path());
    let profile_id = read_string_option(&args, "--profile");
    let registry_path = required_path_option(&args, "--registry", "export")?;
    let out = required_path_option(&args, "--out", "export")?;
    let policy = crate::support::load_policy_file(policy_path.clone())?;
    let profile = selected_traceability_profile(&policy, profile_id.as_deref())?;
    let evidence_files = collect_traceability_files(&root, &profile.evidence_sources)?;
    let policy_bytes = read_bounded_file(&policy_path, MAX_POLICY_BYTES, "traceability policy")?;
    let repository = CapabilityRoot::open(CapabilityRootKind::Repository, &root)?;
    let specifications = read_accepted_specifications(&repository)?;
    let registry = read_registry(&registry_path)?;
    let registry_report = verify_content_bound_requirement_registry(
        &registry,
        specifications,
        &policy_bytes,
        &registry.repository_id,
    );
    if !registry_report.valid {
        print_json(export_receipt(
            &root,
            &out,
            None,
            Vec::new(),
            registry_report.issues,
        ))?;
        std::process::exit(1);
    }

    let evaluation = evaluate_content_bound_trace_index(&registry, &profile, evidence_files);
    let Some(index) = evaluation.index else {
        print_json(export_receipt(
            &root,
            &out,
            None,
            evaluation.issues,
            Vec::new(),
        ))?;
        std::process::exit(1);
    };
    let bytes = canonical_index_bytes(&index)?;
    let (output_root, output_relative) =
        CapabilityRoot::open_file_parent(CapabilityRootKind::EvidenceBundle, &out, true)?;
    output_root.write_new(&output_relative, &bytes)?;
    print_json(export_receipt(
        &root,
        &out,
        Some(&index),
        Vec::new(),
        Vec::new(),
    ))
}

fn run_verify(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.trace_index.cli]
    // r[impl cairn.trace_index.freshness]
    reject_unknown_options(
        &args,
        &["--root", "--policy", "--profile", "--registry", "--index"],
    )?;
    reject_positional_arguments(&args)?;
    let root = read_option(args.clone(), "--root", ".");
    let policy_path = read_option(args.clone(), "--policy", default_policy_path());
    let profile_id = read_string_option(&args, "--profile");
    let registry_path = required_path_option(&args, "--registry", "verify")?;
    let index_path = required_path_option(&args, "--index", "verify")?;
    let receipt = current_root_trace_index_verification_receipt(
        &root,
        &policy_path,
        profile_id.as_deref(),
        &registry_path,
        &index_path,
        &index_path,
    )?;
    let valid = receipt.get("valid").and_then(Value::as_bool) == Some(true);
    print_json(receipt)?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

pub(crate) fn current_root_trace_index_verification_receipt(
    root: &Path,
    policy_path: &Path,
    profile_id: Option<&str>,
    registry_path: &Path,
    index_path: &Path,
    displayed_index_path: &Path,
) -> Result<Value, Box<dyn Error>> {
    let resolved_policy_path = if policy_path.is_absolute() {
        policy_path.to_path_buf()
    } else {
        root.join(policy_path)
    };
    let policy = crate::support::load_policy_file(resolved_policy_path.clone())?;
    let profile = selected_traceability_profile(&policy, profile_id)?;
    let evidence_files = collect_traceability_files(root, &profile.evidence_sources)?;
    let policy_bytes = read_bounded_file(
        &resolved_policy_path,
        MAX_POLICY_BYTES,
        "traceability policy",
    )?;
    let repository = CapabilityRoot::open(CapabilityRootKind::Repository, root)?;
    let specifications = read_accepted_specifications(&repository)?;
    let registry = read_registry(registry_path)?;
    let registry_report = verify_content_bound_requirement_registry(
        &registry,
        specifications,
        &policy_bytes,
        &registry.repository_id,
    );
    if !registry_report.valid {
        return Ok(trace_index_verify_receipt(
            root,
            displayed_index_path,
            None,
            Vec::new(),
            registry_report.issues,
        ));
    }
    let index = read_index(index_path)?;
    let report = verify_content_bound_trace_index(&index, &registry, &profile, evidence_files);
    Ok(trace_index_verify_receipt(
        root,
        displayed_index_path,
        Some(&index),
        report.issues,
        Vec::new(),
    ))
}

fn run_show(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.trace_index.query]
    // r[impl cairn.trace_index.cli]
    let Some(requirement_id) = args
        .first()
        .filter(|value| !value.is_empty() && !value.starts_with('-'))
    else {
        return Err("traceability show requires <requirement-id>".into());
    };
    reject_unknown_options(&args[1..], &["--root", "--index"])?;
    reject_positional_arguments(&args[1..])?;
    let root = read_option(args.clone(), "--root", ".");
    let index_path = required_path_option(&args, "--index", "show")?;
    let index = read_index(&index_path)?;
    let evaluation = query_content_bound_trace_index(&index, requirement_id);
    let valid = evaluation.query.is_some() && evaluation.issues.is_empty();
    let query = evaluation.query.map(|query| {
        json!({
            "requirement": query.requirement,
            "links": query.links,
        })
    });
    print_json(with_portable_receipt_hash(
        &root,
        json!({
            "command": "traceability show",
            "schema": TRACE_INDEX_SHOW_RECEIPT_SCHEMA_V1,
            "root": path_string(&root),
            "index": path_string(&index_path),
            "index_blake3": index.index_blake3,
            "requirement_id": requirement_id,
            "query": query,
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "boundary": CONTENT_BOUND_TRACE_INDEX_BOUNDARY,
            "issues": evaluation.issues,
        }),
    ))?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

fn run_compare(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    reject_unknown_options(
        &args,
        &["--root", "--policy", "--fixtures", "--out", "--observations"],
    )?;
    reject_positional_arguments(&args)?;
    let root = read_option(args.clone(), "--root", ".");
    let policy_path = read_option(args.clone(), "--policy", default_policy_path());
    let fixtures = required_path_option(&args, "--fixtures", "compare")?;
    let out = required_path_option(&args, "--out", "compare")?;
    let observations_out = read_string_option(&args, "--observations")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from);

    let resolved_policy = if policy_path.is_absolute() {
        policy_path.clone()
    } else {
        root.join(&policy_path)
    };
    let policy_bytes = read_bounded_file(
        &resolved_policy,
        MAX_POLICY_BYTES,
        "traceability policy",
    )?;
    let policy_blake3 = blake3::hash(&policy_bytes).to_hex().to_string();
    let manifest_path = fixtures.join(COMPARISON_MANIFEST_FILE);
    let manifest_bytes = read_bounded_file(
        &manifest_path,
        MAX_COMPARISON_MANIFEST_BYTES,
        "Buoy comparison manifest",
    )?;
    let manifest_value: Value = serde_json::from_slice(&manifest_bytes).map_err(|error| {
        format!(
            "failed to parse Buoy comparison manifest {}: {error}",
            manifest_path.display()
        )
    })?;
    let manifest: BuoyComparisonManifest =
        parse_buoy_comparison_manifest_value(&manifest_value)
            .map_err(|issue| format!("invalid Buoy comparison manifest: {}", issue.message))?;
    let manifest_blake3 = buoy_comparison_manifest_blake3(&manifest_bytes);

    let mut fixture_blake3 = BTreeMap::new();
    for case in &manifest.cases {
        if case.execution.mode == BUOY_COMPARISON_MODE_NOT_EXECUTED {
            continue;
        }
        let relative = case.fixture.as_ref().ok_or_else(|| {
            format!(
                "comparison case {} requires a fixture path before execution",
                case.id
            )
        })?;
        let fixture_root = fixtures.join(relative);
        let files = read_comparison_fixture_files(&fixture_root)?;
        fixture_blake3.insert(
            case.id.clone(),
            buoy_comparison_fixture_tree_blake3(&files),
        );
    }

    let work = comparison_work_directory()?;
    let execution = execute_comparison_cases(
        &manifest,
        &fixtures,
        &work,
        &policy_bytes,
    );
    let observations = match execution {
        Ok(observations) => observations,
        Err(error) => {
            let _ = fs::remove_dir_all(&work);
            return Err(error);
        }
    };
    for case in &manifest.cases {
        if case.execution.mode == BUOY_COMPARISON_MODE_NOT_EXECUTED {
            continue;
        }
        let relative = case.fixture.as_ref().ok_or_else(|| {
            format!(
                "comparison case {} requires a fixture path before execution",
                case.id
            )
        })?;
        let files = read_comparison_fixture_files(&fixtures.join(relative))?;
        let current = buoy_comparison_fixture_tree_blake3(&files);
        if fixture_blake3.get(&case.id) != Some(&current) {
            let _ = fs::remove_dir_all(&work);
            return Err(format!(
                "comparison fixture changed during execution: {}",
                case.id
            )
            .into());
        }
    }
    if let Some(directory) = observations_out {
        if let Err(error) = write_comparison_observations(&directory, &observations) {
            let _ = fs::remove_dir_all(&work);
            return Err(error);
        }
    }
    let inputs = BuoyComparisonInputs {
        manifest_blake3,
        policy_blake3,
        fixture_blake3,
        observations,
    };
    let evaluation = evaluate_buoy_comparison(&manifest, &inputs);
    let valid = evaluation.valid;
    let mut receipt = evaluation.receipt;
    receipt["root"] = json!(path_string(&root));
    receipt["fixtures"] = json!(path_string(&fixtures));
    receipt["policy"] = json!(path_string(&resolved_policy));
    let receipt = with_portable_receipt_hash(&root, receipt);
    let bytes = (canonical_json(&receipt) + "\n").into_bytes();
    let (output_root, output_relative) =
        CapabilityRoot::open_file_parent(CapabilityRootKind::EvidenceBundle, &out, true)?;
    output_root.write_new(&output_relative, &bytes)?;
    print_json(receipt)?;
    let _ = fs::remove_dir_all(&work);
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

fn execute_comparison_cases(
    manifest: &BuoyComparisonManifest,
    fixtures: &Path,
    work: &Path,
    policy_bytes: &[u8],
) -> Result<Vec<BuoyComparisonObservation>, Box<dyn Error>> {
    let mut observations = Vec::new();
    let mut base_artifacts: BTreeMap<String, (Vec<u8>, Vec<u8>)> = BTreeMap::new();
    for case in &manifest.cases {
        if case.execution.mode == BUOY_COMPARISON_MODE_NOT_EXECUTED {
            continue;
        }
        let relative = case.fixture.as_ref().ok_or_else(|| {
            format!(
                "comparison case {} requires a fixture path before execution",
                case.id
            )
        })?;
        let case_root = work.join(&case.id);
        let repository = case_root.join("repo");
        copy_comparison_fixture_tree(&fixtures.join(relative), &repository)?;
        let policy = repository.join(COMPARISON_POLICY_RELATIVE);
        write_comparison_file(&policy, policy_bytes)?;

        let run = match case.execution.mode.as_str() {
            BUOY_COMPARISON_MODE_EXPORT_AND_VERIFY => {
                let registry = repository.join("target/requirement-registry.json");
                let registry_run = run_cairn_command(&registry_export_arguments(
                    &repository,
                    &case.id,
                    &registry,
                ))?;
                if registry_run.exit_code != 0 {
                    registry_run
                } else {
                    let index = repository.join("target/trace-index.json");
                    let index_run = run_cairn_command(&index_export_arguments(
                        &repository,
                        &policy,
                        &registry,
                        &index,
                    ))?;
                    if index_run.exit_code != 0 {
                        index_run
                    } else {
                        let verification = run_cairn_command(&index_verify_arguments(
                            &repository,
                            &policy,
                            &registry,
                            &index,
                        ))?;
                        if verification.exit_code == 0 {
                            base_artifacts.insert(
                                case.id.clone(),
                                (fs::read(&registry)?, fs::read(&index)?),
                            );
                        }
                        verification
                    }
                }
            }
            BUOY_COMPARISON_MODE_VERIFY_BASE_INDEX
            | BUOY_COMPARISON_MODE_REJECT_BASE_INDEX => {
                let base_id = case
                    .execution
                    .base
                    .as_deref()
                    .ok_or_else(|| "base-index comparison execution requires a base case".to_string())?;
                let (base_registry, base_index) = base_artifacts
                    .get(base_id)
                    .ok_or_else(|| format!("base case {base_id} was not executed"))?
                    .clone();
                let registry = repository.join("target/requirement-registry.json");
                let preparation = match case.execution.registry_source.as_deref() {
                    Some("base") => {
                        write_comparison_file(&registry, &base_registry)?;
                        None
                    }
                    Some("case") => Some(run_cairn_command(&registry_export_arguments(
                        &repository,
                        &case.id,
                        &registry,
                    ))?),
                    _ => return Err("base-index comparison execution requires a registry source".into()),
                };
                if preparation
                    .as_ref()
                    .is_some_and(|run| run.exit_code != 0)
                {
                    preparation.expect("preparation run")
                } else {
                    let index = repository.join("target/trace-index.json");
                    let index_bytes =
                        if case.execution.mode == BUOY_COMPARISON_MODE_REJECT_BASE_INDEX {
                            if case.execution.index_mutation.as_deref()
                                != Some(BUOY_COMPARISON_INDEX_MUTATION_SCHEMA_VERSION)
                            {
                                return Err(
                                    "schema-drift comparison execution requires the schema_version mutation"
                                        .into(),
                                );
                            }
                            mutate_comparison_index_schema(&base_index)?
                        } else {
                            base_index
                        };
                    write_comparison_file(&index, &index_bytes)?;
                    run_cairn_command(&index_verify_arguments(
                        &repository,
                        &policy,
                        &registry,
                        &index,
                    ))?
                }
            }
            other => return Err(format!("unsupported comparison execution mode: {other}").into()),
        };
        observations.push(BuoyComparisonObservation {
            case_id: case.id.clone(),
            exit_code: run.exit_code,
            stdout: run.stdout,
            stderr: run.stderr,
        });
    }
    Ok(observations)
}

struct ComparisonRun {
    exit_code: i32,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

fn run_cairn_command(args: &[String]) -> Result<ComparisonRun, Box<dyn Error>> {
    let output = Command::new(std::env::current_exe()?).args(args).output()?;
    Ok(ComparisonRun {
        exit_code: output.status.code().unwrap_or(-1),
        stdout: output.stdout,
        stderr: output.stderr,
    })
}

fn registry_export_arguments(
    repository: &Path,
    revision: &str,
    registry: &Path,
) -> Vec<String> {
    vec![
        "requirement-registry".to_string(),
        "export".to_string(),
        "--root".to_string(),
        path_string(repository),
        "--policy".to_string(),
        COMPARISON_POLICY_RELATIVE.to_string(),
        "--repository-id".to_string(),
        "onixresearch/buoy-comparison".to_string(),
        "--revision-kind".to_string(),
        "opaque".to_string(),
        "--revision".to_string(),
        revision.to_string(),
        "--out".to_string(),
        path_string(registry),
    ]
}

fn index_export_arguments(
    repository: &Path,
    policy: &Path,
    registry: &Path,
    index: &Path,
) -> Vec<String> {
    vec![
        "traceability".to_string(),
        "export".to_string(),
        "--root".to_string(),
        path_string(repository),
        "--policy".to_string(),
        path_string(policy),
        "--registry".to_string(),
        path_string(registry),
        "--out".to_string(),
        path_string(index),
    ]
}

fn index_verify_arguments(
    repository: &Path,
    policy: &Path,
    registry: &Path,
    index: &Path,
) -> Vec<String> {
    vec![
        "traceability".to_string(),
        "verify".to_string(),
        "--root".to_string(),
        path_string(repository),
        "--policy".to_string(),
        path_string(policy),
        "--registry".to_string(),
        path_string(registry),
        "--index".to_string(),
        path_string(index),
    ]
}

fn mutate_comparison_index_schema(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut value: Value = serde_json::from_slice(bytes)?;
    value["schema_version"] = json!("cairn.content-bound-trace-index.v999");
    Ok((canonical_json(&value) + "\n").into_bytes())
}

fn comparison_work_directory() -> Result<PathBuf, Box<dyn Error>> {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    let directory = std::env::temp_dir().join(format!(
        "cairn-buoy-comparison-{}-{nanos}",
        std::process::id()
    ));
    fs::create_dir(&directory)?;
    Ok(directory)
}

fn read_comparison_fixture_files(
    root: &Path,
) -> Result<Vec<(String, Vec<u8>)>, Box<dyn Error>> {
    let mut files = Vec::new();
    let mut pending = vec![(root.to_path_buf(), String::new())];
    let mut total_bytes = 0u64;
    while let Some((directory, prefix)) = pending.pop() {
        let mut entries = fs::read_dir(&directory)?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let Some(name) = entry.file_name().to_str().map(str::to_string) else {
                return Err(format!(
                    "comparison fixture path is not valid UTF-8: {}",
                    entry.path().display()
                )
                .into());
            };
            let relative = if prefix.is_empty() {
                name
            } else {
                format!("{prefix}/{name}")
            };
            let file_type = entry.file_type()?;
            if file_type.is_symlink() {
                return Err(format!(
                    "comparison fixture must not contain symlinks: {}",
                    entry.path().display()
                )
                .into());
            }
            if file_type.is_dir() {
                pending.push((entry.path(), relative));
                continue;
            }
            if !file_type.is_file() {
                return Err(format!(
                    "comparison fixture entry is not a regular file: {}",
                    entry.path().display()
                )
                .into());
            }
            let metadata = entry.metadata()?;
            if metadata.len() > MAX_COMPARISON_FIXTURE_FILE_BYTES {
                return Err(format!(
                    "comparison fixture file exceeds the size bound: {}",
                    entry.path().display()
                )
                .into());
            }
            total_bytes = total_bytes.saturating_add(metadata.len());
            if total_bytes > MAX_COMPARISON_FIXTURE_TOTAL_BYTES {
                return Err("comparison fixture tree exceeds the total size bound".into());
            }
            if files.len() >= MAX_COMPARISON_FIXTURE_FILES {
                return Err("comparison fixture tree exceeds the file count bound".into());
            }
            files.push((relative, fs::read(entry.path())?));
        }
    }
    Ok(files)
}

fn copy_comparison_fixture_tree(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    for (relative, bytes) in read_comparison_fixture_files(source)? {
        write_comparison_file(&destination.join(relative), &bytes)?;
    }
    Ok(())
}

fn write_comparison_file(path: &Path, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, bytes)?;
    Ok(())
}

fn write_comparison_observations(
    directory: &Path,
    observations: &[BuoyComparisonObservation],
) -> Result<(), Box<dyn Error>> {
    fs::create_dir(directory)?;
    for observation in observations {
        let case_directory = directory.join(&observation.case_id);
        fs::create_dir(&case_directory)?;
        fs::write(case_directory.join("stdout.json"), &observation.stdout)?;
        fs::write(case_directory.join("stderr.txt"), &observation.stderr)?;
        fs::write(
            case_directory.join("exit.txt"),
            observation.exit_code.to_string(),
        )?;
        fs::write(
            case_directory.join("identity.json"),
            format!(
                "{}\n",
                canonical_json(&json!({
                    "case": observation.case_id,
                    "exit": observation.exit_code,
                    "stdout_blake3": blake3::hash(&observation.stdout).to_hex().to_string(),
                    "stderr_blake3": blake3::hash(&observation.stderr).to_hex().to_string(),
                }))
            ),
        )?;
    }
    Ok(())
}

fn export_receipt(
    root: &Path,
    out: &Path,
    index: Option<&ContentBoundTraceIndex>,
    issues: Vec<ContentBoundTraceIndexIssue>,
    registry_issues: Vec<ContentBoundRequirementRegistryIssue>,
) -> Value {
    let valid = index.is_some() && issues.is_empty() && registry_issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "traceability export",
            "schema": TRACE_INDEX_EXPORT_RECEIPT_SCHEMA_V1,
            "index_schema": CONTENT_BOUND_TRACE_INDEX_SCHEMA_V1,
            "root": path_string(root),
            "output": path_string(out),
            "profile_id": index.map(|value| value.profile.id.clone()),
            "index_blake3": index.map(|value| value.index_blake3.clone()),
            "requirement_registry_blake3": index.map(|value| value.requirement_registry_blake3.clone()),
            "requirement_count": index.map_or(0, |value| value.requirements.len()),
            "source_count": index.map_or(0, |value| value.sources.len()),
            "link_count": index.map_or(0, |value| value.links.len()),
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "boundary": CONTENT_BOUND_TRACE_INDEX_BOUNDARY,
            "issues": issues,
            "registry_issues": content_bound_requirement_registry_issues_json(&registry_issues),
        }),
    )
}

pub(crate) fn trace_index_verify_receipt(
    root: &Path,
    index_path: &Path,
    index: Option<&ContentBoundTraceIndex>,
    issues: Vec<ContentBoundTraceIndexIssue>,
    registry_issues: Vec<ContentBoundRequirementRegistryIssue>,
) -> Value {
    let valid = index.is_some() && issues.is_empty() && registry_issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "traceability verify",
            "schema": TRACE_INDEX_VERIFY_RECEIPT_SCHEMA_V1,
            "root": path_string(root),
            "index": path_string(index_path),
            "index_blake3": index.map(|value| value.index_blake3.clone()),
            "requirement_count": index.map_or(0, |value| value.requirements.len()),
            "source_count": index.map_or(0, |value| value.sources.len()),
            "link_count": index.map_or(0, |value| value.links.len()),
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "boundary": CONTENT_BOUND_TRACE_INDEX_BOUNDARY,
            "issues": issues,
            "registry_issues": content_bound_requirement_registry_issues_json(&registry_issues),
        }),
    )
}

fn read_registry(path: &Path) -> Result<ContentBoundRequirementRegistry, Box<dyn Error>> {
    let value = read_bounded_json_artifact(path, MAX_REGISTRY_BYTES, "requirement registry")?;
    parse_content_bound_requirement_registry_value(&value).map_err(|issue| {
        format!(
            "invalid content-bound requirement registry: {}",
            issue.message
        )
        .into()
    })
}

fn read_index(path: &Path) -> Result<ContentBoundTraceIndex, Box<dyn Error>> {
    let value = read_bounded_json_artifact(path, MAX_TRACE_INDEX_BYTES, "trace index")?;
    parse_content_bound_trace_index_value(&value)
        .map_err(|issue| format!("invalid content-bound trace index: {}", issue.message).into())
}

fn read_bounded_json_artifact(
    path: &Path,
    max_bytes: u64,
    kind: &str,
) -> Result<Value, Box<dyn Error>> {
    let bytes = read_bounded_file(path, max_bytes, kind)?;
    serde_json::from_slice(&bytes)
        .map_err(|error| format!("failed to parse {kind} {}: {error}", path.display()).into())
}

fn read_bounded_file(path: &Path, max_bytes: u64, kind: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let (root, relative) =
        CapabilityRoot::open_file_parent(CapabilityRootKind::EvidenceBundle, path, false)?;
    root.read_bounded(&relative, max_bytes).map_err(|error| {
        format!("failed to read bounded {kind} {}: {error}", path.display()).into()
    })
}

fn required_path_option(
    args: &[String],
    option: &str,
    command: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    read_string_option(args, option)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .ok_or_else(|| format!("traceability {command} requires {option} <PATH>").into())
}

fn canonical_index_bytes(index: &ContentBoundTraceIndex) -> Result<Vec<u8>, Box<dyn Error>> {
    let value = serde_json::to_value(index)?;
    Ok((canonical_json(&value) + "\n").into_bytes())
}
