# Evidence-chain readiness

Cairn evaluates stack evidence through a typed, payload-opaque readiness core. The core receives already-loaded identity and linkage records; it does not read files, parse evidence payload internals, discover policy, execute tools, or grant runtime authority.

## Typed boundary

`EvidenceChainReadinessInput` binds:

- evidence kind and profile version;
- canonical envelope BLAKE3 identity;
- Valence validation artifact identity and its canonical-envelope link;
- Mantle binding identity and its Valence, source, and binary links;
- required policy identities;
- claim scope;
- bounded compatibility projection identities; and
- required non-claim text.

`EvidenceChainPolicy` owns optional or required mode, the Cairn gate-policy identity, required policy hashes, and an explicit profile registry. `function-address` is the first built-in profile. Additional profiles must be explicitly registered with exact versions, bounded non-empty claim scopes, and bounded non-empty non-claims; unknown present evidence never becomes a successful optional result.

## Mode semantics

- Optional absent evidence is skipped.
- Optional present evidence is fully validated and fails if invalid.
- Required missing or invalid evidence fails.
- Passing present evidence receives a deterministic chain hash over normalized typed links and the gate-policy identity.

Compatibility projections remain subordinate to the canonical envelope identity. Set-like policy, projection, and non-claim inputs are normalized before chain hashing so traversal or declaration order does not change identity.

## Claim boundary

A passing result proves typed identity linkage and lifecycle-policy conformance for the declared profile only. It does not prove evidence truth, Rust semantic correctness, compiler correctness, whole-program safety, verifier soundness, runtime authority, or release eligibility.

Filesystem measurement, canonical receipt recomputation, and signature verification are separate shell-owned responsibilities introduced by later authenticated-input work. This core does not turn caller-supplied digest text into measured-byte authority by itself.

<!-- r[verify cairn.stack_provenance_gates.evidence_chain_readiness.validation] -->
