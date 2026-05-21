## ADDED Requirements

### Requirement: Split determinism replay CLI source uses focused submodules

r[cli_maintainability.determinism_replay_submodules.focused_modules] The system MUST split the determinism replay CLI source into focused submodules that preserve a small facade and isolate cohesive replay responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.determinism_replay_submodules.focused_modules.visible_boundaries]
- GIVEN the submodule split is implemented
- WHEN a reviewer inspects `crates/cairn-cli/src/determinism/replay`
- THEN replay option parsing, policy decoding, execution, receipt construction, rendering, and shared DTOs are separated into named modules
- AND the facade no longer contains the full large replay review surface

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.determinism_replay_submodules.focused_modules.narrow_bridges]
- GIVEN replay submodules share types or helpers
- WHEN the implementation exposes cross-module APIs
- THEN visibility is limited to the determinism replay module tree except for already-required crate consumers
- AND broad crate-root shims or unrelated helper moves are avoided

### Requirement: Split determinism replay CLI source remains behavior preserving

r[cli_maintainability.determinism_replay_submodules.behavior_preserved] The system MUST preserve public behavior while splitting determinism replay CLI source into focused submodules.

#### Scenario: Focused checks pass

r[cli_maintainability.determinism_replay_submodules.behavior_preserved.focused_checks]
- GIVEN the submodule split is implemented
- WHEN the operator runs focused determinism replay tests
- THEN the focused checks pass without intentional receipt, issue-code, policy, fixture, or hash drift

#### Scenario: Repository checks pass

r[cli_maintainability.determinism_replay_submodules.behavior_preserved.repository_checks]
- GIVEN the submodule split is implemented
- WHEN the operator runs repository validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit
