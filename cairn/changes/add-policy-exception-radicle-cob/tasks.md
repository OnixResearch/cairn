# Tasks: Add policy-exception Radicle COB collaboration

## Phase 1: Profile and request binding

- [ ] [serial] I1 Inventory policy identities, rule IDs, scope grammars, rationale fields, compensating controls, validity bounds, roles, transitions, and compatibility obligations. r[cairn.policy_exception_cob.profile]
- [ ] [serial] I2 Add typed Nickel policy-exception profiles with named scope, role, approval, validity, observation-source, transition, and protocol bounds. r[cairn.policy_exception_cob.policy]
- [ ] [serial] I3 Add the `xyz.onixresearch.policy-exception` schema, stable exception identity, strict type admission, and exact request-root binding. r[cairn.policy_exception_cob.profile]
- [ ] [serial] I4 Add canonical typed scope validation that rejects implicit wildcards, unsafe selectors, unknown rule IDs, and policy identity mismatch. r[cairn.policy_exception_cob.scope]
- [ ] [parallel] V1 Add positive and negative profile tests for exact scope, malformed policy references, wildcard ambiguity, unsafe selectors, invalid bounds, mixed kinds, and deterministic identities. r[cairn.policy_exception_cob.validation]

## Phase 2: Authority, decisions, and validity

- [ ] [serial] I5 Add immutable requester, reviewer, approver, revoker, and validity-observer role sets with action-specific authorization. r[cairn.policy_exception_cob.authority]
- [ ] [serial] I6 Add request-root revisions that preserve history and make prior review and approval facts inapplicable. r[cairn.policy_exception_cob.approval]
- [ ] [serial] I7 Add review findings, approval thresholds, rejection, decision conflicts, and complete-basis resolution. r[cairn.policy_exception_cob.approval]
- [ ] [serial] I8 Add pure finite-bound validation plus explicit typed time-observation admission without ambient clock access. r[cairn.policy_exception_cob.validity]
- [ ] [serial] I9 Add applicability, revocation, expiration, closure, and supersession classification over exact current facts. r[cairn.policy_exception_cob.applicability]
- [ ] [parallel] V2 Add unauthorized transition, stale approval, policy drift, scope drift, missing observation, expired bound, revocation, conflict, and ambient-authority negative tests. r[cairn.policy_exception_cob.validation]

## Phase 3: Backends and shell

- [ ] [serial] I10 Add policy-exception portable-envelope dispatch with exact BLAKE3 semantic identity round trips. r[cairn.policy_exception_cob.backends]
- [ ] [serial] I11 Add `xyz.onixresearch.policy-exception` direct-COB create, load, update, conflict, resolution, and typed dispatch through the shared adapter. r[cairn.policy_exception_cob.backends]
- [ ] [serial] I12 Add preview-first request, status, update, review, approve, reject, observe-validity, revoke, expire, close, conflict, resolve, supersede, and export commands. r[cairn.policy_exception_cob.cli]
- [ ] [parallel] V3 Add shell tests for policy snapshot drift, observation drift, locks, symlinks, unsafe paths, stale plans, signer drift, repository drift, and atomic writes. r[cairn.policy_exception_cob.cli]

## Phase 4: Receipts, compatibility, and closeout

- [ ] [serial] I13 Add deterministic receipts and documentation for identities, scope, controls, roles, decisions, observations, applicability, conflicts, backends, and non-claims. r[cairn.policy_exception_cob.receipts]
- [ ] [serial] I14 Add provenance-bound static policy-exception COB fixture data and a separate direct read-only consumer. r[cairn.policy_exception_cob.interop]
- [ ] [parallel] V4 Validate request, revise, review, approve, classify, revoke, expire, close, supersede, portable round trip, and direct-consumer round trip. r[cairn.policy_exception_cob.interop]
- [ ] [parallel] V5 Reject malformed manifests, unsigned history, mixed types, stale policy, stale approval, invalid observations, unauthorized transitions, and tampered provenance. r[cairn.policy_exception_cob.interop]
- [ ] [serial] V6 Run dependency gates, policy freshness, formatting, workspace tests, Clippy, core purity, Tiger Style, traceability, Cairn validation, all change gates, release readiness, COB compatibility, self-dogfood, and the supported-system flake check. r[cairn.policy_exception_cob.validation]

## Verification Coverage

- `Scenario: Policy-exception type rejects another workflow kind` -> I3, V1
- `Scenario: Implicit wildcard scope fails closed` -> I4, V1
- `Scenario: Requester cannot approve an exception` -> I5, V2
- `Scenario: Request revision makes prior approval inapplicable` -> I6, V2
- `Scenario: Conflicting decisions block applicability` -> I7, V2
- `Scenario: Pure reduction does not read a clock` -> I8, V2
- `Scenario: Conflicting validity observations block applicability` -> I8, V2
- `Scenario: Expiration cannot be reversed by an older observation` -> I8, I9, V2
- `Scenario: Expired or revoked exception is inapplicable` -> I9, V2
- `Scenario: Portable and direct backends preserve semantic identity` -> I10, I11, V4
- `Scenario: Preview writes nothing` -> I12, V3
- `Scenario: Independent consumer reads exception history` -> I14, V4
- `Scenario: Malformed history fails closed` -> V5
- `Scenario: Approved state does not grant authority` -> I13, V6
