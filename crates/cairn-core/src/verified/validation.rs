use crate::verified::{
    policy::Policy,
    repo::{ChangeStatus, SpecSource},
};

pub fn validate_changes(changes: &[ChangeStatus], policy: &Policy) -> Vec<String> {
    let mut issues = Vec::with_capacity(changes.len().saturating_mul(3));
    for change in changes {
        issues.extend(change_issues(change, policy));
    }
    issues
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateStage {
    Proposal,
    Design,
    Tasks,
}

impl GateStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Proposal => "proposal",
            Self::Design => "design",
            Self::Tasks => "tasks",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateIssue {
    pub message: String,
    pub scope: String,
    pub promoted: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateReport {
    pub change: String,
    pub stage: String,
    pub verdict: String,
    pub issues: Vec<GateIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateArtifact {
    pub path: String,
    pub markdown: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateSnapshot {
    pub change: ChangeStatus,
    pub proposal: Option<GateArtifact>,
    pub design: Option<GateArtifact>,
    pub tasks: Option<GateArtifact>,
    pub delta_specs: Vec<GateArtifact>,
}

impl GateSnapshot {
    pub fn from_change(change: ChangeStatus) -> Self {
        Self {
            change,
            proposal: None,
            design: None,
            tasks: None,
            delta_specs: Vec::new(),
        }
    }
}

pub fn evaluate_gate(change: &ChangeStatus, stage: GateStage, policy: &Policy) -> GateReport {
    evaluate_gate_snapshot(&GateSnapshot::from_change(change.clone()), stage, policy)
}

pub fn evaluate_gate_snapshot(
    snapshot: &GateSnapshot,
    stage: GateStage,
    policy: &Policy,
) -> GateReport {
    // r[impl cairn.supersede.gate_policy]
    // r[impl cairn.supersede.gate_policy.references]
    // r[impl cairn.supersede.gate_policy.task_links]
    // r[impl cairn.supersede.gate_policy.exemptions]
    let change = &snapshot.change;
    let mut issues = Vec::with_capacity(4);
    if !policy
        .gate_stages
        .iter()
        .any(|candidate| candidate == stage.as_str())
    {
        issues.push(gate_issue(
            format!("stage not enabled by policy: {}", stage.as_str()),
            policy,
        ));
    }
    issues.extend(required_artifact_issues(change, stage, policy));
    if matches!(stage, GateStage::Tasks) {
        issues.extend(
            change_issues(change, policy)
                .into_iter()
                .map(|message| gate_issue(message, policy)),
        );
    }
    issues.extend(reference_hygiene_issues(snapshot, stage, policy));
    let verdict = if issues.is_empty() { "PASS" } else { "FAIL" }.to_string();
    GateReport {
        change: change.name.clone(),
        stage: stage.as_str().to_string(),
        verdict,
        issues,
    }
}

fn reference_hygiene_issues(
    snapshot: &GateSnapshot,
    stage: GateStage,
    policy: &Policy,
) -> Vec<GateIssue> {
    if snapshot.delta_specs.is_empty() {
        return Vec::new();
    }
    let known = known_delta_references(&snapshot.delta_specs);
    let mut artifacts = Vec::with_capacity(3);
    if let Some(artifact) = &snapshot.proposal {
        artifacts.push(artifact);
    }
    if matches!(stage, GateStage::Design | GateStage::Tasks) {
        if let Some(artifact) = &snapshot.design {
            artifacts.push(artifact);
        }
    }
    if matches!(stage, GateStage::Tasks) {
        if let Some(artifact) = &snapshot.tasks {
            artifacts.push(artifact);
        }
    }
    let mut issues = Vec::with_capacity(artifacts.len().saturating_mul(2).saturating_add(1));
    for artifact in artifacts {
        issues.extend(unknown_reference_issues(artifact, &known, policy));
        issues.extend(malformed_exemption_issues(artifact, policy));
    }
    if matches!(stage, GateStage::Tasks) {
        if let Some(tasks) = &snapshot.tasks {
            issues.extend(unlinked_task_issues(tasks, &known, policy));
        }
    }
    issues
}

fn known_delta_references(delta_specs: &[GateArtifact]) -> Vec<String> {
    let marker_bound = delta_specs
        .iter()
        .map(|artifact| reference_marker_bound(&artifact.markdown))
        .sum();
    let mut refs = Vec::with_capacity(marker_bound);
    for artifact in delta_specs {
        refs.extend(reference_markers(&artifact.markdown));
    }
    refs.sort();
    refs.dedup();
    refs
}

fn unknown_reference_issues(
    artifact: &GateArtifact,
    known: &[String],
    policy: &Policy,
) -> Vec<GateIssue> {
    reference_markers(&artifact.markdown)
        .into_iter()
        .filter(|reference| !is_external_trace_reference(reference) && !known.contains(reference))
        .map(|reference| {
            gate_issue(
                format!(
                    "{}: unknown requirement reference r[{reference}]",
                    artifact.path
                ),
                policy,
            )
        })
        .collect()
}

fn malformed_exemption_issues(artifact: &GateArtifact, policy: &Policy) -> Vec<GateIssue> {
    let mut issues = Vec::with_capacity(artifact.markdown.lines().count());
    for (line_number, line) in artifact.markdown.lines().enumerate() {
        for marker in exemption_markers(line) {
            if exemption_marker_is_valid(marker, policy) {
                continue;
            }
            issues.push(gate_issue(
                format!(
                    "{}:{}: malformed gate exemption; expected gate-exempt[{}]",
                    artifact.path,
                    line_number + 1,
                    exemption_marker_hint(policy)
                ),
                policy,
            ));
        }
    }
    issues
}

fn exemption_markers(line: &str) -> Vec<&str> {
    let mut markers = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find("gate-exempt[") {
        let after = &rest[start + "gate-exempt[".len()..];
        let Some(end) = after.find(']') else {
            markers.push(after);
            break;
        };
        markers.push(&after[..end]);
        rest = &after[end + 1..];
    }
    markers
}

fn exemption_marker_is_valid(marker: &str, policy: &Policy) -> bool {
    let fields = marker_fields(marker);
    if !policy
        .exemption_marker_policy
        .required_fields
        .iter()
        .all(|field| {
            fields
                .iter()
                .any(|(name, value)| name == field && !value.is_empty())
        })
    {
        return false;
    }
    policy
        .exemption_marker_policy
        .date_fields
        .iter()
        .all(|field| {
            fields
                .iter()
                .any(|(name, value)| name == field && is_iso_date(value))
        })
}

fn marker_fields(marker: &str) -> Vec<(&str, &str)> {
    marker
        .split_whitespace()
        .filter_map(|part| part.split_once('='))
        .collect()
}

fn is_iso_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[..4].iter().all(u8::is_ascii_digit)
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[8..].iter().all(u8::is_ascii_digit)
}

fn exemption_marker_hint(policy: &Policy) -> String {
    policy
        .exemption_marker_policy
        .required_fields
        .iter()
        .map(|field| format!("{field}=..."))
        .collect::<Vec<_>>()
        .join(" ")
}

fn unlinked_task_issues(
    artifact: &GateArtifact,
    known: &[String],
    policy: &Policy,
) -> Vec<GateIssue> {
    let mut issues = Vec::with_capacity(artifact.markdown.lines().count());
    for (line_number, line) in artifact.markdown.lines().enumerate() {
        let trimmed = line.trim_start();
        if !is_checkbox_task(trimmed) || has_dependency_marker(trimmed, policy) {
            continue;
        }
        let refs = reference_markers(trimmed);
        if refs.iter().any(|reference| known.contains(reference)) {
            continue;
        }
        issues.push(gate_issue(
            format!(
                "{}:{}: task missing known requirement reference",
                artifact.path,
                line_number + 1
            ),
            policy,
        ));
    }
    issues
}

fn is_checkbox_task(trimmed: &str) -> bool {
    trimmed.starts_with("- [ ]")
        || trimmed.starts_with("- [x]")
        || trimmed.starts_with("- [X]")
        || trimmed.starts_with("- [~]")
}

fn has_dependency_marker(trimmed: &str, policy: &Policy) -> bool {
    policy.task_marker_tokens.iter().any(|marker| {
        if marker.id != "depends" {
            return false;
        }
        match marker.kind.as_str() {
            "prefix" | "depends-prefix" => {
                trimmed.contains(dependency_marker_prefix(&marker.token))
            }
            "literal" => trimmed.contains(&marker.token),
            _ => false,
        }
    })
}

fn dependency_marker_prefix(token: &str) -> &str {
    token.split('<').next().unwrap_or(token)
}

fn is_external_trace_reference(reference: &str) -> bool {
    reference.starts_with("impl ")
        || reference.starts_with("verify ")
        || reference.starts_with("depends ")
}

fn reference_markers(markdown: &str) -> Vec<String> {
    let mut refs = Vec::with_capacity(reference_marker_bound(markdown));
    let mut rest = markdown;
    while let Some(start) = rest.find("r[") {
        let after = &rest[start + 2..];
        let Some(end) = after.find(']') else {
            break;
        };
        let reference = after[..end].trim();
        if !reference.is_empty() && reference != "..." {
            refs.push(reference.to_string());
        }
        rest = &after[end + 1..];
    }
    refs
}

fn reference_marker_bound(markdown: &str) -> usize {
    markdown.matches("r[").count()
}

fn required_artifact_issues(
    change: &ChangeStatus,
    stage: GateStage,
    policy: &Policy,
) -> Vec<GateIssue> {
    let mut issues = Vec::with_capacity(3);
    if !change.has_proposal {
        issues.push(gate_issue(
            format!("{}: missing proposal.md", change.name),
            policy,
        ));
    }
    if matches!(stage, GateStage::Design | GateStage::Tasks) && change.delta_specs.is_empty() {
        issues.push(gate_issue(
            format!("{}: missing delta specs", change.name),
            policy,
        ));
    }
    if matches!(stage, GateStage::Design | GateStage::Tasks) && !change.has_design {
        issues.push(gate_issue(
            format!("{}: missing design.md", change.name),
            policy,
        ));
    }
    if matches!(stage, GateStage::Tasks) && !change.has_tasks {
        issues.push(gate_issue(
            format!("{}: missing tasks.md", change.name),
            policy,
        ));
    }
    issues
}

fn gate_issue(message: String, policy: &Policy) -> GateIssue {
    GateIssue {
        message,
        scope: policy.gate_finding_scope.clone(),
        promoted: policy.gate_finding_promoted.clone(),
    }
}

pub fn change_issues(change: &ChangeStatus, policy: &Policy) -> Vec<String> {
    let mut issues = Vec::with_capacity(3);
    if !change.has_proposal {
        issues.push(format!("{}: missing proposal.md", change.name));
    }
    if !change.has_tasks {
        issues.push(format!("{}: missing tasks.md", change.name));
    }
    if let Some(message) = unmarked_tasks_issue(change, policy) {
        issues.push(message);
    }
    issues
}

fn unmarked_tasks_issue(change: &ChangeStatus, policy: &Policy) -> Option<String> {
    let counts = change.task_counts?;
    if !policy.task_markers_enabled {
        return None;
    }
    if policy.task_marker_strictness != "strict" {
        return None;
    }
    if counts.unmarked == 0 {
        return None;
    }
    Some(format!(
        "{}: {} task(s) missing concurrency markers",
        change.name, counts.unmarked
    ))
}

pub fn validate_spec_sources(sources: &[SpecSource]) -> Vec<String> {
    let mut ordered = sources.to_vec();
    ordered.sort_by(|left, right| left.path.cmp(&right.path).then(left.id.cmp(&right.id)));
    let mut issues = Vec::with_capacity(ordered.len().saturating_mul(2));
    for source in &ordered {
        issues.extend(validate_spec_source(source));
    }
    issues
}

pub fn validate_spec_source(source: &SpecSource) -> Vec<String> {
    // r[impl cairn.supersede.validate_ears] r[impl cairn.supersede.validate_ears.accept]
    // r[impl cairn.supersede.validate_ears.weak_requirement]
    // r[impl cairn.supersede.validate_ears.incomplete_scenario]
    // r[impl cairn.supersede.validate_ears.boundary]
    let lines = source.markdown.lines().collect::<Vec<_>>();
    let mut issues = Vec::with_capacity(lines.len().saturating_div(4));
    for (index, line) in lines.iter().enumerate() {
        let Some((level, heading)) = markdown_heading(line) else {
            continue;
        };
        if level == 3 && heading.starts_with("Requirement:") {
            issues.extend(validate_requirement_block(source, &lines, index, heading));
        }
        if level == 4 && heading.starts_with("Scenario:") {
            issues.extend(validate_scenario_block(source, &lines, index, heading));
        }
    }
    issues
}

fn validate_requirement_block(
    source: &SpecSource,
    lines: &[&str],
    index: usize,
    heading: &str,
) -> Option<String> {
    let end = next_heading_at_or_above(lines, index + 1, 3);
    if has_requirement_strength(&lines[index + 1..end]) {
        return None;
    }
    Some(format!(
        "{}: {} missing requirement strength (MUST/SHALL/SHOULD/MAY)",
        source.path, heading
    ))
}

fn validate_scenario_block(
    source: &SpecSource,
    lines: &[&str],
    index: usize,
    heading: &str,
) -> Option<String> {
    let end = next_heading_at_or_above(lines, index + 1, 4);
    let block = &lines[index + 1..end];
    let mut missing = Vec::with_capacity(3);
    if !has_scenario_clause(block, "GIVEN") {
        missing.push("GIVEN");
    }
    if !has_scenario_clause(block, "WHEN") {
        missing.push("WHEN");
    }
    if !has_scenario_clause(block, "THEN") {
        missing.push("THEN");
    }
    if missing.is_empty() {
        return None;
    }
    Some(format!(
        "{}: {} missing {} clause(s)",
        source.path,
        heading,
        missing.join("/")
    ))
}

fn markdown_heading(line: &str) -> Option<(usize, &str)> {
    let trimmed = line.trim_start();
    let marker_len = trimmed.chars().take_while(|value| *value == '#').count();
    if marker_len == 0 || !trimmed[marker_len..].starts_with(' ') {
        return None;
    }
    Some((marker_len, trimmed[marker_len..].trim()))
}

fn next_heading_at_or_above(lines: &[&str], start: usize, max_level: usize) -> usize {
    lines[start..]
        .iter()
        .position(|line| markdown_heading(line).is_some_and(|(level, _heading)| level <= max_level))
        .map_or(lines.len(), |offset| start + offset)
}

fn has_requirement_strength(lines: &[&str]) -> bool {
    lines.iter().any(|line| {
        let upper = line.to_ascii_uppercase();
        contains_word(&upper, "MUST")
            || contains_word(&upper, "SHALL")
            || contains_word(&upper, "SHOULD")
            || contains_word(&upper, "MAY")
    })
}

fn has_scenario_clause(lines: &[&str], clause: &str) -> bool {
    lines.iter().any(|line| {
        let normalized = normalize_scenario_line(line);
        normalized.starts_with(clause)
    })
}

fn normalize_scenario_line(line: &str) -> String {
    line.trim_start()
        .trim_start_matches('-')
        .trim_start_matches('*')
        .trim_start()
        .trim_start_matches("**")
        .to_ascii_uppercase()
}

fn contains_word(text: &str, needle: &str) -> bool {
    text.split(|value: char| !value.is_ascii_alphanumeric())
        .any(|word| word == needle)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::verified::policy::{ExemptionMarkerPolicy, ReceiptSchema, TaskMarker};
    use crate::verified::repo::{SpecSource, TaskCounts};

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
