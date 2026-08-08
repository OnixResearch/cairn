# Design: Split Repo Core

## Boundaries

Keep `crates/cairn-core/src/verified/repo/mod.rs` as the public facade and move implementation into cohesive pure-core submodules:

- `types.rs`: public DTO/read-model structs.
- `status.rs`: repository status assembly.
- `changes.rs`: change status/list/detail read models and filtering.
- `metadata.rs`: active-change metadata parsing and policy validation.
- `specs.rs`: spec list/detail summaries and title extraction.
- `tasks.rs`: task marker counting.

## Behavior preservation

The facade re-exports the same public functions and structs used by `cairn-core`, `cairn-cli`, and downstream callers. Existing unit tests remain attached to the facade so they exercise the public module API after the split.

## Verification

Run focused repo-core unit tests first, then formatting, core/workspace tests, clippy, native Cairn validation, Tracey coverage, and flake checks.
