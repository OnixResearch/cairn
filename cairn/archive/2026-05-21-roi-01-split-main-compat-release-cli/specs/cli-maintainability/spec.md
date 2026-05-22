## ADDED Requirements

### Requirement: Split remaining main CLI compatibility and release helpers uses focused boundaries

r[cli_maintainability.main_compat_release_followup.focused_modules] The system MUST address `crates/cairn-cli/src/main.rs` (~680 lines) with focused boundaries that reduce review surface while preserving existing behavior.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.main_compat_release_followup.focused_modules.visible_boundaries]
- GIVEN the ROI slice is implemented
- WHEN a reviewer inspects the affected source, tests, docs, or evidence
- THEN release-readiness, rename-classification, migration, plan, and receipt presentation helpers are separated into named modules
- AND the original target no longer remains the sole large review surface for that family

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.main_compat_release_followup.focused_modules.narrow_bridges]
- GIVEN another module or operator workflow still consumes the affected behavior
- WHEN the implementation exposes cross-module functions, shared helpers, or evidence pointers
- THEN only the narrow bridge surface needed by those consumers is exposed
- AND broad crate-root shims, unrelated helper moves, and tag or receipt churn are avoided

### Requirement: Split remaining main CLI compatibility and release helpers remains behavior preserving

r[cli_maintainability.main_compat_release_followup.behavior_preserved] The system MUST preserve existing public behavior, deterministic evidence, fixture expectations, and issue codes while draining this ROI slice.

#### Scenario: Focused checks pass

r[cli_maintainability.main_compat_release_followup.behavior_preserved.focused_checks]
- GIVEN the ROI slice is implemented
- WHEN the operator runs release-readiness, migration/lifecycle, rename-classification, and full CLI integration checks
- THEN the focused checks pass without intentional receipt, issue-code, policy, fixture, or hash drift

#### Scenario: Repository checks pass

r[cli_maintainability.main_compat_release_followup.behavior_preserved.repository_checks]
- GIVEN the ROI slice is implemented
- WHEN the operator runs native Cairn validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit
