## Why

After command-family splits, `main.rs` still owns release-readiness, rename-classification, migration, plan JSON, and generic receipt helpers. Extracting these leaves `main.rs` closer to dispatch-only.

Current target: `crates/cairn-cli/src/main.rs` (~1,258 lines).

## What Changes

- Add a behavior-preserving refactor package for release-readiness, rename-classification, migration, and plan/receipt presentation helpers.
- Define module/test boundaries before implementation so each split can be landed as a narrow verified slice.
- Keep public CLI behavior, deterministic JSON receipts, issue codes, fixture output, policy hashes, and Tracey references stable unless a later explicit Cairn changes them.

## Impact

- **Files**: `crates/cairn-cli/src/main.rs` (~1,258 lines) and any newly introduced sibling modules/test targets.
- **Testing**: release-readiness, migration/lifecycle, rename-classification, and full CLI integration checks, then repository validation, Tracey coverage, Clippy, and diff checks.
- **Compatibility**: No public command, schema, or receipt semantics are intended to change.
