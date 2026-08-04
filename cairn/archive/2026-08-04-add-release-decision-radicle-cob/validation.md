# Validation: Add release-decision Radicle COB collaboration

Date: 2026-08-04

## Baseline

The change branch started at `043f1d28ba623aea33940bd9f9e251ffb650d8d2`.

That commit added the verification-obligation workflow surface. This change adds a dedicated release-decision core (candidate binding, evidence roles, blockers, review-basis votes, verdicts, withdrawal/invalidation/supersession), a direct-COB adapter, and a portable CLI for the `xyz.onixresearch.release-decision` profile.

## Implemented surface

The pure release-decision core binds one immutable candidate descriptor, keeps candidate, decision, provider, operation, evidence, and artifact identities separate, models evidence roles and validation summaries, blockers with append-only dispositions, a BLAKE3 review-basis root, votes, and policy-named verdicts with explicit decision conflicts. An approved decision does not execute a release.

The portable CLI exposes create, status, evidence, finding, vote, withdraw, invalidate, supersede, conflict, and export, all preview-first.

The direct-COB adapter (`CairnReleaseDecisionCob`) reuses the generic store. Existing `cairn-change`, `rfd`, `adr`, and `verification-obligation` COB refs, commits, and bytes stay unchanged.

## Policy identities

The generated policy is unchanged by this change. Field-set digest: `9f44153637df26127588d81e70bb2095f07f64c4e7d5c009608f6c70b6ca961a`. Policy hash: `587d6dabf9e73c050ff15562797eef98cc2b348fc2383563565e7583803d44ed`.

## Focused verification

- 8 release-decision core tests (valid root and identities, candidate substitution rejection, proposer-cannot-approve, evidence role preservation, unresolved blocker, approval, withdrawal, portable round trip).
- 1 release-decision CLI test (create/status/approve/conflict).
- Existing `cairn-change`, `rfd`, `adr`, and `verification-obligation` COB crate tests still pass (compatibility preserved).

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

An approved release-decision receipt does not execute a release, move a tag, or deploy. It proves only the recorded neutral state, configured role checks, evidence validation summaries, review-basis and vote identities, deterministic reduction, and a bounded lifecycle fact. It does not prove artifact publication, synchronization, remote trust, source correctness, or release eligibility.

## Lifecycle evidence

Archive execution receipt: `20c3052d97ae2e3680f071acc7d09fcb61fe8b0d1e5d8797bec34e12e2cc33f0`.

Post-archive validation, traceability, and release readiness passed.

The accepted specification is `cairn/specs/release-decision-radicle-cob/spec.md`.
