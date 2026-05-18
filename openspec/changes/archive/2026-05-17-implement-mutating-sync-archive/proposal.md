## Why

`cairn sync` and `cairn archive` now produce deterministic dry-run plans, but operators still need a safe explicit path to execute those plans. Without a specified mutation mode, implementation risks bypassing the FCIS boundary or introducing unclear data-loss behavior.

## What Changes

- Add explicit non-dry-run execution modes for `sync` and `archive`.
- Preserve dry-run-by-default behavior and stable JSON receipts.
- Execute only actions that are present in the pure core plan and only when the plan is not blocked.
- Emit mutation receipts that bind requested change, actions attempted, actions completed, and output paths.

## Out of Scope

- Automatic conflict resolution for spec merges.
- Remote publishing, git commits, or pushing.
- Changing OpenSpec semantics beyond applying already-planned local filesystem moves/copies.

## Capabilities

### Modified Capabilities
- `sync-archive`: Adds explicit opt-in execution for previously dry-run-only plans.

## Impact

- **Files**: CLI shell execution path, pure plan/receipt models, fixtures/tests, README.
- **APIs**: CLI flags for non-dry-run execution; JSON receipt additions.
- **Testing**: temp-repo fixture integration tests for successful mutation and blocked no-op cases.
