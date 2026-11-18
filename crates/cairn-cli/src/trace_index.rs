use std::{
    error::Error,
    path::{Path, PathBuf},
};

use cairn_core::{
    canonical_json, content_bound_requirement_registry_issues_json,
    evaluate_content_bound_trace_index, evaluate_structural_trace_index,
    parse_content_bound_requirement_registry_value, parse_content_bound_trace_index_value,
    query_content_bound_trace_index, verify_content_bound_requirement_registry,
    verify_content_bound_trace_index, verify_structural_trace_index,
    ContentBoundRequirementRegistry, ContentBoundRequirementRegistryIssue, ContentBoundTraceIndex,
    ContentBoundTraceIndexIssue, RustItemConfiguration, TraceabilityFile,
    CONTENT_BOUND_TRACE_INDEX_BOUNDARY, CONTENT_BOUND_TRACE_INDEX_SCHEMA_V1,
    TRACEABILITY_ANCHOR_RUST_ITEM_V1,
};
use serde_json::{json, Value};

use crate::{
    requirement_id_registry::read_accepted_specifications,
    support::{
        default_policy_path, path_string, print_json, read_option, read_string_option,
        reject_positional_arguments, reject_unknown_options, with_portable_receipt_hash,
        CapabilityRoot, CapabilityRootKind,
    },
    tracey::{collect_traceability_files_with_bound, selected_traceability_profile},
};

mod rust_items;

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

pub(crate) fn run_trace_index(subcommand: &str, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    match subcommand {
        "export" => run_export(args),
        "verify" => run_verify(args),
        "show" => run_show(args),
        other => Err(format!("unknown traceability command: {other}").into()),
    }
}

fn run_export(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.trace_index.cli]
    reject_unknown_options(
        &args,
        &["--root", "--policy", "--profile", "--registry", "--out", "--configuration", "--previous-index"],
    )?;
    reject_positional_arguments(&args)?;
    let root = read_option(args.clone(), "--root", ".");
    let policy_path = read_option(args.clone(), "--policy", default_policy_path());
    let profile_id = read_string_option(&args, "--profile");
    let registry_path = required_path_option(&args, "--registry", "export")?;
    let out = required_path_option(&args, "--out", "export")?;
    let policy = crate::support::load_policy_file(policy_path.clone())?;
    let profile = selected_traceability_profile(&policy, profile_id.as_deref())?;
    let evidence_files = collect_selected_traceability_files(&root, &profile)?;
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

    if let Some(explicit) = read_string_option(&args, "--configuration") {
        if profile.rust_configuration_path.as_deref() != Some(explicit.as_str()) {
            return Err("configuration path differs from selected structural profile".into());
        }
    }
    let previous = read_string_option(&args, "--previous-index").map(|path| read_index(Path::new(&path))).transpose()?;
    if previous.is_some() && profile.anchor_kind != TRACEABILITY_ANCHOR_RUST_ITEM_V1 {
        return Err("--previous-index is only valid for rust_item_v1".into());
    }
    let evaluation = match evaluate_current_trace_index(&root, &registry, &profile, evidence_files, previous.as_ref()) {
        Ok(evaluation) => evaluation,
        Err(error) => {
            print_json(export_receipt(&root, &out, None, vec![item_issue(error.to_string())], Vec::new()))?;
            std::process::exit(1);
        }
    };
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
        &["--root", "--policy", "--profile", "--registry", "--index", "--configuration"],
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
        read_string_option(&args, "--configuration").as_deref().map(Path::new),
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
    configuration_path: Option<&Path>,
) -> Result<Value, Box<dyn Error>> {
    let resolved_policy_path = if policy_path.is_absolute() {
        policy_path.to_path_buf()
    } else {
        root.join(policy_path)
    };
    let policy = crate::support::load_policy_file(resolved_policy_path.clone())?;
    let index = read_index(index_path)?;
    let selected = profile_id.or_else(|| (index.structural.is_some()).then_some(index.profile.id.as_str()));
    let profile = selected_traceability_profile(&policy, selected)?;
    let evidence_files = collect_selected_traceability_files(root, &profile)?;
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
    let report = (|| -> Result<cairn_core::ContentBoundTraceIndexVerification, Box<dyn Error>> {
        if index.structural.is_some() {
            let configured = profile.rust_configuration_path.as_deref().ok_or("selected structural profile has no configuration path")?;
            let path = configuration_path.unwrap_or_else(|| Path::new(configured));
            if path != Path::new(configured) { return Err("configuration path differs from selected structural profile".into()); }
            let configuration = read_configuration(root, path)?;
            let preliminary = evaluate_content_bound_trace_index(&registry, &profile, evidence_files.clone());
            let facts = if let Some(preliminary) = preliminary.index {
                rust_items::facts(&evidence_files, &preliminary.links, &configuration)?
            } else { Vec::new() };
            Ok(verify_structural_trace_index(&index, &registry, &profile, evidence_files, facts, configuration))
        } else {
            Ok(verify_content_bound_trace_index(&index, &registry, &profile, evidence_files))
        }
    })();
    let report = match report {
        Ok(report) => report,
        Err(error) => return Ok(trace_index_verify_receipt(root, displayed_index_path, Some(&index), vec![item_issue(error.to_string())], Vec::new())),
    };
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
    reject_unknown_options(&args[1..], &["--root", "--index", "--registry", "--policy", "--profile", "--configuration"])?;
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
    let mut receipt = json!({
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
    });
    if let Some(structural) = &index.structural {
        let active = structural.bindings.iter().filter(|binding| binding.requirement_id == *requirement_id).collect::<Vec<_>>();
        let historical = structural.history.iter().filter(|binding| binding.requirement_id == *requirement_id).collect::<Vec<_>>();
        let status = if !active.is_empty() { "unavailable" } else if !historical.is_empty() { "historical" } else { "absent" };
        receipt["bindings"] = serde_json::to_value(active)?;
        receipt["history"] = serde_json::to_value(historical)?;
        receipt["binding_status"] = json!(status);
        receipt["freshness"] = json!("unavailable: pass --registry to compare current root");
        if let Some(registry) = read_string_option(&args, "--registry") {
            let policy = read_option(args.clone(), "--policy", default_policy_path());
            let profile = read_string_option(&args, "--profile");
            let configuration = read_string_option(&args, "--configuration");
            let verification = current_root_trace_index_verification_receipt(&root, &policy, profile.as_deref(),
                Path::new(&registry), &index_path, &index_path, configuration.as_deref().map(Path::new))?;
            let fresh = verification["valid"].as_bool() == Some(true);
            receipt["binding_status"] = json!(if status != "unavailable" { status } else if fresh { "current" } else { "stale" });
            receipt["freshness"] = verification;
        }
    }
    print_json(with_portable_receipt_hash(&root, receipt))?;
    if !valid {
        std::process::exit(1);
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
            "index_schema": index.map_or(CONTENT_BOUND_TRACE_INDEX_SCHEMA_V1, |value| value.schema_version.as_str()),
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

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RustConfigurationFile {
    source_path: String,
    schema_version: String,
    target: String,
    features: Vec<String>,
    declared_features: Vec<String>,
    cfg: Vec<String>,
}

fn read_configuration(root: &Path, path: &Path) -> Result<RustItemConfiguration, Box<dyn Error>> {
    let relative = path.to_str().ok_or("configuration path is not UTF-8")?;
    if !crate::support::is_safe_relative_reference(relative) || relative.len() > 4096
        || relative.contains(':') || relative.contains('\\') || relative.contains('\0') {
        return Err("Rust configuration path must be safe and relative to --root".into());
    }
    let repository = CapabilityRoot::open(CapabilityRootKind::Repository, root)?;
    let bytes = repository.read_bounded(path, 64 * 1024)?;
    let document: RustConfigurationFile = serde_json::from_slice(&bytes)?;
    if document.source_path != relative {
        return Err("Rust configuration source_path must match its saved logical path".into());
    }
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"cairn.rust-item-configuration-file.v1\0");
    hasher.update(&bytes);
    Ok(RustItemConfiguration {
        source_path: document.source_path, file_blake3: hasher.finalize().to_hex().to_string(),
        schema_version: document.schema_version, target: document.target,
        features: document.features, declared_features: document.declared_features, cfg: document.cfg,
    })
}

pub(crate) fn evaluate_current_trace_index(
    root: &Path,
    registry: &ContentBoundRequirementRegistry,
    profile: &cairn_core::TraceabilityProfile,
    files: Vec<TraceabilityFile>,
    previous: Option<&ContentBoundTraceIndex>,
) -> Result<cairn_core::ContentBoundTraceIndexEvaluation, Box<dyn Error>> {
    if profile.anchor_kind != TRACEABILITY_ANCHOR_RUST_ITEM_V1 {
        return Ok(evaluate_content_bound_trace_index(registry, profile, files));
    }
    let configured = profile.rust_configuration_path.as_deref().ok_or("structural profile requires rust_configuration_path")?;
    let configuration = read_configuration(root, Path::new(configured))?;
    let preliminary = evaluate_content_bound_trace_index(registry, profile, files.clone());
    let Some(index) = preliminary.index.as_ref() else { return Ok(preliminary) };
    let facts = rust_items::facts(&files, &index.links, &configuration)?;
    Ok(evaluate_structural_trace_index(registry, profile, files, facts, configuration, previous))
}

pub(crate) fn collect_selected_traceability_files(root: &Path, profile: &cairn_core::TraceabilityProfile) -> Result<Vec<TraceabilityFile>, Box<dyn Error>> {
    let limit = (profile.anchor_kind == TRACEABILITY_ANCHOR_RUST_ITEM_V1).then_some(1024 * 1024);
    collect_traceability_files_with_bound(root, &profile.evidence_sources, limit)
}

fn item_issue(message: String) -> ContentBoundTraceIndexIssue {
    ContentBoundTraceIndexIssue { code: "trace_index.item_unsupported".into(), field_path: "structural".into(), message }
}
