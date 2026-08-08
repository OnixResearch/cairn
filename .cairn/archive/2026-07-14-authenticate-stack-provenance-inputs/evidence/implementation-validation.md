# Implementation validation

Date: 2026-07-14

## Implemented boundary

Cairn now measures explicitly supplied readiness artifacts in the CLI shell, recomputes primary receipt identity after excluding only the root `receipt_hash` self-hash field, measures reviewed Nickel policy exports and the pinned Valence corrected-profile archive receipt, and verifies domain-separated Ed25519 signatures against explicit producer/key/purpose policy. Pure core evaluation consumes typed observations and fails closed on measurement, canonicalization, policy, signer, purpose, subject, parent-link, role, schema, and release-profile mismatches.

The `onix-release` authentication profile requires both ordinary evidence modes to be `required` and both authenticated reports to pass. Diagnostic mode remains available but always emits `onix_release_ready: false`.

This proves measured-byte linkage, configured producer authorization, and lifecycle-policy conformance only. It does not prove source correctness, verifier soundness, build correctness, semantic equivalence, runtime authority, or universal release fitness.

## Reviewed identities

- Stack-provenance Nickel export BLAKE3: `93993e99e87206d232b64efcc8631611aad3b5b0c4b81855e7e1a0cdcc391cff`
- Function-address Nickel export BLAKE3: `62c88a0263720dea4c37a48ef46921c46e5dd3e4924fff096b84a65d5738a412`
- Pinned Valence corrected-profile archive receipt BLAKE3: `a1c63619b0f72140c6e350b61ed18d3d552310067406ba0b819ace462d1f1e48`
- Valence archive manifest BLAKE3: `62e11014c3b029ac269e6c2e04da9c020c350dd093e9af207f6c060faae1a33e`
- Valence provider revision: `1f4f6fe903a34c22d2cda53cd534ce58bbf84147`
- Final generated Cairn policy hash: `10d913848c62452412777811f7013ff33e54afaed56fac5a70ee174c67e95ab4`

## Focused rails

- `cargo test -p cairn-core authenticated_inputs`: 3 passed.
- `cargo test -p cairn-cli --bin cairn shell_verifies`: 1 passed; actual Ed25519 verification accepted matching measured bytes and rejected post-signature tampering.
- `cargo test -p cairn-cli --test release_receipts`: 27 passed, including the required-profile missing-authentication probe.
- `cargo test -p cairn-core`: 298 passed.
- `cargo test --workspace`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cairn policy export --check`: passed.
- `cairn traceability coverage --root . --json`: passed with no missing or dangling requirement IDs.
- `cairn validate --root .`: passed.
- Proposal, design, and tasks gates for `authenticate-stack-provenance-inputs`: passed.

Negative coverage includes fabricated measured digests, post-signature byte tampering, stale receipt self-hashes, unsupported canonicalization profiles, stale and placeholder policy identities, malformed or missing signatures, wrong purposes, unauthorized and revoked keys, unlinked parent injection, missing required gates, and diagnostic-mode non-release behavior.

## Nix and CI rail

The checked-in workflow `.github/workflows/authenticated-evidence-flake-check.yml` has one verification command: `nix flake check`.

A local exact `nix flake check` completed builds but could not publish its result because the host configuration names a missing signing key:

```text
error: opening file "/run/secrets/vars/nix-signing-key/key": No such file or directory
```

The bounded local-equivalent command disabled only unavailable host signing and remote builders:

```text
nix flake check --option secret-key-files '' --option builders ''
```

It passed with `all checks passed!`, including policy freshness, authenticated-profile export freshness, stale-policy detection, Rust tests, clippy, rustfmt, Cairn validation, traceability coverage, package checks, and self-dogfood checks.
