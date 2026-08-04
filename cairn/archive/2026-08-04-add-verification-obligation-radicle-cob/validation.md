# Validation: Add verification-obligation Radicle COB collaboration

Date: 2026-08-04

## Baseline

The change branch started at `c368e6e7c077964be8c6f2f91d1fc37af8d6a774`.

That commit added the ADR document collaboration surface. This change adds a shared transport-neutral workflow core and the `xyz.onixresearch.verification-obligation` profile (claim roots, submitter/contributor/verifier/invalidator roles, findings, evidence roles, discharge/invalidation), a direct-COB adapter, and a portable CLI.

## Implemented surface

The pure workflow core models claim-root revisions, immutable role sets, fields, findings with blocker classes and dispositions, evidence references with verification-role and claim-scope preservation, and conflict-free discharge/rejection/invalidation. Post-discharge field edits are rejected as immutable.

The portable CLI exposes import, status, update, evidence, finding, response, discharge, reject, invalidate, conflict, resolve, and export, all preview-first.

The direct-COB adapter (`CairnVerificationObligationCob`) reuses the generic store. Existing `cairn-change`, `rfd`, and `adr` COB refs, commits, and bytes stay unchanged.

## Policy identities

The generated policy is unchanged by this change. Field-set digest: `9f44153637df26127588d81e70bb2095f07f64c4e7d5c009608f6c70b6ca961a`. Policy hash: `587d6dabf9e73c050ff15562797eef98cc2b348fc2383563565e7583803d44ed`.

## Focused verification

- 8 workflow-collaboration core tests (neutral state, kind rejection, submitter-cannot-discharge, evidence-role promotion, unresolved-blocker blocker, conflict-free discharge + immutability, claim revision advance, concurrent conflict + resolution).
- 2 verification-obligation CLI tests (import/status/discharge-blocked and import/export round trip).
- Existing `cairn-change`, `rfd`, and `adr` COB crate tests still pass (compatibility preserved).

## Commands and results

- `cargo fmt --all -- --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `nix run path:$PWD#cairn -- policy export --check`: passed.
- Proposal, design, and tasks gates: `verdict: PASS`.
- Traceability: valid with no missing or dangling references.
- MCP smoke and release readiness: `valid: true`.
- Slotcar and `nix flake check path:$PWD -L`: passed.

## Claim boundary

A passing verification-obligation receipt proves only the recorded neutral state, selected local backend observations, configured role checks, deterministic reduction, and a bounded evidence/decision surface. It does not prove remote synchronization, evidence truth, proof soundness, semantic correctness, or release eligibility.

## Lifecycle evidence

Archive execution receipt: `1b4717492c42485f58df131868b0a303716ac7d91e45c8ed95d65cdca90badf0`.

Post-archive validation, traceability, and release readiness passed.

The accepted specifications are `cairn/specs/workflow-collaboration/spec.md` and `cairn/specs/verification-obligation-radicle-cob/spec.md`.
