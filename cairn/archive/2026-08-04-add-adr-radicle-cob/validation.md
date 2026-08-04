# Validation: Add ADR Radicle COB collaboration

Date: 2026-08-04

## Baseline

The change branch started at `f238379fef3c229b74dbbecabe07b43780eb8b9c`.

That commit added the RFD document collaboration surface. This change adds the ADR kind, an approver role with post-acceptance immutability, the `xyz.onixresearch.adr` direct-COB adapter, and a portable ADR CLI, building on the shared document core.

## Implemented surface

The shared core now models the ADR kind and lifecycle (`Proposed`, `Review`, `Accepted`, `Deprecated`, `Published`, `Superseded`), an immutable approver role, and decision immutability: an accepted decision freezes its source artifacts.

The portable ADR CLI exposes import, status, update, accept, deprecate, supersede, conflict, resolve, number, and publish, all preview-first.

The direct-COB adapter (`CairnAdrCob` in `onix-cairn-radicle-cob`) maps signed `xyz.onixresearch.adr` entries to the pure reducer. Existing `cairn-change` and `rfd` COB refs, commits, and bytes stay unchanged.

## Policy identities

Field-set digest: `9f44153637df26127588d81e70bb2095f07f64c4e7d5c009608f6c70b6ca961a`.

Policy hash: `587d6dabf9e73c050ff15562797eef98cc2b348fc2383563565e7583803d44ed`.

Policy-bound agent fixtures, replay receipt hash, and lifecycle goldens were refreshed together.

## Focused verification

- 9 document-collaboration core tests (7 RFD + 2 ADR), including approver-required acceptance, post-acceptance immutability rejection, and ADR kind/schema admission.
- 1 ADR CLI test: import → approve → status(accepted) → post-accept edit rejected as immutable.
- Direct-COB crate tests still pass for RFD and the existing Cairn-change COB (compatibility preserved).

## Commands and results

- `cargo fmt --all -- --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `nix run path:$PWD#cairn -- policy export --check`: passed.
- `nix run path:$PWD#cairn -- validate --root $PWD`: `valid: true`.
- Proposal, design, and tasks gates: `verdict: PASS`.
- Traceability: `2036/2036`, no missing or dangling references.
- MCP smoke: `verdict: pass`.
- Release readiness: `valid: true`, `verdict: pass`.
- Slotcar: passed.
- `nix flake check path:$PWD -L`: passed.

## Claim boundary

A passing ADR receipt proves only the recorded neutral state, selected local backend observations, configured role checks, deterministic reduction, and a bounded materialization plan. It does not prove remote publication, synchronization, global number uniqueness, semantic review quality, link truth, source correctness, or release eligibility.

## Lifecycle evidence

Archive execution receipt: `9d36977d7f6a1830dd41aac79383e77f11c2fed923fb6583e7c0cc5fbded7c81`.

Post-archive validation, traceability, and release readiness passed.

The accepted specification is `cairn/specs/adr-radicle-cob/spec.md`.
