## MODIFIED Requirements

### Requirement: Cairn repository initialization
r[cairn.supersede.init] The system MUST initialize the native `cairn/` repository layout without requiring the upstream OpenSpec CLI.

#### Scenario: Initialize empty repository
r[cairn.supersede.init.empty]
- GIVEN a writable project root without an `cairn/` directory
- WHEN the operator runs `cairn init --root <path>`
- THEN the system creates `cairn/specs`, `cairn/changes`, and `cairn/archive`
- AND the JSON receipt identifies the `cairn` layout

### Requirement: Cairn native lifecycle layout
r[cairn.supersede.native_layout] The system MUST use the native `cairn/` lifecycle layout for normal runtime commands and MUST NOT silently fall back to legacy `openspec/` lifecycle content.

#### Scenario: Use native layout only
r[cairn.supersede.native_layout.native_only]
- GIVEN a repository containing only legacy `openspec/` lifecycle content
- WHEN the operator runs read, status, doctor, validate, gate, sync, or archive commands
- THEN the command evaluates the native `cairn/` layout and does not read legacy lifecycle content

#### Scenario: Native layout is selected when both roots exist
r[cairn.supersede.native_layout.prefer]
- GIVEN a repository containing both `cairn/` and legacy `openspec/` lifecycle content
- WHEN the operator runs read, status, doctor, validate, sync, or archive commands
- THEN the system uses the native `cairn/` layout
- AND the JSON receipt identifies the selected layout

### Requirement: OpenSpec migration bridge
r[cairn.supersede.migrate_openspec] The system MUST provide a deterministic migration bridge from legacy `openspec/` layout to native `cairn/` layout without using legacy content as an implicit runtime fallback.

#### Scenario: Migration remains explicit
- GIVEN a repository containing only legacy `openspec/` lifecycle content
- WHEN the operator runs `cairn migrate openspec --root <path>`
- THEN the CLI emits a deterministic migration plan from legacy `openspec/` paths into native `cairn/` paths
- AND normal runtime commands do not need to read legacy content before migration executes