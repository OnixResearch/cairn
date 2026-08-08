# CLI Maintainability Specification Delta

## ADDED Requirements

### Requirement: Split agent receipt bundle verification into focused modules uses focused modules

r[cli_maintainability.agent_receipt_bundle_submodule_split.focused_modules] The system MUST split `crates/cairn-cli/src/agent_receipt/bundle.rs` (17.1KB) into focused modules or test targets that preserve a small facade and isolate cohesive responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.agent_receipt_bundle_submodule_split.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a reviewer inspects the affected source or test tree
- THEN bundle loading, replay verification, release-readiness linkage, deterministic issue collection, and receipt rendering are separated into named modules or test targets
- AND `crates/cairn-cli/src/agent_receipt/bundle.rs` no longer remains the sole large review surface for that family

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.agent_receipt_bundle_submodule_split.focused_modules.narrow_bridges]
- GIVEN another module or operator workflow still consumes the affected behavior
- WHEN the implementation exposes cross-module functions, shared helpers, or evidence pointers
- THEN only the narrow bridge surface needed by those consumers is exposed
- AND broad crate-root shims, unrelated helper moves, and receipt churn are avoided

### Requirement: Split agent receipt bundle verification into focused modules remains behavior preserving

r[cli_maintainability.agent_receipt_bundle_submodule_split.behavior_preserved] The system MUST preserve existing public behavior, deterministic evidence, fixture expectations, and issue codes while draining this split.

#### Scenario: Focused checks pass

r[cli_maintainability.agent_receipt_bundle_submodule_split.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN the operator runs agent receipt bundle verification checks, release-readiness bundle checks, and MCP smoke checks
- THEN the focused checks pass without intentional receipt, issue-code, policy, fixture, or hash drift

#### Scenario: Repository checks pass

r[cli_maintainability.agent_receipt_bundle_submodule_split.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs native Cairn validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit
