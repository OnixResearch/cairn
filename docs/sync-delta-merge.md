# Fail-closed sync delta merge

`cairn sync <change>` performs semantic preflight before authorizing any accepted-spec write. Every change spec must use one or more explicit operation sections:

```markdown
## ADDED Requirements

### Requirement: Stable title
r[domain.stable_id] The system MUST describe the new behavior.

## MODIFIED Requirements

### Requirement: A title that may be renamed
r[domain.existing_id] The system MUST describe the replacement behavior.

## REMOVED Requirements

### Requirement: Retired title
r[domain.retired_id]
```

A full accepted-spec projection with `## Requirements` is not a delta and is blocked. Empty sections, operation text outside requirement blocks, fenced example headings, missing or multiple requirement IDs, duplicate IDs, conflicting adds, and unknown modification targets are also blockers. Repair the change spec into explicit operations and rerun the dry-run; do not manually patch the accepted spec after a no-op. r[verify cairn.sync_archive.delta_merge.preflight]

Requirement identity comes from the one bare `r[...]` marker before the block's first scenario. Headings are mutable presentation text, so a MODIFIED operation may rename a heading while retaining the stable ID. Accepted specs and operation sets must have unique requirement IDs. r[verify cairn.sync_archive.delta_merge.identity]

ADDED requires an absent target, MODIFIED requires one existing target, and REMOVED deletes one existing target. An identical add or modification and an already-absent removal are explicit `already_applied` no-ops. A no-op execution reports `mutated=false` and emits no mutation manifest. Any semantic blocker clears every action, so one malformed document prevents writes to all destinations in the change. r[verify cairn.sync_archive.delta_merge.operations] r[verify cairn.sync_archive.delta_merge.mutation_truth]

Sync receipts expose `merge_preflight.documents`. Each document binds root-portable source and destination paths, destination existence, BLAKE3 identities for the delta bytes, accepted pre-state, and expected result, normalized operations, outcomes, and deterministic diagnostics. The sync `input_hash`, `plan_hash`, and `receipt_hash` therefore change when source bytes, accepted bytes, policy identity, operations, or expected results change. Passing preflight proves deterministic application of the supplied Markdown operations only; it does not prove requirement quality, implementation correctness, or release eligibility. r[verify cairn.sync_archive.delta_merge.receipt_binding]
