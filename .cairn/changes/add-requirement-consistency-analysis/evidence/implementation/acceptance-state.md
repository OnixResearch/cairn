# Requirement consistency analysis acceptance state

Status: **compiles; 7/10 core tests pass. Three analysis cases disagree.**

## Repaired during acceptance

`FormulaFragment` derived `PartialOrd, Ord` while containing `FiniteExpr`, which
implements neither, so the crate did not build. The derives are removed; no code
path sorts fragments (`admitted.sort()` sorts identities, not fragment values).

## Outstanding failures

| Test | Observed | Expected |
|---|---|---|
| `redundant_and_constraining_guarantees_are_distinguished` | 0 constraining guarantees | 1 — “the guarantee duplicated by an assumption adds no excluded behavior” |
| `contradictory_guarantees_conflict_only_when_correspondence_is_declared` | `Vacuous` | `Inconsistent` |
| `exhausted_bounds_and_disabled_queries_never_conclude_consistent` | 2 | 1 |

These are analysis results, not wiring: the first fails to distinguish a guarantee
that is merely redundant from one that constrains behaviour, the second classifies
a genuinely contradictory pair as vacuous, and the third reports one more
incomplete/unsupported entry than the fixture declares.

They share an area — how a guarantee's excluded models are computed and how a
correspondence-declared pair is combined — so a fix should start from the
strong-Kleene enumeration in `analyze_consistency` and the
`guarantee_constraining` query, with the contradictory-pair case checked first
because a vacuous classification there is the most consequential error: it would
report a contradiction as if the requirement constrained nothing.

No archive, sync or completion claim is made here.

## Claim boundary

A finite model is not unbounded validity, reactivity, prose correspondence or
implementation correctness; no external solver runs; findings never resolve
obligations or change gate verdicts. Correspondence stays `not_reviewed` unless an
admitted review declaration is supplied, and cross-contract identity is never
inferred without an explicit correspondence that admission verifies.
