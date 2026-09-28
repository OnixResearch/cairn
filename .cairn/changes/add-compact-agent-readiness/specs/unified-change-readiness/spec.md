# Compact readiness projection delta

## ADDED Requirements

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
r[cairn.agent_readiness.conformance] The system MUST retain positive and negative full-versus-compact fixtures and measure complete diagnostic tasks without weakening existing validator or mutation boundaries.

#### Scenario: A compact diagnostic loop preserves the full decision
- GIVEN matching successful and blocked fixtures
- WHEN the benchmark compares full and compact loops including detail calls
- THEN it records total tokens, calls, and equal oracle outcomes with bounded claims

#### Scenario: A compact view cannot grant future authority
- GIVEN a previously ready compact report and later changed inputs
- WHEN a mutation command runs
- THEN its existing fresh preflight remains authoritative regardless of the cached compact result
