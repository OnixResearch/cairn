# Split Repo Core

## Summary

Split `crates/cairn-core/src/verified/repo.rs` into a focused `repo/` module tree while preserving the existing public `verified::repo` facade, DTOs, and behavior.

## Rationale

`repo.rs` is the largest remaining pure-core file and mixes repository status construction, active-change read models, metadata parsing/filtering, spec summaries, and task counting. A focused module split reduces review surface without changing CLI behavior or receipt shapes.

## Non-goals

- No public CLI behavior changes.
- No schema, receipt, or JSON presentation changes.
- No filesystem or shell behavior added to `cairn-core`.
