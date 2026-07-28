# Tasks: Add RFD Radicle COB collaboration

## Phase 1: Inventory and shared core

- [ ] [serial] I1 Inventory existing RFD source formats, metadata, lifecycle states, authority roles, numbering rules, links, publication paths, and compatibility obligations. r[cairn.rfd_cob.profile]
- [ ] [serial] I2 Add typed Nickel document and RFD profiles with named bounds, artifact rules, role rules, transition rules, and deterministic generated projections. r[cairn.document_collaboration.policy]
- [ ] [serial] I3 Add pure neutral document IDs, typed base revisions, artifacts, links, discussion records, role facts, lifecycle records, evidence records, and BLAKE3 identities. r[cairn.document_collaboration.semantic_core]
- [ ] [serial] I4 Add the deterministic operation reducer with whole-artifact multi-head conflicts and complete-basis resolution. r[cairn.document_collaboration.reducer]
- [ ] [parallel] V1 Add positive and negative core tests for bounds, malformed state, duplicate actions, unknown bases, conflicts, stale links, mixed kinds, and deterministic identities. r[cairn.document_collaboration.validation]

## Phase 2: RFD protocol

- [ ] [serial] I5 Add the `xyz.onixresearch.rfd` schema, artifact profile, lifecycle transitions, typed links, and bounded discussion actions. r[cairn.rfd_cob.profile]
- [ ] [serial] I6 Add immutable author, participant, reviewer, and publisher role admission and action authorization. r[cairn.rfd_cob.authority]
- [ ] [serial] I7 Add stable document identity and set-once publication-number planning against an explicit local registry snapshot. r[cairn.rfd_cob.identity]
- [ ] [parallel] V2 Add unauthorized edit, comment, review, number, publication, duplicate-number, and object-ID-as-number negative tests. r[cairn.rfd_cob.authority]

## Phase 3: Backends and shell

- [ ] [serial] I8 Add a BLAKE3-bound portable JSON envelope and exact round-trip validation for RFD state. r[cairn.document_collaboration.portable]
- [ ] [serial] I9 Generalize reusable direct-COB storage code while preserving existing Cairn change bytes, hashes, refs, and dependency boundaries. r[cairn.document_collaboration.radicle_adapter]
- [ ] [serial] I10 Add preview-first RFD import, status, checkout, update, conflict, resolution, and publication commands for portable and direct-COB backends. r[cairn.rfd_cob.cli]
- [ ] [parallel] V3 Add positive and negative shell tests for locks, symlinks, unsafe paths, stale previews, signer drift, repository drift, local-content drift, and atomic publication. r[cairn.rfd_cob.materialization]

## Phase 4: Compatibility and closeout

- [ ] [serial] I11 Add upstream-produced static RFD COB fixture provenance and a separate direct read-only consumer. r[cairn.rfd_cob.interop]
- [ ] [parallel] V4 Validate create, update, discussion, review, conflict, resolution, and direct consumer round trips. r[cairn.rfd_cob.interop]
- [ ] [parallel] V5 Reject malformed manifests, unsigned history, mismatched actors, mixed COB types, unsupported schemas, and tampered fixture provenance. r[cairn.rfd_cob.interop]
- [ ] [serial] I12 Add deterministic receipts and documentation for identity, authority, conflicts, numbering, publication, backend observations, and non-claims. r[cairn.rfd_cob.receipts]
- [ ] [serial] V6 Run policy freshness, formatting, workspace tests, Clippy, core purity, Tiger Style, traceability, Cairn validation, all change gates, release readiness, COB compatibility, self-dogfood, and the supported-system flake check. r[cairn.rfd_cob.validation]

## Verification Coverage

- `Scenario: Neutral state needs no Radicle context` -> I3, V1
- `Scenario: Concurrent artifact edits remain visible` -> I4, V1
- `Scenario: Portable round trip preserves state` -> I8, V3
- `Scenario: Existing Cairn change compatibility remains stable` -> I9, V6
- `Scenario: RFD type rejects another document kind` -> I5, V2
- `Scenario: Author cannot accept or publish` -> I6, V2
- `Scenario: Provider object ID cannot become the RFD number` -> I7, V2
- `Scenario: Conflicted RFD cannot publish` -> I10, V3
- `Scenario: Independent consumer reads RFD history` -> I11, V4
- `Scenario: Malformed history fails closed` -> V5
- `Scenario: Local success keeps bounded claims` -> I12, V6
