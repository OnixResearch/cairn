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
