# Split semantic-review core boundaries

## Why

`crates/cairn-core/src/verified/semantic_review/mod.rs` remains a large review surface because its public facade also carries semantic-review regression tests and fixtures.

## What

Move the tests/fixtures behind a dedicated `tests.rs` submodule while preserving the existing facade exports, schema constants, public DTOs, Tracey refs, receipt behavior, and metrics-regression coverage.

## Impact

No semantic-review capability or schema changes are intended. This is a maintainability-only split that completes the existing accepted semantic-review core split boundary with focused verification.
