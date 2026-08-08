# Validation: Add RFD Radicle COB collaboration

Date: 2026-08-04

## Baseline

The change branch started at `4b6e95090b49b547a3e1f94d0c96b2dbefd4234b`.

That commit had no RFD or document-collaboration surface. This change adds a transport-neutral document core, an `xyz.onixresearch.rfd` protocol, a portable backend, and a direct Radicle COB adapter in the `onix-cairn-radicle-cob` crate, while leaving the existing Cairn-change COB paths byte-stable.

## Implemented surface

The pure core models neutral document state with deterministic whole-artifact multi-head conflicts, complete-basis resolution, immutable role authority, set-once publication numbering, BLAKE3 materialization, and a tamper-rejecting portable envelope.

The CLI exposes preview-first `rfd import/status/checkout/update/conflict/resolve/number/publish` over the portable backend with capability-root reads and bounded writes.

The direct COB adapter maps signed `xyz.onixresearch.rfd` entries to the pure reducer. The existing `xyz.onixresearch.cairn-change` adapter, refs, commits, and bytes are unchanged.

## Policy identities

Field-set digest: `9f44153637df26127588d81e70bb2095f07f64c4e7d5c009608f6c70b6ca961a`.

Policy hash: `1b7242389219cf32ac7d776f68cbcbec966da3a3e0b6e710b0d23b2fbf8a39c9`.

Policy-bound agent fixtures, replay receipt hash, and lifecycle goldens were refreshed together.

## Focused verification

- 7 document-collaboration core tests: neutral state, conflicts, resolution, atomic rejection, role authority, portable round trip + tamper, numbering identity.
- 2 RFD CLI tests: preview-first import/status/update/publish and numbering plan.
- Direct-COB crate tests: RFD create/update/inspect round trip, existing change-COB round trip (compatibility), unauthorized/symlink/concurrent-lock negatives, and direct primitive dependency audit.

## Commands and results

- `cargo fmt --all -- --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `nix run path:$PWD#cairn -- policy export --check`: passed.
- `nix run path:$PWD#cairn -- validate --root $PWD`: `valid: true`.
- Proposal, design, and tasks gates: `verdict: PASS`.
- Traceability: `1999/1999`, no missing or dangling references.
- MCP smoke: `verdict: pass`.
- Release readiness: `valid: true`, `verdict: pass`.
- Slotcar: passed.
- `nix flake check path:$PWD -L`: passed.

## Claim boundary

Passing RFD validation proves only the recorded neutral state, selected local backend observations, configured role checks, deterministic reduction, and a bounded materialization plan. It does not prove publication, synchronization, global number uniqueness, remote trust, semantic review quality, link truth, source correctness, or release eligibility.

## Lifecycle evidence

Archive execution receipt: `738ef0a02e9a5180c5de0ccf4da62e3164958e63d1dbfb46e3c451094f2c4fcb`.

Post-archive validation, traceability, and release readiness passed.

The accepted specifications are `cairn/specs/document-collaboration/spec.md` and `cairn/specs/rfd-radicle-cob/spec.md`.
