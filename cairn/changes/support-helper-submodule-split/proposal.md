# Proposal: Split CLI support helpers into focused modules

## Summary

Create a behavior-preserving split plan for `crates/cairn-cli/src/support.rs` (22.7KB) so the remaining large CLI/review surface is decomposed into focused modules without changing public command behavior, receipt semantics, issue codes, or policy contracts.

## Problem

`crates/cairn-cli/src/support.rs` is one of the largest remaining Cairn Rust files after the recent command-family splits. Its size increases review cost, makes future CLI changes more conflict-prone, and concentrates unrelated responsibilities behind one source target.

## Proposed change

Split the target into a small facade plus cohesive sibling modules covering usage/help text, argument helpers, path/report helpers, and shared support DTOs. Keep helper APIs private or `pub(super)` by default, and expose only narrow `pub(crate)` bridge functions required by existing consumers.

## Non-goals

- Change public CLI flags, command names, receipt schemas, policy semantics, or issue codes.
- Combine this refactor with new release/evidence/lifecycle capabilities.
- Introduce broad common helper modules or crate-root re-export shims.

## Verification

Use focused behavior checks for this family (support usage smoke, CLI help tests, lifecycle integration tests, and full workspace gates), then run repository formatting, tests, linting, native Cairn validation, Tracey coverage, and diff checks before sync/archive.
