# Cairn Bounded Tree adoption

## ADDED Requirements

### Requirement: Completed immutable prerequisite

r[cairn.bounded_tree_adoption.prerequisite] Cairn MUST adopt only a completed `bounded-tree` release pinned to one immutable Radicle revision.

#### Scenario: Reviewed dependency is available
- GIVEN passing `bounded-tree` completion evidence and one immutable Radicle revision
- WHEN Cairn dependency admission runs
- THEN the exact revision MUST be accepted without a sibling path or fallback

#### Scenario: Shared repository is incomplete
- GIVEN a failed gate, unchecked implementation task, mutable revision, or missing Radicle source
- WHEN admission runs
- THEN adoption MUST remain blocked

### Requirement: Archive scan compatibility

r[cairn.bounded_tree_adoption.archive_scan] Cairn MUST preserve accepted archive file paths and bytes through shared bounded observations.

#### Scenario: Valid archive is scanned
- GIVEN a valid archive candidate
- WHEN old and shared-backed scans run
- THEN their ordered archive file inputs MUST match

#### Scenario: Archive source changes
- GIVEN a candidate that changes during observation
- WHEN the shared-backed scan runs
- THEN Cairn MUST emit a bounded source-change issue without accepting stale bytes

### Requirement: Archive policy compatibility

r[cairn.bounded_tree_adoption.archive_policy] Cairn MUST retain depth, file-count, per-file, total-byte, link, special-file, and issue-ordering policy.

#### Scenario: Policy blocker occurs
- GIVEN a candidate that violates one maintained archive bound or file-kind rule
- WHEN both scans run
- THEN their stable issue identifiers and deterministic order MUST match

#### Scenario: Shared defaults are proposed
- GIVEN an adapter that replaces Cairn limits or issue policy with shared defaults
- WHEN boundary review runs
- THEN the adapter MUST be rejected

### Requirement: Pure core remains effect free

r[cairn.bounded_tree_adoption.core_boundary] Filesystem traversal from `bounded-tree` MUST remain in `cairn-cli` and MUST NOT enter `cairn-core`.

#### Scenario: CLI loads archive facts
- GIVEN accepted shared observations
- WHEN the CLI maps them into archive inputs
- THEN `cairn-core` MUST receive owned loaded values only

#### Scenario: Core imports the filesystem shell
- GIVEN a dependency path from `cairn-core` to `bounded-tree-cap`
- WHEN the purity gate runs
- THEN the gate MUST fail

### Requirement: Positive and negative parity gate

r[cairn.bounded_tree_adoption.parity] Cairn SHALL remove local scan mechanics only after valid and invalid fixture parity passes.

#### Scenario: Complete parity passes
- GIVEN matching archive inputs and issue results
- WHEN cutover readiness is evaluated
- THEN duplicated scan mechanics MAY be removed

#### Scenario: Any parity case differs
- GIVEN an input, issue identifier, or ordering mismatch
- WHEN cutover readiness is evaluated
- THEN local scan mechanics MUST remain active

### Requirement: Cairn evidence boundary

r[cairn.bounded_tree_adoption.boundary] Cairn MUST retain lifecycle policy, receipt meaning, evidence truth review, and release decisions.

#### Scenario: Dependency observation is recorded
- GIVEN a passing shared scan
- WHEN Cairn writes lifecycle evidence
- THEN it MAY record the exact dependency and bounded observation result

#### Scenario: Dependency result proves release eligibility
- GIVEN evidence that promotes a shared scan to lifecycle correctness or release eligibility
- WHEN evidence validation runs
- THEN validation MUST fail

### Requirement: Coupled rollback

r[cairn.bounded_tree_adoption.rollback] Cairn MUST record a rollback that restores prior scan code and dependency state together.

#### Scenario: Adoption regression appears
- GIVEN a post-cutover archive or issue regression
- WHEN rollback executes
- THEN the recorded pre-adoption source and dependency state MUST be restored

#### Scenario: Rollback is incomplete
- GIVEN a rollback that restores only source or dependency state
- WHEN rollback validation runs
- THEN validation MUST fail
