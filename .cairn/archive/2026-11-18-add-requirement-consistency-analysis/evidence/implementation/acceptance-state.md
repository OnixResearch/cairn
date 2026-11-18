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

**Root cause identified for the vacuity errors.** In `evaluate_query`, an
unsatisfiable goal is mapped to `Vacuous` whenever `plan.vacuous_when_unsatisfiable`
is set. That shortcut is only sound when the *withheld* premise set is itself
satisfiable: the constraining query is `withheld ∧ ¬guarantee`, so unsatisfiability
means "the guarantee adds nothing" only if `withheld` has models. When `withheld`
is already unsatisfiable the implication is vacuous in both directions, and the
honest status is `Inconsistent` (the surrounding set is contradictory) — which is
exactly the contradictory-with-correspondence case that currently reports
`Vacuous`.

The fix is to condition the vacuous classification on a satisfiability check of
the withheld set (a second bounded solve sharing the same work budget), so:

| withheld set | goal `withheld ∧ ¬g` | status |
|---|---|---|
| satisfiable | unsatisfiable | `Vacuous` (the guarantee excludes nothing) |
| satisfiable | satisfiable | `Consistent` (the guarantee constrains) |
| unsatisfiable | either | `Inconsistent` (others contradict) |

That is also why the redundant-vs-constraining count is wrong: the same shortcut
collapses the constraining case into the vacuous one.

Starting there is preferable to adjusting the three assertions: a vacuous
classification for a contradictory pair would report a contradiction as if the
requirement constrained nothing.

No archive, sync or completion claim is made here.

## Claim boundary

A finite model is not unbounded validity, reactivity, prose correspondence or
implementation correctness; no external solver runs; findings never resolve
obligations or change gate verdicts. Correspondence stays `not_reviewed` unless an
admitted review declaration is supplied, and cross-contract identity is never
inferred without an explicit correspondence that admission verifies.
