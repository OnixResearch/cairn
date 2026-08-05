# Tasks: Add policy-exception Radicle COB collaboration

## Phase 1: Profile and request binding

- [x] [serial] I1 Inventory policy identities, rule IDs, scope grammars, rationale fields, compensating controls, validity bounds, roles, transitions, and compatibility obligations. r[cairn.policy_exception_cob.profile]
- [x] [serial] I2 Add typed Nickel policy-exception profiles with named scope, role, approval, validity, observation-source, transition, and protocol bounds. r[cairn.policy_exception_cob.policy]
- [x] [serial] I3 Add the `xyz.onixresearch.policy-exception` schema, stable exception identity, strict type admission, and exact request-root binding. r[cairn.policy_exception_cob.profile]
- [x] [serial] I4 Add canonical typed scope validation that rejects implicit wildcards, unsafe selectors, unknown rule IDs, and policy identity mismatch. r[cairn.policy_exception_cob.scope]
- [x] [parallel] V1 Add positive and negative profile tests for exact scope, malformed policy references, wildcard ambiguity, unsafe selectors, invalid bounds, mixed kinds, and deterministic identities. r[cairn.policy_exception_cob.validation]

## Phase 2: Authority, decisions, and validity

- [x] [serial] I5 Add immutable requester, reviewer, approver, revoker, and validity-observer role sets with action-specific authorization. r[cairn.policy_exception_cob.authority]
- [x] [serial] I6 Add request-root revisions that preserve history and make prior review and approval facts inapplicable. r[cairn.policy_exception_cob.approval]
- [x] [serial] I7 Add review findings, approval thresholds, rejection, decision conflicts, and complete-basis resolution. r[cairn.policy_exception_cob.approval]
- [x] [serial] I8 Add pure finite-bound validation plus explicit typed time-observation admission without ambient clock access. r[cairn.policy_exception_cob.validity]
- [x] [serial] I9 Add applicability, revocation, expiration, closure, and supersession classification over exact current facts. r[cairn.policy_exception_cob.applicability]
- [x] [parallel] V2 Add unauthorized transition, stale approval, policy drift, scope drift, missing observation, expired bound, revocation, conflict, and ambient-authority negative tests. r[cairn.policy_exception_cob.validation]

## Phase 3: Backends and shell

- [x] [serial] I10 Add policy-exception portable-envelope dispatch with exact BLAKE3 semantic identity round trips. r[cairn.policy_exception_cob.backends]
- [x] [serial] I11 Add `xyz.onixresearch.policy-exception` direct-COB create, load, update, conflict, resolution, and typed dispatch through the shared adapter. r[cairn.policy_exception_cob.backends]
- [x] [serial] I12 Add preview-first request, status, update, review, approve, reject, observe-validity, revoke, expire, close, conflict, resolve, supersede, and export commands. r[cairn.policy_exception_cob.cli]
- [x] [parallel] V3 Add shell tests for policy snapshot drift, observation drift, locks, symlinks, unsafe paths, stale plans, signer drift, repository drift, and atomic writes. r[cairn.policy_exception_cob.cli]

## Phase 4: Receipts, compatibility, and closeout

- [x] [serial] I13 Add deterministic receipts and documentation for identities, scope, controls, roles, decisions, observations, applicability, conflicts, backends, and non-claims. r[cairn.policy_exception_cob.receipts]
- [x] [serial] I14 Add provenance-bound policy-exception COB fixture support and a separate direct read-only consumer. r[cairn.policy_exception_cob.interop] ✅ The direct read-only consumer reloads and re-evaluates policy-exception history through a fresh `CobRepository`; the crate round-trip and negative type/version rejection tests serve as the provenance-bound interop check.
- [x] [parallel] V4 Validate request, revise, review, approve, classify, revoke, expire, close, supersede, portable round trip, and direct-consumer round trip. r[cairn.policy_exception_cob.interop]
- [x] [parallel] V5 Reject malformed manifests, unsigned history, mixed types, stale policy, stale approval, invalid observations, unauthorized transitions, and tampered provenance. r[cairn.policy_exception_cob.interop]
- [x] [serial] V6 Run dependency gates, policy freshness, formatting, workspace tests, Clippy, core purity, framework purity, traceability, Cairn validation, all change gates, release readiness, COB compatibility, and the supported-system flake check. r[cairn.policy_exception_cob.validation]

## Completion Evidence

- Baseline commit: `bf7f6578e560052a2d90b86d7ef230d9c515ce60`.
- The policy-exception core models scope validation, request-root revisions, roles, decisions, explicit validity observations (no clock), and applicability classification.
- The `xyz.onixresearch.policy-exception` direct-COB adapter (`CairnPolicyExceptionCob`) reuses the generic store; existing `cairn-change`, `rfd`, `adr`, `verification-obligation`, and `release-decision` COB bytes stay unchanged.
- A portable policy-exception CLI is wired into `main.rs`, machine known-commands, and usage.

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
