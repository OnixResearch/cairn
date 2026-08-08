# Validation: Adopt bounded tree

Date: 2026-08-04

## Baseline

The change branch started at `cd4a3c7a87aeb9327f946314458f56af86290f53` (current `origin/main`).

## Prerequisite

`bounded-tree` archived `establish-bounded-tree` and published a passing immutable Radicle revision `43d3d0be7eb9ad86f815d323a0bf92553d5606f7` (RID `rad:zqhtZvsteJhxCJE96dMAZSZ9y1PX`, branch `cairn/establish-bounded-tree-20260803`). `devenv test` passed on 2026-08-03.

## Dependency pin

`bounded-tree-core` and `bounded-tree-cap` are vendored under `vendor/` at the pinned revision and declared as workspace path deps in `cairn-cli`. There is no sibling path, mutable branch, or GitHub fallback.

## Implemented surface

The archive compaction scan admits each repository-relative path through `bounded_tree_core::RelativePath` exact-byte admission inside `cairn-cli`. A malformed member emits a bounded-path issue and the scan continues; accepted regular-file members become existing pure `ArchiveFileInput` values in `cairn-core`. Depth, file-count, per-file, total-byte, symlink, and special-file policy plus issue identifiers and ordering remain unchanged.

## Parity

The shared scan preserves accepted archive inputs and deterministic order, and retains link and special-file issue identifiers. Two parity tests cover the positive and negative cases.

## Evidence boundary

The change records only the exact pinned dependency and bounded observation result. It does not claim archive-truth correctness, semantic-review completeness, or release eligibility.

## Commands and results

- `cargo test --workspace`: all 51 suites pass, 0 failures.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `nix run path:$PWD#cairn -- policy export --check`: passed.
- proposal, design, and tasks gates: `verdict: PASS`.
- Traceability: valid, no missing or dangling references.
- `cairn validate --root .`, agent-receipt MCP smoke, and release readiness: `valid: true`.

## Claim boundary

This change proves only that Cairn's archive compaction scan admits paths through the shared bounded-tree exact-byte admission and preserves its own policy and issue order. It does not prove that `bounded-tree` is correct, that an archive scan is an atomic snapshot, or that a release is eligible. Those authorities remain Cairn's.

## Lifecycle evidence

Archive execution receipt: `27357cbc07376089536cff9599ae2e746a569fdc3dc3a76b7a5a8a42775a6968`.

Post-archive validation, traceability, and release readiness passed. The accepted specification is `cairn/specs/bounded-tree-adoption/spec.md`.
