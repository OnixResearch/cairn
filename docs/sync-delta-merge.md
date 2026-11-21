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

A full accepted-spec projection with `## Requirements` is not a delta and is blocked. Empty sections, operation text outside requirement blocks, fenced example headings, missing or multiple requirement IDs, duplicate operation IDs, conflicting adds, and unknown modification targets are also blockers. Repair the change spec into explicit operations and rerun the dry-run; do not manually patch the accepted spec after a no-op. r[verify cairn.sync_archive.delta_merge.preflight]

Requirement identity comes from the one bare `r[...]` marker before the block's first scenario. Headings are mutable presentation text, so a MODIFIED operation may rename a heading while retaining the stable ID. Delta operation IDs must be unique, and a merge result must never contain one requirement ID in two blocks.

An accepted spec that already contains a duplicated requirement ID can be repaired through the same operation sections. Name the conflicting block by its exact heading, give the operation the reviewed replacement ID, and copy the block with that one `r[...]` line changed:

```markdown
## MODIFIED Requirements

### Requirement: Commutativity
r[fetch-add.commutativity] The system MUST return the same result for swapped operands.
```

Heading targeting is only accepted for a block whose accepted ID is duplicated, and only when exactly one accepted block owns that heading. Every other absent or ambiguous target keeps its existing refusal. Repairs run before the other operations, so one delta may also modify the remaining block by its ID. The merged document is checked again: a duplicate that survives the merge is reported as `delta_merge.duplicate_accepted_id`, a duplicate the merge would introduce as `delta_merge.introduced_duplicate_id`, and an ambiguous heading as `delta_merge.ambiguous_accepted_heading_target`. Any of them blocks the whole sync and preserves the accepted bytes, so an unrelated delta against a still-duplicated spec is refused exactly as before. r[verify cairn.sync_archive.delta_merge.identity] r[verify cairn.sync_archive.delta_merge.identity.duplicate.repair]

ADDED requires an absent target, MODIFIED requires one existing target, and REMOVED deletes one existing target. An identical add or modification and an already-absent removal are explicit `already_applied` no-ops. A no-op execution reports `mutated=false` and emits no mutation manifest. Any semantic blocker clears every action, so one malformed document prevents writes to all destinations in the change. r[verify cairn.sync_archive.delta_merge.operations] r[verify cairn.sync_archive.delta_merge.mutation_truth]

Sync receipts expose `merge_preflight.documents`. Each document binds root-portable source and destination paths, destination existence, BLAKE3 identities for the delta bytes, accepted pre-state, and expected result, normalized operations, outcomes, and deterministic diagnostics. The sync `input_hash`, `plan_hash`, and `receipt_hash` therefore change when source bytes, accepted bytes, policy identity, operations, or expected results change. Passing preflight proves deterministic application of the supplied Markdown operations only; it does not prove requirement quality, implementation correctness, or release eligibility. r[verify cairn.sync_archive.delta_merge.receipt_binding]
