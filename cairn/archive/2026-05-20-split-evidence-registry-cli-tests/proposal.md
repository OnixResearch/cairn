# split-evidence-registry-cli-tests

## Why

`crates/cairn-cli/tests/cli.rs` still carries the evidence/registry end-to-end chain as a very large integration-test island. The test is valuable, but its size makes future evidence-registry changes harder to review and raises the blast radius of policy/fixture churn.

## What Changes

- Move the cohesive evidence export/import/republish/index/registry/mirror/select/fetch-plan/fetch-execute integration coverage from `cli.rs` into a dedicated integration test file.
- Preserve existing assertions, helper behavior, public CLI behavior, JSON receipt shapes, and fixture/golden contracts.
- Keep only shared/general CLI tests in `cli.rs`.

## Non-Goals

- No CLI behavior changes.
- No policy/schema changes.
- No fixture/golden refresh unless required by the move itself.
- No broad test-helper framework extraction beyond the minimum needed by the split.

## Verification

- Focused evidence-registry integration tests pass from the new test target.
- Existing CLI integration tests pass after the moved coverage is removed from `cli.rs`.
- Workspace format, clippy, tests, Cairn validate, Tracey coverage, and diff checks pass.
