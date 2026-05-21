## Why

The evidence registry test target covers build/verify, labels, mirror verification, selection, fetch-plan, and fetch-execute in one large file. Splitting keeps the registry proof chain readable while preserving end-to-end coverage.

Current target: `crates/cairn-cli/tests/evidence_registry.rs` (~1,129 lines).

## What Changes

- Add a behavior-preserving refactor package for evidence registry integration-test families.
- Define module/test boundaries before implementation so each split can be landed as a narrow verified slice.
- Keep public CLI behavior, deterministic JSON receipts, issue codes, fixture output, policy hashes, and Tracey references stable unless a later explicit Cairn changes them.

## Impact

- **Files**: `crates/cairn-cli/tests/evidence_registry.rs` (~1,129 lines) and any newly introduced sibling modules/test targets.
- **Testing**: focused registry test targets plus the full registry/evidence chain checks, then repository validation, Tracey coverage, Clippy, and diff checks.
- **Compatibility**: No public command, schema, or receipt semantics are intended to change.
