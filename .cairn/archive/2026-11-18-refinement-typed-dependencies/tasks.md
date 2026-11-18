# Refinement and typed dependency tasks

## Contract and implementation

- [x] [serial] [task:policy] Author the standalone Nickel policy with admitted relations, assurances, feasibility methods, roots, proof checkers and bounds. r[cairn.refinement.dependencies]
- [x] [serial] [task:decomposition] [after:policy] Add versioned decomposition records binding exact parent/child revisions, scope, environment assumptions, selected alternatives, finite composition and integration obligations. r[cairn.refinement.decomposition]
- [x] [serial] [task:assurance] [after:decomposition] Add the implication relation, four assurance distinctions, weakest-assurance propagation and policy-admitted checked-proof checker. r[cairn.refinement.assurance]
- [x] [serial] [task:feasibility] [after:assurance] Add the separately justified feasibility disposition and contradictory/unresolved premise handling. r[cairn.refinement.feasibility]
- [x] [serial] [task:dependencies] [after:feasibility] Add typed dependency records and per-relation meaning, evidence matching, coverage, staleness and applicability triggers alongside legacy `depends_on`. r[cairn.refinement.dependencies]
- [x] [serial] [task:semantic] [after:dependencies] Add semantic producer-guarantee/consumer-assumption bindings with compatible contexts and retained witness identities. r[cairn.refinement.semantic_dependency]
- [x] [serial] [task:closure] [after:semantic] Add admitted guarantee and policy-root discharge with residual assumption closure, original sources and discharge paths. r[cairn.refinement.discharge]
- [x] [serial] [task:acyclic] [after:closure] Enforce an acyclic requirement justification graph with specific cycle blockers. r[cairn.refinement.acyclic]
- [x] [serial] [task:explanations] [after:acyclic] Add deterministic explanations for pending, supported, contradicted, infeasible and unsupported obligations plus reevaluation triggers. r[cairn.refinement.explanations]
- [x] [serial] [task:cli] [after:explanations] Add the read-only `refinement explain` CLI with bounded reads and no lifecycle authority. r[cairn.refinement.inspection]

## Acceptance and handoff

- [x] [serial] [task:positive] [after:cli] Test the native completion pilot, supported dependency relations, semantic discharge, residual closure and deterministic replay. r[cairn.refinement.decomposition] r[cairn.refinement.discharge] r[cairn.refinement.dependencies]
- [x] [serial] [task:negative] [after:positive] Test pending integration obligations, child-only support, partial `any_of` selection, contradictory/unresolved assumptions, contradicted children, stale/ambiguous/out-of-scope evidence, coverage gaps, witness mismatch, missing checkers and cycles. r[cairn.refinement.feasibility] r[cairn.refinement.semantic_dependency] r[cairn.refinement.acyclic]
- [x] [serial] [task:consumer] [after:negative] Retain the Kiln dependency fixture bullets as positive and negative core/CLI consumer cases without live multi-repository execution. r[cairn.refinement.dependencies]
- [x] [serial] [task:compatibility] [after:consumer] Confirm existing task ordering, archive dependencies, implementation-ready output and cross-repository v1 semantics are unchanged. r[cairn.refinement.compatibility]
- [x] [serial] [task:verify] [after:compatibility] Run focused core and CLI tests, policy freshness, formatting, workspace tests, Clippy, validation and traceability. Retain current-source evidence. r[cairn.refinement.inspection]
- [x] [serial] [task:handoff] [after:verify] Prepare reviewed specification sync and archive previews only after implementation evidence passes. r[cairn.refinement.inspection]

Executed evidence: **19 core** `refinement` tests and **5 CLI**
`refinement_typed_dependencies` tests pass. `cairn policy export --check` is
fresh and `cairn validate --root .` reports 0 issues after sync.

Defects repaired during acceptance, each recorded in
`evidence/implementation/acceptance-state.md`:

- `assert_acyclic` was called but never defined; implemented as a colour-marked
  depth-first traversal reporting the requirement-id cycle path.
- Two test helpers were shadowed by same-named locals (`E0618`); resolved with
  explicit `self::` paths.
- The shared semantic fixture declared mismatched resource bounds against the
  dependency context. The conservative reading is kept — **resource bounds are
  part of context identity** — and the fixture now declares the same bounds on both
  sides rather than the comparison being relaxed.

Scope: typed relations carry named authoritative evidence and non-claims; semantic
discharge requires producer guarantee, consumer assumption, compatible context and
a witness binding both statement identities; matching names or receipt identities
alone never discharge. No live multi-repository execution.
