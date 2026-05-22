# Cli Maintainability Specification

## Purpose

Defines the `cli-maintainability` capability.

## Requirements

### Requirement: Split agent receipt CLI source uses focused modules

r[cli_maintainability.agent_receipt_cli_source.focused_modules] The system MUST split `crates/cairn-cli/src/agent_receipt.rs` (~762 lines) into focused modules or test targets that preserve a small facade and isolate cohesive responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.agent_receipt_cli_source.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a reviewer inspects the affected source or test tree
- THEN agent receipt validation, bundle verification, smoke, bounded IO, and issue helpers are separated into named modules or test targets
- AND the original target no longer remains the sole large-file review surface for that family

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.agent_receipt_cli_source.focused_modules.narrow_bridges]
- GIVEN another module still consumes behavior from the split family
- WHEN the implementation exposes cross-module functions
- THEN only the narrow bridge functions needed by those consumers are `pub(crate)`
- AND broad crate-root re-export shims are avoided

### Requirement: Split agent receipt CLI source remains behavior preserving

r[cli_maintainability.agent_receipt_cli_source.behavior_preserved] The system MUST preserve public behavior while splitting agent receipt validation, bundle verification, smoke, bounded IO, and issue helpers.

#### Scenario: Focused checks pass

r[cli_maintainability.agent_receipt_cli_source.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN the operator runs agent receipt focused integration tests, MCP smoke bridge, and release-readiness bundle checks
- THEN the focused checks pass without intentional receipt, issue-code, or fixture drift

#### Scenario: Repository checks pass

r[cli_maintainability.agent_receipt_cli_source.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs repository validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit

### Requirement: Split MCP CLI source uses focused modules

r[cli_maintainability.mcp_cli_source.focused_modules] The system MUST split `crates/cairn-cli/src/mcp.rs` (~922 lines) into focused modules or test targets that preserve a small facade and isolate cohesive responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.mcp_cli_source.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a reviewer inspects the affected source or test tree
- THEN MCP transport, schema, read-tool, mutation-tool, and agent-receipt bridge code are separated into named modules or test targets
- AND the original target no longer remains the sole large-file review surface for that family

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.mcp_cli_source.focused_modules.narrow_bridges]
- GIVEN another module still consumes behavior from the split family
- WHEN the implementation exposes cross-module functions
- THEN only the narrow bridge functions needed by those consumers are `pub(crate)`
- AND broad crate-root re-export shims are avoided

### Requirement: Split MCP CLI source remains behavior preserving

r[cli_maintainability.mcp_cli_source.behavior_preserved] The system MUST preserve public behavior while splitting MCP transport, schema, read-tool, mutation-tool, and agent-receipt bridge code.

#### Scenario: Focused checks pass

r[cli_maintainability.mcp_cli_source.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN the operator runs focused MCP integration tests plus agent receipt smoke and determinism bridge checks
- THEN the focused checks pass without intentional receipt, issue-code, or fixture drift

#### Scenario: Repository checks pass

r[cli_maintainability.mcp_cli_source.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs repository validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit

### Requirement: Split remaining main CLI compatibility and release source uses focused modules

r[cli_maintainability.main_compat_release_cli.focused_modules] The system MUST split `crates/cairn-cli/src/main.rs` (~1,258 lines) into focused modules or test targets that preserve a small facade and isolate cohesive responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.main_compat_release_cli.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a reviewer inspects the affected source or test tree
- THEN release-readiness, rename-classification, migration, and plan/receipt presentation helpers are separated into named modules or test targets
- AND the original target no longer remains the sole large-file review surface for that family

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.main_compat_release_cli.focused_modules.narrow_bridges]
- GIVEN another module still consumes behavior from the split family
- WHEN the implementation exposes cross-module functions
- THEN only the narrow bridge functions needed by those consumers are `pub(crate)`
- AND broad crate-root re-export shims are avoided

### Requirement: Split remaining main CLI compatibility and release source remains behavior preserving

r[cli_maintainability.main_compat_release_cli.behavior_preserved] The system MUST preserve public behavior while splitting release-readiness, rename-classification, migration, and plan/receipt presentation helpers.

#### Scenario: Focused checks pass

r[cli_maintainability.main_compat_release_cli.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN the operator runs release-readiness, migration/lifecycle, rename-classification, and full CLI integration checks
- THEN the focused checks pass without intentional receipt, issue-code, or fixture drift

#### Scenario: Repository checks pass

r[cli_maintainability.main_compat_release_cli.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs repository validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit

### Requirement: Split evidence manifest CLI source uses focused modules

r[cli_maintainability.evidence_manifest_cli_source.focused_modules] The system MUST split `crates/cairn-cli/src/evidence_manifest.rs` (~701 lines) into focused modules or test targets that preserve a small facade and isolate cohesive responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.evidence_manifest_cli_source.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a reviewer inspects the affected source or test tree
- THEN evidence manifest generation, verification, role validation, metadata hashing, and receipt helpers are separated into named modules or test targets
- AND the original target no longer remains the sole large-file review surface for that family

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.evidence_manifest_cli_source.focused_modules.narrow_bridges]
- GIVEN another module still consumes behavior from the split family
- WHEN the implementation exposes cross-module functions
- THEN only the narrow bridge functions needed by those consumers are `pub(crate)`
- AND broad crate-root re-export shims are avoided

### Requirement: Split evidence manifest CLI source remains behavior preserving

r[cli_maintainability.evidence_manifest_cli_source.behavior_preserved] The system MUST preserve public behavior while splitting evidence manifest generation, verification, role validation, metadata hashing, and receipt helpers.

#### Scenario: Focused checks pass

r[cli_maintainability.evidence_manifest_cli_source.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN the operator runs evidence manifest focused tests plus evidence export/registry manifest-binding checks
- THEN the focused checks pass without intentional receipt, issue-code, or fixture drift

#### Scenario: Repository checks pass

r[cli_maintainability.evidence_manifest_cli_source.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs repository validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit

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

### Requirement: Split evidence registry integration tests uses focused modules

r[cli_maintainability.evidence_registry_tests.focused_modules] The system MUST split `crates/cairn-cli/tests/evidence_registry.rs` (~1,129 lines) into focused modules or test targets that preserve a small facade and isolate cohesive responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.evidence_registry_tests.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a reviewer inspects the affected source or test tree
- THEN evidence registry integration-test families are separated into named modules or test targets
- AND the original target no longer remains the sole large-file review surface for that family

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.evidence_registry_tests.focused_modules.narrow_bridges]
- GIVEN another module still consumes behavior from the split family
- WHEN the implementation exposes cross-module functions
- THEN only the narrow bridge functions needed by those consumers are `pub(crate)`
- AND broad crate-root re-export shims are avoided

### Requirement: Split evidence registry integration tests remains behavior preserving

r[cli_maintainability.evidence_registry_tests.behavior_preserved] The system MUST preserve public behavior while splitting evidence registry integration-test families.

#### Scenario: Focused checks pass

r[cli_maintainability.evidence_registry_tests.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN the operator runs focused registry test targets plus the full registry/evidence chain checks
- THEN the focused checks pass without intentional receipt, issue-code, or fixture drift

#### Scenario: Repository checks pass

r[cli_maintainability.evidence_registry_tests.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs repository validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit

### Requirement: Split lifecycle integration tests further uses focused modules

r[cli_maintainability.lifecycle_tests.focused_modules] The system MUST split `crates/cairn-cli/tests/lifecycle.rs` (~968 lines) into focused modules or test targets that preserve a small facade and isolate cohesive responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.lifecycle_tests.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a reviewer inspects the affected source or test tree
- THEN native lifecycle integration-test subfamilies are separated into named modules or test targets
- AND the original target no longer remains the sole large-file review surface for that family

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.lifecycle_tests.focused_modules.narrow_bridges]
- GIVEN another module still consumes behavior from the split family
- WHEN the implementation exposes cross-module functions
- THEN only the narrow bridge functions needed by those consumers are `pub(crate)`
- AND broad crate-root re-export shims are avoided

### Requirement: Split lifecycle integration tests further remains behavior preserving

r[cli_maintainability.lifecycle_tests.behavior_preserved] The system MUST preserve public behavior while splitting native lifecycle integration-test subfamilies.

#### Scenario: Focused checks pass

r[cli_maintainability.lifecycle_tests.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN the operator runs new lifecycle-focused test targets plus the full lifecycle suite during transition
- THEN the focused checks pass without intentional receipt, issue-code, or fixture drift

#### Scenario: Repository checks pass

r[cli_maintainability.lifecycle_tests.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs repository validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit

### Requirement: Split residual CLI integration tests uses focused modules

r[cli_maintainability.cli_integration_tests.focused_modules] The system MUST split `crates/cairn-cli/tests/cli.rs` (~1,671 lines) into focused modules or test targets that preserve a small facade and isolate cohesive responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.cli_integration_tests.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a reviewer inspects the affected source or test tree
- THEN remaining monolithic CLI integration-test families are separated into named modules or test targets
- AND the original target no longer remains the sole large-file review surface for that family

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.cli_integration_tests.focused_modules.narrow_bridges]
- GIVEN another module still consumes behavior from the split family
- WHEN the implementation exposes cross-module functions
- THEN only the narrow bridge functions needed by those consumers are `pub(crate)`
- AND broad crate-root re-export shims are avoided

### Requirement: Split residual CLI integration tests remains behavior preserving

r[cli_maintainability.cli_integration_tests.behavior_preserved] The system MUST preserve public behavior while splitting remaining monolithic CLI integration-test families.

#### Scenario: Focused checks pass

r[cli_maintainability.cli_integration_tests.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN the operator runs new focused integration-test targets plus the remaining `cli` target
- THEN the focused checks pass without intentional receipt, issue-code, or fixture drift

#### Scenario: Repository checks pass

r[cli_maintainability.cli_integration_tests.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs repository validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit

### Requirement: Split receipt and release CLI integration tests uses focused modules

r[cli_maintainability.receipt_release_tests.focused_modules] The system MUST split receipt, agent-evidence, release-readiness, and signed-manifest integration tests from `crates/cairn-cli/tests/cli.rs` into focused modules or test targets that preserve a small facade and isolate cohesive responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.receipt_release_tests.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a reviewer inspects the affected integration-test tree
- THEN public receipt schema, agent receipt replay, release-readiness bundle, and signed evidence manifest checks are separated into a named module or test target
- AND unrelated residual CLI error smoke tests remain outside that focused receipt/release family

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.receipt_release_tests.focused_modules.narrow_bridges]
- GIVEN helper code is needed by the moved receipt/release test target
- WHEN the implementation exposes or duplicates helper functions
- THEN only the narrow local helpers needed by that test target are present
- AND broad mutable shared support or crate-root test shims are avoided

### Requirement: Split receipt and release CLI integration tests remains behavior preserving

r[cli_maintainability.receipt_release_tests.behavior_preserved] The system MUST preserve public behavior while splitting receipt, agent-evidence, release-readiness, and signed-manifest integration tests.

#### Scenario: Focused checks pass

r[cli_maintainability.receipt_release_tests.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN the operator runs the new focused receipt/release test target plus the remaining `cli` target
- THEN the focused checks pass without intentional receipt, issue-code, fixture, policy-hash, or Tracey-reference drift

#### Scenario: Repository checks pass

r[cli_maintainability.receipt_release_tests.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs repository validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit

### Requirement: Split release receipt integration-test target uses focused submodules

r[cli_maintainability.release_receipts_submodules.focused_modules] The system MUST split the `release_receipts` integration-test target into focused submodules that preserve a small facade and isolate cohesive receipt/release responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.release_receipts_submodules.focused_modules.visible_boundaries]
- GIVEN the submodule split is implemented
- WHEN a reviewer inspects `crates/cairn-cli/tests/release_receipts.rs` and its sibling module directory
- THEN portable receipt hash, agent bundle, signed manifest, release-readiness, public schema, and agent replay checks are separated into named modules
- AND the facade no longer contains the full large receipt/release review surface

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.release_receipts_submodules.focused_modules.narrow_bridges]
- GIVEN helpers are shared by the receipt/release submodules
- WHEN the implementation exposes local support functions
- THEN helper visibility is limited to the test target's module tree
- AND broad crate-root test shims or mutable shared state are avoided

### Requirement: Split release receipt integration-test target remains behavior preserving

r[cli_maintainability.release_receipts_submodules.behavior_preserved] The system MUST preserve public behavior while splitting the release receipt integration-test target into focused submodules.

#### Scenario: Focused checks pass

r[cli_maintainability.release_receipts_submodules.behavior_preserved.focused_checks]
- GIVEN the submodule split is implemented
- WHEN the operator runs the reorganized `release_receipts` integration-test target
- THEN the focused check passes without intentional receipt, issue-code, fixture, policy-hash, or Tracey-reference drift

#### Scenario: Repository checks pass

r[cli_maintainability.release_receipts_submodules.behavior_preserved.repository_checks]
- GIVEN the submodule split is implemented
- WHEN the operator runs repository validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit

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

### Requirement: Split lifecycle integration tests into focused targets uses focused boundaries

r[cli_maintainability.lifecycle_tests_followup.focused_modules] The system MUST address `crates/cairn-cli/tests/lifecycle.rs` (~632 lines) with focused boundaries that reduce review surface while preserving existing behavior.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.lifecycle_tests_followup.focused_modules.visible_boundaries]
- GIVEN the ROI slice is implemented
- WHEN a reviewer inspects the affected source, tests, docs, or evidence
- THEN change creation, sync/archive, mutation manifest, validation, and negative lifecycle cases are separated into named test modules or targets
- AND the original target no longer remains the sole large review surface for that family

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.lifecycle_tests_followup.focused_modules.narrow_bridges]
- GIVEN another module or operator workflow still consumes the affected behavior
- WHEN the implementation exposes cross-module functions, shared helpers, or evidence pointers
- THEN only the narrow bridge surface needed by those consumers is exposed
- AND broad crate-root shims, unrelated helper moves, and tag or receipt churn are avoided

### Requirement: Split lifecycle integration tests into focused targets remains behavior preserving

r[cli_maintainability.lifecycle_tests_followup.behavior_preserved] The system MUST preserve existing public behavior, deterministic evidence, fixture expectations, and issue codes while draining this ROI slice.

#### Scenario: Focused checks pass

r[cli_maintainability.lifecycle_tests_followup.behavior_preserved.focused_checks]
- GIVEN the ROI slice is implemented
- WHEN the operator runs focused lifecycle and lifecycle-mutation integration tests
- THEN the focused checks pass without intentional receipt, issue-code, policy, fixture, or hash drift

#### Scenario: Repository checks pass

r[cli_maintainability.lifecycle_tests_followup.behavior_preserved.repository_checks]
- GIVEN the ROI slice is implemented
- WHEN the operator runs native Cairn validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit
