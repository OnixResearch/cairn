# Proposal: Split test harness source into focused modules

## Summary

Create a behavior-preserving split plan for `crates/cairn-cli/src/test_harness.rs` (15.5KB) so the remaining large CLI/review surface is decomposed into focused modules without changing public command behavior, receipt semantics, issue codes, or policy contracts.

## Problem

`crates/cairn-cli/src/test_harness.rs` is one of the largest remaining Cairn Rust files after the recent command-family splits. Its size increases review cost, makes future CLI changes more conflict-prone, and concentrates unrelated responsibilities behind one source target.

## Proposed change

Split the target into a small facade plus cohesive sibling modules covering case definitions, rail execution, output classification, artifact hashing, and receipt rendering. Keep helper APIs private or `pub(super)` by default, and expose only narrow `pub(crate)` bridge functions required by existing consumers.

## Non-goals

- Change public CLI flags, command names, receipt schemas, policy semantics, or issue codes.
- Combine this refactor with new release/evidence/lifecycle capabilities.
- Introduce broad common helper modules or crate-root re-export shims.

## Verification

Use focused behavior checks for this family (test harness unit tests plus repository validation and full workspace gates), then run repository formatting, tests, linting, native Cairn validation, Tracey coverage, and diff checks before sync/archive.
