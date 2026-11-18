# Refinement and typed dependency tasks

## Contract and implementation

- [ ] [serial] [task:policy] Author the standalone Nickel policy with admitted relations, assurances, feasibility methods, roots, proof checkers and bounds. r[cairn.refinement.dependencies]
- [ ] [serial] [task:decomposition] [after:policy] Add versioned decomposition records binding exact parent/child revisions, scope, environment assumptions, selected alternatives, finite composition and integration obligations. r[cairn.refinement.decomposition]
- [ ] [serial] [task:assurance] [after:decomposition] Add the implication relation, four assurance distinctions, weakest-assurance propagation and policy-admitted checked-proof checker. r[cairn.refinement.assurance]
- [ ] [serial] [task:feasibility] [after:assurance] Add the separately justified feasibility disposition and contradictory/unresolved premise handling. r[cairn.refinement.feasibility]
- [ ] [serial] [task:dependencies] [after:feasibility] Add typed dependency records and per-relation meaning, evidence matching, coverage, staleness and applicability triggers alongside legacy `depends_on`. r[cairn.refinement.dependencies]
- [ ] [serial] [task:semantic] [after:dependencies] Add semantic producer-guarantee/consumer-assumption bindings with compatible contexts and retained witness identities. r[cairn.refinement.semantic_dependency]
- [ ] [serial] [task:closure] [after:semantic] Add admitted guarantee and policy-root discharge with residual assumption closure, original sources and discharge paths. r[cairn.refinement.discharge]
- [ ] [serial] [task:acyclic] [after:closure] Enforce an acyclic requirement justification graph with specific cycle blockers. r[cairn.refinement.acyclic]
- [ ] [serial] [task:explanations] [after:acyclic] Add deterministic explanations for pending, supported, contradicted, infeasible and unsupported obligations plus reevaluation triggers. r[cairn.refinement.explanations]
- [ ] [serial] [task:cli] [after:explanations] Add the read-only `refinement explain` CLI with bounded reads and no lifecycle authority. r[cairn.refinement.inspection]

## Acceptance and handoff

- [ ] [serial] [task:positive] [after:cli] Test the native completion pilot, supported dependency relations, semantic discharge, residual closure and deterministic replay. r[cairn.refinement.decomposition] r[cairn.refinement.discharge] r[cairn.refinement.dependencies]
- [ ] [serial] [task:negative] [after:positive] Test pending integration obligations, child-only support, partial `any_of` selection, contradictory/unresolved assumptions, contradicted children, stale/ambiguous/out-of-scope evidence, coverage gaps, witness mismatch, missing checkers and cycles. r[cairn.refinement.feasibility] r[cairn.refinement.semantic_dependency] r[cairn.refinement.acyclic]
- [ ] [serial] [task:consumer] [after:negative] Retain the Kiln dependency fixture bullets as positive and negative core/CLI consumer cases without live multi-repository execution. r[cairn.refinement.dependencies]
- [ ] [serial] [task:compatibility] [after:consumer] Confirm existing task ordering, archive dependencies, implementation-ready output and cross-repository v1 semantics are unchanged. r[cairn.refinement.compatibility]
- [ ] [serial] [task:verify] [after:compatibility] Run focused core and CLI tests, policy freshness, formatting, workspace tests, Clippy, validation and traceability. Retain current-source evidence.
- [ ] [serial] [task:handoff] [after:verify] Prepare reviewed specification sync and archive previews only after implementation evidence passes.

Implementation and acceptance tasks remain open. Save evidence under `evidence/`.
Planning receipts do not satisfy implementation tasks or authorize archive execution.
No live multi-repository execution is part of this change.
