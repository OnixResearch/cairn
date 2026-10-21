## ADDED Requirements

### Requirement: Consumer-owned release readiness selection

The system SHALL support a typed consumer-owned release-readiness selection of repository-relative MCP completion contract/results, Preserves corpus, and maintained external command obligations without disabling any required check. r[cairn.release_consumer.selection]

#### Scenario: Selected consumer evidence

- **GIVEN** a declared consumer release selection and genuine completion evidence
- **WHEN** a consumer provides valid own inputs and an explicit compatible policy
- **THEN** standalone MCP smoke and release readiness use those exact inputs and policy, preserve read-only tool semantics, and report passing only when the actual completion validator accepts the evidence.

#### Scenario: Invalid or stale consumer evidence

- **GIVEN** a declared consumer release selection
- **WHEN** selected paths are unsafe or missing, selection is malformed, policy/results are stale, a required completion fails, or the corpus is stale
- **THEN** the selected checks fail without falling back to Cairn self-fixtures, suppressing obligations, refreshing expected evidence, or mutating consumer state.

#### Scenario: Self-host compatibility

- **GIVEN** Cairn's maintained self-host policy and inputs
- **WHEN** no consumer release selection is present
- **THEN** Cairn retains its existing self-host input locations and obligations.
