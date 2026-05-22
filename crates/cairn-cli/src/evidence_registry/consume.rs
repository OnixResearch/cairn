use std::{
    fs,
    path::{Path, PathBuf},
};

use serde_json::{json, Value};

use crate::agent_receipt::{read_json_artifact, AGENT_RECEIPT_MAX_BYTES};
use crate::support::{path_string, read_string_option, with_portable_receipt_hash};
use cairn_core::canonical_json_hash;

use super::build_verify::evidence_registry_verify_receipt;
use super::common::{evidence_registry_issue, evidence_registry_resolve_index_path};
use super::{
    EvidenceRegistryFetchPlanOptions, EvidenceRegistrySelectOptions, EvidenceRegistryVerifyOptions,
};

#[derive(Default, Clone)]
pub(crate) struct EvidenceRegistryConsumeSelectors {
    pub(super) bundle_kind: Option<String>,
    pub(super) policy_hash: Option<String>,
    pub(super) channel: Option<String>,
    pub(super) release: Option<String>,
    pub(super) platform: Option<String>,
    pub(super) compatibility_class: Option<String>,
    pub(super) index_receipt_hash: Option<String>,
    pub(super) index_verification_receipt_hash: Option<String>,
    pub(super) bundle_receipt_hash: Option<String>,
    pub(super) artifact_role: Option<String>,
    pub(super) artifact_hash: Option<String>,
    pub(super) coverage_audit_receipt_hash: Option<String>,
    pub(super) release_readiness_receipt_hash: Option<String>,
}
pub(super) fn evidence_registry_consume_selectors(
    args: &[String],
) -> EvidenceRegistryConsumeSelectors {
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
pub(super) fn evidence_registry_select_receipt(options: &EvidenceRegistrySelectOptions) -> Value {
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

pub(super) fn evidence_registry_fetch_plan_receipt(
    options: &EvidenceRegistryFetchPlanOptions,
) -> Value {
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
