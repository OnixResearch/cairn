## Context

The archive compaction shell walks repository capabilities, sorts entries, rejects links and special files, reads bounded bytes, and records policy-specific issues. The resulting `ArchiveInput` belongs to Cairn lifecycle behavior.

The shared repository can provide the bounded scan facts. It cannot decide archive eligibility or issue meaning.

## Decisions

### Decision: Block on shared completion

**Choice:** Do not add the dependency until `bounded-tree` has passing completion evidence and one immutable Radicle revision.

**Rationale:** Cairn release and lifecycle gates cannot depend on a mutable sibling source.

### Decision: Keep effects in the CLI shell

**Choice:** Use the shared capability shell only from `cairn-cli`. Keep `cairn-core` pure over loaded archive inputs and issues.

**Rationale:** A shared filesystem dependency must not weaken Cairn's functional-core boundary.

### Decision: Preserve archive policy and diagnostics

**Choice:** Translate shared observations into existing archive file inputs and issue identifiers. Preserve current limits, regular-file-only policy, deterministic issue ordering, and early-stop behavior where it is public.

**Rationale:** Those decisions belong to archive compaction, not generic tree admission.

### Decision: Dual-run complete failure behavior

**Choice:** Compare valid archive inputs and every maintained negative fixture. Include depth, file-count, per-file, total-byte, unreadable, symlink, special-file, and source-change cases.

**Rationale:** Cairn receipts depend on deterministic failure classes and ordering.

### Decision: Keep lifecycle evidence bounded

**Choice:** Cairn receipts may record the pinned shared revision and observation result. They must not claim that `bounded-tree` proves archive truth, lifecycle correctness, or release eligibility.

**Rationale:** Dependency evidence does not transfer decision authority.

## Risks / Trade-offs

- Existing issue ordering can constrain adapter structure.
- Archive compaction reads bytes into memory while the shared shell prefers streaming member facts.
- The adapter may retain one explicit bounded byte-loading step.
- Radicle outages remain visible and fail closed.
