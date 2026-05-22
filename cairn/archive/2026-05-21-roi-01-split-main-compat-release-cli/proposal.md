# Split remaining main CLI compatibility and release helpers

## Problem

`main.rs` is still the largest CLI source file after earlier extractions. It now mainly carries dispatch plus compatibility, release-readiness, rename-classification, migration, plan, and receipt presentation seams that can be isolated without public behavior changes.

## Proposed Change

Extract the remaining compatibility/release helper families from `main.rs` into focused modules while keeping top-level command dispatch small and preserving existing CLI output, receipts, issue codes, and bridge APIs.

## Scope

This is an ROI-ranked planning package for `crates/cairn-cli/src/main.rs` (~680 lines). The implementation should remain narrow and behavior-preserving unless a later change explicitly updates public contracts.

## Non-Goals

- No public command, receipt schema, migration, or readiness semantic changes.
- No broad helper cleanup outside the compatibility/release seams.
- No implementation of unrelated release evidence generation.

## Verification

- Run the focused checks named in the task list.
- Run native Cairn validation and Tracey coverage.
- Run formatting/linting/diff checks appropriate to the touched Rust/Nix files.
