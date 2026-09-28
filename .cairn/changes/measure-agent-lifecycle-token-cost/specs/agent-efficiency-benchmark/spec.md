# Agent efficiency benchmark delta

## ADDED Requirements

### Requirement: Explicit comparable cohorts
r[cairn.agent_efficiency.cohort] The system MUST bind benchmark observations to explicit scenario, source, policy, model, tokenizer or usage source, agent configuration, catalog, and oracle identities.

#### Scenario: Matching cohorts admit a paired comparison
- GIVEN baseline and candidate runs with equal required cohort fields and an explicit changed product revision
- WHEN the comparator admits the pair
- THEN it preserves both product identities and permits only the declared comparison

#### Scenario: Different completion criteria prevent comparison
- GIVEN runs with different or missing oracle, model, or accounting identities
- WHEN the comparator admits the pair
- THEN it reports incomparable inputs without a savings verdict

### Requirement: Exact token accounting
r[cairn.agent_efficiency.accounting] The system MUST distinguish uncached input, cached input, output, and optional reasoning tokens under an explicit accounting convention without duplicate charges.

#### Scenario: Complete usage produces reproducible totals
- GIVEN unique call observations with admitted nonnegative counts and declared reasoning inclusion
- WHEN the pure core aggregates a run
- THEN it returns exact totals and separate retry, tool-call, repeated-read, and duration observations

#### Scenario: Missing or conflicting accounting stays unavailable
- GIVEN absent usage, mixed counting conventions, duplicate call IDs, negative counts, or arithmetic overflow
- WHEN the core admits the run
- THEN it rejects invalid counts or reports tokens unavailable without a byte-based estimate

### Requirement: Independent outcome classification
r[cairn.agent_efficiency.outcomes] The system MUST preserve repository-oracle outcomes independently from agent success statements, process exit status, task checkboxes, and lifecycle conformance.

#### Scenario: Correctly completed and blocked work remain separate
- GIVEN a fresh fixture oracle that accepts completed work or the expected blocked outcome
- WHEN the report classifies the run
- THEN it records correct completion or correct blocked behavior as distinct outcomes

#### Scenario: An agent success statement cannot hide missing evidence
- GIVEN an agent claims completion while the fixture oracle rejects missing or stale required evidence
- WHEN the report classifies the run
- THEN it records false completion and retains the run's token cost

### Requirement: Bounded observation import
r[cairn.agent_efficiency.import] The system MUST import only explicitly selected bounded observations and keep provider access, clocks, filesystem effects, and transcript capture outside the pure core.

#### Scenario: An admitted saved run produces a redacted report
- GIVEN safe local inputs that satisfy typed Nickel-derived schemas and named resource bounds
- WHEN the CLI imports and reports the run
- THEN it emits deterministic bounded summaries with BLAKE3 identities and no raw prompt or credential content

#### Scenario: Unsafe or incomplete inputs cannot produce a complete report
- GIVEN a linked path, oversized file, malformed trace, unknown schema, or truncated observation sequence
- WHEN the shell admits the input
- THEN it reports the exact bounded error class without network access or fabricated records

### Requirement: Maintained comparisons and non-claims
r[cairn.agent_efficiency.conformance] The system MUST retain positive and negative paired-run fixtures and distinguish representation-cost replays from end-to-end agent comparisons.

#### Scenario: A declared live cohort measures complete task cost
- GIVEN an explicitly authorized host run with a declared repetition budget and current scenario oracles
- WHEN the report compares baseline and candidate attempts
- THEN it reports all attempted-run costs, outcome classes, uncertainty, and scoped token changes

#### Scenario: An empty or failed cohort cannot claim efficiency
- GIVEN no correctly completed runs, unavailable token data, or worse correctness under the same oracle
- WHEN the report evaluates the comparison
- THEN it withholds an improvement claim and preserves failures rather than discarding expensive attempts
