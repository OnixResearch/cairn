## Why

The determinism replay, coverage-audit, verify-receipt, entropy, and policy surfaces share one large shell file. Splitting by surface makes receipt changes easier to audit without changing replay semantics.

Current target: `crates/cairn-cli/src/determinism.rs` (~1,758 lines).

## What Changes

- Add a behavior-preserving refactor package for determinism replay, verify-receipt, coverage-audit, policy, and entropy command families.
- Define module/test boundaries before implementation so each split can be landed as a narrow verified slice.
- Keep public CLI behavior, deterministic JSON receipts, issue codes, fixture output, policy hashes, and Tracey references stable unless a later explicit Cairn changes them.

## Impact

- **Files**: `crates/cairn-cli/src/determinism.rs` (~1,758 lines) and any newly introduced sibling modules/test targets.
- **Testing**: focused determinism integration tests plus replay/receipt smoke commands, then repository validation, Tracey coverage, Clippy, and diff checks.
- **Compatibility**: No public command, schema, or receipt semantics are intended to change.
