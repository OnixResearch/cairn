# Split developer-experience CLI modules

## Summary

Split the developer-experience CLI implementation into focused modules while preserving the existing public command dispatch and receipt behavior.

## Rationale

`crates/cairn-cli/src/developer_experience.rs` mixes template listing/application, prompt-to-change scaffolding, reverse-spec discovery, filesystem helpers, and receipt construction in one file. The module is now the largest CLI source file and has become harder to review without changing behavior.

A behavior-preserving split keeps the `developer_experience` façade used by `main.rs` while moving cohesive command families into submodules.

## Scope

- Preserve the existing `template`, `scaffold change`, and `discover-specs` CLI behavior.
- Preserve receipt field names, mutation semantics, safe path checks, and policy IDs.
- Keep `crates/cairn-cli/src/developer_experience/mod.rs` as the façade called by `main.rs`.
- Move template, scaffold, discovery, and shared write helpers into focused submodules.

## Non-goals

- No user-facing CLI syntax change.
- No new developer-experience capability.
- No core planner changes.
- No broad test reorganization.
