# Test Harness Delta

## ADDED Requirements

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
