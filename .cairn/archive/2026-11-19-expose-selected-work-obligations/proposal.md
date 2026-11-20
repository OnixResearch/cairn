# Expose a versioned selected-work snapshot with typed workflow obligations

## Why

Kiln #83 and Lattice #26 need one Cairn-owned, typed view of selected work:
the selected change or authored task, its native readiness, context and source
identities, and the workflow obligations that decide completion. Cairn #14 asks
for that versioned selected-work snapshot. Cairn #12 already compiles guidance
and an evaluable completion predicate from one admitted obligation definition,
but its export section still needs a selected-work transport and typed
per-obligation facts. Without them, a consumer must parse guidance prose to
recover obligation identities, selection state and native evidence references.

A first selected-work implementation was written on branch
`change/selected-work-snapshot-20261117` and was never integrated into
`main`, although the changelog already lists it. This change ports that
contract onto the current tree and extends it with the #12 obligation export.

## What Changes

- Add the versioned `cairn.selected-work.v1` Rust DTO, pure composition and
  independent portable admission in `cairn_core::verified::selected_work`,
  with a checked Nickel contract and generated bounds policy.
- Add the read-only `cairn selected-work` command and the `cairn_selected_work`
  MCP tool, sharing one producer, with explicit root or lifecycle-store
  selection, authored task selection, prerequisite closure, an aggregate
  observation budget, local and portable disclosure, and identity-bound detail.
- Add typed per-obligation facts to `cairn_core::workflow_obligations`:
  node kind, parent, alternative, selection state, applicability conditions,
  exact native evidence references, expected observations, command classes,
  exception identities and three identities (`obligation_blake3`,
  `guidance_blake3`, `condition_blake3`). A consumer verifier recomputes them.
- Add an opt-in `--obligation-definition` / `obligation_definition` selection
  and optional `--obligation-observations` / `obligation_observations` input.
  The snapshot then carries the bound predicate, structured guidance, typed
  facts and evaluation, bound to its own verification subject, context and
  workflow policy, under a new `cohort.obligations` identity.
- Keep the obligation section explicitly `not_selected` when no definition is
  requested. Missing and unsupported observations remain remaining
  obligations; transport success grants no execution or lifecycle authority.
- Add the selected-work tool to the MCP discovery profiles and a flake check
  for the contract, generated bounds and positive/negative fixtures.

## Impact

- Current consumers: Kiln #83 (typed campaign, worker and evidence views) and
  Lattice #26 (workflow intent compilation). Immediate outcome: one exact
  producer contract they can adopt. Durable contribution: a reusable
  selected-work transport over existing native evaluators. Maintenance owner:
  Cairn maintainers. Adoption path: Kiln admits the portable snapshot with a
  retained expected cohort; Lattice maps typed obligation facts into its
  `WorkflowIntent`. Repeatability: core, consumer and real CLI/MCP fixtures.
- Legacy `implementation-ready`, drain, readiness and `cairn obligation`
  defaults are unchanged.
