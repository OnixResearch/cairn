# Document Collaboration Specification

## Purpose

Defines the `document-collaboration` capability.

## Requirements

### Requirement: Document collaboration policy is typed

r[cairn.document_collaboration.policy] Cairn MUST define typed Nickel profiles for supported document kinds, artifact rules, role rules, lifecycle transitions, publication rules, link relations, discussion fields, and named protocol bounds.

#### Scenario: Policy export is deterministic

r[cairn.document_collaboration.policy.deterministic]
- GIVEN a valid document collaboration policy
- WHEN Cairn exports the runtime projection
- THEN the generated document profile MUST be deterministic and bound to the policy identity.

#### Scenario: Invalid profile fails before runtime use

r[cairn.document_collaboration.policy.invalid]
- GIVEN a document profile has duplicate kinds, unsafe paths, unknown roles, invalid transitions, unnamed limits, or conflicting publication rules
- WHEN Cairn validates the Nickel source
- THEN export MUST fail and MUST NOT replace the generated runtime profile.

### Requirement: Document collaboration has a transport-neutral semantic core

r[cairn.document_collaboration.semantic_core] Cairn MUST represent collaborative document state with a kind, stable document ID, workspace identity, typed base revision, policy hash, exact artifacts, typed links, discussion, lifecycle, evidence, conflicts, and explicit role facts without requiring provider identities.

#### Scenario: Neutral state needs no Radicle context

r[cairn.document_collaboration.semantic_core.neutral]
- GIVEN valid bounded document inputs and explicit authority facts
- WHEN Cairn initializes and reduces neutral document state
- THEN no Radicle profile, repository ID, COB object ID, Git-only field, node, or transport input MUST be required.

#### Scenario: Provider identity cannot replace semantic identity

r[cairn.document_collaboration.semantic_core.identity]
- GIVEN equivalent neutral state is stored by different providers
- WHEN Cairn derives document content and state identities
- THEN both providers MUST produce equal BLAKE3 semantic identities independent of provider object IDs.

### Requirement: Document reduction is deterministic and conflict-preserving

r[cairn.document_collaboration.reducer] Cairn MUST reduce valid document operations deterministically and MUST preserve concurrent heads instead of selecting a winner by timestamp, arrival order, or provider identity.

#### Scenario: Disjoint actions converge

r[cairn.document_collaboration.reducer.disjoint]
- GIVEN valid concurrent operations affect different artifact, discussion, link, or lifecycle fields
- WHEN peers evaluate the operations in different valid orders
- THEN they MUST derive equal reduced state and equal BLAKE3 state identity.

#### Scenario: Concurrent artifact edits remain visible

r[cairn.document_collaboration.reducer.conflict]
- GIVEN concurrent operations replace the same artifact revision without superseding each other
- WHEN the reducer evaluates both operations
- THEN both heads MUST remain visible as an explicit conflict and neither head MUST become an implicit winner.

#### Scenario: Complete resolution preserves history

r[cairn.document_collaboration.reducer.resolve]
- GIVEN an authorized resolution names every current head for each conflicted field
- WHEN the reducer applies the resolution
- THEN one new head MUST supersede all named heads and the prior revisions MUST remain in history.

#### Scenario: Invalid operation is atomic

r[cairn.document_collaboration.reducer.invalid]
- GIVEN an operation has a duplicate ID, repeated action, unknown basis, mixed document kind, unauthorized actor, or protocol-bound violation
- WHEN the reducer evaluates the operation
- THEN it MUST reject the complete operation without returning partially updated trusted state.

### Requirement: Portable document collaboration preserves semantic identity

r[cairn.document_collaboration.portable] Cairn MUST support a deterministic BLAKE3-bound portable JSON envelope for every supported document profile before that profile requires a Radicle COB backend.

#### Scenario: Portable round trip preserves state

r[cairn.document_collaboration.portable.roundtrip]
- GIVEN valid conflict-free or conflicted neutral document state
- WHEN Cairn serializes and reloads the portable envelope
- THEN exact state, conflicts, content identity, state identity, schema, and backend label MUST remain unchanged.

#### Scenario: Tampered envelope is rejected

r[cairn.document_collaboration.portable.tamper]
- GIVEN a portable envelope has changed state, hashes, profile, kind, schema, or backend label
- WHEN Cairn validates the envelope
- THEN validation MUST fail without returning trusted document state.

### Requirement: Direct Radicle COB remains an optional adapter

r[cairn.document_collaboration.radicle_adapter] Cairn MUST expose supported document types through direct `radicle-cob` storage traits over one explicit Git repository without moving repository effects into the pure core or requiring a Radicle runtime.

#### Scenario: Signed entry maps to pure inputs

r[cairn.document_collaboration.radicle_adapter.mapping]
- GIVEN a signed COB entry has a supported type, schema, action encoding, actor, operation ID, and root-bound authority
- WHEN Cairn evaluates the entry
- THEN the adapter MUST pass only deterministic admitted in-memory values to the matching document reducer.

#### Scenario: Existing Cairn change compatibility remains stable

r[cairn.document_collaboration.radicle_adapter.compatibility]
- GIVEN a supported historical `xyz.onixresearch.cairn-change` object and existing compatibility fixture
- WHEN shared storage code is generalized for documents
- THEN existing change refs, signed commits, action bytes, canonical hashes, reduced state, and fixture identities MUST remain unchanged.

#### Scenario: Radicle runtime remains absent

r[cairn.document_collaboration.radicle_adapter.boundary]
- GIVEN a supported document uses the direct COB backend
- WHEN product dependencies and commands are audited
- THEN the umbrella `radicle` crate, `radicle-node`, profiles, identity documents, discovery, synchronization, and transport behavior MUST remain absent.

### Requirement: Document collaboration validation includes negative evidence

r[cairn.document_collaboration.validation] Cairn MUST validate each document profile with positive and negative reducer, authority, backend, materialization, receipt, and interoperability fixtures.

#### Scenario: Failure behavior is maintained

r[cairn.document_collaboration.validation.negative]
- GIVEN malformed state, unauthorized actions, stale bases, conflicts, unsafe paths, mixed types, tampered history, or provider drift
- WHEN the maintained document validation suite runs
- THEN each invalid case MUST fail with deterministic diagnostics and MUST NOT report successful mutation.
