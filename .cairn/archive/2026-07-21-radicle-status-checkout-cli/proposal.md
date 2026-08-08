# Proposal: Radicle Status and Checkout CLI

## Why

Cairn can import a native active change into a typed local Radicle COB, but operators cannot yet inspect that object against their working tree or safely materialize it back into native lifecycle files. Without a first-party read and checkout path, operators must inspect raw refs or copy files manually, which bypasses typed reduction, content-root checks, conflict visibility, current-authority checks, capability-bounded paths, and deterministic receipts.

## What Changes

- Add read-only `cairn radicle status <change>` over an explicit Radicle home and repository ID.
- Report typed object/state identity, semantic content identity, lifecycle and evidence staleness, content conflicts, current profile delegate authority, policy drift, and exact local materialization status.
- Add preview-first `cairn radicle checkout <change>` with explicit `--execute` mutation.
- Refuse checkout when content is conflicted, current profile authority or matching signer-possession proof is absent, inputs drift, the local destination differs, or an unsupported local entry exists.
- Materialize exact bytes through a capability-bounded prepared directory and atomic directory rename, then verify the resulting local view.
- Emit deterministic BLAKE3-bound receipts with bounded local-only claims.

## Impact

- **Core**: Add a pure status/checkout projection and local comparison plan under `crates/cairn-core/src/verified/`.
- **Adapter**: Add typed status loading and a current-authority checkout guard to `crates/cairn-radicle/`.
- **CLI**: Extend `crates/cairn-cli/src/radicle_change.rs`, capability operations, usage, and docs.
- **Testing**: Add positive exact/no-op cases and negative conflict, authority, drift, unsupported-entry, and overwrite cases; run focused, workspace, Clippy, formatting, purity, lifecycle, traceability, and Nix rails.
