# Workflow Obligations Specification

## Purpose

Defines the `workflow-obligations` capability.

## Requirements

### Requirement: One admitted definition drives guidance and completion

r[cairn.workflow_obligations.policy] Cairn MUST admit an optional versioned workflow obligation policy against the already admitted generated workflow catalog. Each definition MUST bind the exact workflow ID and version, an exact intent identity, action preconditions, expected observations, progress facts, a completion expression, policy-exception references, an optional bounded temporal extension and non-claims. Unknown schemas, profiles, node kinds, predicate kinds, command classes, predicates, duplicate node IDs and exceeded bounds MUST be rejected with stable diagnostics.

#### Scenario: References are checked, not duplicated
- GIVEN the checked obligation policy and a current admitted generated workflow catalog
- WHEN admission runs
- THEN every command class, predicate, node and workflow reference MUST resolve to the admitted catalog or policy
- AND a missing or stale reference MUST reject the policy rather than fall back.

#### Scenario: Binding covers intent, policy and catalog
- GIVEN an admitted definition, generated workflow policy hash and admitted catalog
- WHEN the obligation binding is computed
- THEN it MUST include the definition ID, workflow ID and version, intent identity, policy hash and catalog identity
- AND an evaluation that presents a different binding MUST be a scope mismatch.

### Requirement: Guidance and predicate share one meaning

r[cairn.workflow_obligations.predicate] Cairn MUST derive the machine-readable completion predicate and obligation summaries from the same admitted definition and selected alternatives, binding the subject and context revisions, exact native evidence requirement IDs and accepted revisions, and the descriptor's permitted command classes.

#### Scenario: Predicate identity changes with semantic inputs
- GIVEN the same definition and evidence requirements
- WHEN the subject, context, policy, catalog, definition or selected alternative changes
- THEN the predicate identity MUST change
- AND reusing evidence from the old binding MUST remain stale or out of scope.

r[cairn.workflow_obligations.guidance] Cairn MUST derive agent guidance from the same admitted definition. Guidance MUST list completion obligations with their native requirement and accepted revision, alternatives, the selected alternative, action preconditions, expected observations, progress facts, exception paths, temporal model, permitted command classes and non-claims.

#### Scenario: Missing selection is explicit
- GIVEN an `any_of` node with no selected alternative
- WHEN guidance is rendered
- THEN it MUST state that no alternative is selected and completion is unresolved
- AND it MUST NOT choose an ambient alternative.

r[cairn.workflow_obligations.adapter_invariance] Presentation adapters MUST change layout and prose only.

#### Scenario: Every adapter carries identical obligations
- GIVEN the pilot workflow and all admitted guidance adapters
- WHEN each adapter renders the workflow with obligations
- THEN every obligation ID, native evidence requirement, accepted revision and permitted command class MUST match across adapters
- AND adapter text MUST differ only in layout or formatting.

#### Scenario: Presentation cannot weaken the predicate
- GIVEN a presentation with a different heading level, bullet character or format
- WHEN guidance is rendered and the predicate is derived
- THEN every evidence requirement and the full permitted command-class set MUST remain
- AND the predicate identity MUST be unchanged.

### Requirement: Categories stay separate and claims are not evidence

r[cairn.workflow_obligations.evaluation] Cairn MUST evaluate only the completion expression for delivery. Action preconditions, expected observations and progress facts MUST be reported separately. Activity counts, checked task boxes and an agent success statement MUST never discharge a completion obligation. Blocked and abandoned terminal observations MUST be non-successful.

#### Scenario: Complete native evidence proves the workflow
- GIVEN every required native predicate observation is satisfied and in the exact subject, context, policy and revision scope
- WHEN the selected alternative and conditional applicability are resolved
- THEN the report MUST be `proved`, list the satisfied obligations and leave `remaining` empty.

#### Scenario: Missing or stale evidence blocks
- GIVEN a missing, stale, duplicate, out-of-scope or unsupported observation
- WHEN evaluation runs
- THEN the affected node MUST be `unknown` or `unsupported`
- AND the obligation MUST appear in `remaining`
- AND the outcome MUST NOT be `proved`.

#### Scenario: Partial alternative branches do not combine
- GIVEN an `any_of` with a complete and an incomplete alternative
- WHEN only the incomplete alternative is selected
- THEN only that branch MUST be evaluated
- AND evidence from the complete unselected branch MUST NOT combine into success.

### Requirement: Bounded temporal checks use the declared observation model

r[cairn.workflow_obligations.temporal] Cairn MAY support a bounded ordering check between two named events in a declared finite trace interval. The definition MUST declare the trace identity, both event names, observer identity, clock basis, required completeness and finite span. Supplied observations MUST match those declarations.

#### Scenario: Missing or partial temporal evidence cannot satisfy
- GIVEN a partial interval, absent trace, unknown observer or clock, missing event, or event outside the declared horizon
- WHEN the temporal node is evaluated
- THEN the node MUST be `unknown` or `unsatisfied`
- AND completion MUST NOT be proved.

#### Scenario: Finite result is not unbounded assurance
- GIVEN a satisfied finite ordering result
- WHEN the explanation is reported
- THEN it MUST state that the result implies no unbounded safety or liveness.

### Requirement: Native exception paths preserve the original condition

r[cairn.workflow_obligations.exception] Cairn MUST reference native policy-exception identities, request revisions, policy identities, scope rules and exactly one compensating-control obligation. An exception path MUST require current native applicability and satisfied control evidence bound to the same subject, context, policy and revision.

#### Scenario: Missing control evidence remains unsatisfied
- GIVEN a current exception and an original unsatisfied condition
- WHEN the compensating-control observation is missing or unsatisfied
- THEN the condition MUST remain unsatisfied
- AND completion MUST NOT be proved or accepted under the exception.

#### Scenario: Revocation, expiry or revision change invalidates
- GIVEN a previously accepted exception path
- WHEN the native exception is revoked or expired, or its request revision, policy identity, scope rule or declared control changes
- THEN the path MUST be inapplicable
- AND the original condition MUST remain unsatisfied.

#### Scenario: Acceptance never proves the original
- GIVEN an applicable exception with satisfied control evidence and an original unsatisfied condition
- WHEN evaluation accepts the path
- THEN the report MUST be `exception_accepted` with `proved` false
- AND the original node, its `unsatisfied` state and its basis MUST remain visible.

### Requirement: Export and legacy compatibility

r[cairn.workflow_obligations.export] Cairn MUST export the bound predicate and obligations for selected-work and Kiln consumers by the existing selected-work surface when available, without changing its defaults. Worker guidance, campaign inspection and operator explanations for the same admitted snapshot MUST retain identical obligation IDs, meanings, alternatives and evidence requirements. Missing or unsupported observations MUST NOT become an empty satisfied obligation set.

#### Scenario: Consumer fixtures compare guidance, predicate and remaining obligations
- GIVEN a producer/consumer fixture before and after an evidence or policy change, including an incomplete alternative
- WHEN the exported predicate, guidance and report are compared
- THEN obligation IDs and meanings MUST match for the same admitted snapshot
- AND the changed evidence MUST appear as satisfied or remaining rather than silently disappearing.

r[cairn.workflow_obligations.legacy_compatibility] Workflows without an admitted obligation definition MUST render exactly as before, including contents, BLAKE3 identity and descriptor identity. Existing generated workflow commands, gate contracts, readiness meanings and policy defaults MUST remain unchanged.

#### Scenario: Legacy workflow rendering is unchanged
- GIVEN a workflow such as `explore` with no obligation definition
- WHEN obligation-aware rendering runs
- THEN its bytes, content identity and descriptor identity MUST equal the legacy renderer's output.

r[cairn.workflow_obligations.authority] The obligation compiler and evaluator MUST NOT grant execution, approval, readiness or gate authority, and MUST NOT add an executor, clock service, scheduler or monitoring process. Native evidence owners, exception applicability, readiness, the Pass | Block gate, Kiln and Lattice retain their authority.

#### Scenario: Advisory boundary is published
- GIVEN any admitted or rejected result
- WHEN the report or inspection is emitted
- THEN it MUST carry the explicit boundary and non-claims
- AND no result MAY be presented as an execution authorization or native approval.

### Requirement: Typed per-obligation consumer facts

r[cairn.workflow_obligations.consumer_facts] Cairn MUST derive typed per-obligation facts from the same admitted definition, predicate and guidance used by evaluation. Each fact MUST carry its node kind, parent, alternative, children, selection state, applicability conditions, exact native evidence references, expected observations, command classes, temporal check, native exception reference and recomputable obligation, guidance and remaining-condition identities. A consumer verifier MUST refuse facts that omit nodes or native requirements, promote an unselected alternative, alter an evidence revision or carry stale identities.

#### Scenario: Selection changes presentation identities but not meaning
- GIVEN the `verify` definition with and without a selected alternative
- WHEN facts are derived
- THEN obligation identities MUST stay equal, and guidance and remaining-condition identities MUST change.

#### Scenario: Tampered facts
- GIVEN facts derived for a predicate
- WHEN an evidence revision, selection state, exception fact, node or identity is changed or removed
- THEN verification MUST fail.
