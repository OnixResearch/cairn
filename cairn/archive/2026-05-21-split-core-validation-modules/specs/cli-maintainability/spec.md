## ADDED Requirements

### Requirement: Split core validation modules uses focused modules

r[cli_maintainability.core_validation_modules.focused_modules] The system MUST split `crates/cairn-core/src/verified/validation.rs` (~792 lines) into focused modules or test targets that preserve a small facade and isolate cohesive responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.core_validation_modules.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a reviewer inspects the affected source or test tree
- THEN pure validation core helpers and issue construction are separated into named modules or test targets
- AND the original target no longer remains the sole large-file review surface for that family

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.core_validation_modules.focused_modules.narrow_bridges]
- GIVEN another module still consumes behavior from the split family
- WHEN the implementation exposes cross-module functions
- THEN only the narrow bridge functions needed by those consumers are `pub(crate)`
- AND broad crate-root re-export shims are avoided

### Requirement: Split core validation modules remains behavior preserving

r[cli_maintainability.core_validation_modules.behavior_preserved] The system MUST preserve public behavior while splitting pure validation core helpers and issue construction.

#### Scenario: Focused checks pass

r[cli_maintainability.core_validation_modules.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN the operator runs core validation unit tests, lifecycle validation integration tests, and FCIS purity smoke
- THEN the focused checks pass without intentional receipt, issue-code, or fixture drift

#### Scenario: Repository checks pass

r[cli_maintainability.core_validation_modules.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs repository validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit
