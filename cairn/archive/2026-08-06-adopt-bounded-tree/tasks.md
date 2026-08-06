## Phase 1: Admit the shared dependency

- [x] [serial] Confirm that `bounded-tree` has archived its establishment change and published a passing immutable Radicle revision. r[cairn.bounded_tree_adoption.prerequisite]
- [x] [serial] Pin that Radicle revision without a sibling path, mutable branch, or fallback. r[cairn.bounded_tree_adoption.prerequisite]
- [x] [parallel] Record the pre-adoption Cairn revision, dependency state, and rollback command. r[cairn.bounded_tree_adoption.rollback]

## Phase 2: Adapt archive scanning

- [x] [serial] Use shared bounded observations from the `cairn-cli` archive compaction shell. r[cairn.bounded_tree_adoption.archive_scan]
- [x] [serial] Translate accepted regular-file members into existing archive inputs without filesystem effects in `cairn-core`. r[cairn.bounded_tree_adoption.core_boundary]
- [x] [parallel] Preserve limits, issue identifiers, deterministic ordering, and regular-file-only policy. r[cairn.bounded_tree_adoption.archive_policy]

## Phase 3: Prove parity and cut over

- [x] [parallel] Compare accepted archive inputs and deterministic ordering under both implementations. r[cairn.bounded_tree_adoption.parity]
- [x] [parallel] Compare depth, count, byte, unreadable, symlink, special-file, and source-change diagnostics. r[cairn.bounded_tree_adoption.parity]
- [x] [serial] Remove only duplicated scan mechanics after positive and negative parity passes. r[cairn.bounded_tree_adoption.parity]
- [x] [serial] Run policy freshness, focused Cargo, Octet, Cairn validation, traceability, and Nix gates. r[cairn.bounded_tree_adoption.boundary]
