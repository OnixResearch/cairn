# Proposal: Split native lifecycle CLI source into focused modules

## Summary

Create a behavior-preserving split plan for `crates/cairn-cli/src/native.rs` (13.7KB) so the remaining large CLI/review surface is decomposed into focused modules without changing public command behavior, receipt semantics, issue codes, or policy contracts.

## Problem

`crates/cairn-cli/src/native.rs` is one of the largest remaining Cairn Rust files after the recent command-family splits. Its size increases review cost, makes future CLI changes more conflict-prone, and concentrates unrelated responsibilities behind one source target.

## Proposed change

Split the target into a small facade plus cohesive sibling modules covering change creation, validation/doctor read models, gate execution, sync/archive mutation shell, and receipt presentation. Keep helper APIs private or `pub(super)` by default, and expose only narrow `pub(crate)` bridge functions required by existing consumers.

## Non-goals

- Change public CLI flags, command names, receipt schemas, policy semantics, or issue codes.
- Combine this refactor with new release/evidence/lifecycle capabilities.
- Introduce broad common helper modules or crate-root re-export shims.

## Verification

Use focused behavior checks for this family (lifecycle and lifecycle_mutations integration tests plus native validate/gate smoke checks), then run repository formatting, tests, linting, native Cairn validation, Tracey coverage, and diff checks before sync/archive.
