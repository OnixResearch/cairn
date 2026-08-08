# Radicle Change COB

## Why

Cairn active changes are durable lifecycle units, but collaboration currently occurs through direct edits to repository files. Radicle Collaborative Objects provide signed Git-backed operation histories and deterministic materialization, yet Cairn has no lossless active-change state or reducer that can safely implement a custom COB type.

A whole-project COB would couple unrelated changes and make every lifecycle edit contend on one history. The first integration should instead represent one active Cairn change per COB while leaving accepted specs, implementation source, and executed archive commits canonical in Git.

## What Changes

- Add a lossless, versioned active-change model that preserves proposal, design, tasks, metadata, and delta-spec text without treating the existing structured lifecycle bundle as complete. r[cairn.radicle_change_cob.lossless_change_state]
- Add a pure deterministic reducer whose explicit revision bases merge disjoint edits and retain same-artifact concurrent edits as conflicts until a later operation resolves every head. r[cairn.radicle_change_cob.deterministic_reducer]
- Add BLAKE3 semantic state identity plus subject-bound evidence and lifecycle transition records with explicit non-claims. r[cairn.radicle_change_cob.evidence_boundary]
- Add a native Radicle `Cob`/`CobWithType` adapter for `xyz.onixresearch.cairn-change` that maps signed operation identity facts into the pure reducer without giving repository I/O to `cairn-core`. r[cairn.radicle_change_cob.radicle_adapter]
- Add deterministic materialization planning back to the native `cairn/changes/<slug>/` file layout. r[cairn.radicle_change_cob.materialized_view]

## Impact

- **Files**: `crates/cairn-core`, a new `crates/cairn-radicle` adapter crate, workspace manifests, lifecycle specification, tests, and Radicle reference documentation.
- **Testing**: focused core tests before and after implementation; positive and negative reducer, conflict, authorization, evidence, materialization, serialization, and adapter checks; Cairn validation and proposal/design/tasks gates.
- **Compatibility**: no existing CLI command or Markdown workflow changes in this slice. A later Cairn will own repository discovery, signing, import, checkout, publish, and archive execution commands.
