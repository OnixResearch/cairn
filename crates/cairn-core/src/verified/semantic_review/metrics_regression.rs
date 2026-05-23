use std::collections::BTreeSet;

use serde_json::{json, Value};

use crate::verified::evidence::canonical_json_hash;

use super::common::{push_issue, require_string};
use super::{SemanticReviewIssue, SemanticReviewReport, REVIEW_CATEGORIES, REVIEW_SOURCES};

pub const REVIEW_METRICS_REGRESSION_SCHEMA_V1: &str =
    "cairn.semantic-review.metrics-regression-registry.v1";

const REQUIRED_CATEGORY_FIELDS: &[&str] = &[
    "code",
    "metrics_class",
    "metrics_stage",
    "metrics_source",
    "diagnostic_code",
    "requirement_id",
    "guidance_anchor",
    "positive_fixture",
    "negative_fixture",
    "evidence_ref",
];

const METRICS_STAGES: &[&str] = &[
    "proposal", "design", "tasks", "spec", "plan", "code", "diff",
];

pub fn validate_review_metrics_registry(value: &Value) -> SemanticReviewReport {
    // r[impl cairn.semantic_review.review_metrics_regression_rail.registry_binds_surfaces]
    let mut issues = Vec::new();
    require_string(
        value,
        "schema_version",
        REVIEW_METRICS_REGRESSION_SCHEMA_V1,
        &mut issues,
    );
    validate_categories(value, &mut issues);
    issues.sort_by(|left, right| {
        left.field_path
            .cmp(&right.field_path)
            .then(left.code.cmp(&right.code))
            .then(left.message.cmp(&right.message))
    });
    let valid = issues.is_empty();
    let verdict = if valid { "PASS" } else { "FAIL" }.to_string();
    let receipt_hash = canonical_json_hash(&json!({
        "schema_version": REVIEW_METRICS_REGRESSION_SCHEMA_V1,
        "valid": valid,
        "verdict": verdict,
        "issues": super::common::issues_json(&issues),
    }));
    SemanticReviewReport {
        valid,
        verdict,
        issues,
        receipt_hash,
    }
}

fn validate_categories(value: &Value, issues: &mut Vec<SemanticReviewIssue>) {
    let Some(categories) = value.get("categories").and_then(Value::as_array) else {
        push_issue(
            issues,
            "missing_registry_categories",
            "categories",
            "registry must declare at least one category",
        );
        return;
    };
    if categories.is_empty() {
        push_issue(
            issues,
            "empty_registry_categories",
            "categories",
            "registry must declare at least one category",
        );
        return;
    }

    let mut codes = BTreeSet::new();
    let mut diagnostics = BTreeSet::new();
    let mut requirement_ids = BTreeSet::new();
    for (index, category) in categories.iter().enumerate() {
        validate_category_fields(category, index, issues);
        track_unique(category, index, "code", &mut codes, issues);
        track_unique(category, index, "diagnostic_code", &mut diagnostics, issues);
        track_unique(
            category,
            index,
            "requirement_id",
            &mut requirement_ids,
            issues,
        );
    }
}

fn validate_category_fields(category: &Value, index: usize, issues: &mut Vec<SemanticReviewIssue>) {
    if !category.is_object() {
        push_issue(
            issues,
            "invalid_registry_category",
            &format!("categories[{index}]"),
            "category entry must be an object",
        );
        return;
    }
    for field in REQUIRED_CATEGORY_FIELDS {
        match category.get(*field).and_then(Value::as_str) {
            Some(text) if !text.trim().is_empty() => {}
            Some(_) | None => push_issue(
                issues,
                "missing_registry_binding",
                &format!("categories[{index}].{field}"),
                "category registry entry is missing a required non-empty binding",
            ),
        }
    }
    validate_allowed(category, index, "metrics_class", REVIEW_CATEGORIES, issues);
    validate_allowed(category, index, "metrics_stage", METRICS_STAGES, issues);
    validate_allowed(category, index, "metrics_source", REVIEW_SOURCES, issues);
}

fn validate_allowed(
    category: &Value,
    index: usize,
    field: &str,
    allowed: &[&str],
    issues: &mut Vec<SemanticReviewIssue>,
) {
    if let Some(actual) = category.get(field).and_then(Value::as_str) {
        if !actual.trim().is_empty() && !allowed.contains(&actual) {
            push_issue(
                issues,
                "invalid_registry_binding",
                &format!("categories[{index}].{field}"),
                "category registry binding is not policy-known",
            );
        }
    }
}

fn track_unique(
    category: &Value,
    index: usize,
    field: &str,
    seen: &mut BTreeSet<String>,
    issues: &mut Vec<SemanticReviewIssue>,
) {
    if let Some(actual) = category.get(field).and_then(Value::as_str) {
        if actual.trim().is_empty() {
            return;
        }
        if !seen.insert(actual.to_string()) {
            push_issue(
                issues,
                "duplicate_registry_binding",
                &format!("categories[{index}].{field}"),
                "category registry binding must be unique",
            );
        }
    }
}
