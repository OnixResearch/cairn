## Why

The determinism and replay command family is still embedded in `crates/cairn-cli/src/main.rs`, which keeps the primary CLI shell large even after the evidence command extractions. A focused module boundary will make replay policy, receipt verification, and coverage-audit behavior easier to maintain without changing public CLI output.

## What Changes

- Move determinism/replay command parsing and handlers from `main.rs` into `crates/cairn-cli/src/determinism.rs`.
- Preserve all existing `cairn determinism ...` command names, JSON shapes, receipt hashes where inputs are unchanged, and failure issue codes.
- Export only the narrow bridge API needed by release-readiness or other command families, if any.

## Scope

In scope: Keep public determinism/replay CLI behavior byte-for-byte compatible where deterministic output is expected., Reduce `main.rs` by one cohesive command-family block., Run focused determinism tests before full gates..

Out of scope: Changing replay policy semantics, adding new replay cases, or changing receipt schemas., Moving unrelated agent-receipt or release-readiness code in the same slice..

## Verification

Run Cairn validation and proposal/design/tasks gates for this change package. During implementation, run focused tests for the moved or added command family, then the maintained repository gates before commit.
