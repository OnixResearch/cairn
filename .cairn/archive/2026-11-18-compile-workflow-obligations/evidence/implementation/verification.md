# Workflow obligation verification

## Executed on 2026-11-18 in this worktree

- `cairn-policy/generated/workflow-obligations.json` regenerated from
  `cairn-policy/workflow-obligations.ncl`.
- `cairn policy export --check`: fresh.
- `cargo test -p cairn-core --lib workflow_obligations`: 15 passed.
- `cargo test -p cairn-core --lib refinement_typed_dependencies`: 19 passed
  (the merged refinement module, refactored only for lint cleanliness).
- `cargo test -p cairn-cli --test workflow_obligations`: 6 passed.
- `cargo fmt --check` and `cargo clippy -p cairn-core -p cairn-cli
  --all-targets -- -D warnings`: clean.
- `cairn validate --root .`: valid, 0 issues.
- `cairn sync compile-workflow-obligations --root . --execute`: executed.
- `cairn traceability coverage --root .`: complete.

## Defects corrected before those runs

1. `ExceptionSnapshot.control_evidence` was never consulted. The compensating
   control predicate was resolved only from `request.evidence`, so an applicable
   exception with satisfied control evidence stayed `unsatisfied`. `evidence_for`
   now reads the snapshot's declared control evidence with the same exact
   requirement, revision, subject, context, policy and observation binding, and
   general evidence cannot substitute for it.
2. `validate_selection` recursed into child expressions while still checking the
   whole selection, so every leaf rejected a selection naming a sibling `any_of`
   node. Node membership is now checked once at the root; only
   declared-alternative membership recurses.
3. `cairn-policy/workflow-obligations.ncl` bound lambda parameters `predicate`,
   `alternatives` and `condition` to record fields of the same name, which is
   infinite recursion in Nickel and made the export fail.
4. The committed projection had lost `id` and `kind` on the `any_of` node; the
   regenerated projection carries `id: verify.traceability`, `kind: any_of` and
   its `receipt-chain` / `claim-revision` alternatives.
5. Three compile errors and several lint findings in the authored tests and the
   merged refinement module (shadowed `request` bindings, a moved
   `control_state`, an unused `mut`, a duplicated acyclicity function, identical
   `if` arms, and redundant iterator shapes).

## Boundary

The obligation model decides completion over the declared finite observation
model and supplied native evidence. It does not prove implementation
correctness, does not execute work, does not monitor anything, and does not grant
authority. An accepted exception records that current applicability and the
declared compensating control hold; it never marks the original obligation
proved. Guidance rendering carries only layout, so a presentation edit cannot add
evidence or widen command classes.
