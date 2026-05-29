use crate::verified::{policy::Policy, repo::ChangeStatus};

use super::changes::change_issues;
use super::references::reference_hygiene_issues;

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
pub struct EvidenceStrengthSummary {
    pub docs_only: usize,
    pub fixture: usize,
    pub probe: usize,
    pub property: usize,
    pub model: usize,
    pub formal: usize,
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

pub fn classify_gate_evidence(snapshot: &GateSnapshot) -> EvidenceStrengthSummary {
    // r[impl cairn.structured_lifecycle.probes]
    // r[impl cairn.structured_lifecycle.migration]
    let mut summary = EvidenceStrengthSummary {
        docs_only: 0,
        fixture: 0,
        probe: 0,
        property: 0,
        model: 0,
        formal: 0,
    };
    for marker in gate_markdown_blobs(snapshot)
        .flat_map(|markdown| markdown.lines())
        .filter_map(verify_marker_text)
    {
        classify_verify_marker(&mut summary, marker);
    }
    summary
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

pub(super) fn gate_issue(message: String, policy: &Policy) -> GateIssue {
    GateIssue {
        message,
        scope: policy.gate_finding_scope.clone(),
        promoted: policy.gate_finding_promoted.clone(),
    }
}

fn gate_markdown_blobs(snapshot: &GateSnapshot) -> impl Iterator<Item = &str> {
    snapshot
        .proposal
        .iter()
        .chain(snapshot.design.iter())
        .chain(snapshot.tasks.iter())
        .chain(snapshot.delta_specs.iter())
        .map(|artifact| artifact.markdown.as_str())
}

fn verify_marker_text(line: &str) -> Option<&str> {
    let marker_start = line.find("r[verify ")? + "r[verify ".len();
    let marker_end = line[marker_start..].find(']')? + marker_start;
    Some(&line[marker_start..marker_end])
}

fn classify_verify_marker(summary: &mut EvidenceStrengthSummary, marker: &str) {
    if marker.contains("formal") || marker.contains("proof") || marker.contains("verus") {
        summary.formal += 1;
    } else if marker.contains("model") {
        summary.model += 1;
    } else if marker.contains("property") {
        summary.property += 1;
    } else if marker.contains("probe") {
        summary.probe += 1;
    } else if marker.contains("fixture") {
        summary.fixture += 1;
    } else {
        summary.docs_only += 1;
    }
}
