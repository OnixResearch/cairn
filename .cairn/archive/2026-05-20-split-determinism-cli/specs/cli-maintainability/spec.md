## ADDED Requirements

### Requirement: Module boundary preserves determinism CLI behavior

r[cli_maintainability.determinism_module.behavior] The system MUST implement the `cairn determinism` command family behind a focused CLI module boundary without changing supported subcommands, deterministic receipt fields, or failure issue codes.

#### Scenario: Determinism replay behavior is preserved

r[cli_maintainability.determinism_module.behavior.determinism.replay.behavior.is.preserved]

- GIVEN a repository with existing determinism replay fixtures and policy
- WHEN the operator runs the moved `cairn determinism replay` command with the same arguments as before
- THEN the command emits the same supported receipt contract and exits with the same success or failure semantics

#### Scenario: Saved replay verification behavior is preserved

r[cli_maintainability.determinism_module.behavior.saved.replay.verification.behavior.is.preserved]

- GIVEN a saved replay receipt fixture
- WHEN the operator runs the moved `cairn determinism verify-receipt` command
- THEN the command validates policy, attestation, case, group, and receipt-hash linkage without changing issue-code semantics

### Requirement: Main CLI shell delegates determinism work

r[cli_maintainability.determinism_module.shell] The system SHOULD keep top-level `main.rs` responsible for dispatch only while determinism-specific parsing, helpers, and receipt orchestration live in the determinism module.

#### Scenario: Top-level dispatch remains thin

r[cli_maintainability.determinism_module.shell.top.level.dispatch.remains.thin]

- GIVEN the determinism module exists
- WHEN the top-level CLI receives a `determinism` command
- THEN `main.rs` delegates to the module instead of containing the command-family implementation


### Requirement: Implementation Task Traceability

r[cli_maintainability.determinism_module.behavior.task_traceability] The system MUST keep implementation tasks traceable to scenario-level verification targets during this change.

#### Scenario: Replay task coverage

r[cli_maintainability.determinism_module.behavior.replay]

- GIVEN the change package tasks for `split-determinism-cli`
- WHEN implementation or verification work is performed for this task
- THEN the task MUST produce the stated evidence without changing unrelated Cairn behavior.

#### Scenario: Delegates task coverage

r[cli_maintainability.determinism_module.shell.delegates]

- GIVEN the change package tasks for `split-determinism-cli`
- WHEN implementation or verification work is performed for this task
- THEN the task MUST produce the stated evidence without changing unrelated Cairn behavior.

#### Scenario: Verify Receipt task coverage

r[cli_maintainability.determinism_module.behavior.verify_receipt]

- GIVEN the change package tasks for `split-determinism-cli`
- WHEN implementation or verification work is performed for this task
- THEN the task MUST produce the stated evidence without changing unrelated Cairn behavior.
