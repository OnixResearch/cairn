## Why

Agent receipt validation, bundle verification, smoke receipts, bounded IO checks, and issue construction share one CLI shell file. Splitting preserves receipt semantics while simplifying review.

Current target: `crates/cairn-cli/src/agent_receipt.rs` (~762 lines).

## What Changes

- Add a behavior-preserving refactor package for agent receipt validation, bundle verification, smoke, bounded IO, and issue helpers.
- Define module/test boundaries before implementation so each split can be landed as a narrow verified slice.
- Keep public CLI behavior, deterministic JSON receipts, issue codes, fixture output, policy hashes, and Tracey references stable unless a later explicit Cairn changes them.

## Impact

- **Files**: `crates/cairn-cli/src/agent_receipt.rs` (~762 lines) and any newly introduced sibling modules/test targets.
- **Testing**: agent receipt focused integration tests, MCP smoke bridge, and release-readiness bundle checks, then repository validation, Tracey coverage, Clippy, and diff checks.
- **Compatibility**: No public command, schema, or receipt semantics are intended to change.
