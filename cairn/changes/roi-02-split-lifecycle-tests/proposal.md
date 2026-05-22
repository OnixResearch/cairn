# Split lifecycle integration tests into focused targets

## Problem

The lifecycle integration test target remains the largest test file. It mixes creation, sync, archive, mutation-manifest, and validation flows, making narrow lifecycle behavior changes harder to review and rerun.

## Proposed Change

Split lifecycle integration coverage into focused test modules or targets while preserving fixture setup, command behavior, mutation manifests, and existing positive/negative coverage.

## Scope

This is an ROI-ranked planning package for `crates/cairn-cli/tests/lifecycle.rs` (~632 lines). The implementation should remain narrow and behavior-preserving unless a later change explicitly updates public contracts.

## Non-Goals

- No lifecycle command, sync/archive, mutation-manifest, or fixture semantic changes.
- No broad test-support framework rewrite.
- No weakening of existing lifecycle negative coverage.

## Verification

- Run the focused checks named in the task list.
- Run native Cairn validation and Tracey coverage.
- Run formatting/linting/diff checks appropriate to the touched Rust/Nix files.
