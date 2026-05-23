use std::collections::{BTreeMap, BTreeSet};

use serde_json::{json, Value};

use crate::verified::evidence::canonical_json_hash;

use super::common::{push_issue, require_string};
use super::{SemanticReviewIssue, SemanticReviewReport, REVIEW_CATEGORIES, REVIEW_SOURCES};

pub const REVIEW_METRICS_REGRESSION_SCHEMA_V1: &str =
    "cairn.semantic-review.metrics-regression-registry.v1";
pub const REVIEW_METRICS_FIXTURE_COVERAGE_SCHEMA_V1: &str =
    "cairn.semantic-review.metrics-regression-fixtures.v1";
pub const REVIEW_METRICS_GUIDANCE_SYNC_SCHEMA_V1: &str =
    "cairn.semantic-review.metrics-regression-guidance-sync.v1";
pub const REVIEW_METRICS_EVIDENCE_BOUNDARY_SCHEMA_V1: &str =
    "cairn.semantic-review.metrics-regression-evidence-boundary.v1";

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

pub fn validate_review_metrics_guidance_sync(
    registry: &Value,
    sync_surfaces: &Value,
) -> SemanticReviewReport {
    // r[impl cairn.semantic_review.review_metrics_regression_rail.guidance_task_sync]
    let mut issues = validate_review_metrics_registry(registry).issues;
    require_string(
        sync_surfaces,
        "schema_version",
        REVIEW_METRICS_GUIDANCE_SYNC_SCHEMA_V1,
        &mut issues,
    );
    let guidance = string_set_from_entries(
        sync_surfaces,
        "guidance",
        "diagnostics",
        "missing_guidance_sync",
        "operator guidance must declare diagnostics",
        &mut issues,
    );
    let anchors = string_set_from_scalar_entries(
        sync_surfaces,
        "guidance",
        "anchor",
        "missing_guidance_anchor",
        "operator guidance must declare anchors",
        &mut issues,
    );
    let task_requirements = string_set_from_entries(
        sync_surfaces,
        "tasks",
        "requirement_ids",
        "missing_task_sync",
        "task artifacts must declare requirement references",
        &mut issues,
    );
    let spec_requirements = string_set_from_entries(
        sync_surfaces,
        "specs",
        "requirement_ids",
        "missing_spec_sync",
        "spec artifacts must declare requirement references",
        &mut issues,
    );
    validate_guidance_sync(
        registry,
        &guidance,
        &anchors,
        &task_requirements,
        &spec_requirements,
        &mut issues,
    );
    build_report(REVIEW_METRICS_GUIDANCE_SYNC_SCHEMA_V1, issues)
}

pub fn validate_review_metrics_evidence_boundary(
    registry: &Value,
    evidence: &Value,
) -> SemanticReviewReport {
    // r[impl cairn.semantic_review.review_metrics_regression_rail.secret_free_evidence]
    let mut issues = validate_review_metrics_registry(registry).issues;
    require_string(
        evidence,
        "schema_version",
        REVIEW_METRICS_EVIDENCE_BOUNDARY_SCHEMA_V1,
        &mut issues,
    );
    let evidence_refs = evidence_ref_map(evidence, &mut issues);
    validate_category_evidence_refs(registry, &evidence_refs, &mut issues);
    validate_secret_free_json(evidence, "evidence", &mut issues);
    build_report(REVIEW_METRICS_EVIDENCE_BOUNDARY_SCHEMA_V1, issues)
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

fn validate_guidance_sync(
    registry: &Value,
    guidance_diagnostics: &BTreeSet<String>,
    guidance_anchors: &BTreeSet<String>,
    task_requirements: &BTreeSet<String>,
    spec_requirements: &BTreeSet<String>,
    issues: &mut Vec<SemanticReviewIssue>,
) {
    let Some(categories) = registry.get("categories").and_then(Value::as_array) else {
        return;
    };
    for (index, category) in categories.iter().enumerate() {
        validate_category_guidance_sync(
            category,
            index,
            guidance_diagnostics,
            guidance_anchors,
            task_requirements,
            spec_requirements,
            issues,
        );
    }
}

fn validate_category_guidance_sync(
    category: &Value,
    index: usize,
    guidance_diagnostics: &BTreeSet<String>,
    guidance_anchors: &BTreeSet<String>,
    task_requirements: &BTreeSet<String>,
    spec_requirements: &BTreeSet<String>,
    issues: &mut Vec<SemanticReviewIssue>,
) {
    if let Some(diagnostic_code) = non_empty_string(category, "diagnostic_code") {
        if !guidance_diagnostics.contains(diagnostic_code) {
            push_issue(
                issues,
                "guidance_missing_diagnostic",
                &format!("categories[{index}].diagnostic_code"),
                "operator guidance must document the category diagnostic",
            );
        }
    }
    if let Some(guidance_anchor) = non_empty_string(category, "guidance_anchor") {
        if !guidance_anchors.contains(guidance_anchor) {
            push_issue(
                issues,
                "guidance_missing_anchor",
                &format!("categories[{index}].guidance_anchor"),
                "operator guidance must expose the category guidance anchor",
            );
        }
    }
    if let Some(requirement_id) = non_empty_string(category, "requirement_id") {
        if !task_requirements.contains(requirement_id) {
            push_issue(
                issues,
                "task_missing_requirement_ref",
                &format!("categories[{index}].requirement_id"),
                "task artifacts must reference the category requirement id",
            );
        }
        if !spec_requirements.contains(requirement_id) {
            push_issue(
                issues,
                "spec_missing_requirement_ref",
                &format!("categories[{index}].requirement_id"),
                "spec artifacts must reference the category requirement id",
            );
        }
    }
}

fn evidence_ref_map<'a>(
    evidence: &'a Value,
    issues: &mut Vec<SemanticReviewIssue>,
) -> BTreeMap<&'a str, &'a Value> {
    let mut refs = BTreeMap::new();
    let Some(entries) = evidence.get("evidence").and_then(Value::as_array) else {
        push_issue(
            issues,
            "missing_evidence_entries",
            "evidence",
            "evidence boundary must declare sanitized evidence entries",
        );
        return refs;
    };
    if entries.is_empty() {
        push_issue(
            issues,
            "empty_evidence_entries",
            "evidence",
            "evidence boundary must declare sanitized evidence entries",
        );
        return refs;
    }
    for (index, entry) in entries.iter().enumerate() {
        collect_evidence_entry(entry, index, &mut refs, issues);
    }
    refs
}

fn collect_evidence_entry<'a>(
    entry: &'a Value,
    index: usize,
    refs: &mut BTreeMap<&'a str, &'a Value>,
    issues: &mut Vec<SemanticReviewIssue>,
) {
    if !entry.is_object() {
        push_issue(
            issues,
            "invalid_evidence_entry",
            &format!("evidence[{index}]"),
            "evidence entry must be an object",
        );
        return;
    }
    let Some(evidence_ref) = non_empty_string(entry, "ref") else {
        push_issue(
            issues,
            "invalid_evidence_entry",
            &format!("evidence[{index}].ref"),
            "evidence entry must bind a non-empty ref",
        );
        return;
    };
    for field in [
        "category_code",
        "metrics_class",
        "metrics_stage",
        "metrics_source",
    ] {
        if non_empty_string(entry, field).is_none() {
            push_issue(
                issues,
                "missing_evidence_metadata",
                &format!("evidence[{index}].{field}"),
                "evidence entry must include safe category metadata",
            );
        }
    }
    if entry.get("provenance").and_then(Value::as_object).is_none() {
        push_issue(
            issues,
            "missing_evidence_provenance",
            &format!("evidence[{index}].provenance"),
            "evidence entry must include provenance for review",
        );
    }
    if refs.insert(evidence_ref, entry).is_some() {
        push_issue(
            issues,
            "duplicate_evidence_ref",
            &format!("evidence[{index}].ref"),
            "evidence refs must be unique",
        );
    }
}

fn validate_category_evidence_refs(
    registry: &Value,
    evidence_refs: &BTreeMap<&str, &Value>,
    issues: &mut Vec<SemanticReviewIssue>,
) {
    let Some(categories) = registry.get("categories").and_then(Value::as_array) else {
        return;
    };
    for (index, category) in categories.iter().enumerate() {
        let Some(evidence_ref) = non_empty_string(category, "evidence_ref") else {
            continue;
        };
        match evidence_refs.get(evidence_ref) {
            Some(entry) => validate_evidence_metadata_match(category, entry, index, issues),
            None => push_issue(
                issues,
                "missing_evidence_ref",
                &format!("categories[{index}].evidence_ref"),
                "registry evidence_ref must resolve to sanitized evidence",
            ),
        }
    }
}

fn validate_evidence_metadata_match(
    category: &Value,
    entry: &Value,
    index: usize,
    issues: &mut Vec<SemanticReviewIssue>,
) {
    for (category_field, evidence_field) in [
        ("code", "category_code"),
        ("metrics_class", "metrics_class"),
        ("metrics_stage", "metrics_stage"),
        ("metrics_source", "metrics_source"),
    ] {
        let category_value = non_empty_string(category, category_field);
        let evidence_value = non_empty_string(entry, evidence_field);
        if category_value.is_some() && evidence_value.is_some() && category_value != evidence_value
        {
            push_issue(
                issues,
                "evidence_metadata_mismatch",
                &format!("categories[{index}].evidence_ref"),
                "evidence metadata must match registry category bindings",
            );
        }
    }
}

fn validate_secret_free_json(
    value: &Value,
    field_path: &str,
    issues: &mut Vec<SemanticReviewIssue>,
) {
    match value {
        Value::Object(map) => {
            for (key, nested) in map {
                let nested_path = format!("{field_path}.{key}");
                if unsafe_evidence_key(key) {
                    push_issue(
                        issues,
                        "unsafe_evidence_field",
                        &nested_path,
                        "evidence must not include secret-bearing or raw-private fields",
                    );
                }
                validate_secret_free_json(nested, &nested_path, issues);
            }
        }
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                validate_secret_free_json(item, &format!("{field_path}[{index}]"), issues);
            }
        }
        Value::String(text) => {
            if unsafe_evidence_text(text) {
                push_issue(
                    issues,
                    "unsafe_evidence_text",
                    field_path,
                    "evidence text must be sanitized before it is referenced",
                );
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
    }
}

fn unsafe_evidence_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase();
    [
        "api_key",
        "authorization",
        "cookie",
        "credential",
        "connection_string",
        "hidden_prompt",
        "password",
        "provider_payload",
        "raw_transcript",
        "request_body",
        "response_body",
        "secret",
        "system_prompt",
        "token",
    ]
    .iter()
    .any(|needle| key.contains(needle))
        || key == "account_id"
}

fn unsafe_evidence_text(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    [
        "api_key=",
        "authorization:",
        "bearer ",
        "connection_string=",
        "password=",
        "provider payload",
        "raw transcript",
        "secret=",
        "system prompt:",
        "token=",
    ]
    .iter()
    .any(|needle| text.contains(needle))
        || looks_like_email(&text)
}

fn looks_like_email(text: &str) -> bool {
    let Some(at_index) = text.find('@') else {
        return false;
    };
    let before = &text[..at_index];
    let after = &text[at_index + 1..];
    !before.is_empty() && after.contains('.') && after.chars().any(|ch| ch.is_ascii_alphabetic())
}

fn string_set_from_entries(
    value: &Value,
    array_field: &str,
    values_field: &str,
    missing_code: &str,
    missing_message: &str,
    issues: &mut Vec<SemanticReviewIssue>,
) -> BTreeSet<String> {
    let mut values = BTreeSet::new();
    let Some(entries) = value.get(array_field).and_then(Value::as_array) else {
        push_issue(issues, missing_code, array_field, missing_message);
        return values;
    };
    if entries.is_empty() {
        push_issue(issues, missing_code, array_field, missing_message);
        return values;
    }
    for (entry_index, entry) in entries.iter().enumerate() {
        collect_entry_strings(
            entry,
            entry_index,
            array_field,
            values_field,
            &mut values,
            issues,
        );
    }
    values
}

fn string_set_from_scalar_entries(
    value: &Value,
    array_field: &str,
    scalar_field: &str,
    missing_code: &str,
    missing_message: &str,
    issues: &mut Vec<SemanticReviewIssue>,
) -> BTreeSet<String> {
    let mut values = BTreeSet::new();
    let Some(entries) = value.get(array_field).and_then(Value::as_array) else {
        push_issue(issues, missing_code, array_field, missing_message);
        return values;
    };
    if entries.is_empty() {
        push_issue(issues, missing_code, array_field, missing_message);
        return values;
    }
    for (entry_index, entry) in entries.iter().enumerate() {
        match entry.get(scalar_field).and_then(Value::as_str) {
            Some(text) if !text.trim().is_empty() => {
                values.insert(text.to_string());
            }
            Some(_) | None => push_issue(
                issues,
                "invalid_guidance_sync",
                &format!("{array_field}[{entry_index}].{scalar_field}"),
                "sync surface entry must declare a non-empty string binding",
            ),
        }
    }
    values
}

fn collect_entry_strings(
    entry: &Value,
    entry_index: usize,
    array_field: &str,
    values_field: &str,
    values: &mut BTreeSet<String>,
    issues: &mut Vec<SemanticReviewIssue>,
) {
    let Some(strings) = entry.get(values_field).and_then(Value::as_array) else {
        push_issue(
            issues,
            "invalid_guidance_sync",
            &format!("{array_field}[{entry_index}].{values_field}"),
            "sync surface entry must declare string references",
        );
        return;
    };
    if strings.is_empty() {
        push_issue(
            issues,
            "invalid_guidance_sync",
            &format!("{array_field}[{entry_index}].{values_field}"),
            "sync surface entry must declare at least one reference",
        );
        return;
    }
    for (value_index, value) in strings.iter().enumerate() {
        match value.as_str() {
            Some(text) if !text.trim().is_empty() => {
                values.insert(text.to_string());
            }
            Some(_) | None => push_issue(
                issues,
                "invalid_guidance_sync",
                &format!("{array_field}[{entry_index}].{values_field}[{value_index}]"),
                "sync surface references must be non-empty strings",
            ),
        }
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
