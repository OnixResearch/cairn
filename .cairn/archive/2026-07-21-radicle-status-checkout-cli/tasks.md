# Tasks: Radicle Status and Checkout CLI

## Phase 1: Pure Projection

- [x] [serial] Add deterministic status/checkout planning over typed COB state and exact local snapshots, including positive exact/absent cases and negative conflict/divergence cases. r[cairn.radicle_status_checkout.checkout_plan]

## Phase 2: Radicle Read and Authority Shell

- [x] [serial] Add unique typed status loading without signer acquisition and a serialized current-canonical-authority checkout guard with positive and negative adapter tests. r[cairn.radicle_status_checkout.status]
- [x] [serial] Recheck actor, canonical identity, object, and state immediately before filesystem mutation, rejecting drift and retired delegates. r[cairn.radicle_status_checkout.checkout_authority]

## Phase 3: CLI and Materialization

- [x] [serial] Add read-only `cairn radicle status` and deterministic BLAKE3-bound status receipts with explicit context and bounded claims. r[cairn.radicle_status_checkout.receipts]
- [x] [serial] Add preview-first `cairn radicle checkout`, capability-bounded prepared writes, atomic exposure, post-verification, and positive/negative CLI tests. r[cairn.radicle_status_checkout.checkout_execution]

## Phase 4: Documentation and Verification

- [x] [serial] Document operator flow and non-claims; run focused tests, workspace tests, Clippy, formatting, purity, lifecycle gates, traceability, Nix validation, and archive the completed change. r[cairn.radicle_status_checkout.non_claims]

## Verification Evidence

- Baseline and post-change Radicle reducer, adapter, CLI, capability, and full workspace tests passed with positive and negative cases.
- Real explicit-profile storage and keystore signing exercised typed status and the state-bound signer-possession checkout guard.
- Formatting, full workspace Clippy, policy freshness, lifecycle validation/gates, and verified-core purity passed.
- Traceability reported `1627/1627` referenced.
- `nix flake check path:$PWD -L` completed with `all checks passed` on `x86_64-linux`.
- Adversarial audit found and fixed lifecycle-conflict checkout readiness; live multi-peer transport remains an explicit non-claim.
