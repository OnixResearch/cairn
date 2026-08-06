## Why

Cairn archive compaction performs a bounded, sorted capability-rooted tree scan with file-count, depth, per-file, total-byte, symlink, and special-file policy.

This change records a future use of `bounded-tree` for those shared scan mechanics. It remains unstarted until the shared repository publishes a complete immutable Radicle revision.

## What Changes

- Pin the reviewed `bounded-tree` Radicle revision without a sibling path or fallback.
- Adapt archive compaction scanning to shared observations and bounded member reads.
- Preserve Cairn archive candidate models, issue identifiers, lifecycle policy, deterministic receipts, and release decisions.
- Keep the pure `cairn-core` boundary free from filesystem effects.
- Prove accepted archive and negative diagnostic parity before deleting local scan mechanics.
- Retain an explicit rollback.

## Impact

- **Planned files**: archive compaction scan shell, CLI dependency declarations, fixtures, traceability markers, and adoption evidence.
- **Testing**: valid archives, depth and byte bounds, unreadable inputs, links, special files, deterministic issue ordering, Octet, Cargo, and Cairn self-validation.
- **Boundary**: `bounded-tree` supplies observations only. Cairn retains lifecycle and evidence meaning.
- **Current effect**: lifecycle planning only. No Cairn implementation changes are included.
