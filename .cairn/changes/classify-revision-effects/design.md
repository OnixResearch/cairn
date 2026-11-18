# Revision classification design

## Ownership and API

`cairn-policy/revision-classification.ncl` owns comparison bounds, policy-fragment bounds and the disabled transfer policy.
Its standalone projection is `cairn-policy/generated/revision-classification.json`; the CLI embeds that checked artifact.
The default lifecycle policy and receipt hash material are unchanged.

Rust owns the versioned wire model and finite comparison in `cairn_core::revision_classification`, exported at the crate root.
Public entries are `compare_requirement_revisions`, `compare_policy_revisions`, `replay_requirement_witness`, `replay_policy_witness` and `plan_evidence_transfer`.
`RevisionClassificationPolicy::admit` validates the policy schema, hard implementation ceilings and the disabled transfer state.
All errors are tagged explicit outcomes; no error becomes `editorial`, a successful implication or a current receipt.

Both sides reuse `ContentBoundRequirementRow`, `SemanticContract`, `normalize_contract` and `admit_correspondence_review` from #5.
Retained evidence reuses `ScopedClaim` and `claim_identity` from #6. No second registry or claim identity is introduced.

## Finite requirement comparison

`RevisionBinding` names both exact native rows and both normalized interpretation, observable-mapping and contract identities.
The core recomputes those identities from the supplied native rows and contracts, then rejects any drift before comparison.
Both sides use one supplied finite semantics profile. A profile change or native registry freshness is outside this comparison and remains an explicit admission or trust boundary rather than a silent default.

`RevisionMapping` declares one common domain per side domain and one common observable per side observable.
Each common domain lists the common value names and, per value, the prior and current local names. Values map injectively within each side.
Each common observable names its positional common argument domains, result domain and the prior/current local observable names.
Every declared domain and observable must be mapped. Missing, extra, mistyped or non-injective correspondences produce `incompatible` with an `unsupported` code, never `editorial`.

The common domains and observables define a finite universe of total observation functions.
Each side's local finite values are embedded into that declared common universe; a local value set may be a proper subset, and the resulting structural difference prevents an `editorial` label.
Worlds are enumerated deterministically in sorted domain, argument-tuple and result order. The world count, syntax and evaluation steps are bounded with checked arithmetic.
Each side's obligation is `assumptions AND activation IMPLIES guarantee`, evaluated under the same world.
`new implies old` alone is `strengthening`; `old implies new` alone is `weakening`; both are equivalent; neither is `incomparable`.
A failed direction retains the first complete observation table where its premise holds and conclusion fails. `replay_requirement_witness` recomputes the world, both truths and the distinguishing result.

`editorial` requires both implication directions and an unchanged mapped structure: assumptions, activation, guarantee, finite domains and observable declarations including phase.
Different prose, requirement row identities, observable source/projection text and supported/informal wording do not by themselves change the finite structure.
An equivalent revision with a changed finite structure is `clarifying`.
Any unsupported correspondence is `incompatible`; an exhausted world or step bound is `undetermined`. Neither can become `editorial`.

## Assumptions and guarantees

Assumptions, activation and guarantee each receive their own implication relation over the same common worlds.
The overall obligation relation remains the classification input.
A stronger assumption set can reduce the implementation obligation while the guarantee is structurally unchanged; the report states both facts and never collapses them into one label.

## Acceptance-policy fragment

`AcceptanceFragment` declares the supported fragment identifier, a shared accepted-evidence universe, named required checks, admissible evidence roles and allowed assumption sets.
An evidence candidate is a check set, one evidence role and one assumption set.
Acceptance requires every named required check, an admissible role and an allowed assumption set.
The accepted set is compared symbolically as independent check, role and assumption dimensions within the declared universe.
`new accepted subset of old` alone is `tightened` or `strengthening`; the reverse is `widened` or `weakening`; both are equivalent; neither is `incomparable`.
Different universes or an unknown fragment are `incompatible`.
Failed containment retains one replayable candidate accepted by the premise and rejected by the conclusion.
Accepted cardinality, feasibility and emptiness are reported separately from ordering. An empty premise makes containment vacuously true and remains visible as empty.

## Evidence reuse and transfer

Every retained evidence claim is bound to its old native row, interpretation and contract identity, plus current subject, policy, target and environment.
The old receipt is always reported as not current; revision classification never refreshes original applicability.
A claim outside the compared old revision or outside the current context is `not_reusable`.
A strengthening, weakening, incompatible or undetermined revision is `not_reusable` for every retained claim.
An equivalent revision may mark a current retained payload `reusable` only for a fresh derivation with new admission; transfer remains disallowed.
`EvidenceTransferRequest` and `EvidenceTransferRecord` name the future transfer shape. `plan_evidence_transfer` validates the request against the exact comparison and then refuses while transfer is unqualified.

## CLI and tests

`cairn revision classify --requirement FILE [--policy FILE]` and `cairn revision classify --policy-fragment FILE [--policy FILE]` perform bounded reads only.
They print a versioned advisory envelope with the report or a tagged rejection, and write no lifecycle state.
Unknown options, both inputs, missing input, malformed JSON and policy failures are non-accepting outcomes.

The finite corpus covers editorial, clarifying, strengthening, weakening, incompatible, unsupported and inconclusive requirement changes, plus tightened, widened, incomparable, infeasible and bounded policy fragments.
Stale witness and correspondence records are rejected. Witness and policy witnesses replay against the admitted model.
Deterministic replay of identical inputs yields identical identities and reports.

## Non-claims

Bounded implication is not a proof about arbitrary Rust, unbounded state or reactive realizability.
Prose correspondence, observable extraction, reviewer authenticity, native registry freshness and environmental assumptions remain explicit trust dependencies.
Classification does not discharge an obligation, omit a required check, authorize release, mutate lifecycle state or promote old evidence.

## Tests and evidence

Core tests construct exact native rows and normalized contracts for editorial, clarifying, strengthening, weakening, incompatible, unmapped and world-bound cases.
They replay retained requirement witnesses, reject a tampered witness and reject a stale correspondence review.
Policy tests cover tightened, widened, incomparable, infeasible and bounded fragments, replay a failed containment candidate and assert the reported universe and fragment.
CLI tests exercise requirement and policy classification, stale-context evidence refusal, tampered bindings and argument rejection without lifecycle writes.
These are authored tests; parent acceptance runs them and retains current-source evidence.
