# Test Harness Delta

## ADDED Requirements

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
