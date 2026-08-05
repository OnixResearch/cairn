# Proposal: Add policy-exception Radicle COB collaboration

## Why

Policy exceptions are collaborative decisions with exact scope, rationale, compensating controls, approvals, validity bounds, revocation, and closure. Plain files do not provide signed offline operation history or explicit concurrent-state conflicts.

A policy-exception COB can record this review lifecycle without granting authority by itself. Basalt, Cairn, or another policy consumer will retain the decision to honor an approved exception.

## What Changes

- Add the custom COB type `xyz.onixresearch.policy-exception` over the shared neutral workflow collaboration state.
- Bind each exception to an exact policy identity, stable rule ID, typed scope, rationale, compensating controls, and finite validity bounds.
- Separate the stable exception ID from the provider object ID.
- Add immutable requester, reviewer, approver, revoker, and validity-observer authority sets.
- Add review, approval, rejection, revocation, explicit validity observation, expiration, and closure transitions.
- Keep clock access and policy enforcement outside the pure reducer.
- Add preview-first portable and direct-COB workflows, deterministic receipts, positive and negative fixtures, and independent interoperability evidence.

## Impact

- **Dependency**: This change depends on `add-verification-obligation-radicle-cob` for `cairn.workflow-collaboration.v1` and reusable workflow backends.
- **Core**: Add pure exception profile, scope, controls, authority, lifecycle, observation, applicability, conflict, and receipt models.
- **CLI**: Add bounded exception commands with explicit policy snapshots, time observations, Git repositories, and keystores.
- **Policy**: Add typed Nickel exception scope, role, approval, validity, transition, observation-source, and named-bound profiles.
- **Testing**: Add positive and negative scope, authority, policy-drift, stale-approval, validity, expiration, revocation, conflict, receipt, and interoperability fixtures.

## Non-goals

- Do not grant a UCAN capability, bypass a policy check, mutate Nickel policy, or force any consumer to honor an exception.
- Do not read a clock, environment variable, profile, identity document, or remote policy service in the pure reducer.
- Do not claim that a valid signature proves organizational trust, current applicability, or sufficient compensating controls.
- Do not make a COB object ID the exception ID, policy ID, or rule ID.
- Do not add Radicle node, discovery, synchronization, or transport behavior.
