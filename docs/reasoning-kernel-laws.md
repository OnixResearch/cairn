# Reasoning-Kernel Laws and Correspondence Qualification

This specification defines the abstract semantics, algebraic laws, and
Abadi-Lamport-style abstraction mapping for Cairn's finite reasoning kernel
([issue #13](https://github.com/OnixResearch/cairn/issues/13), under roadmap
[issue #4](https://github.com/OnixResearch/cairn/issues/4)).

The specification is defined **independently of Rust execution** and qualified
against Cairn's Rust evaluators through a bounded conformance harness
(`cairn_core::verified::reasoning_kernel_laws`). Existing state-machine receipts
(`state_machine_receipt`) and Kani receipts (`kani_receipt`) keep their exact
schemas and meanings unchanged.

## 1. Qualification Classes and Claim Boundary

Every qualification artifact distinguishes four distinct classes:

| Class | Meaning | Claimed by this qualification? |
| --- | --- | --- |
| `abstract_proof` | Mathematical proof over the abstract transition model alone | No |
| `bounded_model_check` | Exhaustive exploration of the abstract model within finite bounds | No |
| `bounded_conformance` | Executed agreement between the independent reference evaluator and the Rust evaluators over a declared bounded corpus, plus deliberate law-violation detection | **Yes** |
| `mechanized_implementation_refinement` | Mechanized proof that the compiled Rust implementation refines the abstract specification under named assumptions | No |

Agreement with a hand-written reference model is **bounded conformance only**,
not an unqualified implementation proof. Cryptographic assumptions, parser
correctness, external verifier soundness, natural-language correspondence,
broader concurrency, and unbounded liveness remain separately tracked
obligations.

## 2. Abstract State, Inputs, Rules, Events, and Outcomes

For the covered finite claim, support-maintenance, refinement, and
workflow-obligation fragment:

- **Abstract claim state** `S(c) ∈ { Unresolved, Supported, Refuted, Disputed }`:
  - `Unresolved`: no active positive support branch and no active refutation/counterevidence.
  - `Supported`: at least one active positive support branch and zero active counterevidence/refutation branches.
  - `Refuted`: zero active positive support branches and at least one active counterevidence/refutation branch.
  - `Disputed`: at least one active positive support branch **and** at least one active counterevidence/refutation branch.
- **Abstract scope & evidence role**:
  - Each claim and rule binds a nominal scope `(requirement_revision, interpretation, contract, subject, policy, target, environment, bounds)` and a typed evidence role (`Observation`, `StructuralRelationship`, `BoundedProperty`, `CheckerAcceptedProperty`, `ReviewedRequirementCorrespondence`).
  - A rule is applicable only when premise scopes and conclusion scopes satisfy the declared equality/applicability condition; roles never silently promote across layers.
- **Abstract inference rule**:
  - A rule `r = (id, version, polarity, premises, rule_assumptions, scope)` is active iff it is admitted by policy, its applicability scope matches the current context, it is not withdrawn, and every premise `p ∈ premises` is `Supported`.
  - When active, the rule contributes its `polarity ∈ { Support, Refutation }` and propagates the union of `rule_assumptions` plus every premise's residual assumptions.
- **Abstract update events**:
  - `AddEvidence`, `AddBranch`, `WithdrawEvidence`, `WithdrawBranch`, `AddCounterevidence`, `ReviseRequirement`, `ChangePolicy`, `ChangeApplicability`, `InvalidateBranch`.
  - Evaluating a finite ordered event trace produces a deterministic sequence of abstract states; invalid/out-of-scope/unknown inputs yield explicit `Rejected`, and exceeding declared step/depth/size bounds yields explicit `BoundsExhausted`.
- **Unsupported cases**:
  - Unknown schemas, uncatalogued rules, cyclic premise graphs, cross-revision support transfer without an explicit admitted relation, and unreviewed correspondence remain explicitly rejected or unresolved, never promoted to `Supported`.

## 3. Reasoning-Kernel Laws

All nine laws are specified over the abstract model independently of Rust:

1. **Conditional soundness** (`conditional_soundness`):
   If an admitted rule is applicable and every required premise is `Supported` with no active counterevidence, the conclusion is `Supported` and retains the exact union of rule and premise assumptions.
2. **Scope conservation and evidence-role non-promotion** (`scope_conservation_and_role_non_promotion`):
   A claim derived in scope `σ` with role `ρ` cannot be discharged by evidence from a different scope `σ' ≠ σ` or promoted to a stronger role `ρ' ≠ ρ` without an explicit admitted rule.
3. **Obligation conservation through justified refinement** (`obligation_conservation_through_refinement`):
   Decomposing a parent requirement into child guarantees conserves every required obligation (child obligations, integration obligations, implication relations, and residual assumptions); omitting or failing any child or integration obligation prevents parent discharge.
4. **Explicit discharge or retention of every assumption** (`assumption_discharge_or_retention`):
   Every assumption introduced by a premise, rule, or child guarantee must either be explicitly discharged by an admitted compatible guarantee with a bound semantic witness or remain visible in the residual assumption set.
5. **No inference from an unsupported sole premise; alternative valid support remains usable** (`unsupported_sole_premise_and_alternative_support`):
   Withdrawing or invalidating the sole premise of a support branch withdraws that branch (`Unresolved`); when a second independent valid support branch exists, the conclusion remains `Supported` via the surviving branch while the withdrawn branch records its invalidation cause.
6. **Preservation and containment of counterevidence** (`counterevidence_preservation_and_containment`):
   Active counterevidence is never silently dropped: against zero active support it yields `Refuted`, against active support it yields `Disputed`, and it remains contained to its bound claim/dependents rather than corrupting unrelated claims.
7. **Replayability and explanation fidelity** (`replayability_and_explanation_fidelity`):
   Replaying an evaluation over the same admitted inputs yields identical claim states, residual assumptions, and content-bound report identity; every explanation cites only active/recorded branches and assumptions.
8. **Presentation-adapter invariance of workflow obligations** (`presentation_adapter_invariance`):
   Rendering the same admitted workflow obligation definition across every supported presentation adapter (`agent-skills`, `claude`, `codex`) preserves identical obligation IDs, predicate identity, required native evidence references, and command-class authority.
9. **Narrow positive monotonicity** (`narrow_positive_monotonicity`):
   In the fixed positive fragment (fixed policy, fixed requirement/context revisions, no counterevidence, no withdrawals), adding an applicable positive evidence/support branch can only move a claim from `Unresolved` to `Supported` (never reducing support). Conversely, adding counterevidence, changing policy, revising a requirement, or withdrawing applicability may reduce acceptance (`Supported → Disputed | Refuted | Unresolved`).

## 4. Abstraction Mapping to Rust Entry Points

Following Abadi & Lamport, the mapping names the concrete Rust entry points,
observed fields, rejected inputs, and stuttering (internal-only) steps:

| Abstract Operation | Rust Entry Point | Concrete Input / Output Fields | Rejected Inputs | Stuttering Steps |
| --- | --- | --- | --- | --- |
| Claim derivation evaluation | `cairn_core::verified::scoped_claim_derivations::evaluate_derivations` | `EvaluationRequest.{claims, derivations, producers}` → `DerivationReport.{claims[].{claim_id, state, residual_assumptions, support, counterevidence}, report_blake3}` | `Err(DerivationError::{Unsupported, ScopeMismatch, Cycle, BoundExhaustion, InvalidInput})` → `AbstractOutcome::Rejected` / `BoundsExhausted` | Canonical sorting/deduplication of claim lists, assumption sets, and diagnostic order |
| Support maintenance replay | `cairn_core::verified::support_maintenance::evaluate_support_maintenance` | `SupportCase.{claims, evidence, branches, changes}` → `MaintenanceReport.{claims[].{claim_id, baseline_state, state, surviving_branches, withdrawn_branches, support, counterevidence, residual_assumptions}, report_blake3}` | `Err(DerivationError::*)` → `AbstractOutcome::Rejected` / `BoundsExhausted` | Event sequence ordering, branch ID sorting, bound-usage counters |
| Refinement & assume-guarantee discharge | `cairn_core::verified::refinement_typed_dependencies::evaluate_refinement` | `RefinementRequest.{decompositions, dependencies, integration_evidence}` → `RefinementReport.{decompositions[].{state, obligations}, dependencies[].{state}, residual_assumptions, report_blake3}` | `Err(RefinementError::*)` → `AbstractOutcome::Rejected` / `BoundsExhausted` | Stable sorting of obligation explanations and residual assumptions |
| Workflow obligation compilation & rendering | `cairn_core::verified::workflow_obligations::{admit_workflow_obligation_policy, obligation_predicate, evaluate_workflow_obligations, render_with_workflow_obligations}` | `(WorkflowObligationPolicy, AdmittedGeneratedWorkflowCatalog, ObligationRequest)` → `(ObligationPredicate, WorkflowObligationReport, Vec<RenderedGeneratedWorkflowFile>)` | `Err(WorkflowObligationIssue)` → `AbstractOutcome::Rejected` | Adapter layout formatting (front matter, headings, list syntax) |

## 5. Retained Identities and Divergence Traces

`qualify_correspondence` computes and retains deterministic BLAKE3 identities
for the reference model (`model_identity`), law/rule set (`rule_identity`),
abstraction mapping (`mapping_identity`), declared corpus (`corpus_identity`),
and overall receipt (`receipt_hash`). Whenever the implementation disagrees with
the reference evaluator — including under deliberate violation mutants — the
receipt records a `DivergenceRecord` carrying the reproducing input summary,
abstract trace, and implementation trace.
