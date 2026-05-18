## Why

`cairn` should become the primary OpenSpec-compatible engine rather than a helper around the upstream `openspec` CLI. Operators need a deterministic Rust/Nix tool that can initialize repositories, create changes, validate lifecycle state, and execute sync/archive with stronger safety rails.

## What Changes

- Add the first OpenSpec-compatible authoring commands to `cairn`.
- Establish `cairn` as the authority for new repository/change creation while preserving the existing `openspec/` directory layout.
- Keep all filesystem writes explicit, receipt-producing, and shell-layer only.

## Capabilities

### New Capabilities
- `cairn-init`: Initialize an OpenSpec-compatible repository layout.
- `cairn-change-create`: Create a named change with the required artifact skeleton.
- `cairn-supersession`: Document the compatibility path for superseding OpenSpec.

## Impact

- **Files**: CLI shell, pure lifecycle templates, tests, README, accepted supersession spec.
- **APIs**: Adds `cairn init` and `cairn change create` commands.
- **Testing**: Fixture/temp-repo CLI tests, full Rust/Nix/Slotcar/OpenSpec verification.
