# Correspondence review: claim_revision

- Requirement: `cairn.verification_obligation_cob.claim_revision`
- Requirement row: `1de40130cc0a88bb88288b16daa621de599b1508511648332e9e23ef11a3b666`
- Contract: `1e160a4066a673c0cd600aa1e3ac6e5270b71f1c15791a6903e264dc7d0e3405`
- Interpretation: `8b04a2b9d425c8b417693f33c621ef5d922475d5df342565d9e91ceca0bbe648`
- Mapping: `a3727644be01e4c181c36cf3884fc8e5474b3fc9af515f03332f5421542f2790`

## Reviewed statement

Every evidence link, finding, verifier decision and discharge is bound to one
exact claim root, and a new claim revision preserves prior decision facts while
making them inapplicable to the new root.

## Assessment

The activation condition is an admitted claim-field revision, and the guarantee
requires the exact revision token to change, the lifecycle epoch to reset to
`open`, and the selected finite operation history to be preserved. That captures
the review-epoch transition and history retention the requirement names, and it
prevents a prior discharge from being inherited across the epoch boundary.

This is a **partial** mapping and is approved as such. The requirement's stronger
per-fact binding language is not established: `WorkflowState` does not carry an
individual claim-root field on every evidence link, finding and decision. The
contract's `informal_remainder` states this explicitly, together with retained
full claim payloads, authorized complete-basis revision admission, concurrent
heads, numeric increment/overflow and arbitrary history.

## Scope of this approval

Approval covers only the declared supported portion plus correct exclusion of
the declared remainder. It must not be cited as evidence that per-fact claim-root
binding is implemented.

## Reviewer declaration

Independent review performed by the parent integration owner on 2026-11-18 by
reading the accepted requirement and the normalized contract. This is a recorded
declaration, not a signature, not proof, and not a human domain-expert review.
Decision: approved.
