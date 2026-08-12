use std::path::{Path, PathBuf};

use cairn_core::{
    content_bound_requirement_registry_issues_json, evaluate_content_bound_requirement_registry,
    evaluate_content_bound_trace_index, ContentBoundRevision, CONTENT_BOUND_REVISION_KIND_OPAQUE,
};

use super::issue::{evidence_export_issue, evidence_export_nested_issue};
use super::*;
use crate::requirement_id_registry::read_accepted_specifications;
use crate::support::CapabilityRoot;
use crate::trace_index::current_root_trace_index_verification_receipt;
use crate::tracey::{collect_traceability_files, selected_traceability_profile};

const EVIDENCE_REPOSITORY_ID: &str = "cairn-evidence-local";
const EVIDENCE_REVISION: &str = "evidence-snapshot";
const REQUIREMENT_REGISTRY_FILE: &str = "requirement-registry.json";
const TRACE_INDEX_FILE: &str = "trace-index.json";
const TRACE_INDEX_VERIFICATION_FILE: &str = "trace-index-verification.json";
const BYTES_PER_KIBIBYTE: u64 = 1_024;
const KIBIBYTES_PER_MEBIBYTE: u64 = 1_024;
const MAX_POLICY_MEBIBYTES: u64 = 16;
const MAX_POLICY_BYTES: u64 = MAX_POLICY_MEBIBYTES * KIBIBYTES_PER_MEBIBYTE * BYTES_PER_KIBIBYTE;

pub(super) struct TraceEvidencePaths {
    pub(super) registry: PathBuf,
    pub(super) index: PathBuf,
    pub(super) verification: PathBuf,
}

// r[impl cairn.trace_index.evidence_bundle.export]
// r[impl cairn.trace_index.evidence_bundle.portability]
pub(super) fn export_trace_evidence(
    root: &Path,
    policy_path: &Path,
    policy: &Policy,
    repository_root: &CapabilityRoot,
    output_root: &CapabilityRoot,
    out_dir: &Path,
    issues: &mut Vec<Value>,
) -> Option<TraceEvidencePaths> {
    let registry_path = out_dir.join(REQUIREMENT_REGISTRY_FILE);
    let index_path = out_dir.join(TRACE_INDEX_FILE);
    let verification_path = out_dir.join(TRACE_INDEX_VERIFICATION_FILE);

    let specifications = match read_accepted_specifications(repository_root) {
        Ok(specifications) => specifications,
        Err(_) => {
            issues.push(evidence_export_issue(
                "export.trace_registry_specs_failed",
                "requirement_registry",
                "bounded accepted specifications",
                "unreadable",
            ));
            return None;
        }
    };
    let policy_bytes =
        match repository_root.read_bounded(Path::new(default_policy_path()), MAX_POLICY_BYTES) {
            Ok(bytes) => bytes,
            Err(_) => {
                issues.push(evidence_export_issue(
                    "export.trace_registry_policy_failed",
                    "requirement_registry",
                    "bounded generated policy",
                    "unreadable",
                ));
                return None;
            }
        };
    let registry_evaluation = evaluate_content_bound_requirement_registry(
        specifications,
        &policy_bytes,
        EVIDENCE_REPOSITORY_ID,
        ContentBoundRevision {
            kind: CONTENT_BOUND_REVISION_KIND_OPAQUE.to_string(),
            value: EVIDENCE_REVISION.to_string(),
        },
    );
    let Some(registry) = registry_evaluation.registry else {
        issues.extend(
            content_bound_requirement_registry_issues_json(&registry_evaluation.issues)
                .into_iter()
                .map(|issue| evidence_export_nested_issue("export.trace_registry_invalid", issue)),
        );
        return None;
    };
    let registry_value = match serde_json::to_value(&registry) {
        Ok(value) => value,
        Err(_) => {
            issues.push(evidence_export_issue(
                "export.trace_registry_serialize_failed",
                "requirement_registry",
                "serializable content-bound registry",
                "failed",
            ));
            return None;
        }
    };
    write_evidence_json(
        output_root,
        &registry_path,
        &registry_value,
        issues,
        "requirement_registry",
    );

    let profile = match selected_traceability_profile(policy, None) {
        Ok(profile) => profile,
        Err(_) => {
            issues.push(evidence_export_issue(
                "export.trace_profile_invalid",
                "trace_index.profile",
                "selected traceability profile",
                "invalid",
            ));
            return None;
        }
    };
    let evidence_files = match collect_traceability_files(root, &profile.evidence_sources) {
        Ok(files) => files,
        Err(_) => {
            issues.push(evidence_export_issue(
                "export.trace_sources_failed",
                "trace_index.sources",
                "bounded traceability sources",
                "unreadable",
            ));
            return None;
        }
    };
    let index_evaluation = evaluate_content_bound_trace_index(&registry, &profile, evidence_files);
    let Some(index) = index_evaluation.index else {
        issues.extend(index_evaluation.issues.into_iter().map(|issue| {
            json!({
                "code": issue.code,
                "field_path": issue.field_path,
                "expected": "valid content-bound trace index",
                "actual": issue.message,
            })
        }));
        return None;
    };
    let index_value = match serde_json::to_value(&index) {
        Ok(value) => value,
        Err(_) => {
            issues.push(evidence_export_issue(
                "export.trace_index_serialize_failed",
                "trace_index",
                "serializable content-bound trace index",
                "failed",
            ));
            return None;
        }
    };
    write_evidence_json(
        output_root,
        &index_path,
        &index_value,
        issues,
        "trace_index",
    );

    let verification = match current_root_trace_index_verification_receipt(
        root,
        policy_path,
        None,
        &registry_path,
        &index_path,
        Path::new(TRACE_INDEX_FILE),
    ) {
        Ok(receipt) => receipt,
        Err(_) => {
            issues.push(evidence_export_issue(
                "export.trace_index_verification_failed",
                "trace_index_verification",
                "current-root trace-index verification receipt",
                "failed",
            ));
            return None;
        }
    };
    if verification.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_export_issue(
            "export.trace_index_verification_invalid",
            "trace_index_verification",
            "passing current-root trace-index verification receipt",
            "invalid",
        ));
        return None;
    }
    write_evidence_json(
        output_root,
        &verification_path,
        &verification,
        issues,
        "trace_index_verification",
    );

    Some(TraceEvidencePaths {
        registry: registry_path,
        index: index_path,
        verification: verification_path,
    })
}
