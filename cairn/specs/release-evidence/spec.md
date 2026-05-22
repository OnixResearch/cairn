# Release Evidence Specification

## Purpose

Defines the `release-evidence` capability.

## Requirements

### Requirement: Create current-HEAD release evidence index uses focused boundaries

r[release_evidence.current_head_evidence_index.focused_modules] The system MUST address current clean `main` HEAD and existing release/readiness evidence artifacts with focused boundaries that reduce review surface while preserving existing behavior.

#### Scenario: Focused boundaries are visible

r[release_evidence.current_head_evidence_index.focused_modules.visible_boundaries]
- GIVEN the ROI slice is implemented
- WHEN a reviewer inspects the affected source, tests, docs, or evidence
- THEN existing tag, harness, receipt, policy, Tracey, and release-readiness evidence are indexed with stable paths and non-self-referential wording
- AND the original target no longer remains the sole large review surface for that family

#### Scenario: Bridge APIs stay narrow

r[release_evidence.current_head_evidence_index.focused_modules.narrow_bridges]
- GIVEN another module or operator workflow still consumes the affected behavior
- WHEN the implementation exposes cross-module functions, shared helpers, or evidence pointers
- THEN only the narrow bridge surface needed by those consumers is exposed
- AND broad crate-root shims, unrelated helper moves, and tag or receipt churn are avoided

### Requirement: Create current-HEAD release evidence index remains behavior preserving

r[release_evidence.current_head_evidence_index.behavior_preserved] The system MUST preserve existing public behavior, deterministic evidence, fixture expectations, and issue codes while draining this ROI slice.

#### Scenario: Focused checks pass

r[release_evidence.current_head_evidence_index.behavior_preserved.focused_checks]
- GIVEN the ROI slice is implemented
- WHEN the operator runs evidence index review, release-readiness receipt generation/verification, clean worktree status, and diff checks
- THEN the focused checks pass without intentional receipt, issue-code, policy, fixture, or hash drift

#### Scenario: Repository checks pass

r[release_evidence.current_head_evidence_index.behavior_preserved.repository_checks]
- GIVEN the ROI slice is implemented
- WHEN the operator runs native Cairn validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit
