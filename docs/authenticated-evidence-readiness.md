# Authenticated evidence readiness

Cairn can authenticate stack-provenance and function-address inputs before they contribute to release readiness. The CLI shell reads the explicitly supplied evidence paths, measures every artifact with BLAKE3, recomputes the primary receipt identity, measures the reviewed Nickel policy export and pinned Valence integrity receipt, and verifies an Ed25519 signature. The pure core receives only typed observations and evaluates policy, role, identity, linkage, signature-purpose, and release-profile rules.

r[cairn.stack_provenance_gates.authenticated_inputs.docs] **Claim boundary:** a passing result proves measured-byte linkage, configured producer authorization, and lifecycle-policy conformance only. It does not prove source correctness, verifier soundness, build correctness, semantic equivalence, runtime authority, or universal release fitness.

## Policy sources

The reviewed policy profiles are authored in:

- `cairn-policy/stack-provenance-authentication-profile.ncl`
- `cairn-policy/function-address-authentication-profile.ncl`

Their deterministic JSON exports under `cairn-policy/generated/` are measured at runtime. `cairn-policy/default.ncl` pins those export identities and the accepted Valence integrity receipt. Placeholder or stale identities fail closed.

r[cairn.stack_provenance_gates.authenticated_inputs.cross_repo_dependency] The accepted corrected Preserves integrity dependency is recorded at `cairn-policy/evidence/valence-preserves-integrity-archive-receipt.json`. It binds Valence revision `1f4f6fe903a34c22d2cda53cd534ce58bbf84147` and the archived `2026-07-12-harden-preserves-integrity-boundaries` package. This is an identity and profile dependency, not a transfer of Valence correctness claims.

The default profile is `diagnostic` and has no trusted producer keys. An operator must explicitly configure reviewed producer keys and set `release_profile = "onix-release"` before Cairn can report authenticated Onix release readiness. The top-level `onix_release_ready` field remains `false` in diagnostic mode even when other local readiness checks pass. The Onix profile requires both the existing stack-provenance and function-address modes to be `required`, both ordinary gates to pass, and both authentication manifests to pass.

## Authentication manifest

Pass sidecars with:

```text
cairn release-readiness \
  --root . \
  --stack-provenance receipt.json \
  --stack-provenance-graph graph.json \
  --stack-provenance-authentication stack-authentication.json \
  --function-address-valence valence.json \
  --function-address-mantle mantle.json \
  --function-address-kamacite kamacite.json \
  --function-address-graph function-graph.json \
  --function-address-authentication function-authentication.json
```

Each authentication manifest uses schema `cairn.authenticated-evidence.v1` and declares:

- `evidence_kind`;
- accepted `canonicalization_profile`;
- the primary receipt's asserted `supplied_receipt_hash`;
- `{role,path,blake3}` entries for every explicit evidence path;
- the declared reviewed policy-export identity;
- required parent identities;
- the pinned Valence integrity receipt identity; and
- Ed25519 `algorithm`, `producer`, `key_id`, `purpose`, and `signature_hex` fields.

Cairn removes only the root `receipt_hash` self-hash field before recomputing canonical receipt identity. The signature message is canonical JSON under the `cairn.authenticated-evidence.v1` domain and binds the evidence kind, sorted measurements, measured policy export, producer, key ID, purpose, canonicalization profile, recomputed subject identity, sorted parent identities, and Valence integrity receipt identity. Signature verification uses only the public key selected by the configured producer/key pair.

Missing files, mismatched byte identities, unsupported profiles, stale policy exports, missing or revoked keys, wrong purposes, changed subjects, changed parents, and malformed signatures all fail closed. Diagnostic mode may omit manifests, but it never emits authenticated Onix release-ready status.
