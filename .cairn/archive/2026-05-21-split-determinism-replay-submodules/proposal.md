# Split determinism replay CLI source into focused submodules

## Problem

`crates/cairn-cli/src/determinism/replay.rs` is the largest remaining CLI source hotspot. It combines option parsing, policy case/group decoding, entropy/environment attestation, replay execution, receipt assembly, output rendering, and fixture-copy helpers in one file, making deterministic replay changes harder to review.

## Proposed Change

Split `determinism/replay.rs` into a facade plus focused submodules while preserving the public `cairn determinism replay` behavior, receipt shape, issue codes, policy decoding, and existing tests.

## Non-Goals

- No receipt schema, policy, fixture, or golden behavior changes.
- No new replay cases or entropy policy semantics.
- No broad helper extraction outside the determinism replay module tree.
