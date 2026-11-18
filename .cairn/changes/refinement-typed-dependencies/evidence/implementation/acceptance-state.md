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

Both are in semantic-dependency discharge, and both differ only in the reported
reason:

| Test | Observed | Expected |
|---|---|---|
| `semantic_dependency_discharges_assumption_with_path_and_source` | `Unsupported` | a discharged path with source |
| `matching_names_and_receipt_identity_do_not_discharge_without_witness` | `unsupported: context_incompatible` | the test's asserted state |

The second result is semantically *correct in spirit* — a dependency without a
witness must not discharge — but the reported classification does not match what
the test asserts. The first is the opposite: an admitted semantic dependency with
a witness and matching context is being rejected as `Unsupported`, so either the
context-compatibility rule is stricter than the design intends or the fixture's
context fields do not carry what the implementation compares.

This needs a decision on the intended context-compatibility rule before either
side is changed. No archive, sync or completion claim is made here.

## Claim boundary

The typed dependency relations (ordering/archive, verified-specification,
verified-candidate, candidate-input, integrated-revision, lifecycle, semantic)
are implemented with named authoritative evidence and non-claims; semantic
discharge requires producer guarantee, consumer assumption, compatible context and
a witness binding both statement identities. Matching names or receipt identities
alone never discharge. No live multi-repository execution was performed.
