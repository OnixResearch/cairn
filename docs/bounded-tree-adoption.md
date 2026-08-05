# Cairn bounded-tree adoption

Cairn adopts the product-neutral `bounded-tree` mechanism for archive compaction scanning. The dependency is vendored at one immutable Radicle revision. It is not a sibling path or a mutable branch.

## Pinned dependency

- Repository: `rad:zqhtZvsteJhxCJE96dMAZSZ9y1PX` (project `bounded-tree`, default branch `main`).
- Immutable Radicle revision: `43d3d0be7eb9ad86f815d323a0bf92553d5606f7`.
- Published branch: `cairn/establish-bounded-tree-20260803`; delegate signed-refs revision `7c2202e166b4b53b10504a9cc8e4527ebd8332c9`.
- Repository archive BLAKE3: `18174a3bf0bfaee664034f96ecc0a64f81077b5b1384e91d7fe6644df9171e31`.
- Rust source archive BLAKE3 (crates, Cargo.toml, Cargo.lock): `94cd9558691dcaf6f0a024989ee48d27a8d85f899cd2d7cbfd0a0143771e0792`.

The vendored crates live under `vendor/bounded-tree-core` and `vendor/bounded-tree-cap`. They are pinned by copying the reviewed revision; they are not declared through a sibling check-out or any GitHub fallback.

## Scope

The shared observation is the exact-byte path admission in `cairn-cli`'s archive compaction scan (`bounded_tree_core::RelativePath`). It admits a repository-relative path only when every exact byte component is well-formed. A malformed member emits a bounded-path issue and the scan continues; it never blocks the rest of an archive. This keeps Cairn's permissive regular-file-only policy and its exact `archive_compaction.*` issue identifiers and deterministic ordering.

Depth, file-count, per-file, total-byte, symlink, and special-file policy remain Cairn-owned constants in the CLI shell (`cairn-policy`). Filesystem traversal and bounded byte loading remain in `cairn-cli`. `cairn-core` still receives owned loaded values only; it has no dependency on `bounded-tree-core` or `bounded-tree-cap`.

## Parity gate

The change keeps the shared scan active and proves parity with the prior scan on valid and invalid fixtures:

- accepted archive inputs are identical and deterministically ordered;
- symlink, special-file, unreadable, and limit diagnostics keep their stable identifiers and order.

The change removes only scan mechanics that the parity gate proves duplicated.

## Evidence boundary

Cairn records only the exact pinned dependency and the bounded observation result in lifecycle evidence. A shared scan is never promoted to archive-truth correctness, semantic-review completeness, or release eligibility. That authority remains Cairn's.

## Rollback

Pre-adoption state: prior local archive compaction scan logic at the Cairn revision before this change, with no `bounded-tree-core`/`bounded-tree-cap` dependency.

Rollback restores both parts together:

1. Remove `bounded-tree-core` and `bounded-tree-cap` from `crates/cairn-cli/Cargo.toml` and the workspace dependency table, and drop the `vendor/` directory.
2. Restore the pre-adoption `scan.rs` (which contains the original local archive scan without `bounded_tree_core::RelativePath` admission).

Because the shared scan only adds an exact-byte admission that never triggers new issues for real directory entries, reverting the dependency and the scan source together restores prior behavior exactly.
