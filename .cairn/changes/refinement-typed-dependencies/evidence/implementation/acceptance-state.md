# Refinement and typed dependency acceptance state

Status: **compiles; 17/19 core tests pass. Two semantic-discharge cases disagree.**

## Repairs made during acceptance

The branch had never been compiled. Three defects were fixed:

1. `assert_acyclic` was called but never defined. It is now implemented as a
   colour-marked depth-first traversal over the declared parent → child edges,
   reporting `RefinementError::Cycle` with the requirement-id path (falling back to
   row identity) and `BoundExhaustion` past `max_depth`.
2. Two test helpers were shadowed by local bindings of the same name, producing
   `E0618` (`let request = request(...)`, `let mut policy = policy(...)`); the
   calls now use `self::request` / `self::policy`.

`cargo test -p cairn-core --lib refinement` now runs: **17 passed, 2 failed**.

## Outstanding failures

Both failures share one root cause. `evaluate_semantic_dependency` treats a context
as compatible only when the producer and consumer `ArgumentScope` agree with the
dependency's `DependencyContext` on `policy_blake3`, `environment`, `bounds` and
the two targets. In the shared `semantic_fixture` the scopes come from `scope()`
(`bounds = {max_checks: 2}`) while the dependency comes from
`dependency_baseline` (`bounds = {max_tasks: 4}`), so every semantic discharge in
that fixture is classified:

| Test | Observed | Expected |
|---|---|---|
| `semantic_dependency_discharges_assumption_with_path_and_source` | `Unsupported` | discharged path with source |
| `matching_names_and_receipt_identity_do_not_discharge_without_witness` | `unsupported: context_incompatible` | the test's asserted state |

The second result is right in spirit — no witness must not discharge — but it is
reported for the wrong reason.

The open question is narrow and needs a decision rather than a guess: should
resource bounds be part of context identity? Exact equality is the safer reading
(a guarantee checked under one resource envelope is not the same guarantee under
another), but then the fixture must share one bounds map. If bounds are intended
to be non-identifying resource limits, the comparison should drop them and say so.
Changing either side silently would weaken a compatibility check or rewrite a test
to fit an implementation.

No archive, sync or completion claim is made here.

## Claim boundary

The typed dependency relations (ordering/archive, verified-specification,
verified-candidate, candidate-input, integrated-revision, lifecycle, semantic)
are implemented with named authoritative evidence and non-claims; semantic
discharge requires producer guarantee, consumer assumption, compatible context and
a witness binding both statement identities. Matching names or receipt identities
alone never discharge. No live multi-repository execution was performed.
