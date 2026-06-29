use cairn_core::canonical_json_hash;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

use crate::{
    agent_receipt::{read_json_artifact, AGENT_RECEIPT_MAX_BYTES},
    support::{path_string, portable_json_hash, with_portable_content_hash_receipt},
};

use super::{
    bundle::{
        evidence_index_bundle_summary, evidence_index_compare_summary,
        evidence_index_verify_bundle_path,
    },
    issue::{evidence_index_issue, issues_with_index_missing},
    options::EvidenceIndexVerifyOptions,
};

pub(crate) fn evidence_index_verify_receipt(options: &EvidenceIndexVerifyOptions) -> Value {
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
    verify_stated_index_hash(&root, &index, &mut issues);
    let bundle = evidence_index_verify_bundle_path(&index_path, &index);
    let current_summary =
        evidence_index_bundle_summary(&root, &bundle, "evidence_index_verify", &mut issues);
    if let Some(summary) = current_summary.as_ref() {
        evidence_index_compare_summary(&mut issues, &index, summary);
    }
    evidence_index_verify_final_receipt(&root, issues, index_receipt_hash, current_summary)
}

fn verify_stated_index_hash(root: &Path, index: &Value, issues: &mut Vec<Value>) {
    if let Some(stated_hash) = index.get("receipt_hash").and_then(Value::as_str) {
        let mut hash_input = index.clone();
        if let Some(object) = hash_input.as_object_mut() {
            object.remove("receipt_hash");
        }
        if portable_json_hash(root, &hash_input) != stated_hash {
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
}

pub(super) fn evidence_index_verify_final_receipt(
    root: &Path,
    mut issues: Vec<Value>,
    index_receipt_hash: Value,
    current_summary: Option<Value>,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    let empty = json!({});
    let summary = current_summary.as_ref().unwrap_or(&empty);
    with_portable_content_hash_receipt(
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
