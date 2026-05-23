# Design

## Module boundary

Keep `crates/cairn-core/src/verified/semantic_review/mod.rs` as the compatibility facade containing public schema constants, policy enum constants, report DTOs, re-exports, and existing unit tests.

Move cohesive behavior into focused pure-core modules:

- `profile.rs`: semantic-review policy profile JSON.
- `review_receipt.rs`: review receipt hashing and validation.
- `revision_handoff.rs`: revision handoff hashing and validation.
- `analytics.rs`: review ledger aggregation receipt construction.
- `completeness.rs`: completeness receipt construction.
- `common.rs`: narrow shared validation/hash/count helpers.

## Non-goals

- No receipt schema change.
- No issue-code, field-path, or hash-material change.
- No CLI behavior change.
- No filesystem, environment, process, time, async, or JSON-presentation shell code in `cairn-core`.

## Verification

Run focused semantic-review tests, formatting, core/workspace checks, native Cairn validation, Tracey coverage, and diff checks. Behavior-preserving success means tests and deterministic Cairn gates pass without fixture or policy drift.
