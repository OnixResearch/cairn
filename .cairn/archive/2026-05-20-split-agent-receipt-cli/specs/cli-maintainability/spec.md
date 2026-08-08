## ADDED Requirements

### Requirement: Agent receipt CLI module preserves receipt contracts

r[cli_maintainability.agent_receipt_module.behavior] The system MUST implement `cairn agent-receipt` behind a focused CLI module boundary without changing validation semantics, bundle-verification semantics, deterministic JSON shape, or issue-code behavior.

#### Scenario: Fixture validation remains deterministic

r[cli_maintainability.agent_receipt_module.behavior.fixture.validation.remains.deterministic]

- GIVEN existing positive and negative agent receipt fixtures
- WHEN the operator runs the moved agent receipt validation command
- THEN the same validity and issue-code behavior is reported

#### Scenario: Bundle verification remains linked

r[cli_maintainability.agent_receipt_module.behavior.bundle.verification.remains.linked]

- GIVEN an agent evidence bundle with replay and release-readiness artifacts
- WHEN the operator runs the moved bundle verification command
- THEN the command verifies the complete chain without changing schema identity or hash boundaries

### Requirement: Main CLI delegates agent receipt work

r[cli_maintainability.agent_receipt_module.shell] The system SHOULD keep top-level `main.rs` dispatch-only for `agent-receipt` while the receipt module owns parsing and orchestration details.

#### Scenario: Top-level dispatch remains narrow

r[cli_maintainability.agent_receipt_module.shell.top.level.dispatch.remains.narrow]

- GIVEN the agent receipt module exists
- WHEN the top-level CLI receives an `agent-receipt` command
- THEN `main.rs` delegates to the module and does not retain the command-family implementation


### Requirement: Implementation Task Traceability

r[cli_maintainability.agent_receipt_module.contracts.task_traceability] The system MUST keep implementation tasks traceable to scenario-level verification targets during this change.

#### Scenario: Validation task coverage

r[cli_maintainability.agent_receipt_module.contracts.validation]

- GIVEN the change package tasks for `split-agent-receipt-cli`
- WHEN implementation or verification work is performed for this task
- THEN the task MUST produce the stated evidence without changing unrelated Cairn behavior.

#### Scenario: Delegates task coverage

r[cli_maintainability.agent_receipt_module.shell.delegates]

- GIVEN the change package tasks for `split-agent-receipt-cli`
- WHEN implementation or verification work is performed for this task
- THEN the task MUST produce the stated evidence without changing unrelated Cairn behavior.

#### Scenario: Bundle task coverage

r[cli_maintainability.agent_receipt_module.contracts.bundle]

- GIVEN the change package tasks for `split-agent-receipt-cli`
- WHEN implementation or verification work is performed for this task
- THEN the task MUST produce the stated evidence without changing unrelated Cairn behavior.
