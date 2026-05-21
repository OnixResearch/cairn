## Why

The evidence export/verify/import/republish/index command family has become the largest CLI implementation file. Splitting it lowers review risk before adding more evidence publication and consumption rails.

Current target: `crates/cairn-cli/src/evidence.rs` (~2,039 lines).

## What Changes

- Add a behavior-preserving refactor package for evidence export, verify, import, republish, and index command families.
- Define module/test boundaries before implementation so each split can be landed as a narrow verified slice.
- Keep public CLI behavior, deterministic JSON receipts, issue codes, fixture output, policy hashes, and Tracey references stable unless a later explicit Cairn changes them.

## Impact

- **Files**: `crates/cairn-cli/src/evidence.rs` (~2,039 lines) and any newly introduced sibling modules/test targets.
- **Testing**: focused evidence CLI tests plus full CLI/evidence-registry integration tests, then repository validation, Tracey coverage, Clippy, and diff checks.
- **Compatibility**: No public command, schema, or receipt semantics are intended to change.
