# Concurrent change footprints specification delta

## ADDED Requirements

### Requirement: Versioned change footprint over exact revisions

r[cairn.concurrent_footprints.footprint] Cairn MUST bind every admitted change footprint to an exact source revision, an exact intent revision and the exact native requirement rows, interpretation identities, mapping identities and contract identities it changes or consumes, MUST declare consumed assumptions, changed guarantees, preserved invariants, invalidated obligations, affected interfaces and configurations, and explicit unknown or unsupported scope, and MUST NOT introduce a second requirement registry or claim identity.

#### Scenario: Declared identities are recomputed
- GIVEN a footprint revision with a native requirement row and typed contract
- WHEN the footprint is admitted
- THEN its interpretation, mapping and contract identities MUST equal the recomputed identities
- AND a tampered declared identity MUST be a `revision_mismatch` rejection, never a successful analysis

#### Scenario: Incomplete declarations stay explicit
- GIVEN an assumption that consumes a change, guarantee or statement identity the case does not declare
- WHEN the case is analyzed
- THEN the report MUST name the unresolved declaration gap
- AND the case MUST report `declaration_complete = false`
- AND the missing reference MUST NOT be silently ignored as a non-interaction

#### Scenario: Unknown scope stays explicit
- GIVEN a footprint that declares unknown behaviour, unsupported interpretation or bounded exploration
- WHEN an obligation involves that footprint
- THEN its outcome MUST be `unknown` with the declared scope
- AND the report MUST retain the unknown scope

### Requirement: Derived interactions between declared footprints

r[cairn.concurrent_footprints.derivation] Cairn MUST derive an interaction when a changed guarantee affects another change's consumed assumption, when an invalidation affects another change's evidence basis, when invariant commitments overlap, or when interface configuration or operation declarations conflict, and MUST NOT infer behavioural independence from file disjointness, stable syntax or individually passing receipts.

#### Scenario: Disjoint changes interact through an assumption
- GIVEN two changes that share no invariant and only one consumes the other's previous guarantee identity
- WHEN the case is analyzed
- THEN exactly one changed-guarantee interaction MUST be derived
- AND it MUST produce a joint compatibility obligation

#### Scenario: Invalidation affects another evidence basis
- GIVEN one change invalidates an obligation named by another change's evidence basis
- WHEN the case is analyzed
- THEN the interaction MUST name the invalidation, the evidence basis and the affected claims
- AND the obligation MUST require joint evidence

#### Scenario: Overlapping invariant commitments require joint assessment
- GIVEN two changes declare the same invariant id or commitment identity
- WHEN the case is analyzed
- THEN the interaction MUST require joint evidence
- AND an identical declaration MUST NOT be reported as proof of preservation

#### Scenario: Interface declaration conflicts
- GIVEN two changes declare the same interface with different configuration identities, or one removes an interface the other adds or modifies
- WHEN the case is analyzed
- THEN the interaction MUST be `conflict`

### Requirement: Combined-candidate compatibility obligation

r[cairn.concurrent_footprints.obligation] Cairn MUST emit one compatibility obligation naming the combined candidate, participants and their exact revisions, composition order when relevant, context, affected claims and affected invariants, required evidence and a fingerprint, and MUST keep coexistence, invariant preservation and commutativity as separate propositions.

#### Scenario: Composition order is named
- GIVEN an admitted composition covering the interacting participants
- WHEN the obligation is emitted
- THEN the candidate MUST bind the composition id, order and fixed-order flag
- AND every additional admitted order MUST be listed
- AND a pair with no admitted composition MUST be `unknown` with an explicit gap

#### Scenario: Order sensitivity is separate from preservation
- GIVEN two admitted compositions with different orders
- WHEN joint evidence establishes coexistence and invariant preservation only
- THEN commutativity MUST remain `not_established`
- AND the obligation MUST NOT be reported as supported
- AND explicit all-admitted-composition commutativity evidence MUST be able to establish it

#### Scenario: Required integration conditions are visible
- GIVEN an affected invariant requires an integration condition no composition participant provides
- WHEN the obligation is emitted
- THEN the condition MUST be listed with state `missing`
- AND the supplied conditions of the composition participants MUST be listed as `provided`

### Requirement: Four explicit compatibility outcomes

r[cairn.concurrent_footprints.outcomes] Cairn MUST distinguish `conflict`, `needs_joint_evidence`, `supported_within_declared_scope` and `unknown`, MUST NOT treat a satisfiability witness as preservation over all admitted compositions, and MUST NOT let an individually valid receipt discharge a combined-candidate obligation.

#### Scenario: A satisfiability witness is insufficient
- GIVEN joint evidence with witness scope and a supported proposition payload
- WHEN the obligation is evaluated
- THEN coexistence and invariant preservation MUST remain not established
- AND the evidence MUST be retained as insufficient with the witness-scope reason

#### Scenario: Joint evidence names the exact candidate
- GIVEN joint evidence that names the exact obligation, candidate identity, participants, admitted compositions, context and every affected claim
- WHEN the required propositions are supported with adequate scope
- THEN the outcome MUST be `supported_within_declared_scope`
- AND the basis MUST name the evidence and composition identities

#### Scenario: Individual receipts never discharge the combination
- GIVEN individually valid receipts for each participant and no joint evidence
- WHEN the obligation is evaluated
- THEN the outcome MUST be `needs_joint_evidence`
- AND every receipt MUST report `discharges_combined_obligation = false`

#### Scenario: Finite preservation alone is not support
- GIVEN the consumer predicate follows from the current guarantee over the declared finite universe
- WHEN no joint evidence exists for the combined candidate
- THEN the interaction MUST require joint evidence
- AND the outcome MUST NOT be supported

### Requirement: Deterministic explanation path

r[cairn.concurrent_footprints.explanation] Cairn MUST explain the deterministic path from the changed guarantee to the consuming assumption and the resulting obligation, including the finite check, retained witness presence, invalidation or invariant overlap, admitted compositions, evidence admission and the outcome basis.

#### Scenario: Path names the assumption meaning and identities
- GIVEN a changed-guarantee interaction
- WHEN the report is emitted
- THEN its path MUST name the previous and current guarantee identities, the consumed identity, the declared query meaning and dependency
- AND it MUST state the finite world and evaluation counts when a finite check ran
- AND it MUST end with the outcome and its basis

### Requirement: Prior obligation invalidation

r[cairn.concurrent_footprints.invalidation] Cairn MUST invalidate prior obligation applicability when participants, relevant dependencies, configuration or composition change, and MUST report the exact changed dimension.

#### Scenario: Unchanged fingerprint remains applicable
- GIVEN a prior obligation fingerprint identical to the current obligation
- WHEN the case is analyzed
- THEN the prior obligation MUST be applicable

#### Scenario: Each drift dimension is named
- GIVEN a changed participant revision, dependency identity, configuration identity or composition order
- WHEN the case is analyzed
- THEN the prior obligation MUST be inapplicable
- AND the report MUST name `participants_changed`, `dependencies_changed`, `configuration_changed` or `composition_changed` respectively
- AND an absent obligation id MUST report `obligation_absent`

### Requirement: Bounded deterministic exploration

r[cairn.concurrent_footprints.bounds] Cairn MUST take declaration and finite exploration bounds from the checked Nickel projection within fixed implementation ceilings, MUST use checked arithmetic for declaration counts, domain values, worlds and evaluation steps, and MUST produce identical identities and reports for identical admitted inputs.

#### Scenario: Bound exhaustion is explicit
- GIVEN a declared or policy bound that cannot cover the finite universe
- WHEN the analysis runs
- THEN the interaction MUST be `unknown` with the bound basis
- AND the report MUST retain the bound exhaustion
- AND no positive implication MUST be reported

#### Scenario: Identical inputs replay identically
- GIVEN identical case, policy and prior obligations
- WHEN the analysis runs twice
- THEN input, policy, candidate, obligation and report identities MUST be identical

### Requirement: Read-only advisory inspection

r[cairn.concurrent_footprints.inspection] Cairn MUST provide read-only `cairn footprints inspect --case FILE [--policy FILE] [--root DIR]` with bounded no-follow reads, a versioned advisory envelope and explicit rejection outcomes, and MUST NOT write lifecycle state, execute checks, transfer evidence or change a completion decision.

#### Scenario: Advisory report is printed
- GIVEN a valid case and policy
- WHEN the command runs
- THEN it MUST print the versioned report with `lifecycle_authority = none`
- AND a conflict or unknown outcome MUST remain a successful advisory report

#### Scenario: Invalid input is non-accepting
- GIVEN unknown options, a missing case, malformed JSON, an invalid policy or an unavailable default projection
- WHEN the command runs
- THEN it MUST exit nonzero with an explicit rejection
- AND it MUST NOT report a successful analysis

### Requirement: Existing reports remain independent

r[cairn.concurrent_footprints.independence] Cairn MUST keep textual collaboration conflict resolution and verification-impact reports independently usable with their own schemas and meanings, and the concurrent footprint report MUST NOT consume, replace or modify their decisions.

#### Scenario: Independent surfaces are retained
- GIVEN the concurrent footprint report
- WHEN a consumer inspects the available reports
- THEN the collaboration resolution and verification impact surfaces MUST still exist with their own identities
- AND the footprint report MUST state their independence and its own advisory boundary

### Requirement: Consumer-visible conformance corpus

r[cairn.concurrent_footprints.conformance] Cairn MUST retain positive and negative core and CLI cases for compatible, conflicting, order-sensitive, incomplete-declaration, unsupported-interpretation, bounded-exploration and malformed-input declarations, and MUST reject stale or tampered identities rather than accepting them.

#### Scenario: The retained corpus distinguishes each outcome
- GIVEN the retained core and CLI cases
- WHEN the corpus runs
- THEN each compatible, conflicting, order-sensitive and incomplete-declaration case MUST be observable
- AND malformed declarations, tampered identities, invalid compositions and invalid evidence MUST be rejected
