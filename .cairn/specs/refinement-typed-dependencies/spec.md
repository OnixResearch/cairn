# Refinement Typed Dependencies Specification

## Purpose

Defines the `refinement-typed-dependencies` capability.

## Requirements

### Requirement: Versioned finite decomposition

r[cairn.refinement.decomposition] Cairn MUST support an optional versioned decomposition record binding exact parent and child requirement revisions, an exact composition scope, environment assumptions, selected alternatives, finite `all_of`/`any_of`/conditional applicability and integration obligations, with unknown schemas or operators rejected explicitly.

#### Scenario: Exact revisions and scope are part of the argument
- GIVEN a parent revision, child revisions, subject, policy, target, environment and bounds
- WHEN any one of those components changes
- THEN the decomposition identity and explanation MUST change
- AND child evidence from another revision, target, environment or bound scope MUST NOT be combined.

#### Scenario: Composition stays finite and total
- GIVEN an `all_of`, an `any_of` with a declared selected alternative, or a conditional with observed applicability
- WHEN the composition is evaluated
- THEN every reachable obligation MUST be decided from admitted state
- AND an unknown alternative, missing condition observation, empty branch or exhausted bound MUST be an explicit unsupported, pending or bound-exhaustion outcome rather than a partial success.

#### Scenario: `any_of` names one complete strategy
- GIVEN alternatives with partially and fully supported obligations
- WHEN the selected alternative is evaluated
- THEN only that alternative's complete obligation set MUST count
- AND evidence from unselected alternatives MUST NOT be combined or reported as selected.

### Requirement: Distinct implication assurance

r[cairn.refinement.assurance] Cairn MUST represent `environment AND child guarantees AND integration obligations IMPLIES parent` while keeping unassessed relations, reviewed arguments, bounded checks and checked proofs distinct.

#### Scenario: Weakest applicable assurance is published
- GIVEN a reviewed argument, bounded-check child evidence and review or proof relations with different assurance
- WHEN the parent argument is supported
- THEN the parent MUST report the weakest applicable assurance
- AND an unsupported parent MUST NOT be reported with an establishing assurance.

#### Scenario: Unassessed and unchecked proof cannot support
- GIVEN an unassessed implication relation or a checked-proof claim without a policy-admitted checker
- WHEN evaluation runs
- THEN the relation MUST be unsupported or rejected explicitly
- AND child completion, task references or a retained witness MUST NOT upgrade it to proof.

### Requirement: Separately justified feasibility

r[cairn.refinement.feasibility] Cairn MUST require a separately justified feasibility disposition and MUST NOT let contradictory or unresolved assumptions or children yield a vacuously supported parent.

#### Scenario: Feasibility coverage is explicit
- GIVEN a feasible disposition, method, assurance, retained justification and covered premises
- WHEN evaluation runs
- THEN the disposition MUST cover every environment assumption and selected child and use an admitted method and assurance
- AND incomplete coverage or an unadmitted method MUST be unsupported rather than assumed feasible.

#### Scenario: Contradiction is not vacuous truth
- GIVEN the same environment predicate both asserted and negated, a contradicted selected child, or an unresolved environment assumption
- WHEN the parent argument is evaluated
- THEN contradiction MUST make feasibility infeasible and the parent unsupported
- AND an unresolved premise MUST leave the parent pending rather than supported.

### Requirement: Typed dependency relations with named evidence

r[cairn.refinement.dependencies] Cairn MUST support typed dependency contracts alongside legacy `depends_on`, preserving ordering/archive, verified-specification, verified-candidate, candidate-input, integrated-revision, lifecycle and semantic relations as distinct meanings with named authoritative evidence sources.

#### Scenario: Each relation accepts only its own evidence
- GIVEN one dependency record and saved evidence for a different relation, a checked task, worker success or a bare name
- WHEN the dependency is evaluated
- THEN the dependency MUST remain pending or unsupported
- AND a receipt identity alone MUST NOT discharge a relation whose exact fields are absent.

#### Scenario: Integrated revision names every binding
- GIVEN a required integrated revision
- WHEN an admitted integration observation is presented
- THEN it MUST bind the exact producer change, candidate, revision, repository and intended target
- AND a missing, ambiguous, stale or out-of-scope observation MUST remain pending, with relevant target movement adding an applicability reevaluation trigger.

#### Scenario: Whole-change coverage is preserved
- GIVEN a whole-change prerequisite and a receipt covering only some producer tasks, or a selected-task prerequisite and broader evidence
- WHEN coverage is checked
- THEN the partial receipt MUST NOT substitute for the whole change
- AND broader evidence MUST satisfy the narrower selection without strengthening a verified-specification edge into archival.

#### Scenario: Existing dependency semantics remain compatible
- GIVEN existing task ordering, archive dependencies, implementation-ready output and cross-repository v1 receipts
- WHEN the opt-in typed contracts are absent
- THEN their meanings, wire fields and verdicts MUST remain unchanged.

### Requirement: Semantic dependencies need compatible witnesses

r[cairn.refinement.semantic_dependency] Cairn MUST require a semantic dependency to name the producer guarantee, consumer assumption, compatible context and an implication/correspondence witness binding both exact statements.

#### Scenario: Matching names or receipts cannot discharge
- GIVEN a producer guarantee and consumer assumption with matching names or a shared receipt identity but no admitted witness
- WHEN the semantic dependency is evaluated
- THEN it MUST be unsupported
- AND only a policy-admitted witness binding both statement identities and compatible contexts MAY discharge the assumption.

#### Scenario: Incompatible or unestablished guarantees stay blocked
- GIVEN a pending or contradicted producer guarantee, a producer or consumer scope outside the declared context, or a witness statement mismatch
- WHEN the semantic dependency is evaluated
- THEN the specific blocker MUST be reported
- AND the consumer assumption MUST NOT be treated as discharged.

### Requirement: Discharge and residual assumption closure

r[cairn.refinement.discharge] Cairn MUST discharge assumptions through admitted guarantees or retain them as explicit policy-accepted roots, and MUST compute residual assumption closure with original sources and discharge paths.

#### Scenario: Closure retains source and every checked path
- GIVEN a decomposition environment premise, a semantic dependency that discharges it, and an unrelated or policy-accepted premise
- WHEN the report is produced
- THEN each premise MUST identify its original source
- AND discharged premises MUST list their dependency/guarantee/claim paths, while residual, pending and policy-accepted premises remain explicitly distinguished.

#### Scenario: Conflicting premises fail closed
- GIVEN the same assumption identity with conflicting statements or a dependency chain that would let a premise justify itself
- WHEN the case is admitted
- THEN Cairn MUST reject the conflict or cycle explicitly
- AND it MUST NOT silently choose one statement or truncate the graph.

### Requirement: Acyclic initial justification graph

r[cairn.refinement.acyclic] Cairn MUST require an acyclic initial requirement justification graph over decomposition and dependency edges.

#### Scenario: Mutual declarations do not justify one another
- GIVEN requirement A composed from B and requirement B composed from or dependent on A
- WHEN the graph is admitted
- THEN evaluation MUST fail with a specific cycle outcome naming the requirement path
- AND no parent in the cycle MUST be reported supported.

### Requirement: Deterministic obligation explanations and reevaluation

r[cairn.refinement.explanations] Cairn MUST expose deterministic read-only explanations for pending, supported, contradicted, infeasible and unsupported obligations and MUST make requirement, policy, assumption and context changes visible for reevaluation.

#### Scenario: States and links are replayed exactly
- GIVEN the same admitted policy, case, guarantee evidence and previous basis
- WHEN evaluation runs twice
- THEN obligation states, assurance, blockers, residual assumptions, ordering, input identity and report identity MUST be identical
- AND a supported parent MUST remain distinguishable from a conditional parent with explicit residual roots.

#### Scenario: Changes trigger reevaluation
- GIVEN a previous evaluation basis and a changed requirement row, policy identity, assumption statement or context
- WHEN the current basis is compared
- THEN the report MUST name the corresponding trigger
- AND it MUST NOT silently reuse an earlier conclusion.

### Requirement: Read-only advisory inspection

r[cairn.refinement.inspection] Cairn MUST provide a read-only bounded CLI explanation of a refinement and dependency case with explicit admission errors and no lifecycle mutation or registry ownership.

#### Scenario: Positive and pending cases are explained
- GIVEN a supported dependency or a valid case with pending obligations
- WHEN `refinement explain --case FILE [--policy FILE]` runs
- THEN it MUST print the versioned advisory report with exact links, states and boundary
- AND pending or unsupported obligations in a valid case MUST remain a successful advisory explanation.

#### Scenario: Rejection is visible and non-accepting
- GIVEN a malformed case, an unsupported policy, unknown arguments or a cycle
- WHEN explanation runs
- THEN it MUST print an explicit rejected outcome and exit non-zero
- AND it MUST NOT write lifecycle state, execute checks or alter existing command defaults.

### Requirement: Compatibility and retained authority

r[cairn.refinement.compatibility] Refinement and typed dependency support MUST NOT change existing task ordering, archive dependencies, implementation-ready output, cross-repository v1 semantics, verification roles, readiness states or the `Pass | Block` gate.

#### Scenario: Advisory support does not authorize action
- GIVEN a supported parent argument, integration observation or dependency receipt
- WHEN the CLI prints the explanation
- THEN it MUST state that the result is advisory and does not prove behavioral satisfaction, integration correctness, proof or release eligibility
- AND it MUST NOT upgrade any existing completion verdict or execute a lifecycle action.

#### Scenario: Legacy-only cases remain valid
- GIVEN a repository that supplies no refinement policy or typed dependency records
- WHEN existing ordering, archive and cross-repository flows run
- THEN their current behavior and wire meanings MUST remain unchanged
- AND the new policy export MUST remain independent of the legacy default policy.
