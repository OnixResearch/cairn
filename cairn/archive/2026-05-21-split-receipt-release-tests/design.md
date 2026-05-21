## Context

`crates/cairn-cli/tests/cli.rs` still owns several receipt/release-focused tests after lifecycle and MCP splits. These tests share local helpers for policy schema lookup, JSON command execution, temp fixtures, and receipt hash comparisons, but they are cohesive enough to live in one focused integration target.

## Decisions

- Move only the receipt/release family in this slice; leave unrelated missing-spec and invalid-policy smoke tests in `cli.rs`.
- Duplicate a narrow set of local test helpers in the new target instead of introducing broad shared mutable support.
- Preserve existing Tracey references on the moved tests and add maintainability refs for the new accepted split requirements.
- Run both the new focused target and the remaining `cli` target before full repository gates.

## Non-Goals

- No public CLI behavior changes.
- No receipt schema, fixture, issue-code, or policy hash changes.
- No broad common test-support framework in this slice.
