# cairn read parity

## Why

`cairn` can now initialize repositories and create changes, but operators still need upstream OpenSpec-style read commands to discover active changes and accepted specs before `cairn` can replace OpenSpec in daily use.

## What Changes

- Add `cairn change list` for deterministic active-change summaries.
- Add `cairn change show <name>` for a deterministic named-change receipt.
- Add `cairn spec list` for deterministic accepted-spec summaries.
- Add `cairn spec show <id>` for a deterministic accepted-spec receipt.
- Keep filesystem reads and JSON presentation in the CLI shell while pure sorting/model decisions live in `cairn-core`.

## Capabilities

### New Capabilities
- `cairn-read-parity`: Operators can inspect OpenSpec-compatible changes and specs without the upstream CLI.

### Modified Capabilities
- `cairn-supersession`: Extends supersession from authoring parity to read-side workflow parity.

## Impact

- **Files**: CLI shell, pure core read models, tests/goldens, README, self-dogfood check, accepted supersession spec.
- **APIs**: Adds `change list`, `change show`, `spec list`, and `spec show` CLI commands.
- **Dependencies**: No new runtime dependencies.
- **Testing**: CLI integration tests, canonical JSON goldens, self-dogfood, core purity search, Slotcar, and flake checks.
