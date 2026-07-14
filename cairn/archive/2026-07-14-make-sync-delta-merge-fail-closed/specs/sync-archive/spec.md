## ADDED Requirements

### Requirement: Sync delta semantic preflight

r[cairn.sync_archive.delta_merge.preflight] Cairn MUST parse every sync source into a non-empty deterministic set of explicit ADDED, MODIFIED, or REMOVED requirement operations before authorizing mutation.

#### Scenario: Explicit delta operations pass preflight

r[cairn.sync_archive.delta_merge.preflight.valid]
- GIVEN a delta document with supported operation sections and well-formed requirement blocks
- WHEN Cairn builds the sync plan
- THEN the plan MUST inventory the ordered operations and their stable identities

#### Scenario: Full-spec-shaped input fails closed

r[cairn.sync_archive.delta_merge.preflight.full_spec]
- GIVEN a change spec contains an accepted-spec title and `## Requirements` body but no supported delta operation section
- WHEN Cairn plans or executes sync
- THEN Cairn MUST report that no delta operations were recognized
- AND it MUST NOT claim that the proposed requirements were promoted

#### Scenario: Empty or ambiguous operation set fails closed

r[cairn.sync_archive.delta_merge.preflight.empty]
- GIVEN a delta is empty, has an empty operation section, contains only fenced example headings, or has an unsupported or ambiguous section shape
- WHEN Cairn builds the sync plan
- THEN the plan MUST be blocked with deterministic repair guidance

### Requirement: Requirement identity controls sync merge

r[cairn.sync_archive.delta_merge.identity] Cairn MUST resolve accepted and delta requirement blocks by one canonical bare requirement ID per block and MUST reject identity collisions before mutation.

#### Scenario: Renamed modified requirement replaces by ID

r[cairn.sync_archive.delta_merge.identity.rename]
- GIVEN an accepted requirement and a MODIFIED operation share one requirement ID but use different headings
- WHEN Cairn applies the operation
- THEN the old block MUST be replaced by the new block
- AND the accepted spec MUST contain exactly one block with that requirement ID

#### Scenario: Duplicate identity blocks the plan

r[cairn.sync_archive.delta_merge.identity.duplicate]
- GIVEN accepted content or delta operations contain the same canonical requirement ID more than once
- WHEN Cairn builds the sync plan
- THEN the plan MUST report every conflicting location and MUST NOT write any destination

#### Scenario: Missing or multiple block identities fail

r[cairn.sync_archive.delta_merge.identity.malformed]
- GIVEN a requirement operation has no bare requirement ID or has more than one candidate bare requirement ID
- WHEN Cairn parses the delta
- THEN the operation MUST be rejected with a deterministic identity diagnostic

### Requirement: Delta operations have fail-closed idempotent semantics

r[cairn.sync_archive.delta_merge.operations] Cairn MUST enforce deterministic target-cardinality, conflict, and already-applied rules for ADDED, MODIFIED, and REMOVED operations.

#### Scenario: Add conflict is rejected

r[cairn.sync_archive.delta_merge.operations.add]
- GIVEN an ADDED operation targets an existing requirement ID with different content
- WHEN Cairn builds the sync plan
- THEN the plan MUST report an add conflict and MUST preserve the existing block

#### Scenario: Unknown modification is rejected

r[cairn.sync_archive.delta_merge.operations.modify]
- GIVEN a MODIFIED operation targets no accepted requirement ID
- WHEN Cairn builds the sync plan
- THEN the plan MUST report an unknown modification target and MUST NOT append the block as a new requirement

#### Scenario: Repeated operation is an explicit no-op

r[cairn.sync_archive.delta_merge.operations.idempotent]
- GIVEN an operation's desired result is already present or its removal target is already absent
- WHEN Cairn replays the same reviewed delta
- THEN the plan MUST report the operation as already applied without changing bytes

### Requirement: Sync receipts bind merge inputs and results

r[cairn.sync_archive.delta_merge.receipt_binding] Cairn MUST bind delta content, accepted pre-state, normalized operations, expected result identities, and policy identity into portable sync input, plan, and receipt hashes.

#### Scenario: Changed delta bytes change plan identity

r[cairn.sync_archive.delta_merge.receipt_binding.delta]
- GIVEN two sync inputs have the same paths and task metadata but different delta requirement bytes
- WHEN Cairn computes their dry-run plans
- THEN their input or plan hashes MUST differ

#### Scenario: Changed accepted pre-state changes plan identity

r[cairn.sync_archive.delta_merge.receipt_binding.pre_state]
- GIVEN the same delta is evaluated against two different accepted-spec pre-states
- WHEN Cairn computes their dry-run plans
- THEN their input or plan hashes MUST differ and each plan MUST bind its expected after identity

### Requirement: Sync mutation reporting reflects byte effects

r[cairn.sync_archive.delta_merge.mutation_truth] Cairn MUST perform no sync write for a semantically blocked plan and MUST report mutation only when at least one destination's bytes actually change.

#### Scenario: Blocked semantic plan is a no-op

r[cairn.sync_archive.delta_merge.mutation_truth.blocked]
- GIVEN any delta, identity, or target-cardinality blocker exists
- WHEN the operator requests sync execution
- THEN Cairn MUST report `blocked=true` and `mutated=false`
- AND every accepted spec MUST retain its before hash

#### Scenario: Byte-identical result is not mutation

r[cairn.sync_archive.delta_merge.mutation_truth.identical]
- GIVEN every operation is already applied and every computed destination is byte-identical to its pre-state
- WHEN the operator requests sync execution
- THEN Cairn MUST report `mutated=false` and MUST NOT manufacture a changed-file manifest entry

### Requirement: Sync delta merge regressions

r[cairn.sync_archive.delta_merge.tests] Cairn MUST verify delta preflight, identity merge, operation semantics, receipt binding, and mutation truth with positive and negative tests.

#### Scenario: Positive fixtures pass

r[cairn.sync_archive.delta_merge.tests.positive]
- GIVEN valid add, modify, remove, heading-rename, new-spec, multi-spec, and already-applied fixtures
- WHEN focused core and CLI tests run
- THEN every fixture MUST produce the expected deterministic document, operation status, and receipt hashes

#### Scenario: Negative fixtures fail closed

r[cairn.sync_archive.delta_merge.tests.negative]
- GIVEN full-spec-only, empty, malformed, duplicate-ID, unknown-target, conflicting-add, malformed-remove, or fenced-fake-heading fixtures
- WHEN focused core and CLI tests run
- THEN every fixture MUST report its expected diagnostic and MUST preserve all destination bytes
