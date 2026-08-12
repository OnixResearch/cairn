use std::{
    error::Error,
    path::{Path, PathBuf},
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
use serde_json::{json, Value};

use crate::{
    requirement_id_registry::read_accepted_specifications,
    support::{
        default_policy_path, path_string, print_json, read_option, read_string_option,
        with_portable_receipt_hash, CapabilityRoot, CapabilityRootKind,
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
    let Some(requirement_id) = args.first().filter(|value| !value.starts_with('-')) else {
        return Err("traceability show requires <requirement-id>".into());
    };
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
