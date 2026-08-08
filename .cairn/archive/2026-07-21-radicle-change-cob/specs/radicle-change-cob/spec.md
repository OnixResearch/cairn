# Radicle Change COB Specification

## Purpose

Defines the lossless Cairn active-change state, deterministic collaborative reducer, Radicle custom COB adapter, semantic identity, materialization boundary, and bounded evidence claims.

## ADDED Requirements

### Requirement: Lossless active-change state
r[cairn.radicle_change_cob.lossless_change_state] The system MUST represent one active Cairn change as a versioned state that preserves proposal, design, tasks, metadata, and keyed delta-spec text together with repository, base-Git, creator, and policy identities.

#### Scenario: Root state preserves lifecycle artifacts
r[cairn.radicle_change_cob.lossless_change_state.preserve]
- GIVEN a valid active change contains proposal, design, tasks, metadata, and multiple delta specs
- WHEN Cairn initializes the collaborative change state
- THEN the state MUST retain the exact UTF-8 content and stable artifact identity of every supplied lifecycle artifact.

#### Scenario: Malformed or oversized root fails closed
r[cairn.radicle_change_cob.lossless_change_state.invalid]
- GIVEN a root action contains an invalid change ID, unsafe artifact ID, unsupported schema, empty required identity, or content beyond a named protocol limit
- WHEN the reducer initializes the object
- THEN initialization MUST fail with a deterministic error and MUST NOT produce a partial state.

### Requirement: Deterministic conflict-preserving reducer
r[cairn.radicle_change_cob.deterministic_reducer] The system MUST reduce active-change operations deterministically without using last-writer-wins for concurrent edits to the same artifact.

#### Scenario: Disjoint edits converge
r[cairn.radicle_change_cob.deterministic_reducer.disjoint]
- GIVEN two valid concurrent operations edit different artifact IDs from the same state
- WHEN peers evaluate those operations in different valid arrival orders
- THEN both peers MUST derive equal conflict-free state and equal BLAKE3 state identity.

#### Scenario: Concurrent same-artifact edits remain visible
r[cairn.radicle_change_cob.deterministic_reducer.conflict]
- GIVEN two valid operations supersede the same artifact revision without superseding each other
- WHEN the reducer evaluates both operations
- THEN the artifact MUST retain both heads as an explicit conflict and MUST NOT select a winner from timestamps or operation arrival order.

#### Scenario: Resolution supersedes every conflict head
r[cairn.radicle_change_cob.deterministic_reducer.resolve]
- GIVEN an artifact has multiple current heads
- WHEN an authorized operation supplies every head as its revision basis
- THEN the reducer MUST derive one resolved head and MUST preserve the superseded revisions in history.

#### Scenario: Invalid operations are rejected
r[cairn.radicle_change_cob.deterministic_reducer.invalid]
- GIVEN an operation reuses an operation ID, repeats an artifact action, cites an unknown basis, or exceeds a named protocol limit
- WHEN the reducer evaluates the operation
- THEN the reducer MUST reject the whole operation without partially mutating state.

### Requirement: Subject-bound evidence and lifecycle claims
r[cairn.radicle_change_cob.evidence_boundary] The system MUST bind evidence and lifecycle assertions to a conflict-free BLAKE3 content root while preserving explicit non-claims.

#### Scenario: Current evidence is distinguishable from stale evidence
r[cairn.radicle_change_cob.evidence_boundary.stale]
- GIVEN evidence names a valid BLAKE3 subject root
- WHEN lifecycle content changes after the evidence is attached
- THEN Cairn MUST retain the evidence record but MUST distinguish it from evidence bound to the current content root.

#### Scenario: Signed history does not promote lifecycle claims
r[cairn.radicle_change_cob.evidence_boundary.non_claims]
- GIVEN a signed operation attaches a gate, review, sync, archive, or release receipt declaration
- WHEN the collaborative state accepts the declaration shape
- THEN Cairn MUST NOT claim that signature validity or COB presence proves evidence truth, policy conformance, source correctness, archive execution, or release readiness.

#### Scenario: Authority is explicit
r[cairn.radicle_change_cob.evidence_boundary.authority]
- GIVEN a signed actor attempts to edit lifecycle content or advance a lifecycle assertion
- WHEN the reducer evaluates explicit creator and repository-delegate authority facts
- THEN content edits MUST require creator or delegate authority and approval or archive assertions MUST require delegate authority.

### Requirement: Native Radicle custom object adapter
r[cairn.radicle_change_cob.radicle_adapter] The system MUST expose the active-change reducer as the Radicle custom object type `xyz.onixresearch.cairn-change` without moving repository access into the pure core.

#### Scenario: Radicle operations map into pure reducer inputs
r[cairn.radicle_change_cob.radicle_adapter.mapping]
- GIVEN a signed Radicle entry has a valid manifest, action encoding, actor, operation ID, and identity-document reference
- WHEN the custom object adapter initializes or updates a change
- THEN the adapter MUST derive explicit authority facts and pass only deterministic in-memory inputs to the pure reducer.

#### Scenario: Invalid adapter inputs prune their branch
r[cairn.radicle_change_cob.radicle_adapter.reject]
- GIVEN an entry has malformed actions, a missing identity document, unauthorized content mutation, or invalid reducer input
- WHEN Radicle evaluates the custom object history
- THEN the adapter MUST return an error so the invalid operation branch is not materialized as accepted state.

#### Scenario: Aggregate dependencies remain external
r[cairn.radicle_change_cob.radicle_adapter.dependencies]
- GIVEN one change object declares dependencies on other active changes
- WHEN its local reducer evaluates the object
- THEN the reducer MUST preserve the declarations without claiming global existence, acyclicity, readiness, or archival satisfaction, and the Cairn repository shell MUST retain aggregate graph validation authority.

### Requirement: Deterministic native materialized view
r[cairn.radicle_change_cob.materialized_view] The system MUST derive a deterministic native Cairn change file plan from conflict-free collaborative state.

#### Scenario: Conflict-free state maps to native paths
r[cairn.radicle_change_cob.materialized_view.paths]
- GIVEN a conflict-free collaborative change has current artifacts
- WHEN Cairn plans materialization
- THEN the plan MUST sort and map artifacts only to `proposal.md`, `design.md`, `tasks.md`, `metadata.json`, or `specs/<stable-id>/spec.md` beneath the selected change directory with exact content.

#### Scenario: Conflicted state cannot materialize
r[cairn.radicle_change_cob.materialized_view.conflict]
- GIVEN one or more artifacts have multiple current revision heads
- WHEN Cairn plans materialization
- THEN planning MUST fail with deterministic conflict details and MUST NOT produce a partial write plan.
