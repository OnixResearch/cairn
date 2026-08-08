# Proposal: Add ADR Radicle COB collaboration

## Why

Architecture Decision Records need signed offline collaboration, explicit review authority, immutable accepted decisions, and visible supersession history.

The RFD collaboration change introduces a shared backend-neutral document core. ADR support must use that core without treating an ADR as an RFD or weakening ADR lifecycle rules.

This change adds a separate typed ADR profile and custom Radicle COB adapter while keeping published ADR Markdown and Git commits canonical.

## What Changes

- Add the custom COB type `xyz.onixresearch.adr` over the shared neutral document state.
- Add an ADR-specific artifact and lifecycle profile.
- Freeze accepted decision content and require a new ADR for decision replacement.
- Add typed relations for supersession, related decisions, RFD origin, and implementation evidence.
- Separate stable ADR identity and human numbering from the provider object ID.
- Add explicit author, reviewer, approver, and publisher authority.
- Add preview-first portable and direct-COB workflows for import, status, checkout, update, conflict, resolution, numbering, acceptance, and supersession.
- Add positive and negative fixtures, direct COB interoperability evidence, receipts, and bounded non-claims.

## Impact

- **Dependency**: This change depends on `add-rfd-radicle-cob` for the shared document collaboration core and reusable backends.
- **Core**: Add an ADR profile, lifecycle reducer rules, accepted-content immutability, identity, numbering, and lineage validation.
- **CLI**: Add ADR-specific command admission, materialization, acceptance, numbering, and supersession planning.
- **Policy**: Add typed Nickel ADR artifact, role, lifecycle, numbering, lineage, and publication profiles.
- **Testing**: Add portable and signed COB round trips, unauthorized transition failures, post-acceptance edit failures, lineage failures, malformed-history rejection, and compatibility fixtures.

## Non-goals

- Do not make a COB object ID the ADR number.
- Do not mutate accepted decision content in place.
- Do not make COB storage the canonical published ADR.
- Do not claim that signatures, role checks, or acceptance state prove architectural correctness.
- Do not add Radicle node, profile, identity-document, discovery, synchronization, or transport behavior.
