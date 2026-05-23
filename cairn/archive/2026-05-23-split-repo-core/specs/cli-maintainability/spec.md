# Cli Maintainability Delta: Split Repo Core

## Requirements

### Requirement: Split repo core uses focused modules

r[cli_maintainability.repo_core_modules.focused_modules] The system MUST split `crates/cairn-core/src/verified/repo.rs` into focused pure-core modules that preserve a small `verified::repo` facade and isolate cohesive repository read-model responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.repo_core_modules.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a reviewer inspects `crates/cairn-core/src/verified/repo/`
- THEN repo status construction, change read models, metadata parsing/validation, spec summaries, task counting, and DTO types are separated into named pure-core modules
- AND the original target no longer remains the sole large-file review surface for that family

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.repo_core_modules.focused_modules.narrow_bridges]
- GIVEN other core or CLI modules still consume repository read models
- WHEN the implementation exposes split functionality
- THEN the `verified::repo` facade re-exports the existing public functions and DTOs
- AND internal helpers remain module-private or `pub(super)` rather than broad crate-level bridges

### Requirement: Split repo core remains behavior preserving

r[cli_maintainability.repo_core_modules.behavior_preserved] The system MUST preserve public behavior while splitting repository status, change status, metadata, spec summary, and task counting logic.

#### Scenario: Focused checks pass

r[cli_maintainability.repo_core_modules.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN the operator runs repo-core unit tests and core validation checks
- THEN the checks pass without intentional read-model, task-count, metadata-filter, or title-extraction drift

#### Scenario: Repository checks pass

r[cli_maintainability.repo_core_modules.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs native Cairn validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit
