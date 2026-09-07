# Incremental agent observations delta

## ADDED Requirements

### Requirement: Complete observation identity
r[cairn.incremental_observations.identity] The system MUST bind observation continuity to explicit root identity, source, policy, evidence, tool contract, view, selectors, and deterministic ordering.

#### Scenario: Equal admitted observations report unchanged
- GIVEN a prior observation and fresh equal inputs with matching scope and contract identities
- WHEN Cairn compares the observations
- THEN it returns unchanged with the current observation identity and complete-state metadata

#### Scenario: Different cohorts cannot reuse a continuation
- GIVEN changed policy, evidence, view, root identity, or tool contract
- WHEN Cairn admits a prior observation or page cursor
- THEN it rejects continuation or requires a new complete observation rather than claiming unchanged

### Requirement: Source-bound deterministic pages
r[cairn.incremental_observations.pages] The system MUST provide bounded ordered pages with total counts, omission counts, continuation identity, and an explicit complete or partial disposition.

#### Scenario: Pages reconstruct the complete observation
- GIVEN unchanged admitted inputs and a valid page sequence
- WHEN a consumer joins all returned entries
- THEN the result equals the unpaged ordered observation without duplicates or missing entries

#### Scenario: Mutation between pages cannot mix snapshots
- GIVEN source changes after the first page or a malformed or out-of-scope cursor
- WHEN the next page is requested
- THEN it reports stale or invalid continuation without combining old and new entries

### Requirement: Exact observation differences
r[cairn.incremental_observations.delta] The system MUST derive added, changed, and removed entries from explicit prior and current observations without treating a partial observation as a complete baseline.

#### Scenario: A resumed comparison preserves changes and removals
- GIVEN admitted prior and current observations across a fresh process start
- WHEN Cairn derives their difference
- THEN it returns exact changed entries and removal identities under the current deterministic order

#### Scenario: Missing prior state cannot become an empty baseline
- GIVEN an unavailable, malformed, truncated, or partial prior observation
- WHEN the difference is requested
- THEN it reports unavailable or invalid baseline and requires an explicit complete refresh

### Requirement: Shell-owned storage without authority
r[cairn.incremental_observations.storage] The system MUST keep snapshot files and caches in bounded explicit shell adapters and prevent observation reuse from bypassing fresh mutation admission.

#### Scenario: A disposable cache preserves semantic results
- GIVEN the same admitted observations with a warm cache, a cold cache, or a restarted process
- WHEN the shell requests a page or difference
- THEN semantic results agree and ordinary reads do not create hidden durable session state

#### Scenario: Unsafe storage or stale readiness grants no effects
- GIVEN a linked snapshot path, corrupt cache, unknown ownership, or stale ready observation
- WHEN an adapter reads state or a later command requests mutation
- THEN it rejects unsafe state and retains the mutation command's original fresh preflight

### Requirement: Incremental conformance and scoped cost
r[cairn.incremental_observations.conformance] The system MUST retain positive and negative full, paged, unchanged, and difference fixtures and account for complete resumed-loop costs.

#### Scenario: Repeated loops measure retrieval overhead
- GIVEN matched agent tasks with declared cache states and equal source changes
- WHEN the benchmark compares full and incremental observations
- THEN it includes continuation calls, snapshot reads, refreshes, tokens, and unchanged oracle outcomes

#### Scenario: Reduced output does not conceal incomplete state
- GIVEN a consumer skips remaining pages or treats unavailable state as unchanged
- WHEN the fixture oracle evaluates the run
- THEN it rejects false completion and preserves the failed run's cost
