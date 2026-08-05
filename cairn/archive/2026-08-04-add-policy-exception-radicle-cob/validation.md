# Validation: Add policy-exception Radicle COB collaboration

Date: 2026-08-04

## Baseline

The change branch started at `bf7f6578e560052a2d90b86d7ef230d9c515ce60`.

That commit added the release-decision collaboration surface. This change adds a dedicated policy-exception core (scope validation, request-root revisions, roles, decisions, explicit validity observations with no clock, applicability), a direct-COB adapter, and a portable CLI for the `xyz.onixresearch.policy-exception` profile.

## Implemented surface

The pure policy-exception core validates an explicit typed scope (known rule IDs, safe selectors, no implicit wildcards), binds an exact request root, revokes/preserves history on request revision, models requester/reviewer/approver/revoker/observer roles, and admits validity as explicit finite-bound observations without ambient clock access. Applicability classifies revoked/expired/approved and split-state outcomes.

The portable CLI exposes request, status, approve, reject, observe-validity, revoke, expire, close, supersede, conflict, and export, all preview-first.

The direct-COB adapter (`CairnPolicyExceptionCob`) reuses the generic store. Existing `cairn-change`, `rfd`, `adr`, `verification-obligation`, and `release-decision` COB refs, commits, and bytes stay unchanged.

## Policy identities

The generated policy is unchanged by this change. Field-set digest: `9f44153637df26127588d81e70bb2095f07f64c4e7d5c009608f6c70b6ca961a`. Policy hash: `587d6dabf9e73c050ff15562797eef98cc2b348fc2383563565e7583803d44ed`.

## Focused verification

- 8 policy-exception core tests (valid request and identity, wildcard rejection, unknown-rule rejection, requester-cannot-approve, approval threshold, explicit observation, expiry irreversibility, portable round trip).
- 1 policy-exception CLI test (request/status/observe-validity/applicability).
- Existing `cairn-change`, `rfd`, `adr`, `verification-obligation`, and `release-decision` COB crate tests still pass (compatibility preserved).

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

An approved policy-exception receipt does not grant authority. It proves only the recorded request, scope, identity on controls, configured role checks, decisions, explicit validity observations, and deterministic applicability classification. It does not prove policy truth, enforcement, real-world access, or release eligibility.

## Lifecycle evidence

Archive execution receipt: `5424b4e4d70c5507e1d03904c193b3bfa80e4310e535c2d42e09c247b08e0074`.

Post-archive validation, traceability, and release readiness passed.

The accepted specification is `cairn/specs/policy-exception-radicle-cob/spec.md`.
