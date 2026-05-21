## ADDED Requirements

### Requirement: Lifecycle tests are isolated without weakening coverage

r[cli_maintainability.lifecycle_tests.coverage] The system MUST keep native lifecycle CLI coverage in a dedicated integration-test target while preserving positive and negative behavior coverage.

#### Scenario: Moved lifecycle tests retain native lifecycle coverage

r[cli_maintainability.lifecycle_tests.coverage.moved.tests.retain.lifecycle.coverage]

- GIVEN the lifecycle integration-test target exists
- WHEN the operator runs the lifecycle test target
- THEN init, change creation, status, validation, gates, sync, archive, migration, mutation manifests, and lifecycle error cases remain covered

#### Scenario: Original CLI test target is not the only lifecycle proof

r[cli_maintainability.lifecycle_tests.coverage.original.cli.test.target.is.not.the.only.lifecycle.proof]

- GIVEN lifecycle tests have moved
- WHEN the operator searches the large `cli.rs` test target
- THEN lifecycle behavior is not dependent on a monolithic catch-all test file

### Requirement: Lifecycle test split remains behavior preserving

r[cli_maintainability.lifecycle_tests.behavior_preserving] The system SHOULD split lifecycle tests without changing checked-in fixtures, public CLI behavior, or deterministic receipt schemas.

#### Scenario: Focused and full tests pass

r[cli_maintainability.lifecycle_tests.behavior_preserving.focused.and.full.tests.pass]

- GIVEN the lifecycle test split is implemented
- WHEN the operator runs focused lifecycle tests followed by workspace tests
- THEN both focused and full verification pass without fixture drift
