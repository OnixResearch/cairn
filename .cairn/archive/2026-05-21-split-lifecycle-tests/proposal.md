## Why

The first lifecycle test split removed lifecycle coverage from `cli.rs`, but the lifecycle target now mixes init/read/validate, gates, sync/archive, migration, and mutation manifests. Further splitting improves focused verification.

Current target: `crates/cairn-cli/tests/lifecycle.rs` (~968 lines).

## What Changes

- Add a behavior-preserving refactor package for native lifecycle integration-test subfamilies.
- Define module/test boundaries before implementation so each split can be landed as a narrow verified slice.
- Keep public CLI behavior, deterministic JSON receipts, issue codes, fixture output, policy hashes, and Tracey references stable unless a later explicit Cairn changes them.

## Impact

- **Files**: `crates/cairn-cli/tests/lifecycle.rs` (~968 lines) and any newly introduced sibling modules/test targets.
- **Testing**: new lifecycle-focused test targets plus the full lifecycle suite during transition, then repository validation, Tracey coverage, Clippy, and diff checks.
- **Compatibility**: No public command, schema, or receipt semantics are intended to change.
