## Why

Cairn's stack-provenance and function-address readiness paths currently accept caller-supplied JSON identities and compare digest-shaped strings without consistently recomputing those identities from referenced artifact bytes. Fabricated but lexically valid hashes can therefore satisfy fixtures and readiness checks. Optional release modes and placeholder accepted-policy hashes compound the problem by allowing incomplete evidence to appear release-adjacent.

Cairn must authenticate the evidence it counts: measure the supplied bytes, bind them to typed roles and policy identities, verify required producer authorization, and make the Onix release profile fail closed.

## What Changes

- Measure referenced artifact, sidecar, and receipt bytes in the CLI shell and pass typed measured identities into pure readiness cores.
- Recompute canonical receipt identities rather than trusting supplied receipt-hash fields.
- Require trusted-producer signature verification for evidence counted by the Onix release profile.
- Replace placeholder accepted-policy hashes with digests derived from reviewed Nickel policy exports.
- Make stack provenance and function-address gates mandatory for the Onix release profile while preserving explicitly non-release optional modes.
- Add fabricated-hash, tampered-byte, wrong-role, stale-policy, unauthorized-signer, and missing-signature fixtures.
- Add a checked-in CI workflow whose verification scope is `nix flake check`.

## Impact

- **Files**: release-readiness policy, stack-provenance and function-address gates, CLI evidence loading, signature adapters, fixtures, docs, Nix checks, and CI workflow.
- **Dependencies**: this package waits for local evidence-chain and Preserves ingest/readiness changes, and consumes the archived Valence corrected-profile receipt before implementation evidence is accepted.
- **Claims**: a passing gate proves measured-byte linkage, trusted-producer authorization, and configured lifecycle-policy conformance only; it does not prove source correctness, verifier soundness, build correctness, or universal release fitness.
