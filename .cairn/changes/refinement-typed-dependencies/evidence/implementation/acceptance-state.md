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

`cargo test -p cairn-core --lib refinement` ran **17 passed, 2 failed** at that
point; the resolution is recorded below and the suite is now 19 passed.

## Resolved: bounds are part of context identity

Both failures shared one root cause. `evaluate_semantic_dependency` treats a
context as compatible only when the producer and consumer `ArgumentScope` agree
with the dependency's `DependencyContext` on `policy_blake3`, `environment`,
`bounds` and the two targets. The shared `semantic_fixture` took its scopes from
`scope()` (`bounds = {max_checks: 2}`) while `dependency_baseline` supplied
(`bounds = {max_tasks: 4}`), so every semantic discharge in that fixture was
rejected as `unsupported: context_incompatible`.

The conservative reading is kept: **resource bounds are part of context identity**
— a guarantee checked under one resource envelope is not the same guarantee under
another, so relaxing the comparison would weaken the check. The fixture now
declares the same bounds on both sides, and `cargo test -p cairn-core --lib
refinement` is **19 passed, 0 failed**.

That decision is recorded here rather than silently applied: a future change that
wants bounds to be non-identifying must change the comparison and this note
together.

## Claim boundary

The typed dependency relations (ordering/archive, verified-specification,
verified-candidate, candidate-input, integrated-revision, lifecycle, semantic)
are implemented with named authoritative evidence and non-claims; semantic
discharge requires producer guarantee, consumer assumption, compatible context and
a witness binding both statement identities. Matching names or receipt identities
alone never discharge. No live multi-repository execution was performed.
