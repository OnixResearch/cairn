## Why

`cairn` is a strong working name, but it reads as an implementation/detail-first fork of OpenSpec. The project is now its own lifecycle engine with native layout, deterministic receipts, Nickel policy, gates, traceability, and FCIS enforcement. `Cairn` is a more natural, memorable name that preserves the core metaphor: durable markers that guide progress through a trail of changes, specs, evidence, sync, and archive history.

## What Changes

- Establish `Cairn` as the canonical public project, CLI, package, crate, docs, and lifecycle identity.
- Keep legacy `cairn` command/config/layout references only as explicit compatibility aliases during a bounded migration window.
- Require deterministic rename evidence that classifies any remaining `cairn` strings as canonical rename targets, compatibility aliases, historical/archive content, or intentional philosophy references.
- Update user-facing docs and receipts so operators see `cairn` for normal workflows once the implementation lands.

## Capabilities

### Modified Capabilities

- `cairn-supersession`: Rename the supersession product identity from `cairn` to `Cairn` while preserving the native lifecycle/gate/receipt behavior contracts.

## Impact

- **Files**: Cargo package/crate metadata, binary/app names, Nix flake packages/apps/checks, README/docs, repo-maintained Hermes skill, native `cairn/` lifecycle surfaces, fixtures/goldens, tests, and migration docs.
- **APIs**: CLI command name changes from `cairn` to `cairn`; legacy `cairn` may remain as an alias with explicit receipts and deprecation language.
- **Compatibility**: Existing native lifecycle content under `cairn/` must either be migrated to `cairn/` or supported by an explicit compatibility bridge; no implicit ambiguous fallback.
- **Testing**: Focused CLI/golden tests, rename classification evidence, policy freshness checks, Tracey coverage, `cargo test --workspace`, and `nix flake check path:$PWD -L`.
