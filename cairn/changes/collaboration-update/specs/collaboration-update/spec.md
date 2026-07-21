# Collaboration Update Specification

## Purpose

Defines deterministic preview-first artifact updates for backend-neutral Cairn collaborative changes without implicit conflict resolution or overstated provider authority.

## ADDED Requirements

### Requirement: Deterministic collaboration update plan

r[cairn.collaboration_update.plan] The system MUST deterministically compare exact local active-change artifacts with a validated conflict-free collaboration state and produce either an idempotent no-op or sorted artifact actions that cite every current head they supersede.

#### Scenario: Exact additions, edits, and removals are planned

r[cairn.collaboration_update.plan.exact]
- GIVEN a valid collaboration state and an authorized actor with a bounded exact local active-change snapshot
- WHEN local artifacts have been added, edited, or removed
- THEN the plan contains one deterministic action per changed artifact with exact desired content and complete current revision bases
- AND applying the planned operation derives the advertised BLAKE3 content identity

#### Scenario: Equal state is an idempotent no-op

r[cairn.collaboration_update.plan.noop]
- GIVEN local artifacts are byte-equivalent to the conflict-free collaboration state
- WHEN Cairn plans an update
- THEN the plan reports no actions and MUST NOT require provider mutation

#### Scenario: Unsafe update inputs fail closed

r[cairn.collaboration_update.plan.blocked]
- GIVEN the change or policy identity is stale, the actor lacks creator-or-delegate authority, a content or lifecycle head is conflicted, or an artifact violates a protocol bound
- WHEN Cairn plans an update
- THEN planning fails without selecting a conflict winner or producing a partial operation

### Requirement: Portable update execution

r[cairn.collaboration_update.portable_execution] The system MUST update `portable-json.v1` state only through preview-first serialized execution that rechecks policy, exact local artifacts, current envelope identity, and the complete update plan before validated atomic store replacement.

#### Scenario: Portable update replaces expected state

r[cairn.collaboration_update.portable_execution.success]
- GIVEN a valid portable store, matching local change, creator actor, and reviewed update preview
- WHEN the operator requests execution
- THEN Cairn applies the exact planned operation, validates a prepared envelope, atomically replaces the expected store, and reloads the advertised result identities

#### Scenario: Portable drift and unsafe storage do not mutate

r[cairn.collaboration_update.portable_execution.blocked]
- GIVEN the store or local artifacts changed after preview, the actor is not the creator, a lock or prepared file already exists, or the store path is unsafe
- WHEN portable update execution is requested
- THEN Cairn reports deterministic blockers and leaves the trusted store unmodified

### Requirement: Radicle signed update execution

r[cairn.collaboration_update.radicle_execution] The system MUST update a typed Radicle change object only after rechecking current creator-or-delegate authority, signer possession, object identity, state identity, policy, and exact local artifacts under serialized execution.

#### Scenario: Authorized Radicle update reloads exact content

r[cairn.collaboration_update.radicle_execution.success]
- GIVEN one typed Cairn change object, a current creator or delegate profile signer, and a drift-free reviewed update plan
- WHEN the operator requests execution
- THEN Cairn creates a signed typed update, reloads the same object, and verifies that its content identity matches the preview

#### Scenario: Unauthorized or stale Radicle update is rejected

r[cairn.collaboration_update.radicle_execution.blocked]
- GIVEN the signer is neither creator nor current delegate, the identity or object state changed, multiple or missing objects exist, or local artifacts or policy drifted
- WHEN Radicle update execution is requested
- THEN Cairn rejects before a signed update and reports no successful mutation

### Requirement: Collaboration update receipts

r[cairn.collaboration_update.receipts] The system MUST emit deterministic BLAKE3-bound update receipts identifying backend, change, workspace, actor authority observation, input state and content identities, policy, changed artifacts and revision bases, expected result content, execution mode, blockers, and mutation result.

#### Scenario: Preview and execution remain distinguishable

r[cairn.collaboration_update.receipts.modes]
- GIVEN the same valid collaboration update inputs
- WHEN the operator previews and then executes the update
- THEN preview reports `mutated=false` without provider writes
- AND execution reports whether the exact reviewed operation was applied or blocked

### Requirement: Bounded collaboration update claims

r[cairn.collaboration_update.non_claims] The system MUST state that update planning or local provider mutation does not prove conflict resolution, network publication, peer synchronization, remote trust, gate validity, evidence truth, dependency readiness, source correctness, archive execution, or release eligibility.

#### Scenario: Local update is not remote collaboration proof

r[cairn.collaboration_update.non_claims.local]
- GIVEN a passing portable or Radicle update receipt
- WHEN Cairn describes the result
- THEN the claim is limited to the recorded local semantic and provider observations
- AND the receipt MUST NOT promote caller-declared portable identity or local signed Radicle storage into peer availability or lifecycle approval
