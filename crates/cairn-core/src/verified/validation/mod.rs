// r[impl cli_maintainability.core_validation_modules.focused_modules]
// r[impl cli_maintainability.core_validation_modules.focused_modules.visible_boundaries]
// r[impl cli_maintainability.core_validation_modules.focused_modules.narrow_bridges]
// r[impl cli_maintainability.core_validation_modules.behavior_preserved]
// r[verify cli_maintainability.core_validation_modules.behavior_preserved.focused_checks]
// r[verify cli_maintainability.core_validation_modules.behavior_preserved.repository_checks]
mod changes;
mod gates;
mod markdown;
mod references;
mod spec;

pub use changes::{change_issues, validate_changes};
pub use gates::{
    evaluate_gate, evaluate_gate_snapshot, GateArtifact, GateIssue, GateReport, GateSnapshot,
    GateStage,
};
pub use spec::{validate_spec_source, validate_spec_sources};

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::verified::policy::{ExemptionMarkerPolicy, Policy, ReceiptSchema, TaskMarker};
    use crate::verified::repo::{ChangeStatus, SpecSource, TaskCounts};

    fn policy(enabled: bool, strictness: &str) -> Policy {
        Policy {
            name: "test".to_string(),
            version: 1,
            task_markers_enabled: enabled,
            task_marker_strictness: strictness.to_string(),
            task_marker_tokens: vec![
                TaskMarker {
                    id: "parallel".to_string(),
                    token: "[parallel]".to_string(),
                    kind: "literal".to_string(),
                },
                TaskMarker {
                    id: "serial".to_string(),
                    token: "[serial]".to_string(),
                    kind: "literal".to_string(),
                },
                TaskMarker {
                    id: "depends".to_string(),
                    token: "[depends:".to_string(),
                    kind: "prefix".to_string(),
                },
            ],
            gate_stages: vec![
                "proposal".to_string(),
                "design".to_string(),
                "tasks".to_string(),
            ],
            gate_ignore_archives: true,
            gate_finding_scope: "tasks".to_string(),
            gate_finding_promoted: "no".to_string(),
            sync_require_gate: true,
            archive_require_gate: true,
            migrate_require_manifest: true,
            require_receipt_hashes: true,
            receipt_schemas: vec![ReceiptSchema {
                command: "gate".to_string(),
                top_level_keys: vec!["verdict".to_string()],
                action_keys: Vec::new(),
                issue_keys: vec![
                    "message".to_string(),
                    "scope".to_string(),
                    "promoted".to_string(),
                ],
                hash_fields: vec!["receipt_hash".to_string()],
            }],
            exemption_marker_policy: ExemptionMarkerPolicy {
                required_fields: vec![
                    "reason".to_string(),
                    "owner".to_string(),
                    "until".to_string(),
                ],
                date_fields: vec!["until".to_string()],
                optional_fields: vec!["scope".to_string()],
            },
            change_metadata_allowed_groups: vec!["current".to_string(), "future".to_string()],
            change_metadata_group_prefixes: vec!["feature:".to_string()],
            change_metadata_statuses: vec![
                "current".to_string(),
                "planned".to_string(),
                "blocked".to_string(),
                "deferred".to_string(),
            ],
            raw: json!({}),
        }
    }

    fn change() -> ChangeStatus {
        ChangeStatus {
            name: "demo".to_string(),
            path: "openspec/changes/demo".to_string(),
            has_proposal: true,
            has_design: false,
            has_tasks: true,
            delta_specs: Vec::new(),
            task_counts: Some(TaskCounts {
                total: 2,
                done: 0,
                in_progress: 0,
                todo: 2,
                unmarked: 0,
            }),
            metadata: Default::default(),
            metadata_issues: Vec::new(),
        }
    }

    #[test]
    fn accepts_complete_marked_change() {
        assert!(change_issues(&change(), &policy(true, "strict")).is_empty());
    }

    #[test]
    fn rejects_missing_required_change_documents() {
        let mut change = change();
        change.has_proposal = false;
        change.has_tasks = false;
        assert_eq!(
            change_issues(&change, &policy(true, "strict")),
            vec![
                "demo: missing proposal.md".to_string(),
                "demo: missing tasks.md".to_string(),
            ]
        );
    }

    #[test]
    fn rejects_unmarked_tasks_only_when_strict_markers_are_enabled() {
        let mut change = change();
        change.task_counts = Some(TaskCounts {
            total: 1,
            done: 0,
            in_progress: 0,
            todo: 1,
            unmarked: 1,
        });
        assert_eq!(
            change_issues(&change, &policy(true, "strict")),
            vec!["demo: 1 task(s) missing concurrency markers".to_string()]
        );
        assert!(change_issues(&change, &policy(false, "strict")).is_empty());
        assert!(change_issues(&change, &policy(true, "advisory")).is_empty());
    }

    #[test]
    fn evaluates_stage_gates_from_change_snapshot() {
        let policy = policy(true, "strict");
        let proposal = evaluate_gate(&change(), GateStage::Proposal, &policy);
        assert_eq!(proposal.verdict, "PASS");

        let mut incomplete = change();
        incomplete.has_design = false;
        incomplete.delta_specs = Vec::new();
        let design = evaluate_gate(&incomplete, GateStage::Design, &policy);
        assert_eq!(design.verdict, "FAIL");
        assert_eq!(
            design
                .issues
                .iter()
                .map(|issue| issue.message.as_str())
                .collect::<Vec<_>>(),
            vec!["demo: missing delta specs", "demo: missing design.md"]
        );
    }

    #[test]
    fn gate_snapshot_rejects_unknown_refs_unlinked_tasks_and_malformed_exemptions() {
        let mut change = change();
        change.has_design = true;
        change.delta_specs = vec!["openspec/changes/demo/specs/demo/spec.md".to_string()];
        let snapshot = GateSnapshot {
            change,
            proposal: Some(GateArtifact {
                path: "proposal.md".to_string(),
                markdown: "r[demo.unknown] gate-exempt[reason=test owner=agent]".to_string(),
            }),
            design: Some(GateArtifact {
                path: "design.md".to_string(),
                markdown: "r[demo.behavior]".to_string(),
            }),
            tasks: Some(GateArtifact {
                path: "tasks.md".to_string(),
                markdown: "- [ ] [serial] missing link\n- [ ] [parallel] r[demo.behavior] linked"
                    .to_string(),
            }),
            delta_specs: vec![GateArtifact {
                path: "spec.md".to_string(),
                markdown: "r[demo.behavior] The system MUST behave.".to_string(),
            }],
        };

        let report = evaluate_gate_snapshot(&snapshot, GateStage::Tasks, &policy(true, "strict"));
        assert_eq!(report.verdict, "FAIL");
        let messages = report
            .issues
            .iter()
            .map(|issue| issue.message.as_str())
            .collect::<Vec<_>>();
        assert!(messages
            .iter()
            .any(|message| message.contains("unknown requirement reference r[demo.unknown]")));
        assert!(messages
            .iter()
            .any(|message| message.contains("malformed gate exemption")));
        assert!(messages
            .iter()
            .any(|message| message.contains("task missing known requirement reference")));
    }

    #[test]
    fn gate_snapshot_accepts_known_refs_dependencies_and_bounded_exemptions() {
        let mut change = change();
        change.has_design = true;
        change.delta_specs = vec!["openspec/changes/demo/specs/demo/spec.md".to_string()];
        let snapshot = GateSnapshot {
            change,
            proposal: Some(GateArtifact {
                path: "proposal.md".to_string(),
                markdown: "gate-exempt[reason=test owner=agent until=2026-12-31]".to_string(),
            }),
            design: Some(GateArtifact {
                path: "design.md".to_string(),
                markdown: "r[demo.behavior]".to_string(),
            }),
            tasks: Some(GateArtifact {
                path: "tasks.md".to_string(),
                markdown: "- [ ] [serial] r[demo.behavior] linked\n- [ ] [depends:demo.behavior] dependency".to_string(),
            }),
            delta_specs: vec![GateArtifact {
                path: "spec.md".to_string(),
                markdown: "r[demo.behavior] The system MUST behave.".to_string(),
            }],
        };

        let report = evaluate_gate_snapshot(&snapshot, GateStage::Tasks, &policy(true, "strict"));
        assert_eq!(report.verdict, "PASS");
        assert!(report.issues.is_empty());
    }

    #[test]
    fn accepts_ears_style_requirement_and_scenario_structure() {
        let source = SpecSource {
            id: "demo".to_string(),
            path: "openspec/specs/demo/spec.md".to_string(),
            markdown: "### Requirement: Demo\nr[demo] The system MUST do the thing.\n\n#### Scenario: Happy path\n- GIVEN a precondition\n- WHEN an action happens\n- THEN an outcome occurs\n".to_string(),
        };
        assert!(validate_spec_source(&source).is_empty());
    }

    #[test]
    fn rejects_weak_requirements_and_incomplete_scenarios() {
        let source = SpecSource {
            id: "demo".to_string(),
            path: "openspec/specs/demo/spec.md".to_string(),
            markdown: "### Requirement: Demo\nThis feature works.\n\n#### Scenario: Missing when\n- GIVEN a precondition\n- THEN an outcome occurs\n".to_string(),
        };
        assert_eq!(
            validate_spec_source(&source),
            vec![
                "openspec/specs/demo/spec.md: Requirement: Demo missing requirement strength (MUST/SHALL/SHOULD/MAY)".to_string(),
                "openspec/specs/demo/spec.md: Scenario: Missing when missing WHEN clause(s)".to_string(),
            ]
        );
    }
}
