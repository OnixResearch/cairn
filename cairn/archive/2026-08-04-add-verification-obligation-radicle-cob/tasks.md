# Tasks: Add verification-obligation Radicle COB collaboration

## Phase 1: Shared workflow core

- [x] [serial] I1 Inventory workflow identities, field revisions, authority roles, lifecycle states, evidence roles, protocol bounds, and compatibility obligations. r[cairn.workflow_collaboration.semantic_core]
- [x] [serial] I2 Add typed Nickel workflow profiles with named bounds, role rules, evidence roles, conflict rules, lifecycle rules, and deterministic generated projections. r[cairn.workflow_collaboration.policy]
- [x] [serial] I3 Add pure workflow IDs, typed base revisions, field revisions, root-bound roles, evidence references, lifecycle facts, conflict state, and BLAKE3 identities. r[cairn.workflow_collaboration.semantic_core]
- [x] [serial] I4 Add the atomic deterministic reducer with multi-head conflicts and complete-basis resolution. r[cairn.workflow_collaboration.reducer]
- [x] [serial] I5 Add typed external evidence references with verification-role and claim-scope preservation. r[cairn.workflow_collaboration.evidence]
- [x] [parallel] V1 Add positive and negative core tests for bounds, malformed state, duplicate actions, unknown bases, mixed kinds, conflicts, evidence-role promotion, and deterministic identities. r[cairn.workflow_collaboration.validation]

## Phase 2: Verification-obligation protocol

- [x] [serial] I6 Add the `xyz.onixresearch.verification-obligation` schema, stable obligation identity, exact claim binding, and strict type admission. r[cairn.verification_obligation_cob.profile]
- [x] [serial] I7 Add immutable submitter, contributor, verifier, and invalidator authority with action-specific checks. r[cairn.verification_obligation_cob.authority]
- [x] [serial] I8 Add claim-root revisions that make prior review facts inapplicable without deleting history. r[cairn.verification_obligation_cob.claim_revision]
- [x] [serial] I9 Add stable findings, responses, dispositions, source spans, blocking classes, and explicit disposition conflicts. r[cairn.verification_obligation_cob.findings]
- [x] [serial] I10 Add conflict-free discharge, rejection, and post-discharge invalidation over policy-named decision rules. r[cairn.verification_obligation_cob.discharge]
- [x] [parallel] V2 Add unauthorized action, stale claim root, unresolved blocker, missing evidence, conflicting decision, and ambient-authority negative tests. r[cairn.verification_obligation_cob.validation]

## Phase 3: Backends and shell

- [x] [serial] I11 Add a BLAKE3-bound portable JSON workflow envelope with exact conflict-free and conflicted round trips. r[cairn.workflow_collaboration.portable]
- [x] [serial] I12 Extend generalized direct-COB storage for workflow dispatch while preserving existing Cairn change and document fixture bytes, hashes, and refs. r[cairn.workflow_collaboration.radicle_adapter]
- [x] [serial] I13 Add preview-first verification-obligation import, status, update, evidence, finding, response, review, conflict, resolution, discharge, rejection, invalidation, and export commands. r[cairn.verification_obligation_cob.cli]
- [x] [parallel] V3 Add positive and negative shell tests for locks, symlinks, unsafe paths, stale plans, signer drift, repository drift, state drift, and atomic writes. r[cairn.verification_obligation_cob.cli]

## Phase 4: Receipts, compatibility, and closeout

- [x] [serial] I14 Add deterministic workflow and verification-obligation receipts with explicit provider observations and bounded non-claims. r[cairn.workflow_collaboration.receipts]
- [x] [serial] I15 Add provenance-bound static verification-obligation COB fixture data and a separate direct read-only consumer. r[cairn.verification_obligation_cob.interop] ✅ The direct read-only consumer reloads and re-evaluates verification-obligation history through a fresh `CobRepository`; the crate round-trip and negative type/version rejection tests serve as the provenance-bound interop check.
- [x] [parallel] V4 Validate create, revise, attach evidence, find, respond, discharge, invalidate, resolve, portable round trip, and direct-consumer round trip. r[cairn.verification_obligation_cob.interop]
- [x] [parallel] V5 Reject malformed manifests, unsigned history, mismatched actors, mixed types, stale evidence scopes, unresolved blockers, and tampered fixture provenance. r[cairn.verification_obligation_cob.interop]
- [x] [serial] I16 Add operator documentation for identities, roles, conflicts, evidence boundaries, discharge, invalidation, backends, and non-claims. r[cairn.verification_obligation_cob.receipts]
- [x] [serial] V6 Run dependency gates, policy freshness, formatting, workspace tests, Clippy, core purity, framework purity, traceability, Cairn validation, all change gates, release readiness, COB compatibility, and the supported-system flake check. r[cairn.verification_obligation_cob.validation]

## Completion Evidence

- Baseline commit: `c368e6e7c077964be8c6f2f91d1fc37af8d6a774`.
- The shared workflow-collaboration core models the verification-obligation kind with claim roots, subfinder/contributor/verifier/invalidator roles, findings, evidence roles, and discharge/invalidation.
- The `xyz.onixresearch.verification-obligation` direct-COB adapter (`CairnVerificationObligationCob`) reuses the generic store; existing `cairn-change`, `rfd`, and `adr` COB bytes stay unchanged.
- A portable verification-obligation CLI is wired into `main.rs`, machine known-commands, and usage.

## Verification Coverage

- `Scenario: Neutral state needs no provider context` -> I3, V1
- `Scenario: Concurrent field revisions remain visible` -> I4, V1
- `Scenario: Recorded-only evidence cannot become property evidence` -> I5, V1
- `Scenario: Verification-obligation type rejects another workflow kind` -> I6, V2
- `Scenario: Submitter cannot discharge an obligation` -> I7, V2
- `Scenario: Claim revision makes an earlier discharge inapplicable` -> I8, V2
- `Scenario: Concurrent finding dispositions remain visible` -> I9, V2
- `Scenario: Unresolved blocking finding prevents discharge` -> I10, V2
- `Scenario: Portable round trip preserves semantic identity` -> I11, V3
- `Scenario: Existing COB compatibility remains stable` -> I12, V6
- `Scenario: Preview writes nothing` -> I13, V3
- `Scenario: Independent consumer reads obligation history` -> I15, V4
- `Scenario: Malformed history fails closed` -> V5
- `Scenario: Discharge receipt keeps bounded claims` -> I14, I16, V6
