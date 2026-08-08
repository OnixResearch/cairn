# Cli Maintainability Delta: Split Agent Receipt Core

## Requirements

### Requirement: Split agent receipt core uses focused modules

r[cli_maintainability.agent_receipt_core_modules.focused_modules] The system MUST split `crates/cairn-core/src/verified/agent_receipt.rs` into focused pure-core modules that preserve a small `verified::agent_receipt` facade and isolate cohesive receipt-validation responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.agent_receipt_core_modules.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a reviewer inspects `crates/cairn-core/src/verified/agent_receipt/`
- THEN receipt DTOs, schema/hash checks, plan linkage, verification checks, nondeterminism checks, replay-evidence checks, fixture generation, and shared helpers are separated into named pure-core modules
- AND the original target no longer remains the sole large-file review surface for that family

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.agent_receipt_core_modules.focused_modules.narrow_bridges]
- GIVEN CLI and crate-root consumers still use agent receipt validation APIs
- WHEN the implementation exposes split functionality
- THEN the `verified::agent_receipt` facade re-exports the existing public constants, DTOs, fixture, hash, issue, and completion-gate functions
- AND internal helpers remain module-private or `pub(super)` rather than broad crate-level bridges

### Requirement: Split agent receipt core remains behavior preserving

r[cli_maintainability.agent_receipt_core_modules.behavior_preserved] The system MUST preserve public behavior while splitting agent receipt schema, hash, plan linkage, verification, nondeterminism, replay-evidence, and fixture logic.

#### Scenario: Focused checks pass

r[cli_maintainability.agent_receipt_core_modules.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN the operator runs agent receipt core unit tests and validation checks
- THEN the checks pass without intentional hash, fixture, issue-code, issue-message, or completion-gate drift

#### Scenario: Repository checks pass

r[cli_maintainability.agent_receipt_core_modules.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs native Cairn validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit
