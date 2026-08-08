## Why

The visible `cairn/` directory adds product metadata to the main repository file list. A hidden `.cairn/` directory keeps lifecycle data available without adding visual noise.

Existing repositories need an explicit migration path. Normal commands must not silently read the old directory because implicit fallback can hide incomplete migrations.

## What Changes

- Use `.cairn/` as the canonical lifecycle directory for initialization and all normal commands. r[cairn.lifecycle_directory.hidden_default]
- Add preview-first `cairn migrate cairn` support from the old `cairn/` directory to `.cairn/`. r[cairn.lifecycle_directory.legacy_migration]
- Keep the stable `layout` receipt value as `cairn` while paths identify `.cairn/`. r[cairn.lifecycle_directory.receipt_compatibility]
- Move maintained lifecycle content and fixtures to `.cairn/`, then update policy, tests, examples, and current documentation.

## Impact

- **Files**: lifecycle path helpers, migration CLI and MCP schemas, hard-coded shell paths, fixtures, current specs, policy, goldens, examples, and documentation
- **Compatibility**: existing `cairn/` content requires explicit `cairn migrate cairn`; normal commands do not use it as a fallback
- **Testing**: positive initialization and migration tests, negative fallback and conflict tests, focused lifecycle tests, policy freshness, validation, traceability, and the full repository gate
