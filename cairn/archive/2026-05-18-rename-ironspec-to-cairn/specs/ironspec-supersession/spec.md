# Cairn Supersession Delta: Rename to Cairn

## MODIFIED Requirements

### Requirement: Cairn native lifecycle layout
r[cairn.supersede.native_layout] The system MUST use the canonical `cairn/` lifecycle layout for normal runtime commands after the rename and MUST NOT silently fall back to legacy `ironspec/` or `openspec/` lifecycle content.

#### Scenario: Initialize canonical Cairn layout
r[cairn.supersede.native_layout.init_cairn]
- GIVEN a writable project root without a `cairn/` directory
- WHEN the operator runs `cairn init --root <path>`
- THEN the system creates `cairn/specs`, `cairn/changes`, and `cairn/archive`
- AND the JSON receipt identifies the `cairn` layout

#### Scenario: Prefer canonical layout over compatibility layout
r[cairn.supersede.native_layout.prefer_cairn]
- GIVEN a repository containing both canonical `cairn/` lifecycle content and legacy `ironspec/` lifecycle content
- WHEN the operator runs read, status, doctor, validate, gate, sync, or archive commands
- THEN the system uses the canonical `cairn/` layout
- AND the JSON receipt identifies the selected layout

#### Scenario: Reject implicit legacy fallback
r[cairn.supersede.native_layout.no_implicit_legacy_after_rename]
- GIVEN a repository containing only legacy `ironspec/` lifecycle content
- WHEN the operator runs `cairn` read, status, doctor, validate, gate, sync, or archive commands without an explicit compatibility flag or migration command
- THEN the command evaluates the canonical `cairn/` layout
- AND it does not silently read legacy lifecycle content

### Requirement: OpenSpec migration bridge
r[cairn.supersede.migrate_openspec] The system MUST provide deterministic migration bridges from legacy `openspec/` and `ironspec/` layouts to canonical `cairn/` layout without using legacy content as an implicit runtime fallback.

#### Scenario: Dry-run legacy Ironspec migration plan
r[cairn.supersede.migrate_openspec.ironspec_dry_run]
- GIVEN a repository with legacy `ironspec/` lifecycle content
- WHEN the operator runs `cairn migrate ironspec --root <path>`
- THEN the CLI emits a deterministic JSON migration plan from legacy `ironspec/` paths into canonical `cairn/` paths
- AND no filesystem content is mutated

#### Scenario: Execute legacy Ironspec migration copy
r[cairn.supersede.migrate_openspec.ironspec_execute]
- GIVEN a repository with legacy `ironspec/` lifecycle content and no conflicting canonical destination files
- WHEN the operator runs `cairn migrate ironspec --root <path> --execute`
- THEN the system copies planned files into matching canonical `cairn/` paths
- AND the JSON receipt reports `mutated=true`

### Requirement: Rename identity and compatibility
r[cairn.supersede.rename_cairn] The system MUST expose `Cairn` / `cairn` as the canonical project, CLI, package, docs, and runtime lifecycle identity while preserving legacy `ironspec` only as explicit compatibility where needed.

#### Scenario: Canonical CLI identity
r[cairn.supersede.rename_cairn.cli]
- GIVEN a built release after the rename
- WHEN an operator invokes the canonical CLI help, version, status, gate, validate, sync, archive, or migrate commands
- THEN the executable, help text, examples, and deterministic receipts identify the command as `cairn`
- AND they do not present `ironspec` as the canonical command

#### Scenario: Legacy command compatibility is visible
r[cairn.supersede.rename_cairn.legacy_alias]
- GIVEN the legacy `ironspec` command alias is installed during the migration window
- WHEN an operator invokes the alias
- THEN the command executes the matching `cairn` behavior
- AND the JSON receipt or human-facing warning identifies `ironspec` as a compatibility alias for `cairn`

#### Scenario: Canonical packaging metadata
r[cairn.supersede.rename_cairn.packaging]
- GIVEN the Rust workspace and Nix flake after the rename
- WHEN package metadata, crate names, binary names, Nix packages, Nix apps, and flake checks are inspected
- THEN new canonical surfaces use `cairn`
- AND any remaining `ironspec` surfaces are documented compatibility aliases or historical references

#### Scenario: Deterministic rename classification evidence
r[cairn.supersede.rename_cairn.classification]
- GIVEN the rename implementation is ready for review
- WHEN the deterministic rename evidence checker scans repository text surfaces
- THEN every remaining `ironspec` occurrence is classified as `needs-fix`, `compatibility-alias`, `historical-archive`, `requirement-id-history`, or `intentional-reference`
- AND the verification fails when any occurrence remains in `needs-fix`
