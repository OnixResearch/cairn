## ADDED Requirements

### Requirement: Cairn native lifecycle layout

r[cairn.supersede.native_layout] The system MUST support a native `cairn/` lifecycle layout as the preferred runtime layout while retaining legacy `openspec/` fallback for migration.

#### Scenario: Initialize native layout

r[cairn.supersede.native_layout.init]
- GIVEN a writable project root without an `cairn/` directory
- WHEN the operator runs `cairn init --root <path>`
- THEN the system creates `cairn/specs`, `cairn/changes`, and `cairn/archive`
- AND the JSON receipt identifies the `cairn` layout

#### Scenario: Prefer native layout

r[cairn.supersede.native_layout.prefer]
- GIVEN a repository containing both `cairn/` and legacy `openspec/` lifecycle content
- WHEN the operator runs read, status, validate, sync, or archive commands
- THEN the system uses the native `cairn/` layout by default
- AND the JSON receipt identifies the selected layout

#### Scenario: Preserve legacy fallback

r[cairn.supersede.native_layout.legacy_fallback]
- GIVEN a repository with legacy `openspec/` lifecycle content and no native `cairn/` directory
- WHEN the operator runs read, status, validate, sync, or archive commands
- THEN the system reads the legacy layout without requiring migration first

### Requirement: OpenSpec migration bridge

r[cairn.supersede.migrate_openspec] The system MUST provide a deterministic migration bridge from legacy `openspec/` layout to native `cairn/` layout.

#### Scenario: Dry-run migration plan

r[cairn.supersede.migrate_openspec.dry_run]
- GIVEN a repository with legacy `openspec/` lifecycle content
- WHEN the operator runs `cairn migrate openspec --root <path>`
- THEN the CLI emits a deterministic JSON migration plan
- AND no filesystem content is mutated

#### Scenario: Execute migration copy

r[cairn.supersede.migrate_openspec.execute]
- GIVEN a repository with legacy `openspec/` lifecycle content and no conflicting native destinations
- WHEN the operator runs `cairn migrate openspec --root <path> --execute`
- THEN the system copies accepted specs, active changes, and archived changes into native `cairn/` layout
- AND the JSON receipt reports `mutated=true`

#### Scenario: Block conflicting migration

r[cairn.supersede.migrate_openspec.conflict]
- GIVEN a legacy source path whose native destination already exists
- WHEN the operator runs migration with or without `--execute`
- THEN the plan reports the conflict as a blocker
- AND execution does not overwrite the destination

#### Scenario: Preserve FCIS migration boundary

r[cairn.supersede.migrate_openspec.boundary]
- GIVEN native layout and migration support
- WHEN `cairn-core` builds layout or migration decisions
- THEN it consumes only caller-provided path snapshots and performs no filesystem, environment, process, time, stdout/stderr, or JSON presentation effects
