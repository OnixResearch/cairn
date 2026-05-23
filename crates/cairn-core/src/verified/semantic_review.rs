use serde_json::{json, Map, Value};

use super::evidence::canonical_json_hash;

pub const REVIEW_RECEIPT_SCHEMA_V1: &str = "cairn.semantic-review.review-receipt.v1";
pub const REVISION_HANDOFF_SCHEMA_V1: &str = "cairn.semantic-review.revision-handoff.v1";
pub const REVIEW_ANALYTICS_SCHEMA_V1: &str = "cairn.semantic-review.ledger-analytics.v1";
pub const REVIEW_COMPLETENESS_SCHEMA_V1: &str = "cairn.semantic-review.completeness.v1";

pub const REVIEW_KINDS: &[&str] = &["plan", "design", "code", "diff"];
pub const REVIEW_DECISIONS: &[&str] = &["approved", "needs_revision", "escalate", "unknown"];
pub const REVIEW_CATEGORIES: &[&str] = &["omission", "systematic", "incoherent", "infra"];
pub const REVIEW_SEVERITIES: &[&str] = &["low", "medium", "high", "critical"];
pub const REVIEW_SOURCES: &[&str] = &["manual", "agentic", "static", "imported"];
pub const REVISION_DISPOSITIONS: &[&str] =
    &["addressed", "unchanged", "superseded", "deferred", "new"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticReviewIssue {
    pub code: String,
    pub field_path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticReviewReport {
    pub valid: bool,
    pub verdict: String,
    pub issues: Vec<SemanticReviewIssue>,
    pub receipt_hash: String,
}

pub fn semantic_review_policy_profile_json() -> Value {
    // r[impl cairn.semantic_review.review_gate_policy_profiles.profiles]
    // r[impl cairn.semantic_review.review_gate_policy_profiles.profiles.profiles.kinds]
    // r[impl cairn.semantic_review.review_gate_policy_profiles.profiles.profiles.criteria]
    // r[impl cairn.semantic_review.review_gate_policy_profiles.sources]
    json!({
        "schema": "cairn.semantic-review.policy-profile.v1",
        "review_kinds": REVIEW_KINDS,
        "decisions": REVIEW_DECISIONS,
        "categories": REVIEW_CATEGORIES,
        "severities": REVIEW_SEVERITIES,
        "reviewer_sources": REVIEW_SOURCES,
        "revision_dispositions": REVISION_DISPOSITIONS,
        "profiles": [
            {"id": "plan", "required_criteria": ["scope", "requirements", "risks", "omissions"]},
            {"id": "design", "required_criteria": ["architecture", "acceptance", "interfaces", "risks"]},
            {"id": "code", "required_criteria": ["requirements", "tests", "consistency", "regressions"]},
            {"id": "diff", "required_criteria": ["intent", "scope", "side_effects", "verification"]}
        ]
    })
}

pub fn review_receipt_hash(value: &Value) -> String {
    canonical_json_hash(&without_hash_field(value, "receipt_hash"))
}

pub fn validate_review_receipt(
    value: &Value,
    observed_artifact_hashes: &[(String, String)],
) -> SemanticReviewReport {
    // r[impl cairn.semantic_review.semantic_review_receipts.schema]
    // r[impl cairn.semantic_review.semantic_review_receipts.schema.schema.fields]
    // r[impl cairn.semantic_review.review_rejection_taxonomy.policy]
    // r[impl cairn.semantic_review.review_rejection_taxonomy.policy.policy.categories]
    // r[impl cairn.semantic_review.review_rejection_taxonomy.policy.policy.decisions]
    // r[impl cairn.semantic_review.semantic_review_boundary.boundary]
    // r[impl cairn.semantic_review.semantic_review_boundary.boundary.boundary.non_execution]
    // r[impl cairn.semantic_review.semantic_review_boundary.fcis]
    // r[impl cairn.semantic_review.semantic_review_boundary.fcis.fcis.pure_core]
    let mut issues = Vec::new();
    require_string(
        value,
        "schema_version",
        REVIEW_RECEIPT_SCHEMA_V1,
        &mut issues,
    );
    require_known_string(value, "review_kind", REVIEW_KINDS, &mut issues);
    let decision = require_known_string(value, "decision", REVIEW_DECISIONS, &mut issues);
    validate_reviewer(value.get("reviewer"), &mut issues);
    validate_reviewed_inputs(value, observed_artifact_hashes, &mut issues);
    validate_findings(value, decision.as_deref(), &mut issues);
    validate_optional_receipt_hash(value, &mut issues, review_receipt_hash(value));
    report(value, issues)
}

pub fn revision_handoff_hash(value: &Value) -> String {
    canonical_json_hash(&without_hash_field(value, "receipt_hash"))
}

pub fn validate_revision_handoff(value: &Value) -> SemanticReviewReport {
    // r[impl cairn.semantic_review.revision_handoff_receipts.schema]
    // r[impl cairn.semantic_review.revision_handoff_receipts.schema.schema.chain]
    // r[impl cairn.semantic_review.revision_handoff_receipts.schema.schema.dispositions]
    // r[impl cairn.semantic_review.revision_handoff_receipts.validation]
    let mut issues = Vec::new();
    require_string(
        value,
        "schema_version",
        REVISION_HANDOFF_SCHEMA_V1,
        &mut issues,
    );
    require_hex_string(value, "prior_review_receipt_hash", &mut issues);
    validate_optional_hash(value, "followup_review_receipt_hash", &mut issues);
    validate_revision_artifacts(value.get("revision_artifacts"), &mut issues);
    validate_dispositions(value.get("finding_dispositions"), &mut issues);
    validate_optional_receipt_hash(value, &mut issues, revision_handoff_hash(value));
    report(value, issues)
}

pub fn review_ledger_analytics(receipts: &[Value]) -> Value {
    // r[impl cairn.semantic_review.review_ledger_analytics.aggregate]
    // r[impl cairn.semantic_review.review_ledger_analytics.aggregate.aggregate.counts]
    // r[impl cairn.semantic_review.review_ledger_analytics.aggregate.aggregate.hashes]
    let mut valid_receipts = Vec::new();
    let mut invalid_inputs = Vec::new();
    for (index, receipt) in receipts.iter().enumerate() {
        let validation = validate_review_receipt(receipt, &[]);
        if validation.valid {
            valid_receipts.push(receipt.clone());
        } else {
            invalid_inputs.push(json!({
                "index": index,
                "issues": issues_json(&validation.issues),
            }));
        }
    }
    let by_kind = count_string_field(&valid_receipts, "review_kind");
    let by_decision = count_string_field(&valid_receipts, "decision");
    let mut by_category: Map<String, Value> = Map::new();
    for receipt in &valid_receipts {
        if let Some(findings) = receipt.get("findings").and_then(Value::as_array) {
            for finding in findings {
                if let Some(category) = finding.get("category").and_then(Value::as_str) {
                    increment_count(&mut by_category, category);
                }
            }
        }
    }
    let by_category = sort_count_map(&mut by_category);
    let mut input_receipt_hashes = valid_receipts
        .iter()
        .map(review_receipt_hash)
        .collect::<Vec<_>>();
    input_receipt_hashes.sort();
    let mut warnings = Vec::new();
    if valid_receipts.len() < 5 {
        warnings.push(
            json!({"code": "small_sample", "message": "fewer than five valid review receipts"}),
        );
    }
    let mut receipt = json!({
        "schema_version": REVIEW_ANALYTICS_SCHEMA_V1,
        "valid": invalid_inputs.is_empty(),
        "verdict": if invalid_inputs.is_empty() {"PASS"} else {"FAIL"},
        "input_count": receipts.len(),
        "valid_input_count": valid_receipts.len(),
        "invalid_inputs": invalid_inputs,
        "input_receipt_hashes": input_receipt_hashes,
        "counts": {
            "by_review_kind": by_kind,
            "by_decision": by_decision,
            "by_category": by_category,
        },
        "warnings": warnings,
    });
    let hash = canonical_json_hash(&receipt);
    receipt["receipt_hash"] = json!(hash);
    receipt
}

pub fn completeness_receipt(
    requirements: Vec<String>,
    tasks: Vec<String>,
    implementation_refs: Vec<String>,
    verification_refs: Vec<String>,
) -> Value {
    // r[impl cairn.semantic_review.completeness_check_rail.mapping]
    // r[impl cairn.semantic_review.completeness_check_rail.mapping.mapping.active_change]
    // r[impl cairn.semantic_review.completeness_check_rail.mapping.mapping.accepted_spec]
    // r[impl cairn.semantic_review.completeness_check_rail.omissions]
    let mut requirements = sorted_unique(requirements);
    let tasks = sorted_unique(tasks);
    let implementation_refs = sorted_unique(implementation_refs);
    let verification_refs = sorted_unique(verification_refs);
    let mut entries = Vec::new();
    let mut findings = Vec::new();
    for requirement in requirements.drain(..) {
        let has_task = tasks.iter().any(|task| task.contains(&requirement));
        let has_impl = implementation_refs
            .iter()
            .any(|reference| reference == &requirement);
        let has_verify = verification_refs
            .iter()
            .any(|reference| reference == &requirement);
        if !has_task {
            // r[impl cairn.semantic_review.completeness_check_rail.omissions.omissions.missing_task]
            findings.push(json!({"id": format!("missing-task-{requirement}"), "category": "omission", "code": "missing_task_coverage", "requirement": requirement}));
        }
        if has_impl && !has_verify {
            // r[impl cairn.semantic_review.completeness_check_rail.omissions.omissions.missing_verification]
            findings.push(json!({"id": format!("missing-verification-{requirement}"), "category": "omission", "code": "missing_verification_coverage", "requirement": requirement}));
        }
        entries.push(json!({
            "requirement": requirement,
            "has_task": has_task,
            "has_implementation_ref": has_impl,
            "has_verification_ref": has_verify,
        }));
    }
    let mut receipt = json!({
        "schema_version": REVIEW_COMPLETENESS_SCHEMA_V1,
        "valid": findings.is_empty(),
        "verdict": if findings.is_empty() {"PASS"} else {"FAIL"},
        "requirements": entries,
        "tasks": tasks,
        "implementation_refs": implementation_refs,
        "verification_refs": verification_refs,
        "findings": findings,
    });
    let hash = canonical_json_hash(&receipt);
    receipt["receipt_hash"] = json!(hash);
    receipt
}

fn validate_reviewer(reviewer: Option<&Value>, issues: &mut Vec<SemanticReviewIssue>) {
    let Some(reviewer) = reviewer else {
        push_issue(
            issues,
            "missing_field",
            "reviewer",
            "missing reviewer metadata",
        );
        return;
    };
    require_known_string(reviewer, "source", REVIEW_SOURCES, issues);
    if reviewer
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .is_empty()
    {
        push_issue(
            issues,
            "missing_field",
            "reviewer.id",
            "missing reviewer id",
        );
    }
}

fn validate_reviewed_inputs(
    value: &Value,
    observed_artifact_hashes: &[(String, String)],
    issues: &mut Vec<SemanticReviewIssue>,
) {
    // r[impl cairn.semantic_review.semantic_review_receipts.schema.schema.hash_binding]
    let Some(inputs) = value.get("reviewed_inputs").and_then(Value::as_array) else {
        push_issue(
            issues,
            "missing_field",
            "reviewed_inputs",
            "missing reviewed inputs",
        );
        return;
    };
    if inputs.is_empty() {
        push_issue(
            issues,
            "empty_array",
            "reviewed_inputs",
            "reviewed inputs must not be empty",
        );
    }
    for (index, input) in inputs.iter().enumerate() {
        let field = format!("reviewed_inputs[{index}]");
        let path = input.get("path").and_then(Value::as_str).unwrap_or("");
        if path.is_empty() {
            push_issue(
                issues,
                "missing_field",
                &format!("{field}.path"),
                "missing input path",
            );
        }
        let stated_hash = input.get("hash").and_then(Value::as_str).unwrap_or("");
        if !is_blake3_hex(stated_hash) {
            push_issue(
                issues,
                "invalid_hash",
                &format!("{field}.hash"),
                "input hash must be blake3 hex",
            );
        }
        if let Some((_, observed)) = observed_artifact_hashes
            .iter()
            .find(|(observed_path, _)| observed_path == path)
        {
            if observed != stated_hash {
                push_issue(
                    issues,
                    "hash_mismatch",
                    &format!("{field}.hash"),
                    "input hash does not match filesystem artifact",
                );
            }
        }
    }
}

fn validate_findings(value: &Value, decision: Option<&str>, issues: &mut Vec<SemanticReviewIssue>) {
    // r[impl cairn.semantic_review.review_rejection_taxonomy.finding_validation]
    // r[impl cairn.semantic_review.review_rejection_taxonomy.finding_validation.finding_validation.required]
    // r[impl cairn.semantic_review.review_rejection_taxonomy.finding_validation.finding_validation.approved]
    let Some(findings) = value.get("findings").and_then(Value::as_array) else {
        push_issue(
            issues,
            "missing_field",
            "findings",
            "missing findings array",
        );
        return;
    };
    if decision != Some("approved") && findings.is_empty() {
        push_issue(
            issues,
            "missing_findings",
            "findings",
            "non-approved reviews require findings",
        );
    }
    for (index, finding) in findings.iter().enumerate() {
        let field = format!("findings[{index}]");
        if finding
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("")
            .is_empty()
        {
            push_issue(
                issues,
                "missing_field",
                &format!("{field}.id"),
                "missing finding id",
            );
        }
        require_known_string_at(finding, "category", REVIEW_CATEGORIES, &field, issues);
        require_known_string_at(finding, "severity", REVIEW_SEVERITIES, &field, issues);
    }
}

fn validate_revision_artifacts(value: Option<&Value>, issues: &mut Vec<SemanticReviewIssue>) {
    let Some(artifacts) = value.and_then(Value::as_array) else {
        push_issue(
            issues,
            "missing_field",
            "revision_artifacts",
            "missing revision artifacts",
        );
        return;
    };
    if artifacts.is_empty() {
        push_issue(
            issues,
            "empty_array",
            "revision_artifacts",
            "revision artifacts must not be empty",
        );
    }
    for (index, artifact) in artifacts.iter().enumerate() {
        let field = format!("revision_artifacts[{index}]");
        if artifact
            .get("path")
            .and_then(Value::as_str)
            .unwrap_or("")
            .is_empty()
        {
            push_issue(
                issues,
                "missing_field",
                &format!("{field}.path"),
                "missing artifact path",
            );
        }
        validate_hash_field(artifact, "hash", &field, issues);
    }
}

fn validate_dispositions(value: Option<&Value>, issues: &mut Vec<SemanticReviewIssue>) {
    let Some(dispositions) = value.and_then(Value::as_array) else {
        push_issue(
            issues,
            "missing_field",
            "finding_dispositions",
            "missing finding dispositions",
        );
        return;
    };
    if dispositions.is_empty() {
        push_issue(
            issues,
            "empty_array",
            "finding_dispositions",
            "finding dispositions must not be empty",
        );
    }
    for (index, disposition) in dispositions.iter().enumerate() {
        let field = format!("finding_dispositions[{index}]");
        if disposition
            .get("finding_id")
            .and_then(Value::as_str)
            .unwrap_or("")
            .is_empty()
        {
            push_issue(
                issues,
                "missing_field",
                &format!("{field}.finding_id"),
                "missing finding id",
            );
        }
        let disposition_value = require_known_string_at(
            disposition,
            "disposition",
            REVISION_DISPOSITIONS,
            &field,
            issues,
        );
        if disposition_value.as_deref() == Some("addressed") {
            let has_refs = disposition
                .get("evidence_refs")
                .and_then(Value::as_array)
                .map(|refs| !refs.is_empty())
                .unwrap_or(false);
            let has_hashes = disposition
                .get("artifact_hashes")
                .and_then(Value::as_array)
                .map(|hashes| !hashes.is_empty())
                .unwrap_or(false);
            if !has_refs && !has_hashes {
                // r[impl cairn.semantic_review.revision_handoff_receipts.validation.validation.addressed_evidence]
                push_issue(
                    issues,
                    "missing_addressed_evidence",
                    &field,
                    "addressed findings require evidence refs or artifact hashes",
                );
            }
        }
        if disposition_value.as_deref() == Some("new") {
            // r[impl cairn.semantic_review.revision_handoff_receipts.validation.validation.new_findings]
            if disposition
                .get("introduced_by_followup")
                .and_then(Value::as_bool)
                != Some(true)
            {
                push_issue(
                    issues,
                    "missing_new_finding_marker",
                    &field,
                    "new findings must be marked as introduced by follow-up review",
                );
            }
        }
    }
}

fn require_string(
    value: &Value,
    field: &'static str,
    expected: &str,
    issues: &mut Vec<SemanticReviewIssue>,
) {
    match value.get(field).and_then(Value::as_str) {
        Some(actual) if actual == expected => {}
        Some(_) => push_issue(issues, "invalid_field", field, "field has unexpected value"),
        None => push_issue(issues, "missing_field", field, "missing required field"),
    }
}

fn require_known_string(
    value: &Value,
    field: &'static str,
    allowed: &[&str],
    issues: &mut Vec<SemanticReviewIssue>,
) -> Option<String> {
    match value.get(field).and_then(Value::as_str) {
        Some(actual) if allowed.contains(&actual) => Some(actual.to_string()),
        Some(_) => {
            push_issue(issues, "invalid_field", field, "field is not policy-known");
            None
        }
        None => {
            push_issue(issues, "missing_field", field, "missing required field");
            None
        }
    }
}

fn require_known_string_at(
    value: &Value,
    field: &'static str,
    allowed: &[&str],
    prefix: &str,
    issues: &mut Vec<SemanticReviewIssue>,
) -> Option<String> {
    match value.get(field).and_then(Value::as_str) {
        Some(actual) if allowed.contains(&actual) => Some(actual.to_string()),
        Some(_) => {
            push_issue(
                issues,
                "invalid_field",
                &format!("{prefix}.{field}"),
                "field is not policy-known",
            );
            None
        }
        None => {
            push_issue(
                issues,
                "missing_field",
                &format!("{prefix}.{field}"),
                "missing required field",
            );
            None
        }
    }
}

fn require_hex_string(value: &Value, field: &'static str, issues: &mut Vec<SemanticReviewIssue>) {
    match value.get(field).and_then(Value::as_str) {
        Some(actual) if is_blake3_hex(actual) => {}
        Some(_) => push_issue(issues, "invalid_hash", field, "field must be blake3 hex"),
        None => push_issue(
            issues,
            "missing_field",
            field,
            "missing required hash field",
        ),
    }
}

fn validate_optional_hash(
    value: &Value,
    field: &'static str,
    issues: &mut Vec<SemanticReviewIssue>,
) {
    if let Some(actual) = value.get(field).and_then(Value::as_str) {
        if !is_blake3_hex(actual) {
            push_issue(issues, "invalid_hash", field, "field must be blake3 hex");
        }
    }
}

fn validate_hash_field(
    value: &Value,
    field: &'static str,
    prefix: &str,
    issues: &mut Vec<SemanticReviewIssue>,
) {
    match value.get(field).and_then(Value::as_str) {
        Some(actual) if is_blake3_hex(actual) => {}
        Some(_) => push_issue(
            issues,
            "invalid_hash",
            &format!("{prefix}.{field}"),
            "field must be blake3 hex",
        ),
        None => push_issue(
            issues,
            "missing_field",
            &format!("{prefix}.{field}"),
            "missing hash field",
        ),
    }
}

fn validate_optional_receipt_hash(
    value: &Value,
    issues: &mut Vec<SemanticReviewIssue>,
    expected: String,
) {
    if let Some(actual) = value.get("receipt_hash").and_then(Value::as_str) {
        if actual != expected {
            push_issue(
                issues,
                "receipt_hash_mismatch",
                "receipt_hash",
                "receipt hash does not match canonical receipt material",
            );
        }
    }
}

fn report(value: &Value, issues: Vec<SemanticReviewIssue>) -> SemanticReviewReport {
    let valid = issues.is_empty();
    SemanticReviewReport {
        valid,
        verdict: if valid { "PASS" } else { "FAIL" }.to_string(),
        issues,
        receipt_hash: if value.get("schema_version").and_then(Value::as_str)
            == Some(REVISION_HANDOFF_SCHEMA_V1)
        {
            revision_handoff_hash(value)
        } else {
            review_receipt_hash(value)
        },
    }
}

pub fn issues_json(issues: &[SemanticReviewIssue]) -> Value {
    Value::Array(
        issues
            .iter()
            .map(|issue| {
                json!({
                    "code": issue.code,
                    "field_path": issue.field_path,
                    "message": issue.message,
                })
            })
            .collect(),
    )
}

fn without_hash_field(value: &Value, field: &str) -> Value {
    match value {
        Value::Object(object) => {
            let mut cloned = object.clone();
            cloned.remove(field);
            Value::Object(cloned)
        }
        other => other.clone(),
    }
}

fn count_string_field(values: &[Value], field: &str) -> Value {
    let mut counts = Map::new();
    for value in values {
        if let Some(text) = value.get(field).and_then(Value::as_str) {
            increment_count(&mut counts, text);
        }
    }
    sort_count_map(&mut counts)
}

fn increment_count(counts: &mut Map<String, Value>, key: &str) {
    let current = counts.get(key).and_then(Value::as_u64).unwrap_or(0);
    counts.insert(key.to_string(), json!(current + 1));
}

fn sort_count_map(counts: &mut Map<String, Value>) -> Value {
    let mut pairs = counts
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect::<Vec<_>>();
    pairs.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted = Map::new();
    for (key, value) in pairs {
        sorted.insert(key, value);
    }
    Value::Object(sorted)
}

fn sorted_unique(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}

fn is_blake3_hex(value: &str) -> bool {
    value.len() == 64 && value.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn push_issue(issues: &mut Vec<SemanticReviewIssue>, code: &str, field_path: &str, message: &str) {
    issues.push(SemanticReviewIssue {
        code: code.to_string(),
        field_path: field_path.to_string(),
        message: message.to_string(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash() -> String {
        "a".repeat(64)
    }

    #[test]
    fn approved_review_receipt_can_have_no_findings() {
        let value = json!({
            "schema_version": REVIEW_RECEIPT_SCHEMA_V1,
            "review_kind": "plan",
            "decision": "approved",
            "reviewed_inputs": [{"path": "proposal.md", "hash": hash()}],
            "reviewer": {"source": "manual", "id": "reviewer"},
            "findings": []
        });
        let report = validate_review_receipt(&value, &[("proposal.md".to_string(), hash())]);
        assert!(report.valid, "{:?}", report.issues);
    }

    #[test]
    fn non_approved_review_requires_categorized_findings() {
        let value = json!({
            "schema_version": REVIEW_RECEIPT_SCHEMA_V1,
            "review_kind": "design",
            "decision": "needs_revision",
            "reviewed_inputs": [{"path": "design.md", "hash": hash()}],
            "reviewer": {"source": "agentic", "id": "gemini"},
            "findings": []
        });
        let report = validate_review_receipt(&value, &[]);
        assert!(!report.valid);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "missing_findings"));
    }

    #[test]
    fn completeness_receipt_reports_missing_links() {
        // r[verify cairn.semantic_review.completeness_check_rail.mapping]
        // r[verify cairn.semantic_review.completeness_check_rail.mapping.mapping.active_change]
        // r[verify cairn.semantic_review.completeness_check_rail.mapping.mapping.accepted_spec]
        // r[verify cairn.semantic_review.completeness_check_rail.omissions]
        // r[verify cairn.semantic_review.completeness_check_rail.omissions.omissions.missing_task]
        // r[verify cairn.semantic_review.completeness_check_rail.omissions.omissions.missing_verification]
        let receipt = completeness_receipt(
            vec!["cairn.example.requirement".to_string()],
            vec![],
            vec!["cairn.example.requirement".to_string()],
            vec![],
        );
        assert_eq!(receipt["valid"], false);
        let findings = receipt["findings"].as_array().unwrap();
        assert!(findings
            .iter()
            .any(|finding| finding["code"] == "missing_task_coverage"));
        assert!(findings
            .iter()
            .any(|finding| finding["code"] == "missing_verification_coverage"));
    }
}
