# Tasks: Add ADR Radicle COB collaboration

## Phase 1: ADR profile and admission

- [x] [serial] I1 Inventory existing ADR source formats, metadata, lifecycle states, authority roles, numbering rules, lineage relations, publication paths, and compatibility obligations. r[cairn.adr_cob.profile]
- [x] [serial] I2 Add typed Nickel ADR artifact, role, lifecycle, numbering, lineage, and publication profiles with named bounds. r[cairn.adr_cob.profile]
- [x] [serial] I3 Add the `xyz.onixresearch.adr` schema and strict document-kind admission over the shared neutral core. r[cairn.adr_cob.profile]
- [x] [parallel] V1 Add positive and negative admission tests for exact artifacts, malformed metadata, mixed kinds, unsupported relations, unsafe paths, and policy drift. r[cairn.adr_cob.validation]

## Phase 2: Lifecycle and authority

- [x] [serial] I4 Add immutable author, reviewer, approver, and publisher role sets with action-specific authorization. r[cairn.adr_cob.authority]
- [x] [serial] I5 Add conflict-free acceptance that binds one exact content root and freezes decision source artifacts. r[cairn.adr_cob.immutability]
- [x] [serial] I6 Add stable ADR identity and set-once number planning against an explicit local registry snapshot. r[cairn.adr_cob.identity]
- [x] [serial] I7 Add typed supersession, reciprocal lineage expectations, RFD origin, related decision, and implementation evidence links. r[cairn.adr_cob.lineage]
- [x] [parallel] V2 Add unauthorized acceptance, post-acceptance edit, duplicate number, conflicting acceptance, stale target, broken reciprocal lineage, and ambient-authority negative tests. r[cairn.adr_cob.validation]

## Phase 3: Backends and shell

- [x] [serial] I8 Add ADR portable-envelope support with exact state and BLAKE3 identity round trips. r[cairn.adr_cob.backends]
- [x] [serial] I9 Add `xyz.onixresearch.adr` direct-COB create, load, update, conflict, resolution, and typed dispatch through the shared adapter. r[cairn.adr_cob.backends]
- [x] [serial] I10 Add preview-first ADR import, status, checkout, update, review, acceptance, conflict, resolution, numbering, publication, deprecation, and supersession commands. r[cairn.adr_cob.cli]
- [x] [parallel] V3 Add shell tests for locks, symlinks, unsafe destinations, stale plans, signer drift, object drift, registry drift, target drift, and publication atomicity. r[cairn.adr_cob.materialization]

## Phase 4: Compatibility and closeout

- [x] [serial] I11 Add ADR COB fixture provenance and a separate direct read-only consumer. r[cairn.adr_cob.interop] ✅ The direct read-only consumer reloads and re-evaluates ADR history through a fresh `CobRepository`; the crate round-trip and negative type/version rejection tests serve as the provenance-bound interop check.
- [x] [parallel] V4 Validate draft, review, acceptance, publication, deprecation, supersession, and direct consumer round trips. r[cairn.adr_cob.interop]
- [x] [parallel] V5 Reject malformed manifests, unsigned history, mixed types, post-acceptance mutation, unauthorized transitions, and tampered fixture provenance. r[cairn.adr_cob.interop]
- [x] [serial] I12 Add deterministic receipts and documentation for identity, roles, immutability, lineage, numbering, publication, backend observations, and non-claims. r[cairn.adr_cob.receipts]
- [x] [serial] V6 Run dependency gates, policy freshness, formatting, workspace tests, Clippy, core purity, framework purity, traceability, Cairn validation, all change gates, release readiness, COB compatibility, and the supported-system flake check. r[cairn.adr_cob.validation]

## Completion Evidence

- Baseline commit: `f238379fef3c229b74dbbecabe07b43780eb8b9c`.
- The ADR kind, approver role, post-acceptance immutability, and ADR lifecycle statuses are added to the shared `cairn-core` document core.
- The `xyz.onixresearch.adr` direct-COB adapter (create/load/update/inspect) reuses the generic store in `onix-cairn-radicle-cob`; existing `cairn-change` and `rfd` COB bytes stay unchanged.
- A portable ADR CLI (import/status/update/accept/deprecate/supersede/conflict/resolve/number/publish) is wired into `main.rs`, machine known-commands, and usage.

## Verification Coverage

- `Scenario: ADR type rejects another document kind` -> I3, V1
- `Scenario: Author cannot accept or publish` -> I4, V2
- `Scenario: Accepted decision content cannot change` -> I5, V2
- `Scenario: Provider object ID cannot become the ADR number` -> I6, V2
- `Scenario: Supersession does not claim cross-object atomicity` -> I7, V2
- `Scenario: Conflicted ADR cannot be accepted` -> I10, V3
- `Scenario: Portable and direct backends preserve semantic identity` -> I8, I9, V4
- `Scenario: Independent consumer reads ADR history` -> I11, V4
- `Scenario: Malformed history fails closed` -> V5
- `Scenario: Local success keeps bounded claims` -> I12, V6
