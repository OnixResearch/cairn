// r[impl semantic_review.split_semantic_review_core.focused_modules]
// r[impl semantic_review.split_semantic_review_core.focused_modules.visible_boundaries]
// r[impl semantic_review.split_semantic_review_core.focused_modules.narrow_bridges]
mod analytics;
mod common;
mod completeness;
mod metrics_regression;
mod profile;
mod review_receipt;
mod revision_handoff;

pub use analytics::review_ledger_analytics;
pub use common::issues_json;
pub use completeness::completeness_receipt;
pub use metrics_regression::{
    validate_review_metrics_registry, REVIEW_METRICS_REGRESSION_SCHEMA_V1,
};
pub use profile::semantic_review_policy_profile_json;
pub use review_receipt::{review_receipt_hash, validate_review_receipt};
pub use revision_handoff::{revision_handoff_hash, validate_revision_handoff};

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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn hash() -> String {
        "a".repeat(64)
    }

    #[test]
    fn approved_review_receipt_can_have_no_findings() {
        // r[verify semantic_review.split_semantic_review_core.behavior_preserved]
        // r[verify semantic_review.split_semantic_review_core.behavior_preserved.focused_checks]
        // r[verify semantic_review.split_semantic_review_core.behavior_preserved.repository_checks]
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

    #[test]
    fn review_metrics_registry_accepts_complete_category_binding() {
        // r[verify cairn.semantic_review.review_metrics_regression_rail.registry_binds_surfaces]
        let registry = json!({
            "schema_version": REVIEW_METRICS_REGRESSION_SCHEMA_V1,
            "categories": [{
                "code": "tasks-auto-fix",
                "metrics_class": "omission",
                "metrics_stage": "tasks",
                "metrics_source": "agentic",
                "diagnostic_code": "missing-auto-fix-task",
                "requirement_id": "project.review.auto_fix",
                "guidance_anchor": "# auto-fix guidance",
                "positive_fixture": "fixtures/positive-auto-fix",
                "negative_fixture": "fixtures/negative-auto-fix",
                "evidence_ref": "evidence/metrics-snapshot.md"
            }]
        });
        let report = validate_review_metrics_registry(&registry);
        assert!(report.valid, "{:?}", report.issues);
        assert_eq!(report.verdict, "PASS");
    }

    #[test]
    fn review_metrics_registry_rejects_missing_and_duplicate_bindings() {
        // r[verify cairn.semantic_review.review_metrics_regression_rail.registry_binds_surfaces]
        let registry = json!({
            "schema_version": REVIEW_METRICS_REGRESSION_SCHEMA_V1,
            "categories": [
                {
                    "code": "tasks-auto-fix",
                    "metrics_class": "unknown",
                    "metrics_stage": "tasks",
                    "metrics_source": "agentic",
                    "diagnostic_code": "missing-auto-fix-task",
                    "requirement_id": "project.review.auto_fix",
                    "guidance_anchor": "# auto-fix guidance",
                    "positive_fixture": "fixtures/positive-auto-fix",
                    "negative_fixture": "fixtures/negative-auto-fix",
                    "evidence_ref": "evidence/metrics-snapshot.md"
                },
                {
                    "code": "tasks-auto-fix",
                    "metrics_class": "omission",
                    "metrics_stage": "tasks",
                    "metrics_source": "agentic",
                    "diagnostic_code": "missing-auto-fix-task",
                    "requirement_id": "project.review.auto_fix",
                    "guidance_anchor": "",
                    "positive_fixture": "fixtures/positive-auto-fix-2",
                    "negative_fixture": "fixtures/negative-auto-fix-2",
                    "evidence_ref": "evidence/metrics-snapshot.md"
                }
            ]
        });
        let report = validate_review_metrics_registry(&registry);
        assert!(!report.valid);
        let codes = report
            .issues
            .iter()
            .map(|issue| issue.code.as_str())
            .collect::<Vec<_>>();
        assert!(codes.contains(&"invalid_registry_binding"));
        assert!(codes.contains(&"missing_registry_binding"));
        assert!(codes.contains(&"duplicate_registry_binding"));
    }
}
