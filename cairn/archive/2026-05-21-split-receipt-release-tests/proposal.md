## Why

The residual catch-all CLI integration target still mixes public receipt schema checks, agent receipt replay evidence, release-readiness bundle checks, and signed evidence manifest validation with unrelated CLI error smoke tests. Splitting this cohesive receipt/release family out of `crates/cairn-cli/tests/cli.rs` keeps receipt regressions easy to localize and reduces the monolithic target's review surface.

Current target: `crates/cairn-cli/tests/cli.rs` (~1,367 lines after the MCP split).

## What Changes

- Add a behavior-preserving split for receipt, agent-evidence, release-readiness, and signed-manifest CLI integration tests.
- Move the cohesive family into a focused sibling integration target such as `crates/cairn-cli/tests/release_receipts.rs`.
- Keep public CLI behavior, deterministic JSON receipts, issue codes, fixture output, policy hashes, and Tracey references stable.

## Impact

- **Files**: `crates/cairn-cli/tests/cli.rs` and a new focused receipt/release integration-test target.
- **Testing**: focused receipt/release target plus remaining `cli` target, then repository validation, Tracey coverage, Clippy, and diff checks.
- **Compatibility**: No public command, schema, or receipt semantics are intended to change.
