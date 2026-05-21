# Test Harness Delta

## ADDED Requirements

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
