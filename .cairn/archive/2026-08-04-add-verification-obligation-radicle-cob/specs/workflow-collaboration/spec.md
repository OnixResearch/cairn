# Workflow Collaboration Specification Delta

## ADDED Requirements

### Requirement: Workflow collaboration policy is typed

r[cairn.workflow_collaboration.policy] Cairn MUST define typed Nickel profiles for supported workflow kinds, field rules, role rules, evidence roles, lifecycle transitions, conflict rules, decision rules, and named protocol bounds.

#### Scenario: Policy export is deterministic

r[cairn.workflow_collaboration.policy.deterministic]
- GIVEN a valid workflow collaboration policy
- WHEN Cairn exports the runtime projection
- THEN the generated workflow profile MUST be deterministic and bound to the policy identity.

#### Scenario: Invalid profile fails before runtime use

r[cairn.workflow_collaboration.policy.invalid]
- GIVEN a workflow profile has duplicate kinds, unknown roles, invalid transitions, unnamed limits, impossible decision thresholds, or conflicting field rules
- WHEN Cairn validates the Nickel source
- THEN export MUST fail and MUST NOT replace the generated runtime profile.

### Requirement: Workflow collaboration has a provider-neutral semantic core

r[cairn.workflow_collaboration.semantic_core] Cairn MUST represent collaborative workflow state with a kind, stable workflow ID, workspace identity, typed base revision, policy hash, root-bound roles, typed field revisions, evidence references, lifecycle facts, conflicts, and BLAKE3 identities without requiring provider identities.

#### Scenario: Neutral state needs no provider context

r[cairn.workflow_collaboration.semantic_core.neutral]
- GIVEN valid bounded workflow inputs and explicit authority facts
- WHEN Cairn initializes and reduces neutral workflow state
- THEN no Radicle profile, repository ID, COB object ID, Git-only field, node, transport input, filesystem, clock, or environment value MUST be required.

#### Scenario: Provider identity cannot replace semantic identity

r[cairn.workflow_collaboration.semantic_core.identity]
- GIVEN equivalent neutral workflow state is stored by different providers
- WHEN Cairn derives content and state identities
- THEN both providers MUST produce equal BLAKE3 semantic identities independent of provider object and operation IDs.

### Requirement: Workflow reduction is deterministic and conflict-preserving

r[cairn.workflow_collaboration.reducer] Cairn MUST reduce valid workflow operations atomically and deterministically, and it MUST preserve concurrent field or lifecycle heads instead of selecting a winner by timestamp, arrival order, provider identity, or local presentation order.

#### Scenario: Disjoint operations converge

r[cairn.workflow_collaboration.reducer.disjoint]
- GIVEN valid concurrent operations affect disjoint workflow fields
- WHEN peers evaluate the operations in different valid orders
- THEN they MUST derive equal reduced state and equal BLAKE3 state identity.

#### Scenario: Concurrent field revisions remain visible

r[cairn.workflow_collaboration.reducer.conflict]
- GIVEN concurrent operations replace the same current field revision without superseding each other
- WHEN the reducer evaluates both operations
- THEN both heads MUST remain visible as an explicit conflict and neither head MUST become an implicit winner.

#### Scenario: Complete resolution preserves history

r[cairn.workflow_collaboration.reducer.resolve]
- GIVEN an authorized resolution names every current head for each conflicted field
- WHEN the reducer applies the resolution
- THEN one new head MUST supersede all named heads and all prior revisions MUST remain in history.

#### Scenario: Invalid operation is atomic

r[cairn.workflow_collaboration.reducer.invalid]
- GIVEN an operation has a duplicate ID, repeated action, unknown basis, mixed workflow kind, unauthorized actor, malformed threshold fact, or protocol-bound violation
- WHEN the reducer evaluates the operation
- THEN it MUST reject the complete operation without returning partially updated trusted state.

### Requirement: Workflow evidence references preserve roles and scope

r[cairn.workflow_collaboration.evidence] Cairn MUST represent external evidence with a stable schema or media kind, BLAKE3 identity when Cairn owns the identity, preserved external identity metadata when interoperability requires it, `VerificationRole`, claim scope, producer declaration, safe locator, and explicit non-claims.

#### Scenario: Evidence reference remains opaque

r[cairn.workflow_collaboration.evidence.opaque]
- GIVEN a workflow operation attaches a valid external evidence reference
- WHEN the pure reducer admits the operation
- THEN it MUST preserve the declared identity, role, scope, and locator without fetching, executing, parsing, or promoting the referenced payload.

#### Scenario: Recorded-only evidence cannot become property evidence

r[cairn.workflow_collaboration.evidence.role]
- GIVEN an evidence reference is classified as `RecordedOnly`, `Boundary`, or `Unknown`
- WHEN a workflow decision evaluates evidence requirements
- THEN the reference MUST NOT satisfy a policy requirement for `Property` evidence.

#### Scenario: External algorithms stay bounded

r[cairn.workflow_collaboration.evidence.hash]
- GIVEN an interoperability reference uses a non-BLAKE3 external identity
- WHEN Cairn includes it in workflow state
- THEN Cairn MUST preserve the external algorithm as typed metadata and MUST derive the Cairn-owned reference identity with BLAKE3.

### Requirement: Portable workflow collaboration preserves semantic identity

r[cairn.workflow_collaboration.portable] Cairn MUST support a deterministic BLAKE3-bound portable JSON envelope for every supported workflow profile before that profile requires a Radicle COB backend.

#### Scenario: Portable round trip preserves semantic identity

r[cairn.workflow_collaboration.portable.roundtrip]
- GIVEN valid conflict-free or conflicted neutral workflow state
- WHEN Cairn serializes and reloads the portable envelope
- THEN exact state, conflicts, content identity, state identity, schema, kind, and backend label MUST remain unchanged.

#### Scenario: Tampered portable state fails closed

r[cairn.workflow_collaboration.portable.tamper]
- GIVEN a portable envelope has changed state, hashes, profile, kind, schema, or backend label
- WHEN Cairn validates the envelope
- THEN validation MUST fail without returning trusted workflow state.

### Requirement: Direct Radicle COB remains an optional workflow adapter

r[cairn.workflow_collaboration.radicle_adapter] Cairn MUST expose supported workflow types through direct `radicle-cob` storage traits over one explicit Git repository without moving repository effects into the pure core or requiring a Radicle runtime.

#### Scenario: Signed entry maps to pure inputs

r[cairn.workflow_collaboration.radicle_adapter.mapping]
- GIVEN a signed COB entry has a supported type, schema, action encoding, actor, operation ID, and root-bound authority
- WHEN Cairn evaluates the entry
- THEN the adapter MUST pass only deterministic admitted in-memory values to the matching workflow reducer.

#### Scenario: Existing COB compatibility remains stable

r[cairn.workflow_collaboration.radicle_adapter.compatibility]
- GIVEN supported historical Cairn change and document fixtures
- WHEN shared storage code is generalized for workflows
- THEN existing refs, signed commits, action bytes, canonical hashes, reduced states, and fixture identities MUST remain unchanged.

#### Scenario: Radicle runtime remains absent

r[cairn.workflow_collaboration.radicle_adapter.boundary]
- GIVEN a supported workflow uses the direct COB backend
- WHEN product dependencies and commands are audited
- THEN the umbrella `radicle` crate, `radicle-node`, profiles, identity documents, discovery, synchronization, seeding, and transport behavior MUST remain absent.

### Requirement: Workflow receipts distinguish semantics from observations

r[cairn.workflow_collaboration.receipts] Cairn MUST emit deterministic BLAKE3-bound workflow receipts that identify semantic state, provider observations, authority facts, evidence references, conflicts, policy, execution mode, blockers, mutation results, and bounded non-claims as separate fields.

#### Scenario: Provider success does not broaden semantic claims

r[cairn.workflow_collaboration.receipts.provider]
- GIVEN a portable or direct-COB workflow operation succeeds
- WHEN Cairn emits its receipt
- THEN the receipt MUST distinguish semantic identities from provider IDs and MUST NOT claim evidence truth, remote publication, synchronization, convergence, or lifecycle correctness.

#### Scenario: Preview and execution remain distinct

r[cairn.workflow_collaboration.receipts.execution]
- GIVEN an operator reviews a workflow mutation plan and later requests execution
- WHEN Cairn emits both receipts
- THEN each receipt MUST state its execution mode, input identity, plan identity, recheck result, and whether mutation occurred.

### Requirement: Workflow collaboration validation includes failure evidence

r[cairn.workflow_collaboration.validation] Cairn MUST validate each workflow profile with positive and negative reducer, authority, evidence, conflict, backend, shell, receipt, and interoperability fixtures.

#### Scenario: Invalid workflow behavior fails closed

r[cairn.workflow_collaboration.validation.negative]
- GIVEN malformed state, unauthorized actions, stale bases, conflicts, mixed types, evidence-role promotion, unsafe paths, tampered history, or provider drift
- WHEN the maintained workflow validation suite runs
- THEN each invalid case MUST fail with deterministic diagnostics and MUST NOT report successful mutation.
