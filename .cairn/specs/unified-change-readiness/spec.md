# Unified Change Readiness Specification

## Purpose

Defines the `unified-change-readiness` capability.

## Requirements

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

### Requirement: Additive compact machine projection
r[cairn.agent_readiness.view] The system MUST provide an explicit compact readiness view without changing existing full receipts, human summaries, or machine transport semantics.

#### Scenario: CLI and MCP expose the same compact meaning
- GIVEN equal admitted readiness inputs and an explicit agent view
- WHEN CLI and MCP render the compact projection
- THEN lifecycle state, stage states, actions, blockers, and full-report identity agree across transports

#### Scenario: Invalid view combinations fail explicitly
- GIVEN an unknown view or an incompatible human-format request
- WHEN the shell admits the request
- THEN it returns the existing transport failure class without substituting a different view

### Requirement: Required decision fields survive compaction
r[cairn.agent_readiness.decisions] The system MUST preserve report validity, lifecycle classification, every stage state, mutation authority, and the exact source and policy bindings in compact output.

#### Scenario: A blocked report remains usable and blocked
- GIVEN a consistent report with missing completion evidence
- WHEN the compact view is derived
- THEN it retains valid report status separately from blocked or unavailable completion and dependent stages

#### Scenario: A small budget cannot hide required states
- GIVEN the requested byte budget cannot contain the required envelope and stage states
- WHEN the compact projection is admitted
- THEN it reports an insufficient-budget error rather than a truncated success response

### Requirement: Bounded actionable detail
r[cairn.agent_readiness.actions] The system MUST preserve selected actions' exact arguments, prerequisites, cause IDs, targets, expected observations, and advisory authority under deterministic ordering.

#### Scenario: Known remediation has complete arguments
- GIVEN an admitted readiness action with an exact preview argv
- WHEN that action appears in compact output
- THEN its arguments and prerequisites remain intact and contain no added execution permission

#### Scenario: Omitted or unknown actions remain explicit
- GIVEN output bounds omit actions or no reviewed remediation exists
- WHEN the compact view is derived
- THEN it reports omission counts and detail selectors or an unresolved action without invented arguments

### Requirement: Identity-checked detail retrieval
r[cairn.agent_readiness.detail] The system MUST retrieve omitted blockers, actions, and stage decisions only against an explicit expected full-report identity and bounded exact selectors.

#### Scenario: A current selector returns exact detail
- GIVEN the source, policy, evidence, and requested report identity remain equal
- WHEN a client requests one omitted detail
- THEN the response binds that detail to the same report and preserves its exact semantic value

#### Scenario: A stale selector cannot return unrelated current detail
- GIVEN the source, policy, evidence, or selector no longer matches the expected report
- WHEN detail retrieval runs
- THEN it returns a stale or unknown-detail result without silently substituting another report

### Requirement: Projection safety and efficiency evidence
r[cairn.agent_readiness.conformance] The system MUST retain positive and negative full-versus-compact fixtures, record representation sizes for complete diagnostic loops including detail calls, and keep token cost explicitly unavailable when no usage or exact tokenizer observation is supplied.

#### Scenario: A compact diagnostic loop preserves the full decision
- GIVEN matching successful and blocked fixtures
- WHEN both views are observed over the same inputs including detail calls
- THEN representation sizes are recorded with equal oracle outcomes and bounded claims

#### Scenario: A compact view cannot grant future authority
- GIVEN a previously ready compact report and later changed inputs
- WHEN a mutation command runs
- THEN its existing fresh preflight remains authoritative regardless of the cached compact result
