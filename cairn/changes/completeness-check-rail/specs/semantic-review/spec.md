# Completeness Check Rail Specification

## Purpose

Defines the `completeness-check-rail` capability for Cairn semantic trust/review workflows while preserving deterministic receipt and policy boundaries.

## Requirements

### Requirement: The system
r[cairn.semantic_review.completeness_check_rail.mapping] The system MUST produce a deterministic completeness map across requirements, tasks, implementation refs, and verification refs.

#### Scenario: Mapping Active Change
r[cairn.semantic_review.completeness_check_rail.mapping.mapping.active_change]
- GIVEN an active change package
- WHEN Cairn runs the completeness check
- THEN the receipt lists requirement ids and their linked tasks, implementation refs, and verification refs

#### Scenario: Mapping Accepted Spec
r[cairn.semantic_review.completeness_check_rail.mapping.mapping.accepted_spec]
- GIVEN accepted specs with durable refs
- WHEN Cairn runs the completeness check
- THEN the receipt includes accepted requirement coverage without mutating the repository

### Requirement: The system
r[cairn.semantic_review.completeness_check_rail.omissions] The system MUST emit policy-owned omission findings for missing completeness links.

#### Scenario: Omissions Missing Task
r[cairn.semantic_review.completeness_check_rail.omissions.omissions.missing_task]
- GIVEN a requirement with no task or dependency mapping
- WHEN completeness is checked
- THEN Cairn emits an omission finding for missing task coverage

#### Scenario: Omissions Missing Verification
r[cairn.semantic_review.completeness_check_rail.omissions.omissions.missing_verification]
- GIVEN a requirement with implementation refs but no verification refs
- WHEN completeness is checked
- THEN Cairn emits an omission finding for missing verification coverage
