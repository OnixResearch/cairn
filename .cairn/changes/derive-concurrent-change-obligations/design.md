# Concurrent change footprints design

## Ownership and API

`cairn-policy/concurrent-change-footprints.ncl` owns the declaration and finite
exploration bounds. Its standalone projection is
`cairn-policy/generated/concurrent-change-footprints.json`; the CLI reads that
checked artifact by default and accepts `--policy FILE` for an explicit
projection. The legacy default lifecycle policy and receipt hash material are
unchanged.

Rust owns the versioned wire model and derivation in
`cairn_core::concurrent_change_footprints`, exported at the crate root. Public
entries are `analyze_concurrent_changes`, `guarantee_statement_identity`,
`invariant_statement_identity` and `FootprintPolicy::admit`. Every error is a
tagged explicit outcome; no error becomes `supported_within_declared_scope`.

Both sides reuse `ContentBoundRequirementRow`, `SemanticContract`,
`NormalizedContract`, `normalize_contract`, `SemanticProfile`, `FiniteExpr` and
`ObservableRow` from #5. Footprint revisions carry their declared
interpretation, mapping and contract identities, and admission recomputes them
from the supplied native row and contract; any drift is `revision_mismatch`.
No second requirement registry, claim identity or contract identity is
introduced.

## Footprint model

`ChangeFootprint` names one change by exact `change_id`, `source_revision` and
`intent_revision`, then declares:

- `revisions`: exact typed requirement revisions the change touches.
- `guarantees`: a `ChangedGuarantee` with previous and current
  `GuaranteeStatement` values, a declared `strengthened`, `weakened`,
  `replaced`, `removed`, `preserved` or `unknown` behaviour, affected claims,
  interfaces and configurations, and optional unknown detail.
- `assumptions`: a `ConsumedAssumption` naming the producer change, guarantee
  id and exact consumed statement identity, a `strict` or `monotone`
  dependency, the declared query meaning, an optional finite predicate over the
  producer's declared universe, evidence basis ids and configurations.
- `invariants`: a `PreservedInvariant` with an identity-bearing commitment,
  interfaces, configurations, local assumptions and required integration
  conditions.
- `invalidated`: obligations the change removes or replaces, with the reason
  and affected interfaces or configurations.
- `interfaces`: interface id, exact configuration identity, operation kind,
  operation identity and order sensitivity.
- `evidence_basis`: the obligation ids and claims that retained evidence
  depends on.
- `provided_conditions`: integration conditions the change supplies to a
  composed candidate.
- `unknown`: explicit incomplete-footprint, unsupported-interpretation,
  informal-behaviour, bounded-exploration or other scope.

A guarantee or invariant statement is either a finite expression or an
informal statement. Its identity binds the native requirement row, the
declaration id and the exact finite expression or informal text. A consumer
must name the identity it was written against; naming an identity the producer
never declared is an explicit `unresolved_consumed_identity` gap rather than a
silent non-interaction.

## Interaction derivation

For each resolved assumption whose consumed identity is the producer's previous
guarantee identity, the pair `(producer, consumer)` receives a
`changed_guarantee_affects_assumption` interaction. The assessment is:

- `conflict` when the guarantee was removed, or when a strict consumer's exact
  previous identity was weakened or replaced.
- `unknown` when the change declares unknown behaviour, when finite typing is
  unsupported, or when bounded exploration cannot complete.
- Otherwise a finite check runs when previous and current guarantees and the
  consumer predicate are all finite expressions over the same producer
  requirement revision. The check enumerates every total observation table over
  the sorted declared domains and observables within `max_worlds` and
  `max_steps`.
  - previous implies the predicate and current does not: `conflict` with a
    retained distinguishing observation table.
  - current still implies the predicate: `needs_joint_evidence`; finite
    preservation of one assumption never discharges the combined candidate.
  - neither direction holds: `unknown`; the change is not credited or blamed
    for a pre-existing gap.
- Without a finite check, declared strengthening, monotone weakening or
  monotone replacement yields `needs_joint_evidence`; an uninterpreted change
  yields `unknown`.

Invalidations create `invalidation_affects_evidence_basis` when the invalidated
obligation is named by another change's evidence basis, and
`invalidation_undermines_invariant` (`conflict`) when it is named as another
change's preserved invariant.

Invariant commitments overlap when two changes declare the same invariant id or
the same commitment identity; the result is always `needs_joint_evidence`
because an identical declaration is not proof and different declarations need
joint assessment.

Interface declarations with the same id and different configuration identities
are `conflict` (`interface_configuration_mismatch`); a remove operation against
an add or modify operation is `conflict` (`interface_operation_conflict`).

## Compatibility obligations

Interactions are grouped by participant pair. The combined candidate selects
the smallest declared composition covering the pair, binds every participant's
source and intent revision plus requirement identities, the case context, and
every admitted composition order. A composition is admissible only when its
participants and order are declared permutations and `admitted` is true.

The obligation names participants, candidate identity, interaction ids,
context, affected claims, interfaces, configurations, affected invariants,
required integration conditions, required evidence and a fingerprint of
participants, dependencies, configurations and composition.

Coexistence, invariant preservation and commutativity are separate
propositions. `required_evidence` states the scope needed for each. Joint
evidence is admitted only when it names the exact obligation, the exact
combined candidate identity, the participant set, an admitted composition (or
all admitted compositions), the current context and every affected claim. A
`witness` scope never contributes coexistence or preservation. Preservation
over more than one admitted composition requires
`all_admitted_compositions`. Commutativity is `not_applicable` without an
order-sensitive composition, remains `not_established` under a single fixed
order, and is supported only by explicit all-admitted-composition evidence;
invariant preservation never implies it.

Individual receipts are reported with `change_known`, `requirement_known` and
`discharges_combined_obligation = false`. They never discharge the obligation
and are listed separately from joint evidence.

Outcome precedence is `conflict`, then `unknown` from participant unknown scope
or unresolved gaps, then `unknown` from interactions, then
`supported_within_declared_scope` when required propositions are covered, and
otherwise `needs_joint_evidence`.

## Explanations and invalidation

Every obligation carries a deterministic step list: guarantee change,
assumption consumption with meaning and dependency, finite check with world and
evaluation counts and witness presence, invalidation, invariant overlap,
interface conflict, admitted compositions, evidence admission and the final
outcome with its basis.

Each obligation's fingerprint binds participant intent revisions, guarantee and
assumption identities, invalidation and evidence-basis references, invariant
commitment identities, affected interface configurations and composition
orders. `prior_obligations` supplied in the case are compared against the
current report; a changed participant, dependency, configuration or composition
makes the prior obligation inapplicable with explicit reasons, and an absent
obligation id reports `obligation_absent`.

## Bounds and determinism

`FootprintPolicy::admit` validates the schema, non-zero bounds and fixed
implementation ceilings. Declaration counts, composition and evidence counts,
domain values, worlds, evaluation steps and typing depth are bounded with
checked arithmetic; any exhaustion is an explicit `bound_exhaustion` or an
`unknown` interaction basis, never a hidden default.

All maps and lists are sorted before hashing or reporting. Identical admitted
inputs, policy and prior obligations produce identical input, policy, candidate,
obligation and report identities.

## CLI and tests

`cairn footprints inspect --case FILE [--policy FILE] [--root DIR]` performs
bounded no-follow reads only. It prints a versioned advisory envelope with the
report or a tagged rejection, reports `lifecycle_authority = none`, and writes
no lifecycle state. Unknown options, a missing case, malformed JSON, an invalid
policy and an unavailable default projection are non-accepting outcomes.

Core tests cover a compatible pair, the retry/authorization conflict with its
affected invariant and missing integration condition, individual-receipt
non-discharge, order-sensitive composition and commutativity separation,
witness-scope insufficiency, invalidation and prior applicability, unknown
scope, unsupported interpretation, bounded exploration, incomplete declaration,
interface conflicts, malformed declarations, deterministic replay and the
independence of the existing textual-conflict and verification-impact surfaces.
CLI tests exercise the advisory report, the conflict path, supported joint
evidence, order-sensitive propositions and non-accepting rejection outcomes.

## Non-claims

The analysis covers only declared footprints over one finite semantics profile.
Absence of a detected interaction is limited to the declared footprint; complete
semantic conflict detection for arbitrary programs, prose correspondence,
observable extraction, reviewer authenticity and native registry freshness
remain explicit trust dependencies.

The analysis does not prove behavioural independence, merge permission,
implementation correctness, readiness or release eligibility. It does not
resolve a native obligation, transfer or refresh evidence, promote an
individual receipt, or change a gate, verification role, collaboration
resolution or verification-impact decision.

## Publication

The policy projection is generated by the parent with:

```console
nix develop -c nickel export --format json cairn-policy/concurrent-change-footprints.ncl \
  | jq -S . > cairn-policy/generated/concurrent-change-footprints.json
```

The generated file is intentionally not authored here; the CLI reports an
explicit unavailable-policy rejection until the parent generates it or the
caller supplies `--policy`.
