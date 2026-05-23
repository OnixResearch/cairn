# Semantic Review Specification Delta

## ADDED Requirements

### Requirement: Split semantic review core uses focused modules

r[semantic_review.split_semantic_review_core.focused_modules] The system MUST split the semantic-review functional core into focused modules that preserve the existing `cairn_core::verified::semantic_review` facade.

#### Scenario: Focused boundaries are visible

r[semantic_review.split_semantic_review_core.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a maintainer inspects `crates/cairn-core/src/verified/semantic_review/`
- THEN review receipts, revision handoffs, ledger analytics, completeness receipts, policy profiles, and shared validators are separated into named modules
- AND the facade remains small enough to expose constants, DTOs, re-exports, and tests without owning every implementation detail

#### Scenario: Bridge APIs stay narrow

r[semantic_review.split_semantic_review_core.focused_modules.narrow_bridges]
- GIVEN existing CLI and core consumers use semantic-review functions
- WHEN the split exposes functions across module boundaries
- THEN the public facade preserves the existing exported functions and constants
- AND internal helpers remain scoped to the semantic-review module tree unless an existing consumer requires them

### Requirement: Split semantic review core remains behavior preserving

r[semantic_review.split_semantic_review_core.behavior_preserved] The system MUST preserve existing semantic-review public behavior, receipt schemas, hash material, issue codes, deterministic evidence, and Tracey coverage while draining this split.

#### Scenario: Focused checks pass

r[semantic_review.split_semantic_review_core.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN focused semantic-review tests and related core checks run
- THEN they pass without intentional receipt, policy, fixture, hash, or issue-code drift

#### Scenario: Repository checks pass

r[semantic_review.split_semantic_review_core.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs native Cairn validation, formatting, linting, Tracey coverage, and diff checks
- THEN all checks pass and the repository remains ready for commit
