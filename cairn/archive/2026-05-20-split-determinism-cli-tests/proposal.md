## Why

`crates/cairn-cli/tests/cli.rs` remains a large integration-test hotspot. The determinism/replay tests form a clear cluster near the top of the file and can be split into a dedicated integration test target without changing product behavior.

## What Changes

- Move determinism/replay/coverage-audit tests from `tests/cli.rs` into `tests/determinism.rs`.
- Preserve shared fixture behavior and avoid broad common-test-helper churn.
- Keep moved assertions at least as strict as the current tests.

## Scope

In scope: Reduce `tests/cli.rs` size and review friction., Keep determinism tests focused and discoverable., Run the new test target and the full workspace tests..

Out of scope: Changing CLI behavior or receipt schemas., Rewriting all CLI tests or centralizing all helper code..

## Verification

Run Cairn validation and proposal/design/tasks gates for this change package. During implementation, run focused tests for the moved or added command family, then the maintained repository gates before commit.
