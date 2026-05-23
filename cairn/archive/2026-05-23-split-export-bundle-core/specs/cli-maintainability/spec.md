# Cli Maintainability Delta: Split Export Bundle Core

## Requirements

### Requirement: Split export bundle core uses focused modules

r[cli_maintainability.export_bundle_core_modules.focused_modules] The system MUST split `crates/cairn-core/src/verified/content_addressed/export_bundle.rs` into focused pure-core modules that preserve the existing content-addressed evidence facade and isolate cohesive export-bundle validation responsibilities.

#### Scenario: Focused boundaries are visible

r[cli_maintainability.export_bundle_core_modules.focused_modules.visible_boundaries]
- GIVEN the refactor is implemented
- WHEN a reviewer inspects `crates/cairn-core/src/verified/content_addressed/export_bundle/`
- THEN schema, hash material, object membership, pointer linkage, replay metadata, compiler IR, mutation manifest, caveat, entropy, DTO, and shared issue-helper responsibilities are separated into named pure-core modules
- AND the original target no longer remains the sole large-file review surface for export-bundle validation

#### Scenario: Bridge APIs stay narrow

r[cli_maintainability.export_bundle_core_modules.focused_modules.narrow_bridges]
- GIVEN CLI and crate-root consumers still use export-bundle validation APIs
- WHEN the implementation exposes split functionality
- THEN the `verified::content_addressed` facade re-exports the existing public schema constant, DTOs, hash functions, hash-material function, and validator
- AND internal helpers remain module-private or `pub(super)` rather than broad crate-level bridges

### Requirement: Split export bundle core remains behavior preserving

r[cli_maintainability.export_bundle_core_modules.behavior_preserved] The system MUST preserve public behavior while splitting export-bundle schema, hash, object membership, pointer, replay, compiler IR, mutation-manifest, caveat, and entropy validation logic.

#### Scenario: Focused checks pass

r[cli_maintainability.export_bundle_core_modules.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN the operator runs content-addressed core unit tests and export-bundle CLI fixture checks
- THEN the checks pass without intentional hash, fixture, issue-code, issue-message, schema, or validation drift

#### Scenario: Repository checks pass

r[cli_maintainability.export_bundle_core_modules.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs native Cairn validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit
