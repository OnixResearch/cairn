## ADDED Requirements

### Requirement: Determinism tests are isolated without weakening coverage

r[cli_maintainability.determinism_tests.behavior] The system MUST keep determinism/replay CLI coverage in a dedicated integration-test target while preserving positive and negative behavior coverage.

#### Scenario: Moved tests retain replay coverage

r[cli_maintainability.determinism_tests.behavior.moved.tests.retain.replay.coverage]

- GIVEN the determinism integration-test target exists
- WHEN the operator runs the determinism test target
- THEN stable replay, case selection, output modes, mutation rejection, hidden entropy rejection, coverage audit, and saved replay verification remain covered

#### Scenario: Original CLI test target is not the only determinism proof

r[cli_maintainability.determinism_tests.behavior.original.cli.test.target.is.not.the.only.determinism.proof]

- GIVEN determinism tests have moved
- WHEN the operator searches the large `cli.rs` test target
- THEN determinism behavior is not dependent on a monolithic catch-all test file

### Requirement: Test split remains behavior preserving

r[cli_maintainability.determinism_tests.shell] The system SHOULD split determinism tests without changing checked-in fixtures, public CLI behavior, or deterministic receipt schemas.

#### Scenario: Focused and full tests pass

r[cli_maintainability.determinism_tests.shell.focused.and.full.tests.pass]

- GIVEN the test split is implemented
- WHEN the operator runs focused determinism tests followed by workspace tests
- THEN both focused and full verification pass without fixture drift


### Requirement: Implementation Task Traceability

r[cli_maintainability.determinism_tests.coverage.task_traceability] The system MUST keep implementation tasks traceable to scenario-level verification targets during this change.

#### Scenario: Replay task coverage

r[cli_maintainability.determinism_tests.coverage.replay]

- GIVEN the change package tasks for `split-determinism-cli-tests`
- WHEN implementation or verification work is performed for this task
- THEN the task MUST produce the stated evidence without changing unrelated Cairn behavior.

#### Scenario: Focused Full task coverage

r[cli_maintainability.determinism_tests.behavior_preserving.focused_full]

- GIVEN the change package tasks for `split-determinism-cli-tests`
- WHEN implementation or verification work is performed for this task
- THEN the task MUST produce the stated evidence without changing unrelated Cairn behavior.
