## Why

Cairn, Mantle, Molten, and Valence independently implement overlapping artifact-authentication semantics: versioned signed statements, Ed25519 verification, purpose or domain separation, producer and key selection, revocation or generation checks, distinct-key counting, subject linkage, and bounded authentication outcomes. Cairn already separates a pure authenticated-input evaluator from the CLI shell that measures bytes and verifies signatures, but its schemas and policy remain lifecycle-specific. Keeping the neutral subset inside any product invites signature-preimage, key-identity, threshold, and failure-class drift, while making one product the dependency of the others would invert ownership.

The shared boundary is authenticity of an exact typed artifact reference under explicitly supplied verification policy. It is not evidence identity, capability delegation, key storage, trust discovery, artifact truth, authorization, lifecycle conformance, build correctness, or release readiness. That boundary is stable enough for an independent permissively licensed repository, but extraction must begin with a cross-consumer inventory and compatibility corpus rather than silently renaming Cairn's authenticated-evidence schema as canonical.

## What Changes

- Record a reviewed inventory of Cairn, Mantle, Molten, and Valence statement, subject, signer, key-purpose, generation, revocation, threshold, canonicalization, outcome, and retained-authority semantics.
- Create `OnixResearch/artifact-auth` as the independent owner of product-neutral artifact-authentication DTOs, canonical statement preimages, pure policy evaluation, Ed25519 verification, mapping profiles, and positive and negative conformance fixtures.
- Build the standalone implementation from the MIT-or-Apache Cairn core boundary and a product-neutral specification; do not copy AGPL Mantle shell or core code into a permissively licensed package.
- Split the standalone implementation into an authority-free `artifact-auth-core` functional core and a pure `artifact-auth-ed25519` verifier; signing services, private-key handling, key generation, persistence, clocks, network access, and trust discovery remain outside both packages.
- Define a versioned statement that binds domain, purpose, profile, producer, key identity, subject reference, optional parent references, and verifier-supplied currentness facts without depending on JSON, Preserves, OCI, Valence, Cairn, Mantle, Molten, or UCAN semantics.
- Preserve BLAKE3 as the default stack-owned identity algorithm while carrying protocol-required digest algorithms, such as OCI SHA-256, as explicit interoperability references.
- Baseline and dual-run Cairn authenticated-input evaluation against the standalone core before cutover, retain a compatibility projection and rollback path until parity is explained, and consume only one immutable reviewed source revision.
- Publish consumer mapping profiles and create consumer-owned Cairn changes only after the standalone repository has a reviewed release identity; do not mutate Mantle, Molten, or Valence as hidden side effects of the Cairn extraction.

## Impact

- **New repository**: `OnixResearch/artifact-auth`, with Rust core and Ed25519 packages, typed Nickel review profiles, generated conformance data, native Cairn lifecycle, documentation, MIT/Apache licenses, and release evidence.
- **Cairn**: `crates/cairn-core/src/verified/authenticated_inputs.rs`, `crates/cairn-cli/src/authenticated_inputs.rs`, authenticated-evidence policy and fixtures, exact dependency pins, compatibility adapters, tests, docs, and evidence may be touched during implementation; Cairn keeps evidence measurement, lifecycle profiles, readiness gates, and release decisions.
- **Downstream handoff**: Mantle, Molten, and Valence receive mapping profiles and later consumer-owned lifecycle packages rather than workspace-relative dependencies.
- **Testing**: host and `wasm32-unknown-unknown` checks, canonical-preimage vectors, positive and negative Ed25519/policy fixtures, cross-consumer mapping-loss tests, Cairn dual-run parity, exact-source checks, dependency-isolation checks, policy freshness, workspace tests, Cairn validation/gates, Traceability coverage, Slotcar, and Nix checks.

## Out of Scope

- A signing daemon, key-management service, trust-root discovery protocol, transparency log, revocation distribution service, clock, network client, OCI/Radicle adapter, or filesystem CLI.
- Replacing Valence Evidence IR, Kamacite interchange, UCAN capability delegation, Basalt policy enforcement, Cairn lifecycle/readiness policy, or product-specific canonical payload formats.
- Claiming that signature validity proves artifact truth, producer authorization, provenance correctness, lifecycle conformance, build correctness, semantic equivalence, deployment safety, or release eligibility.
