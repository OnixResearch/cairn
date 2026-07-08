# Change: Stack provenance release gates

## Why

Cairn is the lifecycle and release-readiness gate. Once Valence verifies the
stack provenance sidecar and Mantle bundles it as external evidence, Cairn should
be able to require that verified chain before reporting a release-ready stack
state. The gate must stay bounded: Valence owns provenance semantics, Mantle owns
bundle-local evidence, and Cairn owns lifecycle policy conformance.

## What Changes

- Add a Cairn policy path that can require Valence stack-provenance verification
  receipts in release-readiness and evidence bundle verification.
- Verify receipt freshness, policy hash compatibility, artifact hashes, Mantle
  bundle linkage, and bounded claim scope before treating stack provenance as
  present.
- Surface a compact provenance graph summary from Valence reports in Cairn
  release-readiness output without reimplementing Valence semantics.
- Add positive and negative fixtures for complete chains, missing receipts,
  stale policy, digest mismatch, unsupported claim scope, manual-review evidence
  counted as verified, and weakened non-claims.

## Impact

- **Files**: Cairn policy, pure validation core, release-readiness/evidence
  bundle verification shell, fixtures, docs, and tests.
- **Consumers**: Release operators get an explicit readiness result for whether
  stack provenance was Valence-verified and Mantle-bundled.
- **Non-claims**: Cairn gates lifecycle/release policy conformance. It does not
  prove source-code behavioral correctness, Valence verifier soundness, Mantle
  build correctness, or Trellis/Octet proof validity beyond the cited receipts.
