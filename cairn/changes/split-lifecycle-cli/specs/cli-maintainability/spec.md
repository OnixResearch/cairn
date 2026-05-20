## ADDED Requirements

### Requirement: Lifecycle CLI module preserves mutation contracts

r[cli_maintainability.lifecycle_module.behavior] The system MUST implement sync, archive, gate, and change-context CLI handling behind a focused lifecycle module without changing dry-run defaults, explicit execution requirements, mutation manifests, or gate receipt contracts.

#### Scenario: Dry-run remains non-mutating

r[cli_maintainability.lifecycle_module.behavior.dry.run.remains.non.mutating]

- GIVEN a change with delta specs
- WHEN the operator runs sync or archive without an explicit execution flag
- THEN the command emits a dry-run receipt and does not mutate repository files

#### Scenario: Explicit execution keeps mutation evidence

r[cli_maintainability.lifecycle_module.behavior.explicit.execution.keeps.mutation.evidence]

- GIVEN a temp fixture change that is valid for sync or archive execution
- WHEN the operator runs the moved command with the explicit execution flag
- THEN the command emits sorted before/after mutation-manifest evidence and preserves existing blocker semantics

### Requirement: Lifecycle split keeps top-level dispatch thin

r[cli_maintainability.lifecycle_module.shell] The system SHOULD keep lifecycle command orchestration out of `main.rs` while preserving existing command names and argument parsing.

#### Scenario: Top-level lifecycle dispatch delegates

r[cli_maintainability.lifecycle_module.shell.top.level.lifecycle.dispatch.delegates]

- GIVEN the lifecycle module exists
- WHEN the top-level CLI receives sync, archive, gate, or change-context commands
- THEN `main.rs` delegates to the lifecycle module instead of retaining lifecycle implementation bodies


### Requirement: Implementation Task Traceability

r[cli_maintainability.lifecycle_module.contracts.task_traceability] The system MUST keep implementation tasks traceable to scenario-level verification targets during this change.

#### Scenario: Mutation task coverage

r[cli_maintainability.lifecycle_module.contracts.mutation]

- GIVEN the change package tasks for `split-lifecycle-cli`
- WHEN implementation or verification work is performed for this task
- THEN the task MUST produce the stated evidence without changing unrelated Cairn behavior.

#### Scenario: Delegates task coverage

r[cli_maintainability.lifecycle_module.shell.delegates]

- GIVEN the change package tasks for `split-lifecycle-cli`
- WHEN implementation or verification work is performed for this task
- THEN the task MUST produce the stated evidence without changing unrelated Cairn behavior.
