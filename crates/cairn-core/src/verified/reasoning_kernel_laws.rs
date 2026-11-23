//! Independent reference semantics, reasoning-kernel laws, abstraction mapping,
//! and bounded correspondence qualification for Cairn's reasoning kernel.
//!
//! This module implements issue #13 (under roadmap #4). Its reference evaluator
//! operates purely on abstract domain values without calling or sharing logic
//! with `scoped_claim_derivations`, `support_maintenance`,
//! `refinement_typed_dependencies`, or `workflow_obligations`. Correspondence
//! receipts explicitly classify their result as `bounded_conformance` rather
//! than claiming an unqualified or mechanized implementation proof.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::evidence::canonical_json_hash;

pub const REASONING_KERNEL_LAWS_SCHEMA_V1: &str = "cairn.reasoning-kernel-laws.v1";
pub const REASONING_KERNEL_CONFORMANCE_SCHEMA_V1: &str = "cairn.reasoning-kernel-conformance.v1";
pub const REASONING_KERNEL_MODEL_VERSION_V1: &str = "cairn.reasoning-kernel-reference.v1";
pub const REASONING_KERNEL_CONFORMANCE_BOUNDARY: &str = "bounded conformance proves agreement between the independent reference evaluator and the covered Rust entry points over the declared finite corpus and confirms detection of deliberate law violations; it does not constitute an abstract proof, unbounded model check, mechanized implementation-refinement theorem, cryptographic proof, parser proof, external verifier soundness proof, or release eligibility";

pub const REASONING_KERNEL_NON_CLAIMS: &[&str] = &[
    "bounded conformance is not an abstract mathematical proof",
    "bounded conformance is not an unbounded model check or liveness proof",
    "bounded conformance is not a mechanized implementation-refinement theorem",
    "agreement with a hand-written reference model does not prove parser, cryptographic, or external verifier soundness",
    "reasoning-kernel qualification does not alter existing state-machine or Kani receipt meanings or grant release eligibility",
];

/// Distinguishes the exact assurance class established by a qualification run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationClass {
    AbstractProof,
    BoundedModelCheck,
    BoundedConformance,
    MechanizedImplementationRefinement,
}

/// The nine independently specified reasoning-kernel laws from issue #13.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningKernelLaw {
    ConditionalSoundness,
    ScopeConservationAndRoleNonPromotion,
    ObligationConservationThroughRefinement,
    AssumptionDischargeOrRetention,
    UnsupportedSolePremiseAndAlternativeSupport,
    CounterevidencePreservationAndContainment,
    ReplayabilityAndExplanationFidelity,
    PresentationAdapterInvariance,
    NarrowPositiveMonotonicity,
}

impl ReasoningKernelLaw {
    pub const ALL: &'static [ReasoningKernelLaw] = &[
        Self::ConditionalSoundness,
        Self::ScopeConservationAndRoleNonPromotion,
        Self::ObligationConservationThroughRefinement,
        Self::AssumptionDischargeOrRetention,
        Self::UnsupportedSolePremiseAndAlternativeSupport,
        Self::CounterevidencePreservationAndContainment,
        Self::ReplayabilityAndExplanationFidelity,
        Self::PresentationAdapterInvariance,
        Self::NarrowPositiveMonotonicity,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::ConditionalSoundness => "conditional_soundness",
            Self::ScopeConservationAndRoleNonPromotion => {
                "scope_conservation_and_role_non_promotion"
            }
            Self::ObligationConservationThroughRefinement => {
                "obligation_conservation_through_refinement"
            }
            Self::AssumptionDischargeOrRetention => "assumption_discharge_or_retention",
            Self::UnsupportedSolePremiseAndAlternativeSupport => {
                "unsupported_sole_premise_and_alternative_support"
            }
            Self::CounterevidencePreservationAndContainment => {
                "counterevidence_preservation_and_containment"
            }
            Self::ReplayabilityAndExplanationFidelity => "replayability_and_explanation_fidelity",
            Self::PresentationAdapterInvariance => "presentation_adapter_invariance",
            Self::NarrowPositiveMonotonicity => "narrow_positive_monotonicity",
        }
    }

    pub fn statement(self) -> &'static str {
        match self {
            Self::ConditionalSoundness => {
                "Semantically valid premises and an admitted applicable inference rule yield the conclusion under the exact retained assumptions."
            }
            Self::ScopeConservationAndRoleNonPromotion => {
                "Derived claims preserve exact scope identity and never silently promote weaker evidence roles into stronger authority."
            }
            Self::ObligationConservationThroughRefinement => {
                "Refining a parent requirement into child guarantees conserves every child, integration, implication, and residual-assumption obligation."
            }
            Self::AssumptionDischargeOrRetention => {
                "Every introduced assumption is either explicitly discharged by an admitted compatible guarantee witness or retained visibly as residual."
            }
            Self::UnsupportedSolePremiseAndAlternativeSupport => {
                "No conclusion follows from an unsupported sole premise, while a separate valid alternative support branch keeps the conclusion supported."
            }
            Self::CounterevidencePreservationAndContainment => {
                "Active counterevidence is preserved as Refuted or Disputed rather than dropped, and stays contained to its bound claim and dependents."
            }
            Self::ReplayabilityAndExplanationFidelity => {
                "Replaying an evaluation over the same admitted inputs reproduces the exact decision, residual assumptions, and explanation identity."
            }
            Self::PresentationAdapterInvariance => {
                "Rendering the same admitted workflow obligation definition across adapters preserves identical obligation IDs, predicates, evidence references, and command authority."
            }
            Self::NarrowPositiveMonotonicity => {
                "Adding applicable positive support in a fixed positive fragment can only enlarge support, whereas refutation, policy change, revision, or withdrawn applicability may reduce acceptance."
            }
        }
    }
}

/// Abadi-Lamport-style abstraction mapping entry linking abstract semantics to
/// concrete Rust entry points and fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AbstractionMappingEntry {
    pub abstract_operation: String,
    pub rust_entry_point: String,
    pub concrete_input_fields: Vec<String>,
    pub concrete_output_fields: Vec<String>,
    pub rejected_input_mapping: String,
    pub stuttering_steps: Vec<String>,
}

pub fn canonical_abstraction_mapping() -> Vec<AbstractionMappingEntry> {
    vec![
        AbstractionMappingEntry {
            abstract_operation: "claim_derivation_evaluation".to_string(),
            rust_entry_point:
                "cairn_core::verified::scoped_claim_derivations::evaluate_derivations".to_string(),
            concrete_input_fields: vec![
                "ClaimPolicy.{claims, rules, bounds}".to_string(),
                "EvaluationRequest.{claims, derivations, producers}".to_string(),
            ],
            concrete_output_fields: vec![
                "DerivationReport.claims[].claim_id".to_string(),
                "DerivationReport.claims[].state".to_string(),
                "DerivationReport.claims[].residual_assumptions".to_string(),
                "DerivationReport.claims[].support".to_string(),
                "DerivationReport.claims[].counterevidence".to_string(),
                "DerivationReport.report_blake3".to_string(),
            ],
            rejected_input_mapping:
                "Err(DerivationError::BoundExhaustion) -> BoundsExhausted; Err(DerivationError::{Unsupported, ScopeMismatch, Cycle, InvalidInput}) -> Rejected"
                    .to_string(),
            stuttering_steps: vec![
                "canonical sorting and deduplication of claim lists and residual assumptions"
                    .to_string(),
                "deterministic serialization key ordering".to_string(),
            ],
        },
        AbstractionMappingEntry {
            abstract_operation: "support_maintenance_replay".to_string(),
            rust_entry_point:
                "cairn_core::verified::support_maintenance::evaluate_support_maintenance"
                    .to_string(),
            concrete_input_fields: vec![
                "SupportPolicy.{schema, bounds}".to_string(),
                "SupportCase.{claims, evidence, branches, changes, obligations}".to_string(),
            ],
            concrete_output_fields: vec![
                "MaintenanceReport.claims[].claim_id".to_string(),
                "MaintenanceReport.claims[].baseline_state".to_string(),
                "MaintenanceReport.claims[].state".to_string(),
                "MaintenanceReport.claims[].surviving_branches".to_string(),
                "MaintenanceReport.claims[].withdrawn_branches".to_string(),
                "MaintenanceReport.claims[].support".to_string(),
                "MaintenanceReport.claims[].counterevidence".to_string(),
                "MaintenanceReport.claims[].residual_assumptions".to_string(),
                "MaintenanceReport.report_blake3".to_string(),
            ],
            rejected_input_mapping:
                "Err(DerivationError::BoundExhaustion) -> BoundsExhausted; other Err(DerivationError) -> Rejected"
                    .to_string(),
            stuttering_steps: vec![
                "ordering events by declared sequence number".to_string(),
                "sorting active/withdrawn branch IDs and bound-usage counters".to_string(),
            ],
        },
        AbstractionMappingEntry {
            abstract_operation: "refinement_obligation_conservation".to_string(),
            rust_entry_point:
                "cairn_core::verified::refinement_typed_dependencies::evaluate_refinement"
                    .to_string(),
            concrete_input_fields: vec![
                "RefinementPolicy.{schema, bounds}".to_string(),
                "RefinementRequest.{decompositions, dependencies, integration_evidence}".to_string(),
            ],
            concrete_output_fields: vec![
                "RefinementReport.decompositions[].state".to_string(),
                "RefinementReport.decompositions[].obligations".to_string(),
                "RefinementReport.dependencies[].state".to_string(),
                "RefinementReport.residual_assumptions".to_string(),
                "RefinementReport.report_blake3".to_string(),
            ],
            rejected_input_mapping:
                "Err(RefinementError::BoundExhaustion) -> BoundsExhausted; other Err(RefinementError) -> Rejected"
                    .to_string(),
            stuttering_steps: vec![
                "canonical sorting of obligation explanations and residual assumptions".to_string(),
            ],
        },
        AbstractionMappingEntry {
            abstract_operation: "workflow_obligation_adapter_projection".to_string(),
            rust_entry_point:
                "cairn_core::verified::workflow_obligations::{obligation_predicate, evaluate_workflow_obligations, render_with_workflow_obligations}"
                    .to_string(),
            concrete_input_fields: vec![
                "AdmittedWorkflowObligationCatalog".to_string(),
                "AdmittedGeneratedWorkflowCatalog".to_string(),
                "ObligationRequest".to_string(),
            ],
            concrete_output_fields: vec![
                "ObligationPredicate.{predicate_blake3, obligations[].id}".to_string(),
                "WorkflowObligationReport.{satisfied, remaining, state}".to_string(),
                "RenderedGeneratedWorkflowFile.{workflow_id, content_blake3}".to_string(),
            ],
            rejected_input_mapping:
                "Err(WorkflowObligationIssue) -> Rejected without mutating guidance or weakening predicates"
                    .to_string(),
            stuttering_steps: vec![
                "adapter-specific Markdown front matter, headings, and list formatting".to_string(),
            ],
        },
    ]
}

// ---------------------------------------------------------------------------
// Independent Abstract Reference Semantics
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AbstractClaimState {
    Unresolved,
    Supported,
    Refuted,
    Disputed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AbstractPolarity {
    Support,
    Refutation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AbstractEvidenceRole {
    Observation,
    StructuralRelationship,
    BoundedProperty,
    CheckerAcceptedProperty,
    ReviewedRequirementCorrespondence,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AbstractScope {
    pub requirement_revision: String,
    pub policy_id: String,
    pub subject_id: String,
    pub role: AbstractEvidenceRole,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AbstractEvidence {
    pub id: String,
    pub claim_id: String,
    pub polarity: AbstractPolarity,
    pub scope: AbstractScope,
    pub assumptions: BTreeSet<String>,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AbstractRuleBranch {
    pub id: String,
    pub conclusion: String,
    pub polarity: AbstractPolarity,
    pub premises: Vec<String>,
    pub scope: AbstractScope,
    pub assumptions: BTreeSet<String>,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AbstractBounds {
    pub max_claims: usize,
    pub max_steps: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AbstractClaimProblem {
    pub expected_scope: AbstractScope,
    pub claims: BTreeSet<String>,
    pub evidence: Vec<AbstractEvidence>,
    pub branches: Vec<AbstractRuleBranch>,
    pub bounds: AbstractBounds,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AbstractClaimResult {
    pub state: AbstractClaimState,
    pub residual_assumptions: BTreeSet<String>,
    pub active_support_ids: BTreeSet<String>,
    pub active_refutation_ids: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AbstractOutcome {
    Evaluated {
        claims: BTreeMap<String, AbstractClaimResult>,
        steps_used: usize,
    },
    Rejected {
        reason: String,
    },
    BoundsExhausted {
        limit: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum AbstractUpdateEvent {
    AddEvidence(AbstractEvidence),
    AddBranch(AbstractRuleBranch),
    WithdrawEvidence { evidence_id: String },
    WithdrawBranch { branch_id: String },
    ChangePolicy { new_policy_id: String },
    ReviseRequirement { new_revision: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AbstractRefinementProblem {
    pub parent_id: String,
    pub child_obligations: BTreeMap<String, bool>,
    pub integration_obligations: BTreeMap<String, bool>,
    pub child_assumptions: BTreeSet<String>,
    pub discharged_assumptions: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AbstractRefinementOutcome {
    pub parent_satisfied: bool,
    pub unsatisfied_obligations: BTreeSet<String>,
    pub residual_assumptions: BTreeSet<String>,
}

/// Independent least-fixed-point reference evaluator over finite abstract
/// claim/evidence/rule graphs.
pub fn evaluate_reference_claims(problem: &AbstractClaimProblem) -> AbstractOutcome {
    if problem.claims.len() > problem.bounds.max_claims {
        return AbstractOutcome::BoundsExhausted {
            limit: format!(
                "claims {} exceed max_claims {}",
                problem.claims.len(),
                problem.bounds.max_claims
            ),
        };
    }
    for evidence in &problem.evidence {
        if !problem.claims.contains(&evidence.claim_id) {
            return AbstractOutcome::Rejected {
                reason: format!("unknown evidence claim {}", evidence.claim_id),
            };
        }
        if evidence.scope != problem.expected_scope {
            return AbstractOutcome::Rejected {
                reason: format!("scope mismatch on evidence {}", evidence.id),
            };
        }
    }
    for branch in &problem.branches {
        if !problem.claims.contains(&branch.conclusion) {
            return AbstractOutcome::Rejected {
                reason: format!("unknown branch conclusion {}", branch.conclusion),
            };
        }
        for premise in &branch.premises {
            if !problem.claims.contains(premise) {
                return AbstractOutcome::Rejected {
                    reason: format!("unknown branch premise {premise}"),
                };
            }
        }
        if branch.active && branch.scope != problem.expected_scope {
            return AbstractOutcome::Rejected {
                reason: format!("scope mismatch on branch {}", branch.id),
            };
        }
    }
    if has_branch_cycle(&problem.branches) {
        return AbstractOutcome::Rejected {
            reason: "cyclic premise graph".to_string(),
        };
    }

    let mut support_ids: BTreeMap<String, BTreeSet<String>> = problem
        .claims
        .iter()
        .map(|claim| (claim.clone(), BTreeSet::new()))
        .collect();
    let mut refutation_ids: BTreeMap<String, BTreeSet<String>> = problem
        .claims
        .iter()
        .map(|claim| (claim.clone(), BTreeSet::new()))
        .collect();
    let mut assumptions: BTreeMap<String, BTreeSet<String>> = problem
        .claims
        .iter()
        .map(|claim| (claim.clone(), BTreeSet::new()))
        .collect();

    for evidence in &problem.evidence {
        if !evidence.active {
            continue;
        }
        match evidence.polarity {
            AbstractPolarity::Support => {
                support_ids
                    .entry(evidence.claim_id.clone())
                    .or_default()
                    .insert(evidence.id.clone());
                assumptions
                    .entry(evidence.claim_id.clone())
                    .or_default()
                    .extend(evidence.assumptions.iter().cloned());
            }
            AbstractPolarity::Refutation => {
                refutation_ids
                    .entry(evidence.claim_id.clone())
                    .or_default()
                    .insert(evidence.id.clone());
            }
        }
    }

    let mut steps_used = 0usize;
    loop {
        let mut changed = false;
        for branch in &problem.branches {
            if !branch.active {
                continue;
            }
            let premises_supported = !branch.premises.is_empty()
                && branch.premises.iter().all(|premise| {
                    let has_sup = support_ids.get(premise).is_some_and(|set| !set.is_empty());
                    let has_ref = refutation_ids
                        .get(premise)
                        .is_some_and(|set| !set.is_empty());
                    has_sup && !has_ref
                });
            if !premises_supported {
                continue;
            }
            steps_used += 1;
            if steps_used > problem.bounds.max_steps {
                return AbstractOutcome::BoundsExhausted {
                    limit: format!("steps exceeded max_steps {}", problem.bounds.max_steps),
                };
            }
            match branch.polarity {
                AbstractPolarity::Support => {
                    if support_ids
                        .entry(branch.conclusion.clone())
                        .or_default()
                        .insert(branch.id.clone())
                    {
                        changed = true;
                    }
                    let mut combined = branch.assumptions.clone();
                    for premise in &branch.premises {
                        if let Some(premise_assumptions) = assumptions.get(premise) {
                            combined.extend(premise_assumptions.iter().cloned());
                        }
                    }
                    let target = assumptions.entry(branch.conclusion.clone()).or_default();
                    let before = target.len();
                    target.extend(combined);
                    if target.len() > before {
                        changed = true;
                    }
                }
                AbstractPolarity::Refutation => {
                    if refutation_ids
                        .entry(branch.conclusion.clone())
                        .or_default()
                        .insert(branch.id.clone())
                    {
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    let claims = problem
        .claims
        .iter()
        .map(|claim| {
            let sup = support_ids.remove(claim).unwrap_or_default();
            let refut = refutation_ids.remove(claim).unwrap_or_default();
            let state = match (!sup.is_empty(), !refut.is_empty()) {
                (false, false) => AbstractClaimState::Unresolved,
                (true, false) => AbstractClaimState::Supported,
                (false, true) => AbstractClaimState::Refuted,
                (true, true) => AbstractClaimState::Disputed,
            };
            let residual_assumptions = if !sup.is_empty() {
                assumptions.remove(claim).unwrap_or_default()
            } else {
                BTreeSet::new()
            };
            (
                claim.clone(),
                AbstractClaimResult {
                    state,
                    residual_assumptions,
                    active_support_ids: sup,
                    active_refutation_ids: refut,
                },
            )
        })
        .collect();

    AbstractOutcome::Evaluated { claims, steps_used }
}

/// Applies an ordered trace of update events to a base problem, returning the
/// sequence of abstract outcomes after each event.
pub fn evaluate_reference_trace(
    base: &AbstractClaimProblem,
    events: &[AbstractUpdateEvent],
) -> Vec<AbstractOutcome> {
    let mut current = base.clone();
    let mut trace = Vec::with_capacity(events.len() + 1);
    trace.push(evaluate_reference_claims(&current));
    for event in events {
        match event {
            AbstractUpdateEvent::AddEvidence(ev) => current.evidence.push(ev.clone()),
            AbstractUpdateEvent::AddBranch(branch) => current.branches.push(branch.clone()),
            AbstractUpdateEvent::WithdrawEvidence { evidence_id } => {
                for ev in &mut current.evidence {
                    if &ev.id == evidence_id {
                        ev.active = false;
                    }
                }
            }
            AbstractUpdateEvent::WithdrawBranch { branch_id } => {
                for branch in &mut current.branches {
                    if &branch.id == branch_id {
                        branch.active = false;
                    }
                }
            }
            AbstractUpdateEvent::ChangePolicy { new_policy_id } => {
                current.expected_scope.policy_id = new_policy_id.clone();
                for ev in &mut current.evidence {
                    if ev.scope.policy_id != *new_policy_id {
                        ev.active = false;
                        ev.scope.policy_id = new_policy_id.clone();
                    }
                }
                for branch in &mut current.branches {
                    if branch.scope.policy_id != *new_policy_id {
                        branch.active = false;
                        branch.scope.policy_id = new_policy_id.clone();
                    }
                }
            }
            AbstractUpdateEvent::ReviseRequirement { new_revision } => {
                current.expected_scope.requirement_revision = new_revision.clone();
                for ev in &mut current.evidence {
                    if ev.scope.requirement_revision != *new_revision {
                        ev.active = false;
                        ev.scope.requirement_revision = new_revision.clone();
                    }
                }
                for branch in &mut current.branches {
                    if branch.scope.requirement_revision != *new_revision {
                        branch.active = false;
                        branch.scope.requirement_revision = new_revision.clone();
                    }
                }
            }
        }
        trace.push(evaluate_reference_claims(&current));
    }
    trace
}

/// Independent reference evaluator for obligation conservation through
/// refinement and explicit assumption discharge/retention.
pub fn evaluate_reference_refinement(
    problem: &AbstractRefinementProblem,
) -> AbstractRefinementOutcome {
    let mut unsatisfied = BTreeSet::new();
    for (id, satisfied) in &problem.child_obligations {
        if !*satisfied {
            unsatisfied.insert(format!("child:{id}"));
        }
    }
    for (id, satisfied) in &problem.integration_obligations {
        if !*satisfied {
            unsatisfied.insert(format!("integration:{id}"));
        }
    }
    let residual_assumptions = problem
        .child_assumptions
        .difference(&problem.discharged_assumptions)
        .cloned()
        .collect::<BTreeSet<_>>();
    let parent_satisfied = !problem.child_obligations.is_empty() && unsatisfied.is_empty();
    AbstractRefinementOutcome {
        parent_satisfied,
        unsatisfied_obligations: unsatisfied,
        residual_assumptions,
    }
}

fn has_branch_cycle(branches: &[AbstractRuleBranch]) -> bool {
    let mut adjacency: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for branch in branches {
        let entry = adjacency.entry(branch.conclusion.as_str()).or_default();
        for premise in &branch.premises {
            entry.insert(premise.as_str());
        }
    }
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    let nodes: Vec<&str> = adjacency.keys().copied().collect();
    for node in nodes {
        if dfs_cycle(node, &adjacency, &mut visiting, &mut visited) {
            return true;
        }
    }
    false
}

fn dfs_cycle<'a>(
    node: &'a str,
    adjacency: &BTreeMap<&'a str, BTreeSet<&'a str>>,
    visiting: &mut BTreeSet<&'a str>,
    visited: &mut BTreeSet<&'a str>,
) -> bool {
    if visited.contains(node) {
        return false;
    }
    if !visiting.insert(node) {
        return true;
    }
    if let Some(neighbors) = adjacency.get(node) {
        for next in neighbors {
            if dfs_cycle(next, adjacency, visiting, visited) {
                return true;
            }
        }
    }
    visiting.remove(node);
    visited.insert(node);
    false
}

// ---------------------------------------------------------------------------
// Bounded Correspondence Qualification & Divergence Preservation
// ---------------------------------------------------------------------------

/// One declared conformance case pairing a reproducing input, the reference
/// evaluator's abstract trace, and the projected Rust implementation trace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrespondenceCase {
    pub case_id: String,
    pub law: ReasoningKernelLaw,
    pub reproducing_input: Value,
    pub abstract_trace: Value,
    pub implementation_trace: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DivergenceRecord {
    pub case_id: String,
    pub law: ReasoningKernelLaw,
    pub reproducing_input: Value,
    pub abstract_trace: Value,
    pub implementation_trace: Value,
    pub diagnostic: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LawQualificationSummary {
    pub law: ReasoningKernelLaw,
    pub cases_checked: usize,
    pub passed_cases: usize,
    pub divergences: usize,
    pub conforming: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KernelConformanceReceipt {
    pub schema: String,
    pub laws_schema: String,
    pub qualification_class: QualificationClass,
    pub model_version: String,
    pub model_identity: String,
    pub rule_identity: String,
    pub mapping_identity: String,
    pub corpus_identity: String,
    pub valid: bool,
    pub verdict: String,
    pub cases_checked: usize,
    pub law_summaries: Vec<LawQualificationSummary>,
    pub divergences: Vec<DivergenceRecord>,
    pub abstraction_mapping: Vec<AbstractionMappingEntry>,
    pub boundary: String,
    pub non_claims: Vec<String>,
    pub receipt_hash: String,
}

/// Qualifies correspondence between abstract reference traces and projected
/// Rust implementation traces across a declared bounded corpus.
///
/// Any mismatch records a [`DivergenceRecord`] preserving the reproducing
/// input, abstract trace, and implementation trace, and causes `valid = false`.
pub fn qualify_correspondence(cases: &[CorrespondenceCase]) -> KernelConformanceReceipt {
    let mapping = canonical_abstraction_mapping();
    let laws_material: Vec<Value> = ReasoningKernelLaw::ALL
        .iter()
        .map(|law| {
            json!({
                "id": law.id(),
                "statement": law.statement(),
            })
        })
        .collect();
    let model_identity = canonical_json_hash(&json!({
        "model_version": REASONING_KERNEL_MODEL_VERSION_V1,
        "laws_schema": REASONING_KERNEL_LAWS_SCHEMA_V1,
        "qualification_class": QualificationClass::BoundedConformance,
    }));
    let rule_identity = canonical_json_hash(&json!({
        "laws": laws_material,
    }));
    let mapping_identity = canonical_json_hash(&json!({
        "abstraction_mapping": mapping,
    }));
    let corpus_identity = canonical_json_hash(&json!({
        "cases": cases,
    }));

    let mut divergences = Vec::new();
    let mut per_law_counts: BTreeMap<ReasoningKernelLaw, (usize, usize)> = BTreeMap::new();
    for case in cases {
        let entry = per_law_counts.entry(case.law).or_insert((0, 0));
        entry.0 += 1;
        if case.abstract_trace == case.implementation_trace {
            entry.1 += 1;
        } else {
            divergences.push(DivergenceRecord {
                case_id: case.case_id.clone(),
                law: case.law,
                reproducing_input: case.reproducing_input.clone(),
                abstract_trace: case.abstract_trace.clone(),
                implementation_trace: case.implementation_trace.clone(),
                diagnostic: format!(
                    "law {} diverged on case {}: abstract reference trace does not match implementation trace",
                    case.law.id(),
                    case.case_id
                ),
            });
        }
    }

    let law_summaries: Vec<LawQualificationSummary> = ReasoningKernelLaw::ALL
        .iter()
        .map(|&law| {
            let (checked, passed) = per_law_counts.get(&law).copied().unwrap_or((0, 0));
            let div = checked.saturating_sub(passed);
            LawQualificationSummary {
                law,
                cases_checked: checked,
                passed_cases: passed,
                divergences: div,
                conforming: checked > 0 && div == 0,
            }
        })
        .collect();

    let all_laws_covered = law_summaries.iter().all(|summary| summary.conforming);
    let valid = !cases.is_empty() && divergences.is_empty() && all_laws_covered;
    let verdict = if valid { "pass" } else { "fail" }.to_string();
    let non_claims: Vec<String> = REASONING_KERNEL_NON_CLAIMS
        .iter()
        .map(|item| (*item).to_string())
        .collect();

    let mut receipt = KernelConformanceReceipt {
        schema: REASONING_KERNEL_CONFORMANCE_SCHEMA_V1.to_string(),
        laws_schema: REASONING_KERNEL_LAWS_SCHEMA_V1.to_string(),
        qualification_class: QualificationClass::BoundedConformance,
        model_version: REASONING_KERNEL_MODEL_VERSION_V1.to_string(),
        model_identity,
        rule_identity,
        mapping_identity,
        corpus_identity,
        valid,
        verdict,
        cases_checked: cases.len(),
        law_summaries,
        divergences,
        abstraction_mapping: mapping,
        boundary: REASONING_KERNEL_CONFORMANCE_BOUNDARY.to_string(),
        non_claims,
        receipt_hash: String::new(),
    };
    receipt.receipt_hash = canonical_json_hash(&json!(&receipt));
    receipt
}

#[cfg(test)]
mod tests;
