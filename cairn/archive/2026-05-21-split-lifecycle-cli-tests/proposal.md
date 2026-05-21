## Why

`crates/cairn-cli/tests/cli.rs` still owns a large lifecycle/init/change/sync/archive/migration cluster even after the lifecycle command implementation moved out of `main.rs`. A focused lifecycle integration-test target will make the native Cairn lifecycle surface easier to verify and review without changing public behavior.

## What Changes

- Move lifecycle/init/change/status/validate/gate/sync/archive/migration tests from `tests/cli.rs` into `tests/lifecycle.rs`.
- Preserve existing golden/fixture assertions and deterministic receipt checks.
- Keep helper duplication narrow and avoid broad shared test-support refactors.

## Scope

In scope: behavior-preserving integration-test topology, focused lifecycle test target, verification that the catch-all CLI test shrinks while lifecycle coverage remains intact.

Out of scope: changing lifecycle CLI behavior, receipt schemas, fixtures, accepted specs, or command semantics.

## Verification

Run native Cairn validation and proposal/design/tasks gates for this change package. During implementation, run the new focused lifecycle test target, the remaining CLI tests, workspace tests, Cairn validate, Tracey coverage, and diff checks before commit.
