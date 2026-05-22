# Split determinism integration tests into focused targets

## Problem

The determinism integration test target still concentrates replay, saved receipt verification, coverage audit, policy/group/entropy, and receipt-hash assertions in one large review surface.

## Proposed Change

Split determinism integration coverage into focused modules or targets while preserving replay receipts, portable hashes, issue codes, policy linkage, and existing fixture expectations.

## Scope

This is an ROI-ranked planning package for `crates/cairn-cli/tests/determinism.rs` (~584 lines). The implementation should remain narrow and behavior-preserving unless a later change explicitly updates public contracts.

## Non-Goals

- No replay, policy, entropy, coverage-audit, or receipt schema changes.
- No new deterministic surfaces or replay cases.
- No broad fixture regeneration except when exact moved assertions require it.

## Verification

- Run the focused checks named in the task list.
- Run native Cairn validation and Tracey coverage.
- Run formatting/linting/diff checks appropriate to the touched Rust/Nix files.
