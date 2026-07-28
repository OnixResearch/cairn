# Proposal: Add RFD Radicle COB collaboration

## Why

RFD documents need signed offline collaboration, explicit conflict history, lifecycle authority, and deterministic materialization without making Radicle the publication authority.

Cairn already provides these properties for active changes. The implementation is specific to Cairn change artifacts and cannot represent another document family safely.

This change adds a shared backend-neutral document collaboration core and the first typed profile for RFD documents.

## What Changes

- Add a neutral `cairn.document-collaboration.v1` state with BLAKE3 content and state identities.
- Add the custom COB type `xyz.onixresearch.rfd` with an RFD-specific reducer and lifecycle profile.
- Keep published RFD Markdown and Git commits canonical.
- Add a portable JSON backend before the optional direct `radicle-cob` backend.
- Separate stable RFD identity and publication number from the provider object ID.
- Add explicit author, participant, reviewer, and publisher authority.
- Add preview-first import, status, checkout, update, conflict, resolution, and publication planning.
- Add positive and negative fixtures, direct COB interoperability evidence, receipts, and bounded non-claims.

## Impact

- **Core**: Add pure document state, reducer, conflict, authority, identity, link, lifecycle, and materialization models.
- **CLI**: Add bounded filesystem and explicit Git/keystore shells for RFD collaboration.
- **Policy**: Add typed Nickel profiles for document bounds, RFD artifacts, lifecycle transitions, authority, and publication paths.
- **COB adapter**: Generalize reusable direct-COB storage code without adding the umbrella `radicle` crate or a node runtime.
- **Testing**: Add portable round trips, signed COB histories, malformed-history rejection, authority failures, conflicts, publication-number failures, and compatibility fixtures.

## Non-goals

- Do not make a COB object ID the RFD number.
- Do not make COB storage the canonical published document.
- Do not add Radicle discovery, synchronization, seeding, transport, profiles, identity documents, or node control.
- Do not claim line-level text merging, global number uniqueness, remote publication, semantic review, or acceptance correctness.
- Do not add a hosted RFD renderer, search service, webhook service, or number-allocation service.
