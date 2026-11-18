# Correspondence review: freshness

- Requirement: `cairn.supersede.agent_run_receipts.freshness`
- Requirement row: `653914140db582ef1145f36f87c233a9ab57a310f715cadf8a23efa5f5333e5e`
- Contract: `303394b1a46b0fe715c346c50fc05369ffcc85e80bbbc5445d91d666574e4932`
- Interpretation: `8b04a2b9d425c8b417693f33c621ef5d922475d5df342565d9e91ceca0bbe648`
- Mapping: `b59d990b291e6d6853814c1f8c0a3bea993f5d871e21b1d55ce870c909f5028a`

## Reviewed statement

Every completion-eligible result and the aggregate completion receipt are bound
to the final post-mutation subject identity used for the decision.

## Assessment

The guarantee conjoins three exact equalities against the single `final_subject`
observation: the repository contract subject, the aggregate report subject and
every eligible row subject. A changed final subject therefore makes an
otherwise-passing set stale rather than current, which is precisely the
distinction the requirement names.

## Scope of this approval

Approval covers the declared supported portion only. Filesystem subject
measurement, mutation ordering, hash syntax and authenticity, and arbitrary
result universes are correctly excluded.

## Reviewer declaration

Independent review performed by the parent integration owner on 2026-11-18 by
reading the accepted requirement and the normalized contract. This is a recorded
declaration, not a signature, not proof, and not a human domain-expert review.
Decision: approved.
