# Validation: Developer-loop profiles

Date: 2026-08-04

## Baseline

The change branch started at `4b6e8bd9c3195d2ab4dc3d1aca767a10107cc6c2`.

That commit had no `developer-loop` command family. Its baseline policy hash was `794e65e05f3ffe7a872581e7d2cddae45bcea5a6d38cd9a8892a2079050679f2`, and baseline release readiness passed.

This change adds the developer-loop profile surface and a new policy field, which changes the generated policy identity and every check that binds it.

## Implemented surface

The checked policy now owns developer-loop budgets, checks, and one `cairn-local` profile.

The pure core admits profiles, derives read-only plans, builds BLAKE3 cache keys, compares named budgets, classifies effect posture, validates receipts, and orders stable diagnostics.

The CLI exposes read-only `developer-loop list`, `plan`, and `verify`. None of these commands execute the selected checks.

## Policy identities

The generated policy field-set digest is `18dd2f541b4e9f737a8a54e91b8cb3f7206a0593d112e9ba1b8a7b97d3a2a09a`.

The generated policy hash is `0d5cb1655724e88eba5e81e493cdc7b90b210233965dbb785028848b6d0d48fe`.

Policy-bound fixtures were refreshed together: agent completion contract and results, the positive agent receipt, the replay receipt hash, and the lifecycle goldens.

## Focused verification

The focused core suite covers positive and negative profile, plan, budget, cache, posture, diagnostic, and stale-evidence behavior.

The focused CLI suite covers list, plan, and verify for a current and a stale receipt, and proves that no check is executed.

The generated policy admission test confirms the `developer-loop-policy.v1` schema and the `cairn-local` profile.

## Commands and results

- `cargo fmt --all -- --check`: passed.
- `cargo test --workspace`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `nix run path:$PWD#cairn -- policy export --check`: passed.
- `nix run path:$PWD#cairn -- validate --root $PWD`: `valid: true`.
- Proposal gate: `valid: true`, `verdict: PASS`.
- Design gate: `valid: true`, `verdict: PASS`.
- Tasks gate: `valid: true`, `verdict: PASS`.
- Traceability: `1953/1953`, with no missing or dangling references.

## Lifecycle evidence

Sync execution receipt: `18e915df8973ef0447b6fface664b77e0d551035e88c308d026a7e19a278acd0` (plan receipt).

Archive execution receipt: `48faaf9f2c95a53853315bbb67830f8055508066127b4d5a7fae33672695e03a`.

Post-archive validation, traceability, and release readiness passed.

The accepted specification is `cairn/specs/developer-loop-profiles/spec.md`.

## Claim boundary

A passing developer-loop receipt proves profile conformance, current supplied identities, selected check statuses, budget comparison, cache-key shape, and diagnostic safety. It does not prove source correctness, complete test coverage, command hermeticity, network denial, mutation absence, clock accuracy, future performance, proof soundness, full-gate success, or release eligibility.

Roc supplied design inspiration. Cairn does not copy Roc's compiler architecture or promise one universal latency target.
