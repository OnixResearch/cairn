# Semantic Review Delta

## Requirements

### Requirement: Metrics regression rail module split

r[cairn.semantic_review.metrics_regression_split] The system MUST keep the metrics regression rail behavior-preserving while splitting its implementation into focused modules.

#### Scenario: Module Boundaries

r[cairn.semantic_review.metrics_regression_split.boundaries]
- GIVEN the semantic review metrics regression rail is maintained
- WHEN its implementation is split
- THEN the split MUST separate public entrypoints/report hashing from registry, fixture, guidance-sync, evidence-boundary, and shared helper logic

#### Scenario: Public Api

r[cairn.semantic_review.metrics_regression_split.public_api]
- GIVEN existing callers use the semantic review metrics regression validators and schema constants
- WHEN the module split is applied
- THEN the same public functions and constants MUST remain exported from `verified::semantic_review`

#### Scenario: Focused Helpers

r[cairn.semantic_review.metrics_regression_split.focused_helpers]
- GIVEN the split implementation is inspected
- WHEN registry, fixture, guidance, and evidence boundary logic are changed later
- THEN each behavior class MUST live behind a focused private module seam

#### Scenario: Behavior Preserved

r[cairn.semantic_review.metrics_regression_split.behavior_preserved]
- GIVEN existing positive and negative metrics regression fixtures
- WHEN the split implementation validates them
- THEN issue codes, verdicts, and deterministic receipt behavior MUST remain stable

#### Scenario: Focused Verification

r[cairn.semantic_review.metrics_regression_split.focused_verification]
- GIVEN the split is complete
- WHEN focused semantic-review tests run
- THEN they MUST cover registry, fixture, guidance sync, evidence boundary, and split-boundary behavior

#### Scenario: Repository Verification

r[cairn.semantic_review.metrics_regression_split.repository_verification]
- GIVEN the split is archived
- WHEN repository validation runs
- THEN Cairn validation, Tracey coverage, and the repository gate MUST pass
