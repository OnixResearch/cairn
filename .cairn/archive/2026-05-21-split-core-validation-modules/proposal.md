## Why

Core validation is pure but combines requirement parsing, scenario clause validation, active-change validation, and issue construction. Smaller pure modules make FCIS review and future validators safer.

Current target: `crates/cairn-core/src/verified/validation.rs` (~792 lines).

## What Changes

- Add a behavior-preserving refactor package for pure validation core helpers and issue construction.
- Define module/test boundaries before implementation so each split can be landed as a narrow verified slice.
- Keep public CLI behavior, deterministic JSON receipts, issue codes, fixture output, policy hashes, and Tracey references stable unless a later explicit Cairn changes them.

## Impact

- **Files**: `crates/cairn-core/src/verified/validation.rs` (~792 lines) and any newly introduced sibling modules/test targets.
- **Testing**: core validation unit tests, lifecycle validation integration tests, and FCIS purity smoke, then repository validation, Tracey coverage, Clippy, and diff checks.
- **Compatibility**: No public command, schema, or receipt semantics are intended to change.
