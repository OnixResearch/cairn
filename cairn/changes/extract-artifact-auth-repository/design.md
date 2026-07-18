## Context

Cairn's authenticated-evidence path already demonstrates the desired functional-core / imperative-shell shape. `cairn-core::verified::authenticated_inputs` evaluates typed evidence, producer, key, purpose, identity, linkage, and signature observations without I/O. `cairn-cli::authenticated_inputs` reads and measures files, reconstructs canonical receipt bytes, loads public policy material, and performs Ed25519 verification. The current contract is nevertheless Cairn-specific: it names lifecycle evidence kinds, Valence profiles, release modes, and Cairn receipt preimages.

Mantle has several overlapping but product-specific trust paths, including release witnesses and OCI signature companions. Molten has purpose-separated key generations, rotation, revocation, and canonical payload-reference signatures. Valence verifies Radicle entry signatures while keeping Evidence IR identity and authorization separate. The overlap is large enough to justify one authenticity owner, but no product schema can be adopted wholesale without importing product authority or creating a dependency cycle.

The Mantle owner has authorized reuse, modification, and relicensing of Mantle code for this extraction. Mantle's current AGPL repository label is therefore not an architectural blocker. Any selected Mantle source still needs durable file-level provenance, preserved attribution, an explicit owner-approved standalone license grant, and review that product authority has been removed. Implementation source is selected by semantic fit and purity rather than by repository of origin.

## Decisions

### Decision: create a descriptive standalone repository

**Choice:** Create `OnixResearch/artifact-auth` with canonical packages `artifact-auth-core` and `artifact-auth-ed25519`, typed Nickel review profiles, generated fixture projections, a native Cairn lifecycle, and independently versioned release evidence.

**Rationale:** A descriptive repository matches the established `nickel-export` and `host-control` extraction pattern and makes the claim boundary visible. Products can pin authenticity semantics without depending on Cairn lifecycle behavior or another product runtime.

### Decision: inventory and freeze compatibility before implementation selection

**Choice:** Record exact source revisions and map Cairn, Mantle, Molten, and Valence fields and decisions into shared semantics, consumer extensions, retained authority, interoperability metadata, or blockers. Freeze positive and negative vectors before selecting final standalone schemas.

**Rationale:** Similar names such as subject, signer, purpose, revocation, or threshold do not prove identical semantics. The inventory prevents one implementation from becoming canonical by accident and gives the extraction a measurable loss oracle.

### Decision: select source by semantic fit with explicit relicense provenance

**Choice:** Cairn and Mantle implementations are both eligible source material. After the compatibility corpus is frozen, the standalone implementation may refactor whichever pure pieces best match the neutral contract. Every imported source unit records its original repository and revision, material changes, attribution, owner authorization, and MIT/Apache grant. Product-specific shell or policy code is not imported merely because relicensing is possible.

**Rationale:** Sole-owner authorization removes the prior license blocker, but it does not make product semantics neutral. Separating rights clearance from semantic selection permits reuse without turning Mantle's build, OCI, registry, release, signing, or trust decisions into shared authority.

### Decision: authenticate typed references, not arbitrary ambient payload bytes

**Choice:** The canonical statement binds version, domain, purpose, profile identity, producer identity, full key identity, one typed subject reference, sorted unique parent references, and a verifier-context reference. Artifact references carry algorithm and profile explicitly. Stack-owned identities default to BLAKE3; required external algorithms remain labeled interoperability references. Signatures and product routing, local paths, credentials, and policy outcomes are excluded from the preimage.

**Rationale:** Consumers use different payload formats and canonicalizers. Signing one standalone typed reference avoids claiming JSON, Preserves, OCI, Radicle, or Evidence IR equivalence while still preventing cross-domain, cross-purpose, cross-subject, and cross-parent replay.

### Decision: own a versioned injective preimage independent of product codecs

**Choice:** `artifact-auth-core` owns one versioned, length-delimited, fixture-backed preimage encoding over validated fields. Input collections normalize to deterministic order and reject duplicates. JSON and Preserves are optional consumer projections that must revalidate decoded values; neither is the canonical core preimage.

**Rationale:** Depending on a product codec would transfer ownership or create cycles. A small explicit preimage keeps signature semantics stable and can be checked with cross-language vectors without claiming arbitrary serialization equivalence.

### Decision: separate statement/policy logic from Ed25519 verification

**Choice:** `artifact-auth-core` owns DTO validation, preimage construction, BLAKE3 identities, policy evaluation, issue ordering, and bounded decision projection. `artifact-auth-ed25519` owns pure public-key and signature parsing plus Ed25519 verification over core-produced bytes. Neither package signs, generates keys, stores private material, discovers trust, reads files, accesses clocks, or contacts services.

**Rationale:** Consumers need one cryptographic verifier but should retain signing and key lifecycle behind product-specific capability boundaries. The package split keeps the semantic core usable in no-std and Wasm contexts and makes cryptographic dependencies explicit.

### Decision: currentness is supplied evidence, not ambient time

**Choice:** Policy evaluation receives explicit trusted-key records and explicit generation, purpose, revocation, and currentness observations. The core reports whether those supplied facts satisfy the selected profile. It never reads a clock, fetches a revocation list, or claims freshness beyond the observation identity.

**Rationale:** Revocation freshness and key rotation are integration responsibilities. Hiding them behind ambient lookup would make deterministic replay and offline verification impossible.

### Decision: thresholds count full key identities

**Choice:** Threshold and quorum decisions count distinct verified full-public-key BLAKE3 identities. Signer or producer labels remain policy metadata and may map to multiple keys; labels cannot inflate the count or select a passing key without cryptographic verification.

**Rationale:** Mantle already exposes the danger of same-name keys. Full-key counting avoids label collision and duplicate-key quorum inflation while supporting bounded rotation overlap.

### Decision: preserve Evidence IR, UCAN, and product policy boundaries

**Choice:** Valence remains the owner of Evidence IR identities and links. UCAN remains the owner of delegated capability tokens. Cairn, Mantle, Molten, and Valence retain measurement, payload canonicalization, signing, key storage, trust-root selection, authorization, readiness, build, transport, registry, runtime, and release policy. The standalone decision is authenticity evidence only.

**Rationale:** Combining identity, authenticity, capability authority, and release policy would recreate a central product and blur non-claim boundaries. The extraction is valuable only if its dependency direction stays narrow.

### Decision: cut over with exact pins, dual run, and rollback

**Choice:** Cairn first records the existing authenticated-input baseline, then pins one immutable standalone Cargo and Nix source identity. A pure Cairn adapter maps current observations into the standalone contract and projects results back into Cairn's lifecycle-specific schema. Legacy and standalone paths run over identical observations until every difference is equal, intentionally mapped, or blocking. Workspace-relative and floating release dependencies are rejected.

**Rationale:** Authentication drift is security-significant. Exact pins and dual-run evidence make migration reviewable and preserve a fail-closed rollback route without maintaining two canonical implementations indefinitely.

### Decision: downstream adoption remains consumer owned

**Choice:** The standalone repository publishes reviewed Cairn, Mantle, Molten, and Valence mapping profiles. Consumer-owned Cairn changes are created only after a reviewed standalone release identity exists. The producer change does not edit downstream repositories.

**Rationale:** Each consumer owns its dependency, compatibility surface, authority boundary, tests, and release evidence. Cross-repository mutation from the producer would hide those decisions and make rollback ambiguous.

## Alternatives Considered

### Expand Valence to own generic authenticity

Rejected because Valence deliberately owns evidence identity and linkage rather than general artifact authentication, and Mantle or Molten signatures may authenticate subjects that are not Valence Evidence IR artifacts.

### Reuse UCAN as the signed envelope

Rejected because UCAN signs delegated capability tokens with proof-chain and authorization semantics. A detached artifact-authentication statement must not imply capability delegation or ambient authority.

### Keep the neutral core in Cairn

Rejected because Mantle, Molten, and Valence should not depend on lifecycle and readiness packages to verify generic artifact statements. Cairn remains one eligible implementation source and the first planned consumer, not the final owner.

### Extract Mantle's attestation crate directly

Rejected as an unmodified move because Mantle's crate carries release, build, OCI, registry, signing, and trust concepts that are outside the neutral boundary. Owner-authorized Mantle code may still be relicensed and refactored into the standalone packages when provenance is exact and the compatibility corpus proves semantic fit.

### Canonicalize and sign each product payload format directly

Rejected because it would require the standalone repository to own or equate JSON, Preserves, OCI, Radicle, and product receipt semantics. Products instead measure their payloads and supply typed artifact references.

## Risks / Trade-offs

- A narrow typed-reference statement may not remove every product-local signature helper. Product-specific payload construction and trust decisions intentionally remain local.
- A new cryptographic dependency concentrates failure impact. Mitigation requires fixed vectors, malformed-input tests, cross-implementation verification where available, and explicit upgrade Cairns.
- Consumer semantics may prove incompatible. The inventory may validly block or split profiles rather than forcing a union.
- Owner authorization permits Mantle reuse but does not replace durable provenance, attribution, and per-source license evidence; missing records block release.
- Independent releases add pin/update work. Exact source checks and consumer-owned upgrade packages are required to keep that coordination visible.
- A valid signature can be socially misleading. Required non-claims and separate authorization/readiness decisions remain release blockers.
