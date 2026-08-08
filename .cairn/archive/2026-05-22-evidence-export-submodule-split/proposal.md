# Proposal: Split evidence export source into focused modules

## Summary

Create a behavior-preserving split plan for `crates/cairn-cli/src/evidence/export.rs` (16.4KB) so the remaining large CLI/review surface is decomposed into focused modules without changing public command behavior, receipt semantics, issue codes, or policy contracts.

## Problem

`crates/cairn-cli/src/evidence/export.rs` is one of the largest remaining Cairn Rust files after the recent command-family splits. Its size increases review cost, makes future CLI changes more conflict-prone, and concentrates unrelated responsibilities behind one source target.

## Proposed change

Split the target into a small facade plus cohesive sibling modules covering bundle assembly, artifact materialization, manifest binding, release-readiness collection, and receipt rendering. Keep helper APIs private or `pub(super)` by default, and expose only narrow `pub(crate)` bridge functions required by existing consumers.

## Non-goals

- Change public CLI flags, command names, receipt schemas, policy semantics, or issue codes.
- Combine this refactor with new release/evidence/lifecycle capabilities.
- Introduce broad common helper modules or crate-root re-export shims.

## Verification

Use focused behavior checks for this family (evidence export integration tests, manifest generation checks, and release-readiness checks), then run repository formatting, tests, linting, native Cairn validation, Tracey coverage, and diff checks before sync/archive.
