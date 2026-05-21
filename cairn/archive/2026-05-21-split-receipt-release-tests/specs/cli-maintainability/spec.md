## ADDED Requirements

### Requirement: Split receipt and release CLI integration tests uses focused modules

r[cli_maintainability.receipt_release_tests.focused_modules] The system MUST split receipt, agent-evidence, release-readiness, and signed-manifest integration tests from `crates/cairn-cli/tests/cli.rs` into focused modules or test targets that preserve a small facade and isolate cohesive responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.receipt_release_tests.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a reviewer inspects the affected integration-test tree
- THEN public receipt schema, agent receipt replay, release-readiness bundle, and signed evidence manifest checks are separated into a named module or test target
- AND unrelated residual CLI error smoke tests remain outside that focused receipt/release family

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.receipt_release_tests.focused_modules.narrow_bridges]
- GIVEN helper code is needed by the moved receipt/release test target
- WHEN the implementation exposes or duplicates helper functions
- THEN only the narrow local helpers needed by that test target are present
- AND broad mutable shared support or crate-root test shims are avoided

### Requirement: Split receipt and release CLI integration tests remains behavior preserving

r[cli_maintainability.receipt_release_tests.behavior_preserved] The system MUST preserve public behavior while splitting receipt, agent-evidence, release-readiness, and signed-manifest integration tests.

#### Scenario: Focused checks pass

r[cli_maintainability.receipt_release_tests.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN the operator runs the new focused receipt/release test target plus the remaining `cli` target
- THEN the focused checks pass without intentional receipt, issue-code, fixture, policy-hash, or Tracey-reference drift

#### Scenario: Repository checks pass

r[cli_maintainability.receipt_release_tests.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs repository validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit
