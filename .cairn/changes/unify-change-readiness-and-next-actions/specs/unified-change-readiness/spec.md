# Unified change readiness

## ADDED Requirements

### Requirement: Exact readiness snapshot
r[cairn.unified_readiness.snapshot] The system MUST bind each readiness report to the selected change, policy, source snapshot, evidence inputs, and observation scope.

#### Scenario: Equivalent inputs produce equivalent reports
- GIVEN equal admitted snapshots and policy values
- WHEN the pure model derives readiness
- THEN it produces equal ordered semantic results and BLAKE3 identities

#### Scenario: Mixed observations cannot form a ready snapshot
- GIVEN observations with mismatched source, change, or policy identities
- WHEN the model admits the snapshot
- THEN it reports identity conflicts instead of a ready result

### Requirement: Existing decisions retain authority
r[cairn.unified_readiness.decisions] The system MUST compose existing validators while keeping artifact readiness, gate conformance, completion, sync readiness, and archive readiness separate.

#### Scenario: Current evidence satisfies a selected stage
- GIVEN current passing required evidence and satisfied stage dependencies
- WHEN the model evaluates that stage
- THEN it reports the stage as ready with references to its supporting decisions

#### Scenario: Presence and checkboxes do not establish completion
- GIVEN complete artifacts and checked tasks but missing required execution evidence
- WHEN the model derives readiness
- THEN completion and dependent mutation stages remain blocked

### Requirement: Typed causal next actions
r[cairn.unified_readiness.actions] The system MUST return stable blocker IDs and typed next actions with cause, target, prerequisites, authority class, and expected progress.

#### Scenario: A known blocker produces an exact repair action
- GIVEN a stale review with an identified changed artifact
- WHEN the report derives next actions
- THEN it identifies the artifact and required review refresh rather than only repeating the failed gate

#### Scenario: Unknown remediation grants no action authority
- GIVEN an unsupported blocker without a reviewed remediation rule
- WHEN the report derives next actions
- THEN it returns an explicit unresolved diagnostic without invented commands or mutation permission

### Requirement: CLI and MCP semantic parity
r[cairn.unified_readiness.transport] The system MUST expose the same semantic report through CLI and MCP while preserving existing transport and lifecycle verdict distinctions.

#### Scenario: Both transports agree
- GIVEN the same selected root, change, policy, and evidence
- WHEN CLI and MCP request the readiness report
- THEN their semantic decisions, actions, and report identity agree

#### Scenario: Transport completion cannot mask a blocked lifecycle
- GIVEN a valid machine envelope containing a blocked readiness report
- WHEN a consumer interprets the response
- THEN the documented lifecycle classification remains blocked despite successful transport

### Requirement: Read-only bounded composition
r[cairn.unified_readiness.boundary] The system MUST use a pure readiness core and bounded shell observation without task execution, automatic repair, hidden registry access, or authority escalation.

#### Scenario: Report construction causes no product effects
- GIVEN safe explicit inputs and supported observation adapters
- WHEN the operator requests readiness
- THEN the shell only reads admitted facts and emits the report

#### Scenario: Unsafe or unavailable observation remains explicit
- GIVEN a symlink escape, read limit violation, or unavailable evidence provider
- WHEN the shell gathers facts
- THEN the report exposes a bounded blocker and performs no fallback mutation or network call

### Requirement: Maintained readiness acceptance cases
r[cairn.unified_readiness.conformance] The system MUST retain deterministic positive and negative fixtures for causal actions, validator agreement, transport parity, and effect denial.

#### Scenario: A valid stage advances through its existing validator
- GIVEN a fixture whose selected stage satisfies the original validator
- WHEN unified readiness evaluates the fixture
- THEN the same stage classification appears with the expected next action

#### Scenario: Invalid evidence cannot become ready through composition
- GIVEN failed probes, missing completion rows, invalid task order, or blocked cross-repository evidence
- WHEN the fixture compares direct validators with unified readiness
- THEN every applicable blocker remains present and dependent actions remain unavailable
