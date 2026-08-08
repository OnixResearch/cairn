# Collaboration Conflict Resolution Specification

## ADDED Requirements

### Requirement: Deterministic conflict inspection

r[cairn.collaboration_resolution.inspection] The system MUST deterministically inspect every current artifact and lifecycle conflict head while disclosing exact artifact content only for one explicitly requested bounded head.

#### Scenario: Conflict metadata is complete and sorted

r[cairn.collaboration_resolution.inspection.complete]
- GIVEN a validated collaboration state with one or more artifact or lifecycle conflicts
- WHEN Cairn inspects conflicts
- THEN every current conflicting head is reported in deterministic artifact and revision order with actor, deletion status, byte length, and BLAKE3 content identity or typed lifecycle value
- AND the inspection is bound to the current collaboration state identity

#### Scenario: One exact artifact head can be reviewed

r[cairn.collaboration_resolution.inspection.show]
- GIVEN a conflicted artifact and one explicitly named current head
- WHEN the operator requests exact head disclosure
- THEN Cairn returns only that bounded head value and its identity metadata
- AND unknown, stale, non-head, or non-conflicted selectors fail closed

### Requirement: Complete history-preserving resolution plan

r[cairn.collaboration_resolution.plan] The system MUST derive one deterministic resolution operation that explicitly selects one current head for every conflicted artifact, supersedes every competing current head, and resolves any lifecycle conflict only through an explicit conservative `open` reset.

#### Scenario: Selected heads supersede every conflict

r[cairn.collaboration_resolution.plan.complete]
- GIVEN a validated conflicted state and an authorized actor with exactly one current-head selection for every conflicted artifact
- WHEN Cairn plans resolution
- THEN each sorted artifact action copies the selected value and cites the complete current head set as its basis
- AND applying the operation produces conflict-free content with the advertised BLAKE3 state and content identities

#### Scenario: Lifecycle conflict returns explicitly to open

r[cairn.collaboration_resolution.plan.lifecycle]
- GIVEN lifecycle has multiple current heads and all content conflicts have explicit selections
- WHEN the operator explicitly selects lifecycle status `open`
- THEN the plan creates one `open` lifecycle revision for the resolved content root that supersedes every current lifecycle head and carries no approval evidence

#### Scenario: Incomplete or stale choices fail closed

r[cairn.collaboration_resolution.plan.blocked]
- GIVEN selections are missing, duplicated, extra, stale, not current heads, unsupported lifecycle values, unauthorized, policy-stale, change-stale, or applied to conflict-free state
- WHEN Cairn plans resolution
- THEN no partial operation or implicit winner is produced

### Requirement: Portable conflict resolution execution

r[cairn.collaboration_resolution.portable_execution] The system MUST resolve `portable-json.v1` state only through preview-first serialized execution that rechecks policy, current envelope identity, actor, every conflict head, and the complete resolution plan before validated atomic store replacement.

#### Scenario: Portable resolution replaces exact expected state

r[cairn.collaboration_resolution.portable_execution.success]
- GIVEN a valid conflicted portable store, matching creator declaration, and reviewed complete resolution preview
- WHEN the operator requests execution
- THEN Cairn applies the exact operation, validates a prepared envelope, atomically replaces only the expected store, and reloads the advertised conflict-free identities

#### Scenario: Portable drift or unsafe storage does not mutate

r[cairn.collaboration_resolution.portable_execution.blocked]
- GIVEN the store, policy, actor, conflict heads, selectors, lock state, prepared state, or storage safety changed after preview
- WHEN portable resolution execution is requested
- THEN Cairn reports deterministic blockers and leaves the trusted store unmodified

### Requirement: Radicle signed conflict resolution execution

r[cairn.collaboration_resolution.radicle_execution] The system MUST resolve a typed Radicle change object only after rechecking current creator-or-delegate authority, signer possession, object identity, state identity, policy, every conflict head, and the complete plan under serialized execution.

#### Scenario: Authorized Radicle resolution reloads exact state

r[cairn.collaboration_resolution.radicle_execution.success]
- GIVEN one conflicted typed Cairn change object, a current creator or delegate profile signer, and a drift-free complete preview
- WHEN the operator requests execution
- THEN Cairn creates one signed typed resolution, reloads the same object, verifies conflict-free state and the planned content identity, and records the observed result state identity

#### Scenario: Unauthorized or stale Radicle resolution is rejected

r[cairn.collaboration_resolution.radicle_execution.blocked]
- GIVEN the signer is neither creator nor current delegate, identity or object state changed, selectors became stale, multiple or missing objects exist, or policy drifted
- WHEN Radicle resolution execution is requested
- THEN Cairn rejects before a successful signed resolution and reports no successful mutation

### Requirement: Collaboration conflict receipts

r[cairn.collaboration_resolution.receipts] The system MUST emit deterministic BLAKE3-bound conflict inspection and resolution receipts identifying backend, change, workspace, authority observation, input and result identities, every conflicting head, explicit choices, complete revision bases, policy, execution mode, blockers, and mutation result.

#### Scenario: Preview and execution remain distinguishable

r[cairn.collaboration_resolution.receipts.modes]
- GIVEN the same valid complete conflict choices
- WHEN the operator previews and then executes resolution
- THEN preview reports `mutated=false` without provider writes
- AND execution reports whether the exact reviewed operation was applied or blocked

### Requirement: Bounded conflict-resolution claims

r[cairn.collaboration_resolution.non_claims] The system MUST state that head inspection or local provider resolution does not prove semantic merge correctness, losing-head invalidity, lifecycle approval, evidence truth, network publication, peer convergence, remote trust, source correctness, archive execution, or release eligibility.

#### Scenario: Local resolution is not convergence proof

r[cairn.collaboration_resolution.non_claims.local]
- GIVEN a passing portable or Radicle resolution receipt
- WHEN Cairn describes the result
- THEN the claim is limited to the recorded local DAG operation and provider observations
- AND the receipt MUST NOT promote an explicit head choice into semantic correctness, remote availability, or peer agreement
