# Test Harness Real Rails Delta

## Requirements

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
