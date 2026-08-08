## Why

Cairn migration previews expose content hashes, but executed migrations do not bind mutation to the reviewed preview. A source change between preview and execution can change the copied result. Normal commands also report empty lifecycle state when only legacy `cairn/` content exists.

## What Changes

- Add a stable BLAKE3 `plan_hash` to migration previews.
- Require `--plan-hash HASH` for migration execution.
- Recheck source bytes and destination absence before no-replace publication.
- Reject normal lifecycle commands with a migration-required diagnostic when only legacy content exists.

## Impact

- **Files**: lifecycle migration core and shell, root command admission, CLI usage, specifications, documentation, and positive and negative tests
- **Testing**: focused migration tests, lifecycle command diagnostics, workspace tests, Clippy, formatting, policy freshness, traceability, and Nix checks
