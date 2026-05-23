use std::collections::{BTreeMap, BTreeSet};

use serde_json::{json, Value};

use crate::verified::evidence::canonical_json_hash;

use super::common::{push_issue, require_string};
use super::{SemanticReviewIssue, SemanticReviewReport, REVIEW_CATEGORIES, REVIEW_SOURCES};

pub const REVIEW_METRICS_REGRESSION_SCHEMA_V1: &str =
    "cairn.semantic-review.metrics-regression-registry.v1";
pub const REVIEW_METRICS_FIXTURE_COVERAGE_SCHEMA_V1: &str =
    "cairn.semantic-review.metrics-regression-fixtures.v1";

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FixtureOutcome<'a> {
    valid: bool,
    diagnostics: &'a [Value],
}

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
    build_report(REVIEW_METRICS_REGRESSION_SCHEMA_V1, issues)
}

pub fn validate_review_metrics_fixture_pairs(
    registry: &Value,
    fixture_coverage: &Value,
) -> SemanticReviewReport {
    // r[impl cairn.semantic_review.review_metrics_regression_rail.fixture_pair_enforcement]
    let mut issues = validate_review_metrics_registry(registry).issues;
    require_string(
        fixture_coverage,
        "schema_version",
        REVIEW_METRICS_FIXTURE_COVERAGE_SCHEMA_V1,
        &mut issues,
    );
    let fixture_results = fixture_result_map(fixture_coverage, &mut issues);
    validate_fixture_pairs(registry, &fixture_results, &mut issues);
    build_report(REVIEW_METRICS_FIXTURE_COVERAGE_SCHEMA_V1, issues)
}

fn build_report(
    schema_version: &str,
    mut issues: Vec<SemanticReviewIssue>,
) -> SemanticReviewReport {
    issues.sort_by(|left, right| {
        left.field_path
            .cmp(&right.field_path)
            .then(left.code.cmp(&right.code))
            .then(left.message.cmp(&right.message))
    });
    let valid = issues.is_empty();
    let verdict = if valid { "PASS" } else { "FAIL" }.to_string();
    let receipt_hash = canonical_json_hash(&json!({
        "schema_version": schema_version,
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

fn fixture_result_map<'a>(
    fixture_coverage: &'a Value,
    issues: &mut Vec<SemanticReviewIssue>,
) -> BTreeMap<&'a str, FixtureOutcome<'a>> {
    let mut results = BTreeMap::new();
    let Some(fixtures) = fixture_coverage.get("fixtures").and_then(Value::as_array) else {
        push_issue(
            issues,
            "missing_fixture_results",
            "fixtures",
            "fixture coverage must declare fixture results",
        );
        return results;
    };
    if fixtures.is_empty() {
        push_issue(
            issues,
            "empty_fixture_results",
            "fixtures",
            "fixture coverage must declare fixture results",
        );
        return results;
    }
    for (index, fixture) in fixtures.iter().enumerate() {
        collect_fixture_result(fixture, index, &mut results, issues);
    }
    results
}

fn collect_fixture_result<'a>(
    fixture: &'a Value,
    index: usize,
    results: &mut BTreeMap<&'a str, FixtureOutcome<'a>>,
    issues: &mut Vec<SemanticReviewIssue>,
) {
    let path = match fixture.get("path").and_then(Value::as_str) {
        Some(path) if !path.trim().is_empty() => path,
        Some(_) | None => {
            push_issue(
                issues,
                "invalid_fixture_result",
                &format!("fixtures[{index}].path"),
                "fixture result must bind a non-empty path",
            );
            return;
        }
    };
    let Some(valid) = fixture.get("valid").and_then(Value::as_bool) else {
        push_issue(
            issues,
            "invalid_fixture_result",
            &format!("fixtures[{index}].valid"),
            "fixture result must bind a boolean valid field",
        );
        return;
    };
    let Some(diagnostics) = fixture.get("diagnostics").and_then(Value::as_array) else {
        push_issue(
            issues,
            "invalid_fixture_result",
            &format!("fixtures[{index}].diagnostics"),
            "fixture result must bind diagnostics as an array",
        );
        return;
    };
    if diagnostics
        .iter()
        .any(|diagnostic| diagnostic.as_str().is_none())
    {
        push_issue(
            issues,
            "invalid_fixture_result",
            &format!("fixtures[{index}].diagnostics"),
            "fixture diagnostics must be strings",
        );
        return;
    }
    if results
        .insert(path, FixtureOutcome { valid, diagnostics })
        .is_some()
    {
        push_issue(
            issues,
            "duplicate_fixture_result",
            &format!("fixtures[{index}].path"),
            "fixture result path must be unique",
        );
    }
}

fn validate_fixture_pairs(
    registry: &Value,
    fixture_results: &BTreeMap<&str, FixtureOutcome<'_>>,
    issues: &mut Vec<SemanticReviewIssue>,
) {
    let Some(categories) = registry.get("categories").and_then(Value::as_array) else {
        return;
    };
    for (index, category) in categories.iter().enumerate() {
        let Some(diagnostic_code) = non_empty_string(category, "diagnostic_code") else {
            continue;
        };
        validate_positive_fixture(category, index, fixture_results, issues);
        validate_negative_fixture(category, index, diagnostic_code, fixture_results, issues);
    }
}

fn validate_positive_fixture(
    category: &Value,
    index: usize,
    fixture_results: &BTreeMap<&str, FixtureOutcome<'_>>,
    issues: &mut Vec<SemanticReviewIssue>,
) {
    let Some(fixture_ref) = non_empty_string(category, "positive_fixture") else {
        return;
    };
    match fixture_results.get(fixture_ref) {
        Some(outcome) if outcome.valid => {}
        Some(_) => push_issue(
            issues,
            "positive_fixture_failed",
            &format!("categories[{index}].positive_fixture"),
            "positive fixture must pass",
        ),
        None => push_issue(
            issues,
            "missing_fixture_result",
            &format!("categories[{index}].positive_fixture"),
            "positive fixture result is missing",
        ),
    }
}

fn validate_negative_fixture(
    category: &Value,
    index: usize,
    diagnostic_code: &str,
    fixture_results: &BTreeMap<&str, FixtureOutcome<'_>>,
    issues: &mut Vec<SemanticReviewIssue>,
) {
    let Some(fixture_ref) = non_empty_string(category, "negative_fixture") else {
        return;
    };
    match fixture_results.get(fixture_ref) {
        Some(outcome) if outcome.valid => push_issue(
            issues,
            "negative_fixture_passed",
            &format!("categories[{index}].negative_fixture"),
            "negative fixture must fail",
        ),
        Some(outcome) if has_diagnostic(outcome.diagnostics, diagnostic_code) => {}
        Some(_) => push_issue(
            issues,
            "negative_fixture_missing_diagnostic",
            &format!("categories[{index}].negative_fixture"),
            "negative fixture must emit the category diagnostic",
        ),
        None => push_issue(
            issues,
            "missing_fixture_result",
            &format!("categories[{index}].negative_fixture"),
            "negative fixture result is missing",
        ),
    }
}

fn non_empty_string<'a>(value: &'a Value, field: &str) -> Option<&'a str> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|text| !text.trim().is_empty())
}

fn has_diagnostic(diagnostics: &[Value], diagnostic_code: &str) -> bool {
    diagnostics
        .iter()
        .filter_map(Value::as_str)
        .any(|diagnostic| diagnostic == diagnostic_code)
}
