# Tasks: Add verification-obligation Radicle COB collaboration

## Phase 1: Shared workflow core

- [ ] [serial] I1 Inventory workflow identities, field revisions, authority roles, lifecycle states, evidence roles, protocol bounds, and compatibility obligations. r[cairn.workflow_collaboration.semantic_core]
- [ ] [serial] I2 Add typed Nickel workflow profiles with named bounds, role rules, evidence roles, conflict rules, lifecycle rules, and deterministic generated projections. r[cairn.workflow_collaboration.policy]
- [ ] [serial] I3 Add pure workflow IDs, typed base revisions, field revisions, root-bound roles, evidence references, lifecycle facts, conflict state, and BLAKE3 identities. r[cairn.workflow_collaboration.semantic_core]
- [ ] [serial] I4 Add the atomic deterministic reducer with multi-head conflicts and complete-basis resolution. r[cairn.workflow_collaboration.reducer]
- [ ] [serial] I5 Add typed external evidence references with verification-role and claim-scope preservation. r[cairn.workflow_collaboration.evidence]
- [ ] [parallel] V1 Add positive and negative core tests for bounds, malformed state, duplicate actions, unknown bases, mixed kinds, conflicts, evidence-role promotion, and deterministic identities. r[cairn.workflow_collaboration.validation]

## Phase 2: Verification-obligation protocol

- [ ] [serial] I6 Add the `xyz.onixresearch.verification-obligation` schema, stable obligation identity, exact claim binding, and strict type admission. r[cairn.verification_obligation_cob.profile]
- [ ] [serial] I7 Add immutable submitter, contributor, verifier, and invalidator authority with action-specific checks. r[cairn.verification_obligation_cob.authority]
- [ ] [serial] I8 Add claim-root revisions that make prior review facts inapplicable without deleting history. r[cairn.verification_obligation_cob.claim_revision]
- [ ] [serial] I9 Add stable findings, responses, dispositions, source spans, blocking classes, and explicit disposition conflicts. r[cairn.verification_obligation_cob.findings]
- [ ] [serial] I10 Add conflict-free discharge, rejection, and post-discharge invalidation over policy-named decision rules. r[cairn.verification_obligation_cob.discharge]
- [ ] [parallel] V2 Add unauthorized action, stale claim root, unresolved blocker, missing evidence, conflicting decision, and ambient-authority negative tests. r[cairn.verification_obligation_cob.validation]

## Phase 3: Backends and shell

- [ ] [serial] I11 Add a BLAKE3-bound portable JSON workflow envelope with exact conflict-free and conflicted round trips. r[cairn.workflow_collaboration.portable]
- [ ] [serial] I12 Extend generalized direct-COB storage for workflow dispatch while preserving existing Cairn change and document fixture bytes, hashes, and refs. r[cairn.workflow_collaboration.radicle_adapter]
- [ ] [serial] I13 Add preview-first verification-obligation import, status, update, evidence, finding, response, review, conflict, resolution, discharge, rejection, invalidation, and export commands. r[cairn.verification_obligation_cob.cli]
- [ ] [parallel] V3 Add positive and negative shell tests for locks, symlinks, unsafe paths, stale plans, signer drift, repository drift, state drift, and atomic writes. r[cairn.verification_obligation_cob.cli]

## Phase 4: Receipts, compatibility, and closeout

- [ ] [serial] I14 Add deterministic workflow and verification-obligation receipts with explicit provider observations and bounded non-claims. r[cairn.workflow_collaboration.receipts]
- [ ] [serial] I15 Add provenance-bound static verification-obligation COB fixture data and a separate direct read-only consumer. r[cairn.verification_obligation_cob.interop]
- [ ] [parallel] V4 Validate create, revise, attach evidence, find, respond, discharge, invalidate, resolve, portable round trip, and direct-consumer round trip. r[cairn.verification_obligation_cob.interop]
- [ ] [parallel] V5 Reject malformed manifests, unsigned history, mismatched actors, mixed types, stale evidence scopes, unresolved blockers, and tampered fixture provenance. r[cairn.verification_obligation_cob.interop]
- [ ] [serial] I16 Add operator documentation for identities, roles, conflicts, evidence boundaries, discharge, invalidation, backends, and non-claims. r[cairn.verification_obligation_cob.receipts]
- [ ] [serial] V6 Run dependency gates, policy freshness, formatting, workspace tests, Clippy, core purity, Tiger Style, traceability, Cairn validation, all change gates, release readiness, COB compatibility, self-dogfood, and the supported-system flake check. r[cairn.verification_obligation_cob.validation]

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
