# Correspondence review: pass_only

- Requirement: `cairn.supersede.agent_run_receipts.pass_only`
- Requirement row: `34abbfe64cae739b75554f48d89d0e0bc19ca8e55d54cf51a9363ffe84a6f58d`
- Contract: `f8a0bc4982dd437653a6a68e43875852a300f03ecf8c429011efbac04d59ef59`
- Interpretation: `8b04a2b9d425c8b417693f33c621ef5d922475d5df342565d9e91ceca0bbe648`
- Mapping: `e84910ddd981f08e8d8a801a0a972def8c96c24a6f7d5d3a41e89bf0098bb21c`

## Reviewed statement

Exactly one eligible `passed` result is required for every selected required
check, and every other status is non-passing completion evidence.

## Assessment

The guarantee expresses both directions for a declared finite universe: every
present row must be selected, `passed` and eligible; every selected check must
have exactly one present row, with a uniqueness clause over the second slot.
Duplicate passed rows are therefore distinguishable from a single passed row,
and `unknown` can never be selected. Unsupported status strings are rejected by
adapter admission rather than mapped to `passed`.

## Scope of this approval

Approval covers the declared supported portion only. The `eligible` observation
summarizes existing native validation; prerequisite graph traversal, argv and
digest validation, external evidence truth, and cardinalities beyond two rows
and two known checks are correctly declared informal rather than proved by token
equality.

## Reviewer declaration

Independent review performed by the parent integration owner on 2026-11-18 by
reading the accepted requirement and the normalized contract. This is a recorded
declaration, not a signature, not proof, and not a human domain-expert review.
Decision: approved.
