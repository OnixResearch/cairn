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

## Standalone artifact-auth dual run

r[cairn.artifact_auth.repository_extraction.consumer_authority.cairn] Cairn consumes `artifact-auth-core` and `artifact-auth-ed25519` from `ssh://git@github.com/OnixResearch/artifact-auth.git` at reviewed revision `799459346d5416fbd7b9f55840a7371441b55afa`. Cargo and the non-flake Nix input resolve that same immutable revision. SSH credentials authorize retrieval only; they do not change source identity, policy, trust, or verification semantics.

The existing `signature.signature_hex` continues to authenticate Cairn's historical `cairn.authenticated-evidence.v1` canonical JSON. It cannot authenticate the distinct `artifact-auth.statement.v1` preimage. During dual run, a producer may add `signature.artifact_auth_signature_hex`, containing a separate Ed25519 signature over the mapped standalone statement bytes. Omitting this optional field preserves legacy operation and reports a standalone cutover blocker; it does not make the standalone path authoritative.

Release-readiness output retains the legacy report as the only value passed to Cairn's release gate. The diagnostic value also exposes:

- `artifact_auth_standalone`: the bounded standalone projection, when mapping succeeds;
- `artifact_auth_compatibility`: classified preimage, identity, decision, issue, and non-claim drift; and
- `check.artifact_auth_case_explained`: whether the observed case is admissible compatibility evidence.

`artifact_auth_case_explained` is not release readiness. The compatibility report fixes `legacy_authoritative = true`, `standalone_authority_admitted = false`, and `rollback_available = true`. Missing/malformed dedicated signatures, same or malformed preimages, unknown issues, identity/decision drift, unrelated-failure false parity, and weakened non-claims block cutover without bypassing current policy.

### Dependency update procedure

r[cairn.artifact_auth.repository_extraction.cutover] Review the candidate standalone commit, its Cairn profile under `config/consumers/cairn.ncl`, checked projection under `fixtures/consumers/cairn.json`, release evidence, package graph, and license before changing pins. Then:

1. Set the exact SSH URL and full commit in root `Cargo.toml` and the exact matching non-flake input revision in `flake.nix`; never use a branch, tag, sibling path, abbreviated revision, or duplicate source.
2. Regenerate `Cargo.lock` with Cargo and `flake.lock` with `nix flake lock`/`nix flake update`; never edit either lock manually.
3. Run `nix build .#checks.x86_64-linux.artifact-auth-source-pin -L`. It rejects Cargo/Nix revision drift, duplicate packages, floating or workspace-relative sources, incompatible licensing, profile/publication mismatch, and its synthetic negative mutations.
4. Run positive and tamper dual-run fixtures, strict Cargo checks, Slotcar, Cairn lifecycle validation/traceability, and the full Nix rail. Keep the old path authoritative until all drift is explained and a separate lifecycle change explicitly admits authority.

### Rollback procedure

r[cairn.artifact_auth.repository_extraction.cutover.negative] Runtime rollback is immediate: omit `artifact_auth_signature_hex` and continue using the legacy signature/report; standalone absence remains visible as a cutover blocker. For a dependency rollback, restore the last reviewed Cargo/Nix source declarations and adapter revision as one reviewed VCS change, regenerate both lockfiles with their owning tools, rerun the source-pin and full verification rails, and retain the compatibility evidence explaining why rollback occurred. Do not delete or reinterpret receipts to manufacture parity.

r[cairn.artifact_auth.repository_extraction.non_claims] A passing standalone result proves only that supplied public-key material verifies the exact standalone canonical statement and that supplied policy observations satisfy the selected bounded profile. It does **not** prove artifact truth, producer authorization, trust-root correctness, revocation freshness, provenance correctness, lifecycle conformance, build correctness, semantic equivalence, deployment safety, or release eligibility. Cairn continues to own filesystem measurement, canonical receipt reconstruction, trust/currentness collection, lifecycle profile selection, readiness composition, JSON presentation, and process exits.
