# Test Harness Specification

## Purpose

Defines local harness receipt, classification, integrity, self-test, and flake-triage contracts.

## Requirements

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

### Requirement: Classification

r[test_harness.failure_taxonomy.classification] Harness failures MUST be classified before reporting final status.

#### Scenario: Product

r[test_harness.failure_taxonomy.classification.product]
- GIVEN the focused test harness is run
- WHEN the `product` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

#### Scenario: Harness

r[test_harness.failure_taxonomy.classification.harness]
- GIVEN the focused test harness is run
- WHEN the `harness` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

#### Scenario: Environment

r[test_harness.failure_taxonomy.classification.environment]
- GIVEN the focused test harness is run
- WHEN the `environment` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

#### Scenario: Flake

r[test_harness.failure_taxonomy.classification.flake]
- GIVEN the focused test harness is run
- WHEN the `flake` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

### Requirement: Usability

r[test_harness.failure_taxonomy.usability] Harness failure summaries MUST make the next debugging target clear.

#### Scenario: Summary

r[test_harness.failure_taxonomy.usability.summary]
- GIVEN the focused test harness is run
- WHEN the `summary` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

### Requirement: Artifacts

r[test_harness.fixture_artifact_integrity.artifacts] Harness success MUST require declared artifacts to exist and verify.

#### Scenario: Existence

r[test_harness.fixture_artifact_integrity.artifacts.existence]
- GIVEN the focused test harness is run
- WHEN the `existence` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

#### Scenario: Schema Hash

r[test_harness.fixture_artifact_integrity.artifacts.schema_hash]
- GIVEN the focused test harness is run
- WHEN the `schema_hash` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

#### Scenario: Corrupt

r[test_harness.fixture_artifact_integrity.artifacts.corrupt]
- GIVEN the focused test harness is run
- WHEN the `corrupt` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

### Requirement: Fixtures

r[test_harness.fixture_artifact_integrity.fixtures] Fixture lifecycle MUST distinguish canonical fixtures from generated outputs.

#### Scenario: Canonical

r[test_harness.fixture_artifact_integrity.fixtures.canonical]
- GIVEN the focused test harness is run
- WHEN the `canonical` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

#### Scenario: Golden Update

r[test_harness.fixture_artifact_integrity.fixtures.golden_update]
- GIVEN the focused test harness is run
- WHEN the `golden_update` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

### Requirement: Rails

r[test_harness.rail_split_selftest.rails] The harness MUST expose distinct fast and full verification rails.

#### Scenario: Fast

r[test_harness.rail_split_selftest.rails.fast]
- GIVEN the focused test harness is run
- WHEN the `fast` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

#### Scenario: Full

r[test_harness.rail_split_selftest.rails.full]
- GIVEN the focused test harness is run
- WHEN the `full` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

### Requirement: Self Tests

r[test_harness.rail_split_selftest.self_tests] The harness MUST test its own failure-reporting behavior.

#### Scenario: Intentional Failure

r[test_harness.rail_split_selftest.self_tests.intentional_failure]
- GIVEN the focused test harness is run
- WHEN the `intentional_failure` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

#### Scenario: Intentional Success

r[test_harness.rail_split_selftest.self_tests.intentional_success]
- GIVEN the focused test harness is run
- WHEN the `intentional_success` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

### Requirement: Rerun

r[test_harness.flake_triage.rerun] Suspected flaky or timeout failures MUST use bounded same-identity reruns.

#### Scenario: Bounded

r[test_harness.flake_triage.rerun.bounded]
- GIVEN the focused test harness is run
- WHEN the `bounded` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

#### Scenario: Not Greenwash

r[test_harness.flake_triage.rerun.not_greenwash]
- GIVEN the focused test harness is run
- WHEN the `not_greenwash` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

### Requirement: Promotion

r[test_harness.flake_triage.promotion] Repeated flakes MUST have a deterministic regression promotion path.

#### Scenario: Evidence

r[test_harness.flake_triage.promotion.evidence]
- GIVEN the focused test harness is run
- WHEN the `evidence` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

#### Scenario: Fixture

r[test_harness.flake_triage.promotion.fixture]
- GIVEN the focused test harness is run
- WHEN the `fixture` behavior is exercised
- THEN the harness emits deterministic receipt evidence for that behavior

### Requirement: Real Rail Receipts

r[test_harness.real_rails.receipts] The test harness MUST wrap selected existing verification rails and emit deterministic receipt evidence for each run.

#### Scenario: Validate Rail

r[test_harness.real_rails.receipts.validate]
- GIVEN a repository with Cairn lifecycle content
- WHEN the harness runs the `validate` rail
- THEN the receipt records the rail identity, replay command, exit status, and output hashes.

#### Scenario: Tracey Rail

r[test_harness.real_rails.receipts.tracey]
- GIVEN a repository with Tracey markers
- WHEN the harness runs the `tracey-coverage` rail
- THEN the receipt records deterministic tracey rail evidence without replacing Tracey coverage itself.

#### Scenario: Release Readiness Rail

r[test_harness.real_rails.receipts.release_readiness]
- GIVEN release-readiness is available for the repository
- WHEN the harness runs the `release-readiness` rail
- THEN the receipt records release-readiness rail evidence and preserves the replay command.

### Requirement: Rail Failure Classification

r[test_harness.real_rails.classification] Real rail failures MUST be classified using the harness taxonomy rather than reported as raw subprocess failures.

#### Scenario: Product Rail Failure

r[test_harness.real_rails.classification.product]
- GIVEN a real verification rail exits nonzero after launching successfully
- WHEN the harness builds the receipt
- THEN the failure class is `product` and the diagnostic points to the rail output.

#### Scenario: Environment Rail Failure

r[test_harness.real_rails.classification.environment]
- GIVEN a requested rail cannot be launched because the environment is missing a prerequisite
- WHEN the harness builds the receipt
- THEN the failure class is `environment` and the diagnostic points to the missing prerequisite.

### Requirement: Rail Selection

r[test_harness.real_rails.selection] The test harness MUST expose explicit rail selection for existing verification commands without changing their native CLI behavior.

#### Scenario: Known Rails

r[test_harness.real_rails.selection.known]
- GIVEN the user selects a supported real rail
- WHEN the harness constructs the invocation
- THEN the selected command is bounded to the intended Cairn verification rail.

#### Scenario: Unknown Rails

r[test_harness.real_rails.selection.unknown]
- GIVEN the user selects an unsupported real rail
- WHEN the harness constructs the receipt
- THEN the harness reports a deterministic harness failure instead of executing an arbitrary command.
