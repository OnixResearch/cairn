# Concurrent change footprint verification

## Executed on 2026-11-18 in this worktree

- `cairn-policy/generated/concurrent-change-footprints.json` generated from
  `cairn-policy/concurrent-change-footprints.ncl`.
- `cargo test -p cairn-core --lib concurrent_change_footprints`: 15 passed.
- `cargo test -p cairn-cli --test footprints`: 5 passed.
- `cargo fmt --check` and `cargo clippy -p cairn-core -p cairn-cli
  --all-targets -- -D warnings`: clean.
- `cairn policy export --check`: fresh.
- `cairn validate --root .`: valid, 0 issues.
- `cairn sync derive-concurrent-change-obligations --root . --execute`: executed.
- `cairn traceability coverage --root .`: 2470/2470 referenced.

## Defects corrected before those runs

- A public `guarantee_statement_identity` collided with the refinement module's
  function through crate-root glob exports; the footprint one is now
  `footprint_guarantee_identity`.
- `support_maintenance::AnalysisStatus` and
  `revision_classification::{SearchBasis, WitnessReplay}` collided with the
  consistency module's types through the same globs; they are now
  `ClaimAnalysisStatus`, `RevisionSearchBasis` and `RevisionWitnessReplay`.
- The path explanation named only the consumer's requirement. On a guarantee
  change it now names the producer requirement as well, so the reported path
  reads from changed guarantee to consumer assumption to obligation.
- The CLI formatted a `PathBuf` with `Display`, an unused helper duplicated the
  interface lookup, and two clippy findings (identical branches, redundant
  dereference) were corrected.

## Boundary

The analysis derives compatibility obligations from declared footprints. It does
not merge changes, order execution, schedule work or authorize delivery, and it
does not establish behavioural independence from file disjointness or from
individually passing receipts. A retained witness is one model of the declared
finite universe, so it can support a conflict but never coexistence or invariant
preservation over all admitted compositions. Absence of a detected interaction is
bounded to the declared footprint.
