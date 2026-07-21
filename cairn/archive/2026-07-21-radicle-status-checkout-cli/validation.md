# Validation: Radicle Status and Checkout CLI

## Result

The typed read-only status and preview-first checkout slice is validated on `x86_64-linux`. Cairn can load one unique local change COB without a signer, report reducer and local materialization condition, prove possession of a current delegate signer before mutation, serialize checkout, materialize exact bytes through a prepared directory, atomically expose an absent active change, and verify the resulting local view.

## Deterministic Evidence

- Baseline Radicle reducer, adapter, and CLI tests passed before core changes.
- Pure checkout planner tests passed for absent creation, exact no-op, divergent/unsupported destination rejection, content-conflict suppression, and lifecycle-conflict checkout blocking.
- Radicle adapter tests passed for typed read without signer acquisition, current delegate/non-delegate status, missing-object rejection, current identity/object/state drift, serialized checkout guards, and an actual explicit profile/keystore signer-possession flow. The adapter suite passed with 14 tests.
- CLI Radicle tests passed with 8 tests for explicit read-only defaults, exact bounded local scans, unsupported and symlink rejection, preview receipts, blocked/success receipts, exact prepared materialization, post-verification, and existing-destination refusal.
- Capability tests passed with bounded file reads, bounded directory entries, exclusive prepared-directory creation, traversal rejection, and symlink escape rejection.
- Full `cargo test --workspace` passed after implementation.
- Full `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --check` passed.
- Policy freshness, Cairn validation, proposal/design/tasks gates, and verified-core purity passed.
- Traceability coverage passed with `1627/1627` accepted requirement and scenario IDs referenced.
- `nix flake check path:$PWD -L` completed with `all checks passed` on `x86_64-linux`.

## Adversarial Findings

The first candidate allowed a conflict-free content tree to be checkout-ready even when lifecycle revisions had concurrent heads. Status exposed that lifecycle conflict, but checkout did not block. The pure planner now treats lifecycle-head conflict as a checkout blocker while continuing to expose content descriptors in read-only status. A negative reducer fixture proves the corrected behavior.

A second audit hardened filesystem authority from profile-public-key membership to actual signer possession. Checkout execution now loads the matching signer, signs and verifies a domain-separated challenge bound to repository ID, object ID, current canonical identity OID, and typed state hash, then acquires and rechecks the serialized guard. Read-only status still does not load a signer.

The configured advisory model endpoint reported reachable but both review requests failed during fetch. Its output was not used as evidence. Internal mechanism-based audit plus deterministic repository tests remained authoritative.

## Bounded Claims and Remaining Environment Gap

The passing rails establish local typed reduction, deterministic planning, point-in-time current authority and signer-possession observation, cooperative serialization, exact namespace-visible materialization, and immediate post-write verification. They do not establish fsync-level crash durability, resistance to unrelated processes with equivalent host filesystem authority, future delegate authority, or external network behavior.

No live Radicle node or multi-peer transport was exercised. Status and checkout do not prove network publication, peer availability, synchronization, remote trust, gate validity, evidence truth, dependency readiness, source correctness, archive execution, or release eligibility. Stale locks or prepared directories fail closed and require deliberate operator recovery.
