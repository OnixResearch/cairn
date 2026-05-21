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
