# Repair duplicated accepted requirement IDs through deltas

## Why

`cairn sync` refuses any accepted spec that already contains one requirement ID
in two blocks, before it applies any delta operation. A repository that
inherited such a spec cannot repair it through the lifecycle: no delta can be
applied, so an operator has to edit the accepted spec by hand and record the
plan as `already_applied`. Terrane hit this in
`.cairn/archive/2026-11-21-deduplicate-cairn-requirement-ids`: `fetch-add/spec.md`
carried one `fetch-add.associativity` requirement ID on both the "Commutativity" and
"Associativity" blocks, and `constant-folding` and `dead-code-elimination`
carried same-file duplicates.

The lifecycle should offer a reviewed repair instead of a manual edit.

## What Changes

- Stop reporting an accepted duplicate as a preflight blocker for the whole
  document. Apply the delta first, then refuse the merge only when the merged
  document still contains a duplicated requirement ID.
- Permit a MODIFIED operation to retarget one accepted block by its unique
  heading when that block's requirement ID is duplicated, so the operation can
  change the ID that the ID-keyed lookup cannot resolve.
- Run heading-targeted repairs before the remaining operations so an operation
  that keeps targeting the duplicated ID by ID resolves against the repaired
  document deterministically, independent of delta order.
- Add stable diagnostics for ambiguous heading targeting
  (`delta_merge.ambiguous_accepted_heading_target`), for duplicates that remain
  after the merge (`delta_merge.duplicate_accepted_id`), and for duplicates the
  merge would introduce (`delta_merge.introduced_duplicate_id`).
- Add core and CLI fixtures modeled on the Terrane fetch-add and
  constant-folding cases, with positives for repair and replay and negatives for
  each refusal.
- Document the repair shape in `docs/sync-delta-merge.md`.

## Impact

Cairn's pure core owns the merge. No CLI flags, receipt fields, or policy
bytes change. An accepted spec that was previously blocked outright can now be
repaired only by a delta whose reviewed operations are unambiguous and whose
result is duplicate-free; every other blocked case keeps its existing refusal
and preserves accepted bytes.
