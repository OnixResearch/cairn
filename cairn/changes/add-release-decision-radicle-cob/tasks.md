# Tasks: Add release-decision Radicle COB collaboration

## Phase 1: Profile and candidate binding

- [ ] [serial] I1 Inventory release candidate identities, target labels, evidence roles, findings, blocker classes, authority roles, decision rules, lifecycle states, and compatibility obligations. r[cairn.release_decision_cob.profile]
- [ ] [serial] I2 Add typed Nickel release-decision profiles with named target, evidence, role, blocker, verdict, transition, and protocol bounds. r[cairn.release_decision_cob.policy]
- [ ] [serial] I3 Add the `xyz.onixresearch.release-decision` schema, stable decision identity, strict type admission, and immutable candidate-root binding. r[cairn.release_decision_cob.profile]
- [ ] [serial] I4 Add candidate descriptors that keep decision, source, artifact, manifest, release label, evidence, and provider identities separate. r[cairn.release_decision_cob.candidate]
- [ ] [parallel] V1 Add positive and negative profile tests for exact candidates, candidate substitution, malformed labels, mixed kinds, identity aliasing, unsafe bounds, and deterministic roots. r[cairn.release_decision_cob.validation]

## Phase 2: Evidence, blockers, and verdicts

- [ ] [serial] I5 Add immutable proposer, reviewer, approver, withdrawer, and invalidator role sets with action-specific authorization. r[cairn.release_decision_cob.authority]
- [ ] [serial] I6 Add typed external evidence links and validation summaries that compose existing Cairn validators without duplicating them. r[cairn.release_decision_cob.evidence]
- [ ] [serial] I7 Add stable review findings, blockers, responses, append-only dispositions, and explicit disposition conflicts. r[cairn.release_decision_cob.blockers]
- [ ] [serial] I8 Add review-basis roots that make earlier votes inapplicable after evidence, finding, blocker, target, exception, or policy drift. r[cairn.release_decision_cob.decision]
- [ ] [serial] I9 Add policy-named approval and rejection rules plus withdrawal, invalidation, conflict resolution, and supersession. r[cairn.release_decision_cob.lifecycle]
- [ ] [parallel] V2 Add unauthorized action, stale evidence, role promotion, stale vote, unresolved blocker, conflicting verdict, candidate drift, withdrawal, and invalidation negative tests. r[cairn.release_decision_cob.validation]

## Phase 3: Backends and shell

- [ ] [serial] I10 Add release-decision portable-envelope dispatch with exact BLAKE3 semantic identity round trips. r[cairn.release_decision_cob.backends]
- [ ] [serial] I11 Add `xyz.onixresearch.release-decision` direct-COB create, load, update, conflict, resolution, and typed dispatch through the shared adapter. r[cairn.release_decision_cob.backends]
- [ ] [serial] I12 Add preview-first create, status, evidence, finding, blocker, response, review, vote, withdraw, invalidate, conflict, resolve, supersede, and export commands. r[cairn.release_decision_cob.cli]
- [ ] [parallel] V3 Add shell tests for evidence drift, locks, symlinks, unsafe paths, stale plans, signer drift, repository drift, state drift, and atomic writes. r[cairn.release_decision_cob.cli]

## Phase 4: Receipts, compatibility, and closeout

- [ ] [serial] I13 Add deterministic receipts and documentation for identities, candidate roots, roles, evidence, blockers, review bases, votes, verdicts, backends, and non-claims. r[cairn.release_decision_cob.receipts]
- [ ] [serial] I14 Add provenance-bound static release-decision COB fixture data and a separate direct read-only consumer. r[cairn.release_decision_cob.interop]
- [ ] [parallel] V4 Validate create, attach evidence, review, block, dispose, approve, reject, withdraw, invalidate, resolve, supersede, portable round trip, and direct-consumer round trip. r[cairn.release_decision_cob.interop]
- [ ] [parallel] V5 Reject malformed manifests, unsigned history, mixed types, candidate substitution, stale evidence, stale votes, unauthorized terminal facts, and tampered provenance. r[cairn.release_decision_cob.interop]
- [ ] [serial] V6 Run dependency gates, policy freshness, formatting, workspace tests, Clippy, core purity, Tiger Style, traceability, Cairn validation, all change gates, release readiness, COB compatibility, self-dogfood, and the supported-system flake check. r[cairn.release_decision_cob.validation]

## Verification Coverage

- `Scenario: Release-decision type rejects another workflow kind` -> I3, V1
- `Scenario: Provider object ID cannot become the candidate root` -> I4, V1
- `Scenario: Proposer cannot approve a candidate` -> I5, V2
- `Scenario: Recorded-only evidence cannot satisfy property evidence` -> I6, V2
- `Scenario: Unresolved blocker prevents approval` -> I7, V2
- `Scenario: Review-basis drift makes earlier votes inapplicable` -> I8, V2
- `Scenario: Conflicting terminal decisions remain visible` -> I9, V2
- `Scenario: Portable and direct backends preserve semantic identity` -> I10, I11, V4
- `Scenario: Preview writes nothing` -> I12, V3
- `Scenario: Independent consumer reads release-decision history` -> I14, V4
- `Scenario: Malformed history fails closed` -> V5
- `Scenario: Approved decision does not execute a release` -> I13, V6
