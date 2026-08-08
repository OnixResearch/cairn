## Why

Cairn's sync merge recognizes requirement blocks only beneath exact `## ADDED Requirements` and `## MODIFIED Requirements` headings. A full-spec-shaped change can pass validation and every stage gate, then `sync --execute` reports `blocked=false` and `mutated=true` even though no proposed requirement is promoted and the before/after accepted-spec hashes are identical.

The merge also identifies replacements by mutable requirement heading rather than stable `r[...]` identity. Renaming a modified requirement therefore leaves the old block in place and appends a second block with the same requirement ID. Repository validation currently accepts that duplicate identity.

## What Changes

- Parse delta Markdown into typed, deterministic ADDED/MODIFIED/REMOVED operations and reject missing, empty, ambiguous, or malformed operation sets before planning mutation. r[cairn.sync_archive.delta_merge.preflight]
- Key accepted and delta requirement blocks by one canonical bare requirement ID rather than by heading text. r[cairn.sync_archive.delta_merge.identity]
- Define fail-closed and idempotent semantics for add, modify, and remove operations, including renamed headings and already-applied deltas. r[cairn.sync_archive.delta_merge.operations]
- Bind delta bytes, accepted pre-state, normalized operations, and computed results into sync plan and receipt hashes. r[cairn.sync_archive.delta_merge.receipt_binding]
- Report mutation truthfully and perform no write when semantic preflight is blocked or the computed output is byte-identical. r[cairn.sync_archive.delta_merge.mutation_truth]
- Add positive and negative parser, merge, CLI, and determinism fixtures. r[cairn.sync_archive.delta_merge.tests]

## Impact

- **Files**: pure delta parsing/merge planning in `cairn-core`; sync snapshot loading, receipt hashing, guarded writes, policy schemas, fixtures, and workflow guidance in `cairn-cli`/`cairn-policy`.
- **Testing**: full-spec no-op regression, renamed modification, duplicate IDs, malformed sections, target cardinality, idempotent replay, no-write, and cross-root hash stability.
