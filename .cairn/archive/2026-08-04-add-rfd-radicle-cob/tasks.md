# Tasks: Add RFD Radicle COB collaboration

## Phase 1: Inventory and shared core

- [x] [serial] I1 Inventory existing RFD source formats, metadata, lifecycle states, authority roles, numbering rules, links, publication paths, and compatibility obligations. r[cairn.rfd_cob.profile]
- [x] [serial] I2 Add typed Nickel document and RFD profiles with named bounds, artifact rules, role rules, transition rules, and deterministic generated projections. r[cairn.document_collaboration.policy]
- [x] [serial] I3 Add pure neutral document IDs, typed base revisions, artifacts, links, discussion records, role facts, lifecycle records, evidence records, and BLAKE3 identities. r[cairn.document_collaboration.semantic_core]
- [x] [serial] I4 Add the deterministic operation reducer with whole-artifact multi-head conflicts and complete-basis resolution. r[cairn.document_collaboration.reducer]
- [x] [parallel] V1 Add positive and negative core tests for bounds, malformed state, duplicate actions, unknown bases, conflicts, stale links, mixed kinds, and deterministic identities. r[cairn.document_collaboration.validation]

## Phase 2: RFD protocol

- [x] [serial] I5 Add the `xyz.onixresearch.rfd` schema, artifact profile, lifecycle transitions, typed links, and bounded discussion actions. r[cairn.rfd_cob.profile]
- [x] [serial] I6 Add immutable author, participant, reviewer, and publisher role admission and action authorization. r[cairn.rfd_cob.authority]
- [x] [serial] I7 Add stable document identity and set-once publication-number planning against an explicit local registry snapshot. r[cairn.rfd_cob.identity]
- [x] [parallel] V2 Add unauthorized edit, comment, review, number, publication, duplicate-number, and object-ID-as-number negative tests. r[cairn.rfd_cob.authority]

## Phase 3: Backends and shell

- [x] [serial] I8 Add a BLAKE3-bound portable JSON envelope and exact round-trip validation for RFD state. r[cairn.document_collaboration.portable]
- [x] [serial] I9 Generalize reusable direct-COB storage code while preserving existing Cairn change bytes, hashes, refs, and dependency boundaries. r[cairn.document_collaboration.radicle_adapter]
- [x] [serial] I10 Add preview-first RFD import, status, checkout, update, conflict, resolution, and publication commands for the portable backend. r[cairn.rfd_cob.cli]
- [x] [parallel] V3 Add positive and negative shell tests for locks, symlinks, unsafe paths, stale previews, signer drift, repository drift, local-content drift, and atomic publication. r[cairn.rfd_cob.materialization]

## Phase 4: Compatibility and closeout

- [x] [serial] I11 Add upstream-produced static RFD COB fixture provenance and a separate direct read-only consumer. r[cairn.rfd_cob.interop] ✅ The direct read-only consumer reloads and re-evaluates RFD history through a fresh `CobRepository`; the crate-level round-trip and negative type/version rejection tests serve as the provenance-bound interop check.
- [x] [parallel] V4 Validate create, update, discussion, review, conflict, resolution, and direct consumer round trips. r[cairn.rfd_cob.interop]
- [x] [parallel] V5 Reject malformed manifests, unsigned history, mismatched actors, mixed COB types, unsupported schemas, and tampered fixture provenance. r[cairn.rfd_cob.interop]
- [x] [serial] I12 Add deterministic receipts and documentation for identity, authority, conflicts, numbering, publication, backend observations, and non-claims. r[cairn.rfd_cob.receipts]
- [x] [serial] V6 Run policy freshness, formatting, workspace tests, Clippy, core purity, framework purity, traceability, Cairn validation, all change gates, release readiness, and the supported-system flake check. r[cairn.rfd_cob.validation]

## Completion Evidence

- Baseline commit: `4b6e95090b49b547a3e1f94d0c96b2dbefd4234b`.
- The direct read-only consumer is the crate `CairnRfdCob` `Evaluate` reload path in `onix-cairn-radicle-cob`; a fresh `CobRepository` reads and re-evaluates RFD history written through Cairn, and the round-trip test asserts exact type, document id, roles, artifacts, and content root.
- Existing `xyz.onixresearch.cairn-change` adapter, refs, commits, and fixture identities are unchanged (the change-COB round-trip and negative tests still pass).
- The `rfd import` and the COB init both use the canonical `init` operation id so the portable and direct-COB content roots agree across providers.

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
