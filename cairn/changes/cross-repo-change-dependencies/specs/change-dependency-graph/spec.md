# Change Dependency Graph Specification

## Purpose

Adds cross-repo dependency declarations for Cairn change packages.

## Requirements

### Requirement: Cross-repo dependency contract
r[cairn.cross_repo_change_dependencies.contract] Cairn MUST support change dependency rows that reference an upstream repo, change ID, required gate or receipt kind, expected status, and non-claim boundary.

#### Scenario: Satisfied dependency passes
r[cairn.cross_repo_change_dependencies.fixtures.positive]
- GIVEN a change declares an upstream dependency and supplies a matching passing receipt
- WHEN dependency validation runs
- THEN the dependency MUST pass and preserve the upstream repo/change identity.

#### Scenario: Missing dependency fails
r[cairn.cross_repo_change_dependencies.fixtures.negative]
- GIVEN a required upstream dependency has no supplied receipt or has a failed/stale/wrong-repo receipt
- WHEN dependency validation runs
- THEN validation MUST fail with deterministic diagnostics naming the dependency.

### Requirement: Dependency validation receipt
r[cairn.cross_repo_change_dependencies.validation] Cairn MUST emit a deterministic receipt for cross-repo dependency validation.

#### Scenario: Receipt binds dependency inputs
r[cairn.cross_repo_change_dependencies.validation.receipt]
- GIVEN dependency rows and upstream receipt summaries
- WHEN validation runs
- THEN the receipt MUST include dependency count, satisfied count, blocked count, input hash, policy hash, and receipt hash.

### Requirement: Dependency docs
r[cairn.cross_repo_change_dependencies.docs] Cairn documentation MUST state that cross-repo dependency validation proves receipt linkage only.

#### Scenario: Non-claim is visible
r[cairn.cross_repo_change_dependencies.docs.non_claims]
- GIVEN a dependency is satisfied
- WHEN the supported claim is rendered
- THEN it MUST state that the dependency receipt does not prove upstream correctness or downstream integration correctness.

### Requirement: Final validation
r[cairn.cross_repo_change_dependencies.final_validation] The change MUST include positive and negative fixtures plus focused validation evidence before archive.

#### Scenario: Fixture suite covers dependency states
r[cairn.cross_repo_change_dependencies.final_validation.fixtures]
- GIVEN satisfied, missing, failed, stale, and wrong-repo dependency fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass and invalid fixtures MUST fail closed.
