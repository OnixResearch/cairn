## Why

Mantle's reviewed-source release policy needs a stable Cairn producer contract
before it can consume detached review evidence. Existing semantic-review records
do not specify the exact signed bytes, external key policy, source projection,
or role-separated attachment format required by that consumer.

## What Changes

- Add `cairn_core::source_review`: closed, bounded source, claim, policy,
  signing-request and attachment DTOs; domain-separated BLAKE3 logical identities;
  Artifact Auth Ed25519 verification over exact canonical statement bytes.
- Add read-only `cairn source-review signing-request`, `assemble`, and `verify`.
  The operator owns private keys, signatures, trusted-key/currentness inputs and
  the selected claim root. Cairn never generates review keys or signs approvals.
- Count distinct full public-key identities, bind the complete source projection,
  reject stale, revoked, excluded, substituted and malformed evidence, and keep
  authenticated `needs_revision` attribution separate from approval.
- Document the exact producer/consumer boundary and retain positive and negative
  behavioral regressions. Deterministic test signatures are not real approval.

## Impact

- **Current consumer:** Mantle's `crunch-release-core` reviewed-source verifier;
  adoption uses an immutable Cairn source revision and independent verification.
- **Maintenance owner:** Cairn owns the producer DTOs, canonical bytes, CLI and
  compatibility contract; Mantle retains release and build-witness authority.
- **Files:** `crates/cairn-core/src/verified/source_review*`, the core re-export,
  core dependency manifest, CLI dispatch/module/integration test, and README.
- **Testing:** focused producer and actual CLI round trip, complete workspace
  lib/tests, strict Clippy, package formatting, policy freshness, and native
  lifecycle validation/gates before sync/archive and publication.
- **Non-claims:** attribution and linkage do not prove review quality, source or
  build correctness, reproducibility, witness independence, deployment safety,
  or release eligibility. No private-key custody, trust discovery or clock input.

