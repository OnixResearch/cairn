# Proposal: Add verification-obligation Radicle COB collaboration

## Why

Cairn validates review, proof, and lifecycle receipts, but it does not provide a signed collaboration lifecycle for unresolved verification claims.

Teams need to assign a claim, attach external evidence, record findings, request revisions, and reach a bounded discharge decision across offline replicas. Proof bytes and authoritative receipts must remain immutable external artifacts.

This change adds a shared backend-neutral workflow collaboration core and the first typed profile for verification obligations.

## What Changes

- Add a neutral `cairn.workflow-collaboration.v1` state with BLAKE3 content and state identities.
- Add the custom COB type `xyz.onixresearch.verification-obligation` with a dedicated reducer and lifecycle profile.
- Keep proof, review, check, and receipt artifacts outside the collaborative object as typed evidence references.
- Separate the stable obligation ID from the provider object ID.
- Add immutable submitter, contributor, verifier, and invalidator authority sets.
- Add stable claim revisions, findings, dispositions, evidence links, discharge decisions, invalidation, and explicit conflicts.
- Add a portable JSON backend before the optional direct `radicle-cob` backend.
- Add preview-first commands, deterministic receipts, positive and negative fixtures, and independent direct-COB interoperability evidence.

## Impact

- **Dependency**: This change depends on `add-rfd-radicle-cob` for generalized portable and direct-COB storage primitives, not for document semantics.
- **Core**: Add pure workflow state, reducer, authority, evidence-link, finding, conflict, lifecycle, planning, and receipt models.
- **CLI**: Add bounded filesystem and explicit Git/keystore shells for verification-obligation collaboration.
- **Policy**: Add typed Nickel workflow and verification-obligation profiles with named bounds and generated runtime projections.
- **Testing**: Add reducer convergence, malformed-history rejection, authority failures, stale evidence, unresolved findings, conflicts, discharge, invalidation, receipt, and interoperability fixtures.

## Non-goals

- Do not embed proof bytes, source trees, build outputs, or full review receipts in the COB.
- Do not make a signature equivalent to verification, proof validity, discharge, or trust.
- Do not replace Trellis proof semantics, Octet checks, Valence evidence identity, Cairn semantic-review validation, or release-readiness policy.
- Do not add Radicle node, profile, identity-document, discovery, synchronization, or transport behavior.
- Do not use timestamps, arrival order, or last-writer-wins to resolve concurrent state.
