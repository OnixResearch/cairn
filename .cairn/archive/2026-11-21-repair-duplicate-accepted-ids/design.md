# Design

## Context

Delta merge lives in `crates/cairn-core/src/verified/delta_merge.rs`. It is
ordinary Rust; this repository contains no Verus specifications or proofs for
it, so no proof artifacts need to change.

Operations resolve accepted blocks by one canonical requirement ID. When two
accepted blocks share that ID, the ID is not a target: `position()` picks the
first match, so applying an operation by ID would be arbitrary. The current
preflight therefore reports `delta_merge.duplicate_accepted_id` for the accepted
document before any operation is applied, which also blocks the operation that
would have removed the duplicate.

## Choices considered

1. **Keep the preflight and add a dedicated repair section/flag.** A new delta
   operation or a marked block would have to be taught to every consumer of the
   delta format. It also duplicates what MODIFIED already expresses: replacing
   a reviewed block.
2. **Let MODIFIED target a duplicated block by its unique heading.** The delta
   stays a plain MODIFIED operation. The heading is the only unambiguous,
   reviewed target the operator already writes, and it is already checked for
   uniqueness by the merge. Chosen.
3. **Deduplicate automatically by keeping the first block.** It discards a
   repeatable block that may carry distinct reviewed text, and no operation
   records which block was dropped. Refused as unsound.

## Chosen semantics

- Accepted duplicates are recorded on the parsed document instead of blocking
  preflight.
- A MODIFIED operation whose requirement ID is not a unique accepted target is
  applied by heading only when exactly one accepted block owns that heading and
  that block's ID is duplicated. It replaces that block, including its ID.
  Nothing else may be retargeted by heading: a heading that matches another
  accepted block, or a block whose ID is unique, keeps the existing
  `delta_merge.ambiguous_accepted_heading_target` or
  `delta_merge.unknown_modify_target` refusal.
- Repairs run in a first pass against the pre-state duplicate set. The remaining
  operations then run in delta order against the repaired document. A delta that
  repairs one duplicate and also modifies the other block by its ID therefore
  succeeds regardless of the order the operations are written in, and the
  outcome list stays in delta order.
- After every operation, the merged document is checked again. A duplicated ID
  that also existed before the merge is reported as
  `delta_merge.duplicate_accepted_id`; a duplicate the merge would introduce is
  reported as `delta_merge.introduced_duplicate_id`. Any diagnostic preserves
  the accepted bytes, so a delta that leaves a duplicate, targets ambiguously,
  adopts an existing ID, or does not touch the duplicate at all writes nothing.
- Receipt data is unchanged: operations, outcomes, and diagnostics are already
  bound into the merge preflight document, so a repair changes the
  `expected_after_hash`, `plan_hash`, and `receipt_hash` exactly like any other
  mutation.

## Boundary

Applying operations to a caller-supplied accepted document and delta is the
whole change. Validation of accepted specs (including the targeted-context
duplicate check) and registry exports are unchanged and still see the repaired
document after sync.
