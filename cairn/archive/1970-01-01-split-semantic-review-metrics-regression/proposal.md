## Why

`crates/cairn-core/src/verified/semantic_review/metrics_regression.rs` is the largest current maintainability hotspot. It mixes the public metrics regression rail entrypoints with registry validation, fixture-pair checks, guidance/task sync checks, sanitized evidence checks, and shared JSON helpers in one file. That makes future review-regression hardening riskier than it needs to be.

## What Changes

- Split the metrics regression rail into focused private modules while preserving the existing public API and receipt behavior.
- Keep registry, fixture coverage, guidance sync, evidence boundary, and shared helpers behind narrow seams.
- Add explicit tests proving the split boundary remains behavior-preserving.

## Impact

- **Files**: `crates/cairn-core/src/verified/semantic_review/metrics_regression*`, semantic-review specs/tasks.
- **Testing**: focused semantic review metrics regression tests, Cairn validation, Tracey coverage, and final Nix gate.
