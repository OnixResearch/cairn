# Test Harness Delta

## ADDED Requirements

### Requirement: Receipts

r[test_harness.receipts_replay.receipts] Harness runs MUST emit stable JSON receipts for the selected high-value rail.

#### Scenario: Schema

r[test_harness.receipts_replay.receipts.schema]
- GIVEN the focused test harness is run
- WHEN the `schema` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

#### Scenario: Portable Hash

r[test_harness.receipts_replay.receipts.portable_hash]
- GIVEN the focused test harness is run
- WHEN the `portable_hash` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

### Requirement: Replay

r[test_harness.receipts_replay.replay] Seeded or randomized harness failures MUST report deterministic replay instructions.

#### Scenario: Failure Command

r[test_harness.receipts_replay.replay.failure_command]
- GIVEN the focused test harness is run
- WHEN the `failure_command` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

#### Scenario: Regression Fixture

r[test_harness.receipts_replay.replay.regression_fixture]
- GIVEN the focused test harness is run
- WHEN the `regression_fixture` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior
