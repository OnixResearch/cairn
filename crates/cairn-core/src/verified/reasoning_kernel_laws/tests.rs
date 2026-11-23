use super::*;

use std::collections::{BTreeMap, BTreeSet};

use serde_json::json;

use super::super::ast_grep_gate::ast_grep_receipt_hash;
use super::super::content_bound_requirement_registry::{
    ContentBoundRequirementRow, ContentBoundSourceSpan,
};
use super::super::generated_agent_workflow::{
    admit_generated_agent_workflow_policy, GeneratedAgentWorkflowPolicy,
};
use super::super::refinement_typed_dependencies::{
    evaluate_refinement, ArgumentScope, AssumptionAtom, Assurance, ChildGuarantee, Composition,
    Decomposition, DependencyRelation, EnvironmentAssumption, Feasibility, FeasibilityDisposition,
    GuaranteeCase, ImplicationRelation, IntegrationEvidence, IntegrationObligation,
    ObligationState, RefinementBounds, RefinementPolicy, RefinementRequest, RequirementRevision,
    RootAssumption, REFINEMENT_POLICY_SCHEMA_V1, REFINEMENT_SCHEMA_V1,
};
use super::super::scoped_claim_derivations::{
    claim_identity, evaluate_derivations, ClaimContext, ClaimPolicy, ClaimState, DerivationError,
    EvaluationRequest, ProducerInput, Proposition, ScopedClaim, CLAIM_POLICY_SCHEMA_V1,
    CLAIM_SCHEMA_V1,
};
use super::super::support_maintenance::{
    evaluate_support_maintenance, BranchPolarity, Change, ChangeEvent, EvidencePolarity,
    EvidenceRecord, SupportBounds, SupportBranch, SupportCase, SupportPolicy,
    SUPPORT_CASE_SCHEMA_V1, SUPPORT_POLICY_SCHEMA_V1,
};
use super::super::typed_requirement_contracts::{
    normalize_contract, FiniteDomain, FiniteExpr, SemanticContract, SemanticProfile,
};
use super::super::workflow_obligations::{
    admit_workflow_obligation_policy, obligation_predicate, render_with_workflow_obligations,
    WorkflowObligationPolicy,
};

const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const HASH_C: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
const HASH_D: &str = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
const SHIPPED_OBLIGATIONS: &str =
    include_str!("../../../../../cairn-policy/generated/workflow-obligations.json");
const SHIPPED_DEFAULT_POLICY: &str =
    include_str!("../../../../../cairn-policy/generated/cairn-policy.json");

fn map_claim_state(state: ClaimState) -> AbstractClaimState {
    match state {
        ClaimState::Unresolved => AbstractClaimState::Unresolved,
        ClaimState::Supported => AbstractClaimState::Supported,
        ClaimState::Refuted => AbstractClaimState::Refuted,
        ClaimState::Disputed => AbstractClaimState::Disputed,
    }
}

fn req_row(tag: &str) -> ContentBoundRequirementRow {
    ContentBoundRequirementRow {
        requirement_id: format!("cairn.test.laws.{tag}"),
        specification_path: format!(".cairn/specs/laws/{tag}.md"),
        specification_blake3: HASH_A.into(),
        source_span: ContentBoundSourceSpan {
            start_byte: 0,
            end_byte: 64,
            start_line: 1,
            end_line: 1,
        },
        requirement_body_blake3: HASH_B.into(),
        row_blake3: blake3::hash(format!("cairn.test.laws.row.{tag}").as_bytes())
            .to_hex()
            .to_string(),
    }
}

fn claim_context(policy_blake3: &str) -> ClaimContext {
    ClaimContext {
        subject_blake3: HASH_A.into(),
        policy_blake3: policy_blake3.into(),
        target: "onixresearch/cairn".into(),
        environment: "linux-x86_64".into(),
        bounds: BTreeMap::from([("max_checks".to_string(), "4".to_string())]),
    }
}

fn scoped_claim(tag: &str, proposition: Proposition, policy_blake3: &str) -> ScopedClaim {
    ScopedClaim {
        schema: CLAIM_SCHEMA_V1.into(),
        kind: proposition.claim_kind(),
        requirement: req_row(tag),
        interpretation_blake3: HASH_B.into(),
        contract_blake3: HASH_C.into(),
        context: claim_context(policy_blake3),
        proposition,
    }
}

fn support_policy(max_claims: usize) -> SupportPolicy {
    SupportPolicy {
        schema: SUPPORT_POLICY_SCHEMA_V1.into(),
        change_kinds: vec![
            "evidence_stale".into(),
            "evidence_withdrawn".into(),
            "assumption_withdrawn".into(),
            "evidence_admitted".into(),
            "claim_revised".into(),
            "policy_changed".into(),
            "context_changed".into(),
        ],
        bounds: SupportBounds {
            max_claims,
            max_branches: 16,
            max_evidence: 16,
            max_premises: 4,
            max_assumptions: 16,
            max_events: 16,
            max_obligations: 8,
            max_depth: 8,
        },
    }
}

fn abs_scope(policy_id: &str, rev: &str, role: AbstractEvidenceRole) -> AbstractScope {
    AbstractScope {
        requirement_revision: rev.into(),
        policy_id: policy_id.into(),
        subject_id: HASH_A.into(),
        role,
    }
}

fn project_maintenance_claim(
    report: &super::super::support_maintenance::MaintenanceReport,
    claim_id: &str,
) -> Value {
    let entry = report
        .claims
        .iter()
        .find(|claim| claim.claim_id == claim_id)
        .expect("projected claim exists in MaintenanceReport");
    json!({
        "state": map_claim_state(entry.state),
        "residual_assumptions": entry.residual_assumptions,
    })
}

fn project_abstract_claim(outcome: &AbstractOutcome, claim_id: &str) -> Value {
    match outcome {
        AbstractOutcome::Evaluated { claims, .. } => {
            let entry = claims
                .get(claim_id)
                .expect("projected claim exists in AbstractOutcome");
            json!({
                "state": entry.state,
                "residual_assumptions": entry.residual_assumptions,
            })
        }
        other => json!(other),
    }
}

fn semantic_profile() -> SemanticProfile {
    SemanticProfile {
        id: "cairn.requirement-semantics.finite.v1".into(),
        interpretation:
            "finite-enumeration.equality.nominal-domains.total-observables.single-step.v1".into(),
        max_nodes: 32,
        max_depth: 4,
        max_domain_values: 4,
        max_steps: 256,
    }
}

fn semantic_contract(row: &ContentBoundRequirementRow) -> SemanticContract {
    SemanticContract {
        profile: semantic_profile().id,
        proposed_by: "reasoning-laws-test".into(),
        requirement: row.clone(),
        domains: BTreeMap::from([(
            "bool".to_string(),
            FiniteDomain {
                values: vec!["false".into(), "true".into()],
            },
        )]),
        observables: BTreeMap::new(),
        assumptions: Vec::new(),
        activation: FiniteExpr::Bool { value: true },
        guarantee: FiniteExpr::Bool { value: true },
        supported_portion: "laws qualification contract".into(),
        informal_remainder: "not behavioral satisfaction".into(),
    }
}

fn claim_policy(max_claims: usize) -> ClaimPolicy {
    serde_json::from_value(json!({
        "schema": CLAIM_POLICY_SCHEMA_V1,
        "claims": {
            "schema": CLAIM_SCHEMA_V1,
            "kinds": [
                "observation",
                "structural_relationship",
                "bounded_property",
                "checker_accepted_property",
                "reviewed_requirement_correspondence"
            ],
            "propositions": [
                "check_passed",
                "receipt_structurally_valid",
                "native_completion_eligible",
                "reviewed_correspondence",
                "reviewed_completion_decision"
            ]
        },
        "rules": [
            {
                "id": "reviewed_completion_v1",
                "version": 1,
                "assurance_scope": "bounded conformance fixture"
            }
        ],
        "bounds": {
            "max_claims": max_claims,
            "max_derivations": 16,
            "max_producers": 8,
            "max_premises": 4,
            "max_depth": 8
        }
    }))
    .expect("claim policy parses")
}

fn build_refinement_case(
    include_integration_evidence: bool,
) -> (RefinementPolicy, RefinementRequest) {
    let make_rev = |tag: &str| {
        let requirement = req_row(tag);
        let contract = semantic_contract(&requirement);
        let normalized = normalize_contract(&semantic_profile(), &requirement, &contract)
            .expect("normalize contract");
        RequirementRevision {
            requirement,
            interpretation_blake3: normalized.interpretation_blake3,
            contract_blake3: normalized.contract_blake3,
        }
    };
    let parent = make_rev("parent");
    let child_rev = make_rev("child");
    let receipt = json!({
        "tool": {"name": "ast-grep", "version": "0.39.0"},
        "rule_bundle_hash": HASH_C,
        "repository_scope": ".",
        "command": "ast-grep scan --json",
        "output_format": "json",
        "findings_summary": {"total": 0, "errors": 0, "warnings": 0, "infos": 0, "codemods": 0},
        "non_claims": [
            "does not prove source correctness",
            "does not prove requirement satisfaction",
            "does not prove proof discharge",
            "does not prove release eligibility"
        ],
        "findings": []
    });
    let receipt_hash = ast_grep_receipt_hash(&receipt);
    let context = claim_context(HASH_B);
    let prop = Proposition::ReceiptStructurallyValid {
        receipt_blake3: receipt_hash,
    };
    let child_claim = ScopedClaim {
        schema: CLAIM_SCHEMA_V1.into(),
        kind: prop.claim_kind(),
        requirement: child_rev.requirement.clone(),
        interpretation_blake3: child_rev.interpretation_blake3.clone(),
        contract_blake3: child_rev.contract_blake3.clone(),
        context: context.clone(),
        proposition: prop,
    };
    let child_claim_id = claim_identity(&child_claim).expect("child claim identity");
    let guarantee_case = GuaranteeCase {
        id: "guarantee.child".into(),
        policy: claim_policy(16),
        case: EvaluationRequest {
            profile: semantic_profile(),
            native_requirement: child_rev.requirement.clone(),
            semantic_contract: semantic_contract(&child_rev.requirement),
            correspondence_review: None,
            authorized_reviewers: Vec::new(),
            claims: vec![child_claim],
            producers: vec![ProducerInput::StructuralAstGrep {
                context,
                repository_scope: ".".into(),
                receipt,
                trust_dependencies: BTreeSet::from(["structural-rule-bundle:retained".into()]),
            }],
            derivations: Vec::new(),
        },
    };
    let arg_scope = ArgumentScope {
        subject_blake3: HASH_A.into(),
        policy_blake3: HASH_B.into(),
        target: "onixresearch/cairn".into(),
        environment: "linux-x86_64".into(),
        bounds: BTreeMap::from([("max_checks".to_string(), "4".to_string())]),
    };
    let decomp = Decomposition {
        schema: REFINEMENT_SCHEMA_V1.into(),
        id: "decomp.parent".into(),
        parent: parent.clone(),
        scope: arg_scope.clone(),
        children: vec![ChildGuarantee {
            id: "child.one".into(),
            requirement: child_rev,
            guarantee_case: "guarantee.child".into(),
            claim_id: child_claim_id,
            scope: arg_scope.clone(),
        }],
        composition: Composition::AllOf {
            obligations: vec![Composition::Child {
                child: "child.one".into(),
            }],
        },
        relation: ImplicationRelation {
            statement: "child.one implies parent".into(),
            assurance: Assurance::Reviewed,
            witness_blake3: HASH_D.into(),
            proof_checker: None,
        },
        environment: vec![EnvironmentAssumption {
            id: "assumption.env".into(),
            statement: "host retains deterministic execution environment".into(),
            atom: AssumptionAtom {
                predicate: "env-retained".into(),
                asserted: true,
            },
            source: "cairn.test.laws.parent".into(),
        }],
        integration: vec![IntegrationObligation {
            id: "integration.parent".into(),
            statement: "child integrates into parent boundary".into(),
            required: true,
            scope: arg_scope.clone(),
        }],
        feasibility: FeasibilityDisposition {
            disposition: Feasibility::Feasible,
            method: "finite-enumeration-exhaustive.v1".into(),
            assurance: Assurance::Reviewed,
            justification_blake3: HASH_C.into(),
            covered: BTreeSet::from(["child.one".into(), "assumption.env".into()]),
            proof_checker: None,
        },
    };
    let policy = RefinementPolicy {
        schema: REFINEMENT_POLICY_SCHEMA_V1.into(),
        relations: vec![DependencyRelation::Semantic],
        assurances: vec![Assurance::Reviewed, Assurance::BoundedCheck],
        feasibility_methods: vec!["finite-enumeration-exhaustive.v1".into()],
        root_assumptions: vec![RootAssumption {
            id: "assumption.env".into(),
            statement: "host retains deterministic execution environment".into(),
        }],
        proof_checkers: Vec::new(),
        bounds: RefinementBounds {
            max_decompositions: 8,
            max_requirements: 16,
            max_assumptions: 16,
            max_obligations: 16,
            max_alternatives: 8,
            max_dependencies: 8,
            max_evidence: 16,
            max_guarantee_cases: 8,
            max_depth: 4,
        },
    };
    let integration_evidence = if include_integration_evidence {
        vec![IntegrationEvidence {
            obligation_id: "integration.parent".into(),
            statement: "child integrates into parent boundary".into(),
            scope: arg_scope,
            observation_blake3: HASH_D.into(),
            source: "native-integration-check.v1".into(),
        }]
    } else {
        Vec::new()
    };
    let req = RefinementRequest {
        decompositions: vec![decomp],
        guarantee_cases: vec![guarantee_case],
        dependencies: Vec::new(),
        dependency_evidence: Vec::new(),
        integration_evidence,
        applicability: Vec::new(),
        previous_basis: None,
    };
    (policy, req)
}

/// Builds the complete bounded conformance corpus covering all nine laws
/// against Cairn's actual Rust evaluators.
fn build_full_correspondence_corpus() -> Vec<CorrespondenceCase> {
    let mut cases = Vec::new();
    let scope = abs_scope(HASH_B, HASH_B, AbstractEvidenceRole::Observation);
    let pol = support_policy(16);

    // -----------------------------------------------------------------------
    // Law 1: Conditional soundness + Law 4: Assumption retention
    // -----------------------------------------------------------------------
    let p1 = scoped_claim(
        "main",
        Proposition::CheckPassed {
            check_id: "tests".into(),
        },
        HASH_B,
    );
    let concl = scoped_claim("main", Proposition::ReviewedCompletionDecision {}, HASH_B);
    let p1_id = claim_identity(&p1).expect("p1 id");
    let concl_id = claim_identity(&concl).expect("concl id");
    let sound_case = SupportCase {
        schema: SUPPORT_CASE_SCHEMA_V1.into(),
        claims: vec![p1.clone(), concl.clone()],
        evidence: vec![EvidenceRecord {
            id: "ev.p1".into(),
            claim: p1_id.clone(),
            polarity: EvidencePolarity::Support,
            rule: "native-check.v1".into(),
            version: 1,
            assurance_scope: "saved check".into(),
            assumptions: BTreeSet::from(["assumption.toolchain".into()]),
        }],
        branches: vec![SupportBranch {
            id: "br.main".into(),
            conclusion: concl_id.clone(),
            polarity: BranchPolarity::Support,
            rule: "reviewed_completion_v1".into(),
            version: 1,
            premises: vec![p1_id.clone()],
            assumptions: BTreeSet::from(["assumption.reviewer".into()]),
            assurance_scope: "bounded derivation".into(),
            applicability: claim_context(HASH_B),
            cross_revision_relation: None,
        }],
        changes: Vec::new(),
        obligations: Vec::new(),
    };
    let sound_report = evaluate_support_maintenance(&pol, &sound_case).expect("sound eval");
    let sound_problem = AbstractClaimProblem {
        expected_scope: scope.clone(),
        claims: BTreeSet::from([p1_id.clone(), concl_id.clone()]),
        evidence: vec![AbstractEvidence {
            id: "ev.p1".into(),
            claim_id: p1_id.clone(),
            polarity: AbstractPolarity::Support,
            scope: scope.clone(),
            assumptions: BTreeSet::from(["assumption.toolchain".into()]),
            active: true,
        }],
        branches: vec![AbstractRuleBranch {
            id: "br.main".into(),
            conclusion: concl_id.clone(),
            polarity: AbstractPolarity::Support,
            premises: vec![p1_id.clone()],
            scope: scope.clone(),
            assumptions: BTreeSet::from(["assumption.reviewer".into()]),
            active: true,
        }],
        bounds: AbstractBounds {
            max_claims: 16,
            max_steps: 32,
        },
    };
    let sound_abs = evaluate_reference_claims(&sound_problem);
    cases.push(CorrespondenceCase {
        case_id: "law1.conditional_soundness_retains_assumptions".into(),
        law: ReasoningKernelLaw::ConditionalSoundness,
        reproducing_input: json!({ "premises": ["ev.p1"], "branch": "br.main" }),
        abstract_trace: project_abstract_claim(&sound_abs, &concl_id),
        implementation_trace: project_maintenance_claim(&sound_report, &concl_id),
    });

    // Law 4: Explicit discharge or retention (assumption withdrawn invalidates support)
    let mut withdrawn_case = sound_case.clone();
    withdrawn_case.changes.push(ChangeEvent {
        id: "chg.withdraw_assumption".into(),
        sequence: 1,
        reason: "reviewer assumption withdrawn".into(),
        change: Change::AssumptionWithdrawn {
            assumption: "assumption.reviewer".into(),
        },
    });
    let withdrawn_report =
        evaluate_support_maintenance(&pol, &withdrawn_case).expect("withdrawn assumption");
    let withdrawn_trace = evaluate_reference_trace(
        &sound_problem,
        &[AbstractUpdateEvent::WithdrawBranch {
            branch_id: "br.main".into(),
        }],
    );
    cases.push(CorrespondenceCase {
        case_id: "law4.assumption_retained_then_withdrawn".into(),
        law: ReasoningKernelLaw::AssumptionDischargeOrRetention,
        reproducing_input: json!({ "event": "assumption_withdrawn", "assumption": "assumption.reviewer" }),
        abstract_trace: json!({
            "before": project_abstract_claim(&withdrawn_trace[0], &concl_id),
            "after": project_abstract_claim(&withdrawn_trace[1], &concl_id),
        }),
        implementation_trace: json!({
            "before": project_maintenance_claim(&sound_report, &concl_id),
            "after": project_maintenance_claim(&withdrawn_report, &concl_id),
        }),
    });

    // -----------------------------------------------------------------------
    // Law 2: Scope conservation and evidence-role non-promotion
    // -----------------------------------------------------------------------
    let scope_receipt = json!({
        "tool": {"name": "ast-grep", "version": "0.39.0"},
        "rule_bundle_hash": HASH_C,
        "repository_scope": ".",
        "command": "ast-grep scan --json",
        "output_format": "json",
        "findings_summary": {"total": 0, "errors": 0, "warnings": 0, "infos": 0, "codemods": 0},
        "non_claims": [
            "does not prove source correctness",
            "does not prove requirement satisfaction",
            "does not prove proof discharge",
            "does not prove release eligibility"
        ],
        "findings": []
    });
    let mismatch_req = EvaluationRequest {
        profile: semantic_profile(),
        native_requirement: req_row("scope"),
        semantic_contract: semantic_contract(&req_row("scope")),
        correspondence_review: None,
        authorized_reviewers: Vec::new(),
        claims: vec![scoped_claim(
            "scope",
            Proposition::CheckPassed {
                check_id: "tests".into(),
            },
            HASH_B,
        )],
        producers: vec![ProducerInput::StructuralAstGrep {
            context: claim_context(HASH_D), // mismatched policy_blake3!
            repository_scope: ".".into(),
            receipt: scope_receipt,
            trust_dependencies: BTreeSet::new(),
        }],
        derivations: Vec::new(),
    };
    let mismatch_impl = match evaluate_derivations(&claim_policy(16), &mismatch_req) {
        Err(DerivationError::ScopeMismatch(_)) => json!({"outcome": "rejected"}),
        Ok(report) => json!({"outcome": "evaluated", "state": report.claims[0].state}),
        Err(other) => json!({"outcome": format!("{other:?}")}),
    };
    let mismatch_problem = AbstractClaimProblem {
        expected_scope: scope.clone(),
        claims: BTreeSet::from(["c.scope".into()]),
        evidence: vec![AbstractEvidence {
            id: "ev.wrong_scope".into(),
            claim_id: "c.scope".into(),
            polarity: AbstractPolarity::Support,
            scope: abs_scope(HASH_D, HASH_B, AbstractEvidenceRole::StructuralRelationship),
            assumptions: BTreeSet::new(),
            active: true,
        }],
        branches: Vec::new(),
        bounds: AbstractBounds {
            max_claims: 16,
            max_steps: 16,
        },
    };
    let mismatch_abs = match evaluate_reference_claims(&mismatch_problem) {
        AbstractOutcome::Rejected { .. } => json!({"outcome": "rejected"}),
        other => json!(other),
    };
    cases.push(CorrespondenceCase {
        case_id: "law2.scope_mismatch_and_role_non_promotion_rejected".into(),
        law: ReasoningKernelLaw::ScopeConservationAndRoleNonPromotion,
        reproducing_input: json!({ "expected_policy": HASH_B, "supplied_policy": HASH_D }),
        abstract_trace: mismatch_abs,
        implementation_trace: mismatch_impl,
    });

    // -----------------------------------------------------------------------
    // Law 3: Obligation conservation through justified refinement
    // -----------------------------------------------------------------------
    let (ref_pol, ref_req_complete) = build_refinement_case(true);
    let (_, ref_req_missing_int) = build_refinement_case(false);
    let ref_complete_report =
        evaluate_refinement(&ref_pol, &ref_req_complete).expect("complete refinement");
    let ref_missing_report =
        evaluate_refinement(&ref_pol, &ref_req_missing_int).expect("missing integration");
    let abs_ref_complete = evaluate_reference_refinement(&AbstractRefinementProblem {
        parent_id: "decomp.parent".into(),
        child_obligations: BTreeMap::from([("child.one".into(), true)]),
        integration_obligations: BTreeMap::from([("integration.parent".into(), true)]),
        child_assumptions: BTreeSet::from(["assumption.env".into()]),
        discharged_assumptions: BTreeSet::new(),
    });
    let abs_ref_missing = evaluate_reference_refinement(&AbstractRefinementProblem {
        parent_id: "decomp.parent".into(),
        child_obligations: BTreeMap::from([("child.one".into(), true)]),
        integration_obligations: BTreeMap::from([("integration.parent".into(), false)]),
        child_assumptions: BTreeSet::from(["assumption.env".into()]),
        discharged_assumptions: BTreeSet::new(),
    });
    cases.push(CorrespondenceCase {
        case_id: "law3.refinement_conserves_child_integration_and_assumptions".into(),
        law: ReasoningKernelLaw::ObligationConservationThroughRefinement,
        reproducing_input: json!({ "decomposition": "decomp.parent", "integration_present": [true, false] }),
        abstract_trace: json!({
            "complete_satisfied": abs_ref_complete.parent_satisfied,
            "missing_integration_satisfied": abs_ref_missing.parent_satisfied,
            "residual_assumptions": abs_ref_complete.residual_assumptions,
        }),
        implementation_trace: json!({
            "complete_satisfied": ref_complete_report.decompositions[0].state == ObligationState::Supported,
            "missing_integration_satisfied": ref_missing_report.decompositions[0].state == ObligationState::Supported,
            "residual_assumptions": ref_complete_report.decompositions[0].residual_assumption_ids,
        }),
    });

    // -----------------------------------------------------------------------
    // Law 5: No inference from unsupported sole premise; alternative survives
    // -----------------------------------------------------------------------
    let p2 = scoped_claim(
        "main",
        Proposition::CheckPassed {
            check_id: "clippy".into(),
        },
        HASH_B,
    );
    let p2_id = claim_identity(&p2).expect("p2 id");
    let sole_case = SupportCase {
        schema: SUPPORT_CASE_SCHEMA_V1.into(),
        claims: vec![p1.clone(), concl.clone()],
        evidence: vec![EvidenceRecord {
            id: "ev.p1".into(),
            claim: p1_id.clone(),
            polarity: EvidencePolarity::Support,
            rule: "native-check.v1".into(),
            version: 1,
            assurance_scope: "saved check".into(),
            assumptions: BTreeSet::new(),
        }],
        branches: vec![SupportBranch {
            id: "br.sole".into(),
            conclusion: concl_id.clone(),
            polarity: BranchPolarity::Support,
            rule: "reviewed_completion_v1".into(),
            version: 1,
            premises: vec![p1_id.clone()],
            assumptions: BTreeSet::new(),
            assurance_scope: "bounded derivation".into(),
            applicability: claim_context(HASH_B),
            cross_revision_relation: None,
        }],
        changes: vec![ChangeEvent {
            id: "chg.stale_p1".into(),
            sequence: 1,
            reason: "p1 invalidated".into(),
            change: Change::EvidenceWithdrawn {
                evidence: "ev.p1".into(),
            },
        }],
        obligations: Vec::new(),
    };
    let sole_report = evaluate_support_maintenance(&pol, &sole_case).expect("sole eval");
    let alt_case = SupportCase {
        schema: SUPPORT_CASE_SCHEMA_V1.into(),
        claims: vec![p1.clone(), p2.clone(), concl.clone()],
        evidence: vec![
            EvidenceRecord {
                id: "ev.p1".into(),
                claim: p1_id.clone(),
                polarity: EvidencePolarity::Support,
                rule: "native-check.v1".into(),
                version: 1,
                assurance_scope: "saved check".into(),
                assumptions: BTreeSet::new(),
            },
            EvidenceRecord {
                id: "ev.p2".into(),
                claim: p2_id.clone(),
                polarity: EvidencePolarity::Support,
                rule: "native-check.v1".into(),
                version: 1,
                assurance_scope: "saved check".into(),
                assumptions: BTreeSet::from(["assumption.alt".into()]),
            },
        ],
        branches: vec![
            SupportBranch {
                id: "br.primary".into(),
                conclusion: concl_id.clone(),
                polarity: BranchPolarity::Support,
                rule: "reviewed_completion_v1".into(),
                version: 1,
                premises: vec![p1_id.clone()],
                assumptions: BTreeSet::new(),
                assurance_scope: "bounded derivation".into(),
                applicability: claim_context(HASH_B),
                cross_revision_relation: None,
            },
            SupportBranch {
                id: "br.alt".into(),
                conclusion: concl_id.clone(),
                polarity: BranchPolarity::Support,
                rule: "reviewed_completion_v1".into(),
                version: 1,
                premises: vec![p2_id.clone()],
                assumptions: BTreeSet::new(),
                assurance_scope: "bounded derivation".into(),
                applicability: claim_context(HASH_B),
                cross_revision_relation: None,
            },
        ],
        changes: vec![ChangeEvent {
            id: "chg.stale_p1".into(),
            sequence: 1,
            reason: "primary invalidated, alternative survives".into(),
            change: Change::EvidenceWithdrawn {
                evidence: "ev.p1".into(),
            },
        }],
        obligations: Vec::new(),
    };
    let alt_report = evaluate_support_maintenance(&pol, &alt_case).expect("alt eval");
    let sole_abs = evaluate_reference_trace(
        &AbstractClaimProblem {
            expected_scope: scope.clone(),
            claims: BTreeSet::from([p1_id.clone(), concl_id.clone()]),
            evidence: vec![AbstractEvidence {
                id: "ev.p1".into(),
                claim_id: p1_id.clone(),
                polarity: AbstractPolarity::Support,
                scope: scope.clone(),
                assumptions: BTreeSet::new(),
                active: true,
            }],
            branches: vec![AbstractRuleBranch {
                id: "br.sole".into(),
                conclusion: concl_id.clone(),
                polarity: AbstractPolarity::Support,
                premises: vec![p1_id.clone()],
                scope: scope.clone(),
                assumptions: BTreeSet::new(),
                active: true,
            }],
            bounds: AbstractBounds {
                max_claims: 16,
                max_steps: 32,
            },
        },
        &[AbstractUpdateEvent::WithdrawEvidence {
            evidence_id: "ev.p1".into(),
        }],
    );
    let alt_abs = evaluate_reference_trace(
        &AbstractClaimProblem {
            expected_scope: scope.clone(),
            claims: BTreeSet::from([p1_id.clone(), p2_id.clone(), concl_id.clone()]),
            evidence: vec![
                AbstractEvidence {
                    id: "ev.p1".into(),
                    claim_id: p1_id.clone(),
                    polarity: AbstractPolarity::Support,
                    scope: scope.clone(),
                    assumptions: BTreeSet::new(),
                    active: true,
                },
                AbstractEvidence {
                    id: "ev.p2".into(),
                    claim_id: p2_id.clone(),
                    polarity: AbstractPolarity::Support,
                    scope: scope.clone(),
                    assumptions: BTreeSet::from(["assumption.alt".into()]),
                    active: true,
                },
            ],
            branches: vec![
                AbstractRuleBranch {
                    id: "br.primary".into(),
                    conclusion: concl_id.clone(),
                    polarity: AbstractPolarity::Support,
                    premises: vec![p1_id.clone()],
                    scope: scope.clone(),
                    assumptions: BTreeSet::new(),
                    active: true,
                },
                AbstractRuleBranch {
                    id: "br.alt".into(),
                    conclusion: concl_id.clone(),
                    polarity: AbstractPolarity::Support,
                    premises: vec![p2_id.clone()],
                    scope: scope.clone(),
                    assumptions: BTreeSet::new(),
                    active: true,
                },
            ],
            bounds: AbstractBounds {
                max_claims: 16,
                max_steps: 32,
            },
        },
        &[AbstractUpdateEvent::WithdrawEvidence {
            evidence_id: "ev.p1".into(),
        }],
    );
    cases.push(CorrespondenceCase {
        case_id: "law5.sole_premise_withdrawn_and_alternative_survives".into(),
        law: ReasoningKernelLaw::UnsupportedSolePremiseAndAlternativeSupport,
        reproducing_input: json!({ "withdrawn": "ev.p1", "alternative": "br.alt" }),
        abstract_trace: json!({
            "sole_after": project_abstract_claim(&sole_abs[1], &concl_id),
            "alt_after": project_abstract_claim(&alt_abs[1], &concl_id),
        }),
        implementation_trace: json!({
            "sole_after": project_maintenance_claim(&sole_report, &concl_id),
            "alt_after": project_maintenance_claim(&alt_report, &concl_id),
        }),
    });

    // -----------------------------------------------------------------------
    // Law 6: Preservation and containment of counterevidence
    // -----------------------------------------------------------------------
    let counter_case = SupportCase {
        schema: SUPPORT_CASE_SCHEMA_V1.into(),
        claims: vec![p1.clone(), p2.clone()],
        evidence: vec![
            EvidenceRecord {
                id: "ev.p1_sup".into(),
                claim: p1_id.clone(),
                polarity: EvidencePolarity::Support,
                rule: "native-check.v1".into(),
                version: 1,
                assurance_scope: "saved check".into(),
                assumptions: BTreeSet::new(),
            },
            EvidenceRecord {
                id: "ev.p1_ref".into(),
                claim: p1_id.clone(),
                polarity: EvidencePolarity::Counterevidence,
                rule: "native-check.v1".into(),
                version: 1,
                assurance_scope: "saved check".into(),
                assumptions: BTreeSet::new(),
            },
            EvidenceRecord {
                id: "ev.p2_sup".into(),
                claim: p2_id.clone(),
                polarity: EvidencePolarity::Support,
                rule: "native-check.v1".into(),
                version: 1,
                assurance_scope: "saved check".into(),
                assumptions: BTreeSet::new(),
            },
        ],
        branches: Vec::new(),
        changes: Vec::new(),
        obligations: Vec::new(),
    };
    let counter_report = evaluate_support_maintenance(&pol, &counter_case).expect("counter eval");
    let counter_abs = evaluate_reference_claims(&AbstractClaimProblem {
        expected_scope: scope.clone(),
        claims: BTreeSet::from([p1_id.clone(), p2_id.clone()]),
        evidence: vec![
            AbstractEvidence {
                id: "ev.p1_sup".into(),
                claim_id: p1_id.clone(),
                polarity: AbstractPolarity::Support,
                scope: scope.clone(),
                assumptions: BTreeSet::new(),
                active: true,
            },
            AbstractEvidence {
                id: "ev.p1_ref".into(),
                claim_id: p1_id.clone(),
                polarity: AbstractPolarity::Refutation,
                scope: scope.clone(),
                assumptions: BTreeSet::new(),
                active: true,
            },
            AbstractEvidence {
                id: "ev.p2_sup".into(),
                claim_id: p2_id.clone(),
                polarity: AbstractPolarity::Support,
                scope: scope.clone(),
                assumptions: BTreeSet::new(),
                active: true,
            },
        ],
        branches: Vec::new(),
        bounds: AbstractBounds {
            max_claims: 16,
            max_steps: 16,
        },
    });
    cases.push(CorrespondenceCase {
        case_id: "law6.counterevidence_disputes_target_and_contains_unrelated".into(),
        law: ReasoningKernelLaw::CounterevidencePreservationAndContainment,
        reproducing_input: json!({ "disputed": p1_id, "unrelated_supported": p2_id }),
        abstract_trace: json!({
            "target": project_abstract_claim(&counter_abs, &p1_id),
            "unrelated": project_abstract_claim(&counter_abs, &p2_id),
        }),
        implementation_trace: json!({
            "target": project_maintenance_claim(&counter_report, &p1_id),
            "unrelated": project_maintenance_claim(&counter_report, &p2_id),
        }),
    });

    // -----------------------------------------------------------------------
    // Law 7: Replayability and explanation fidelity
    // -----------------------------------------------------------------------
    let replay_one = evaluate_support_maintenance(&pol, &sound_case).expect("replay 1");
    let replay_two = evaluate_support_maintenance(&pol, &sound_case).expect("replay 2");
    let abs_replay_one = evaluate_reference_claims(&sound_problem);
    let abs_replay_two = evaluate_reference_claims(&sound_problem);
    cases.push(CorrespondenceCase {
        case_id: "law7.replay_reproduces_decision_assumptions_and_digest".into(),
        law: ReasoningKernelLaw::ReplayabilityAndExplanationFidelity,
        reproducing_input: json!({ "claim": concl_id, "replay_count": 2 }),
        abstract_trace: json!({
            "identical": abs_replay_one == abs_replay_two,
            "result": project_abstract_claim(&abs_replay_one, &concl_id),
        }),
        implementation_trace: json!({
            "identical": replay_one == replay_two && replay_one.report_blake3 == replay_two.report_blake3,
            "result": project_maintenance_claim(&replay_one, &concl_id),
        }),
    });

    // -----------------------------------------------------------------------
    // Law 8: Presentation-adapter invariance of workflow obligations
    // -----------------------------------------------------------------------
    let wf_policy: WorkflowObligationPolicy =
        serde_json::from_str(SHIPPED_OBLIGATIONS).expect("shipped obligations");
    let default_json: Value =
        serde_json::from_str(SHIPPED_DEFAULT_POLICY).expect("shipped default policy");
    let gen_policy: GeneratedAgentWorkflowPolicy =
        serde_json::from_value(default_json["generated_agent_workflow_policy"].clone())
            .expect("generated workflow policy");
    let gen_catalog = admit_generated_agent_workflow_policy(&gen_policy).expect("catalog admits");
    let obl_catalog = admit_workflow_obligation_policy(&wf_policy, &gen_catalog, HASH_B)
        .expect("obligation catalog admits");
    let verify_def = obl_catalog
        .definitions
        .iter()
        .find(|def| def.definition.workflow_id == "verify")
        .expect("verify definition");
    let pred = obligation_predicate(
        verify_def,
        &obl_catalog.policy,
        &BTreeMap::new(),
        HASH_A,
        HASH_C,
    )
    .expect("obligation predicate");
    let expected_ids: BTreeSet<String> = pred
        .evidence_requirements
        .iter()
        .map(|req| req.requirement_id.clone())
        .collect();
    let mut per_adapter_invariants = BTreeMap::new();
    for adapter in ["agent-skills", "claude", "codex"] {
        let rendered = render_with_workflow_obligations(&gen_catalog, &obl_catalog, adapter)
            .expect("render with obligations");
        let verify_file = rendered
            .iter()
            .find(|file| file.workflow_id == "verify")
            .expect("verify file");
        let all_ids_present = expected_ids
            .iter()
            .all(|id| verify_file.contents.contains(id));
        per_adapter_invariants.insert(
            adapter.to_string(),
            json!({
                "predicate_blake3": pred.predicate_blake3,
                "all_obligation_ids_present": all_ids_present,
            }),
        );
    }
    cases.push(CorrespondenceCase {
        case_id: "law8.workflow_obligation_adapter_invariance".into(),
        law: ReasoningKernelLaw::PresentationAdapterInvariance,
        reproducing_input: json!({ "workflow": "verify", "adapters": ["agent-skills", "claude", "codex"] }),
        abstract_trace: json!({
            "agent-skills": { "predicate_blake3": pred.predicate_blake3, "all_obligation_ids_present": true },
            "claude": { "predicate_blake3": pred.predicate_blake3, "all_obligation_ids_present": true },
            "codex": { "predicate_blake3": pred.predicate_blake3, "all_obligation_ids_present": true },
        }),
        implementation_trace: json!(per_adapter_invariants),
    });

    // -----------------------------------------------------------------------
    // Law 9: Narrow positive monotonicity vs non-monotonic refutation/policy
    // -----------------------------------------------------------------------
    let base_empty = SupportCase {
        schema: SUPPORT_CASE_SCHEMA_V1.into(),
        claims: vec![p1.clone()],
        evidence: Vec::new(),
        branches: Vec::new(),
        changes: Vec::new(),
        obligations: Vec::new(),
    };
    let mut added_pos = base_empty.clone();
    added_pos.changes.push(ChangeEvent {
        id: "chg.add_pos".into(),
        sequence: 1,
        reason: "admit positive observation".into(),
        change: Change::EvidenceAdmitted {
            evidence: EvidenceRecord {
                id: "ev.p1".into(),
                claim: p1_id.clone(),
                polarity: EvidencePolarity::Support,
                rule: "native-check.v1".into(),
                version: 1,
                assurance_scope: "saved check".into(),
                assumptions: BTreeSet::new(),
            },
        },
    });
    let mut policy_changed = added_pos.clone();
    policy_changed.changes.push(ChangeEvent {
        id: "chg.policy".into(),
        sequence: 2,
        reason: "policy revision invalidates prior support".into(),
        change: Change::PolicyChanged {
            previous_policy_blake3: HASH_B.into(),
        },
    });
    let rep_empty = evaluate_support_maintenance(&pol, &base_empty).expect("empty");
    let rep_pos = evaluate_support_maintenance(&pol, &added_pos).expect("pos");
    let rep_pol = evaluate_support_maintenance(&pol, &policy_changed).expect("policy changed");
    let mono_trace = evaluate_reference_trace(
        &AbstractClaimProblem {
            expected_scope: scope.clone(),
            claims: BTreeSet::from([p1_id.clone()]),
            evidence: Vec::new(),
            branches: Vec::new(),
            bounds: AbstractBounds {
                max_claims: 16,
                max_steps: 16,
            },
        },
        &[
            AbstractUpdateEvent::AddEvidence(AbstractEvidence {
                id: "ev.p1".into(),
                claim_id: p1_id.clone(),
                polarity: AbstractPolarity::Support,
                scope,
                assumptions: BTreeSet::new(),
                active: true,
            }),
            AbstractUpdateEvent::ChangePolicy {
                new_policy_id: HASH_D.into(),
            },
        ],
    );
    cases.push(CorrespondenceCase {
        case_id: "law9.narrow_positive_monotonicity_and_policy_nonmonotonicity".into(),
        law: ReasoningKernelLaw::NarrowPositiveMonotonicity,
        reproducing_input: json!({ "events": ["add_positive_evidence", "policy_changed"] }),
        abstract_trace: json!({
            "initial": project_abstract_claim(&mono_trace[0], &p1_id),
            "after_positive_add": project_abstract_claim(&mono_trace[1], &p1_id),
            "after_policy_change": project_abstract_claim(&mono_trace[2], &p1_id),
        }),
        implementation_trace: json!({
            "initial": project_maintenance_claim(&rep_empty, &p1_id),
            "after_positive_add": project_maintenance_claim(&rep_pos, &p1_id),
            "after_policy_change": project_maintenance_claim(&rep_pol, &p1_id),
        }),
    });

    cases
}

#[test]
fn complete_bounded_corpus_agrees_with_independent_reference_semantics() {
    let corpus = build_full_correspondence_corpus();
    let receipt = qualify_correspondence(&corpus);
    assert!(
        receipt.valid,
        "expected full bounded conformance, divergences: {:?}",
        receipt.divergences
    );
    assert_eq!(receipt.verdict, "pass");
    assert_eq!(
        receipt.qualification_class,
        QualificationClass::BoundedConformance
    );
    assert_ne!(
        receipt.qualification_class,
        QualificationClass::AbstractProof
    );
    assert_ne!(
        receipt.qualification_class,
        QualificationClass::BoundedModelCheck
    );
    assert_ne!(
        receipt.qualification_class,
        QualificationClass::MechanizedImplementationRefinement
    );
    assert_eq!(receipt.law_summaries.len(), ReasoningKernelLaw::ALL.len());
    assert!(receipt
        .law_summaries
        .iter()
        .all(|summary| summary.conforming));
    assert!(receipt.divergences.is_empty());
    assert_eq!(receipt.model_identity.len(), 64);
    assert_eq!(receipt.rule_identity.len(), 64);
    assert_eq!(receipt.mapping_identity.len(), 64);
    assert_eq!(receipt.corpus_identity.len(), 64);
    assert_eq!(receipt.receipt_hash.len(), 64);
    assert!(!receipt.non_claims.is_empty());

    // Replaying qualification produces byte-identical receipt_hash and identities.
    let replay = qualify_correspondence(&corpus);
    assert_eq!(receipt, replay);
}

#[test]
fn deliberate_violation_of_every_law_is_detected_with_reproducing_traces() {
    let base_corpus = build_full_correspondence_corpus();
    for &law in ReasoningKernelLaw::ALL {
        let mut mutated = base_corpus.clone();
        let target = mutated
            .iter_mut()
            .find(|case| case.law == law)
            .expect("corpus covers every law");
        let original_input = target.reproducing_input.clone();
        let original_abstract = target.abstract_trace.clone();
        let faulty_impl = json!({
            "violated_law": law.id(),
            "faulty_implementation_trace": true
        });
        target.implementation_trace = faulty_impl.clone();

        let receipt = qualify_correspondence(&mutated);
        assert!(
            !receipt.valid,
            "deliberate violation of {} must fail qualification",
            law.id()
        );
        assert_eq!(receipt.verdict, "fail");
        let summary = receipt
            .law_summaries
            .iter()
            .find(|summary| summary.law == law)
            .expect("law summary");
        assert!(!summary.conforming);
        assert_eq!(summary.divergences, 1);
        assert_eq!(receipt.divergences.len(), 1);
        let div = &receipt.divergences[0];
        assert_eq!(div.law, law);
        assert_eq!(div.reproducing_input, original_input);
        assert_eq!(div.abstract_trace, original_abstract);
        assert_eq!(div.implementation_trace, faulty_impl);
    }
}

#[test]
fn bound_exhaustion_is_explicit_in_both_reference_and_rust_evaluators() {
    let scope = abs_scope(HASH_B, HASH_B, AbstractEvidenceRole::Observation);
    let abs_exhausted = evaluate_reference_claims(&AbstractClaimProblem {
        expected_scope: scope,
        claims: BTreeSet::from(["c1".into(), "c2".into()]),
        evidence: Vec::new(),
        branches: Vec::new(),
        bounds: AbstractBounds {
            max_claims: 1,
            max_steps: 4,
        },
    });
    assert!(matches!(
        abs_exhausted,
        AbstractOutcome::BoundsExhausted { .. }
    ));

    let p1 = scoped_claim(
        "b1",
        Proposition::CheckPassed {
            check_id: "a".into(),
        },
        HASH_B,
    );
    let p2 = scoped_claim(
        "b2",
        Proposition::CheckPassed {
            check_id: "b".into(),
        },
        HASH_B,
    );
    let tight_policy = support_policy(1);
    let tight_case = SupportCase {
        schema: SUPPORT_CASE_SCHEMA_V1.into(),
        claims: vec![p1, p2],
        evidence: Vec::new(),
        branches: Vec::new(),
        changes: Vec::new(),
        obligations: Vec::new(),
    };
    let rust_exhausted = evaluate_support_maintenance(&tight_policy, &tight_case);
    assert!(matches!(
        rust_exhausted,
        Err(DerivationError::BoundExhaustion(_))
    ));
}

#[test]
fn abstraction_mapping_names_all_four_covered_rust_entry_points() {
    let mapping = canonical_abstraction_mapping();
    assert_eq!(mapping.len(), 4);
    let entry_points: Vec<&str> = mapping
        .iter()
        .map(|entry| entry.rust_entry_point.as_str())
        .collect();
    assert!(entry_points
        .iter()
        .any(|ep| ep.contains("scoped_claim_derivations::evaluate_derivations")));
    assert!(entry_points
        .iter()
        .any(|ep| ep.contains("support_maintenance::evaluate_support_maintenance")));
    assert!(entry_points
        .iter()
        .any(|ep| ep.contains("refinement_typed_dependencies::evaluate_refinement")));
    assert!(entry_points
        .iter()
        .any(|ep| ep.contains("workflow_obligations")));
}
