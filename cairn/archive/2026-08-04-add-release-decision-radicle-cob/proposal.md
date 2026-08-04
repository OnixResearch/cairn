# Proposal: Add release-decision Radicle COB collaboration

## Why

Release decisions combine one immutable candidate identity, external evidence references, findings, blockers, approvals, and a final verdict. These decisions often involve several signed actors and can continue while peers are offline.

A release-decision COB can preserve collaborative review history without building, signing, publishing, deploying, rolling back, or proving release eligibility.

## What Changes

- Add the custom COB type `xyz.onixresearch.release-decision` over the shared neutral workflow collaboration state.
- Bind each decision to one immutable candidate root, exact policy identity, typed target labels, evidence references, findings, blockers, and role sets.
- Separate the stable decision ID, candidate root, artifact identities, and provider object ID.
- Add immutable proposer, reviewer, approver, withdrawer, and invalidator authority sets.
- Add policy-defined review-basis, approval, rejection, withdrawal, invalidation, conflict-resolution, and supersession rules.
- Add preview-first portable and direct-COB workflows, deterministic receipts, positive and negative fixtures, and independent interoperability evidence.

## Impact

- **Dependency**: This change depends on `add-verification-obligation-radicle-cob` for `cairn.workflow-collaboration.v1` and reusable workflow backends.
- **Core**: Add pure candidate, evidence-link, finding, blocker, vote, verdict, conflict, lifecycle, and receipt models.
- **CLI**: Add bounded release-decision commands with explicit evidence snapshots, Git repositories, and keystores.
- **Policy**: Add typed Nickel target, evidence, role, blocker, decision, transition, and named-bound profiles.
- **Testing**: Add positive and negative candidate, evidence, authority, stale-vote, blocker, conflict, verdict, withdrawal, invalidation, receipt, and interoperability fixtures.

## Non-goals

- Do not build, sign, publish, deploy, roll back, delete, retain, or transfer release artifacts.
- Do not claim that approvals prove evidence truth, source correctness, build reproducibility, deployment success, or release eligibility.
- Do not make a COB object ID the decision ID, candidate root, release number, or artifact identity.
- Do not replace Cairn release-readiness, release evidence, registry, publication, retention, or rotation validators.
- Do not add Radicle node, profile, identity-document, discovery, synchronization, or transport behavior.
