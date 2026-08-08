# Split circuit primitive content-addressed core validation

## Problem

Circuit primitive receipt validation is the largest pure core file. It combines topology, component, unit, hash, issue-code, and negative-fixture validation concerns in one module, increasing review risk for content-addressed evidence work.

## Proposed Change

Split the circuit primitive validator into focused pure core modules while preserving the receipt contract, BLAKE3/canonical hash behavior, issue codes, and positive/negative validation fixtures.

## Scope

This is an ROI-ranked planning package for `crates/cairn-core/src/verified/content_addressed/circuit_primitive.rs` (~566 lines). The implementation should remain narrow and behavior-preserving unless a later change explicitly updates public contracts.

## Non-Goals

- No circuit primitive receipt schema, hash, issue-code, or semantic validation changes.
- No filesystem, environment, JSON presentation, or CLI orchestration in core.
- No ownership expansion beyond Cairn receipt validation contracts.

## Verification

- Run the focused checks named in the task list.
- Run native Cairn validation and Tracey coverage.
- Run formatting/linting/diff checks appropriate to the touched Rust/Nix files.
