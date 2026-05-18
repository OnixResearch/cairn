## Why

`cairn sync` and `cairn archive` currently expose only placeholder dry-run responses. Operators need deterministic, inspectable plans before any mutation implementation lands.

## What Changes

- Add typed pure planning models for sync and archive dry-runs.
- Have the CLI render those plans as stable JSON.
- Cover both successful planning and blocked/missing-evidence cases with fixture-backed tests.

## Non-Goals

- Performing sync/archive mutations.
- Moving or deleting repository files.
- Adding network or external service dependencies.
