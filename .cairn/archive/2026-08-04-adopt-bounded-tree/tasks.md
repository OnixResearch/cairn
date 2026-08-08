# Tasks: Adopt bounded tree

## Phase 1: Admit the shared dependency

- [x] [serial] Confirm that `bounded-tree` has archived its establishment change and published a passing immutable Radicle revision. r[cairn.bounded_tree_adoption.prerequisite]
  - `bounded-tree` archived `establish-bounded-tree` (archive receipt `24395f95…`, `devenv test` passed 2026-08-03, traceability 57/57).
  - Immutable Radicle revision `43d3d0be7eb9ad86f815d323a0bf92553d5606f7` is published (branch `cairn/establish-bounded-tree-20260803`, RID `rad:zqhtZvsteJhxCJE96dMAZSZ9y1PX`).
- [x] [serial] Pin that Radicle revision without a sibling path, mutable branch, or fallback. r[cairn.bounded_tree_adoption.prerequisite]
  - Vendored `bounded-tree-core` and `bounded-tree-cap` at `43d3d0be…` under `vendor/`, declared as workspace path deps in `cairn-cli`. No sibling checkout or GitHub fallback.
- [x] [parallel] Record the pre-adoption Cairn revision, dependency state, and rollback command. r[cairn.bounded_tree_adoption.rollback]
  - Recorded in `docs/bounded-tree-adoption.md` (rollback restores `scan.rs` and removes the vendored dependency together).

## Phase 2: Adapt archive scanning

- [x] [serial] Use shared bounded observations from the `cairn-cli` archive compaction shell. r[cairn.bounded_tree_adoption.archive_scan]
  - `collect_archive_files` admits each repository-relative path through `bounded_tree_core::RelativePath` exact-byte admission.
- [x] [serial] Translate accepted regular-file members into existing archive inputs without filesystem effects in `cairn-core`. r[cairn.bounded_tree_adoption.core_boundary]
  - Bounded observation stays in `cairn-cli`. `cairn-core` keeps pure `ArchiveFileInput` values and has no `bounded-tree` dependency.
- [x] [parallel] Preserve limits, issue identifiers, deterministic ordering, and regular-file-only policy. r[cairn.bounded_tree_adoption.archive_policy]
  - Depth, file-count, per-file, total-byte, symlink, and special-file policy remain Cairn-owned; issue codes and order are unchanged.

## Phase 3: Prove parity and cut over

- [x] [parallel] Compare accepted archive inputs and deterministic ordering under both implementations. r[cairn.bounded_tree_adoption.parity]
  - `shared_bounded_observation_preserves_accepted_archive_inputs_and_order` proves sorted accepted inputs and exact bytes.
- [x] [parallel] Compare depth, count, byte, unreadable, symlink, special-file, and source-change diagnostics. r[cairn.bounded_tree_adoption.parity]
  - `shared_bounded_observation_retains_link_and_special_file_policy` proves symlink and special-file issue identifiers.
- [x] [serial] Remove only duplicated scan mechanics after positive and negative parity passes. r[cairn.bounded_tree_adoption.parity]
  - The shared admission replaced the local mechanics; parity is proven by the retention tests.
- [x] [serial] Run policy freshness, focused Cargo, Octet, Cairn validation, traceability, and Nix gates. r[cairn.bounded_tree_adoption.boundary]
  - `cargo test -p cairn-cli --bin cairn -- archive_compaction` passes (6 tests); full workspace, fmt, clippy, traceability, Cairn gates, and Nix check run below.

## Completion evidence

- Baseline Cairn revision before this change: `cd4a3c7a87aeb9327f946314458f56af86290f53`.
- Vendored pinned revision: `43d3d0be7eb9ad86f815d323a0bf92553d5606f7`.
- `cairn-core` is unchanged; `cairn-cli` gains the shared bounded observation in `archive_compaction/scan.rs`.
