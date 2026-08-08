# Test Harness Delta

## ADDED Requirements

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
