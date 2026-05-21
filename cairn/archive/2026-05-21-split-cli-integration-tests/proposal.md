## Why

The residual catch-all CLI integration target still mixes MCP, agent receipt, release-readiness, public schema, manifest, and bundle checks. Smaller test targets make focused verification faster and failures easier to localize.

Current target: `crates/cairn-cli/tests/cli.rs` (~1,671 lines).

## What Changes

- Add a behavior-preserving refactor package for remaining monolithic CLI integration-test families.
- Define module/test boundaries before implementation so each split can be landed as a narrow verified slice.
- Keep public CLI behavior, deterministic JSON receipts, issue codes, fixture output, policy hashes, and Tracey references stable unless a later explicit Cairn changes them.

## Impact

- **Files**: `crates/cairn-cli/tests/cli.rs` (~1,671 lines) and any newly introduced sibling modules/test targets.
- **Testing**: new focused integration-test targets plus the remaining `cli` target, then repository validation, Tracey coverage, Clippy, and diff checks.
- **Compatibility**: No public command, schema, or receipt semantics are intended to change.
