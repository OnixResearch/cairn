# Artifact Auth Repository Specification

## Purpose

Defines the `artifact-auth-repository` capability.

## Requirements

### Requirement: Cross-consumer authentication inventory

r[cairn.artifact_auth.repository_extraction.inventory] Cairn MUST record a reviewed inventory of artifact-authentication semantics in Cairn, Mantle, Molten, and Valence before selecting the standalone canonical contract, including exact source identity, signed fields, canonicalization, subject and parent references, producer and key identity, purpose, generation/currentness, revocation, threshold counting, outcomes, non-claims, and retained product authority.

#### Scenario: Complete inventory separates shared and retained semantics

r[cairn.artifact_auth.repository_extraction.inventory.complete]
- GIVEN the four reviewed consumer implementations and their exact source identities
- WHEN the extraction inventory is evaluated
- THEN every field and decision MUST be classified as canonical shared semantics, consumer mapping semantics, retained product authority, interoperability metadata, or an explicit blocker
- AND no product-specific field may become canonical merely because one implementation already exposes it.

#### Scenario: Incomplete inventory blocks extraction

r[cairn.artifact_auth.repository_extraction.inventory.incomplete]
- GIVEN an inventory omits a consumer's preimage, key-purpose, revocation, threshold, failure, or authority semantics
- WHEN standalone contract selection is attempted
- THEN the extraction MUST fail closed with the missing consumer and semantic class
- AND no standalone release identity may be admitted.

### Requirement: Independent workspace packages own the neutral contract

r[cairn.artifact_auth.repository_extraction.repository] Cairn MUST consume product-neutral authentication mechanics from the independently versioned `artifact-auth-core` and `artifact-auth-ed25519` packages in `OnixResearch/onix-artifact`, and those packages MUST retain their own lifecycle, documentation, release profile, source identity, and `MIT OR Apache-2.0` boundary without acquiring product authority.

#### Scenario: Unified source preserves package independence

r[cairn.artifact_auth.repository_extraction.repository.neutral]
- GIVEN one reviewed immutable Artifact workspace revision with authentication and transfer packages
- WHEN Cairn validates the source package graph and package metadata
- THEN the authentication packages retain their narrow dependency edge and no transfer, binding, product, or authority dependency enters them.

#### Scenario: Product-coupled or widened source is rejected

r[cairn.artifact_auth.repository_extraction.repository.product_dependency]
- GIVEN a source package imports a product crate, adds an unreviewed cross-mechanism dependency, uses a workspace-relative consumer path, or lacks exact provenance and compatible licensing
- WHEN source admission runs
- THEN Cairn rejects the source before dependency or release evidence is accepted.

### Requirement: Standalone core remains authority free

r[cairn.artifact_auth.repository_extraction.functional_core] The standalone `artifact-auth-core` package MUST be a deterministic `#![no_std]` plus `alloc` functional core over explicit in-memory values and MUST NOT read files, inspect environment variables, spawn processes, open networks, access clocks, generate or store keys, contact trust services, print diagnostics, or perform persistence.

#### Scenario: Explicit values produce a deterministic decision

r[cairn.artifact_auth.repository_extraction.functional_core.deterministic]
- GIVEN identical validated statement, policy, key, currentness, and cryptographic observation values
- WHEN the core evaluates them on supported host and Wasm targets
- THEN it MUST produce identical normalized decisions, issue ordering, and BLAKE3 identities without ambient state.

#### Scenario: Ambient authority is absent

r[cairn.artifact_auth.repository_extraction.functional_core.no_authority]
- GIVEN the standalone core is compiled and audited
- WHEN forbidden-authority and dependency checks run
- THEN filesystem, environment, process, network, clock, randomness, credential, private-key, and persistence dependencies MUST be absent.

### Requirement: Canonical statement binds typed artifact identity

r[cairn.artifact_auth.repository_extraction.canonical_statement] The standalone core MUST define an injective versioned statement preimage that binds domain, purpose, profile identity, producer identity, full key identity, subject artifact reference, sorted unique parent references, and verifier-context reference while excluding signatures, private material, routing names, local paths, credentials, and product policy decisions.

#### Scenario: Equivalent typed statements have stable bytes

r[cairn.artifact_auth.repository_extraction.canonical_statement.stable]
- GIVEN two statements with identical typed meaning but different input collection order
- WHEN canonical preimage construction runs
- THEN the canonical bytes and BLAKE3 statement identity MUST be identical
- AND the preimage MUST preserve each digest algorithm and interoperability profile explicitly.

#### Scenario: Cross-domain replay is rejected

r[cairn.artifact_auth.repository_extraction.canonical_statement.domain_separation]
- GIVEN a valid signature for one domain, purpose, profile, subject, parent set, producer, or key identity
- WHEN any bound field is changed or the signature is replayed under another profile
- THEN verification MUST fail without coercing the changed statement into the original identity.

### Requirement: Ed25519 verification is pure and explicit

r[cairn.artifact_auth.repository_extraction.ed25519] The standalone `artifact-auth-ed25519` package MUST verify Ed25519 signatures over canonical standalone statement bytes using explicitly supplied public-key material and MUST NOT own signing services, private-key bytes, key generation, key persistence, or ambient key discovery.

#### Scenario: Matching signature verifies

r[cairn.artifact_auth.repository_extraction.ed25519.positive]
- GIVEN canonical statement bytes, a matching Ed25519 public key, and a valid signature
- WHEN pure verification runs
- THEN it MUST return a verified cryptographic observation bound to the full public-key BLAKE3 identity.

#### Scenario: Malformed or mismatched signature fails

r[cairn.artifact_auth.repository_extraction.ed25519.negative]
- GIVEN malformed key material, malformed signature bytes, a wrong key, a changed statement, or an unsupported algorithm label
- WHEN verification runs
- THEN it MUST return a deterministic failure class without panic, fallback algorithm selection, or secret-bearing diagnostics.

### Requirement: Trust policy uses explicit currentness observations

r[cairn.artifact_auth.repository_extraction.policy] The standalone core MUST evaluate only explicitly supplied trusted-key, purpose, generation, revocation, threshold, and currentness observations; signer labels MUST NOT substitute for full key identity, and threshold counts MUST use distinct verified full-key identities.

#### Scenario: Distinct current keys satisfy policy

r[cairn.artifact_auth.repository_extraction.policy.positive]
- GIVEN enough distinct verified full-key identities with admitted purposes, generations, currentness, and non-revoked status
- WHEN policy evaluation runs
- THEN the decision MAY report policy satisfaction for the exact statement and profile.

#### Scenario: Stale or duplicate identity fails policy

r[cairn.artifact_auth.repository_extraction.policy.negative]
- GIVEN duplicate signer labels, duplicate full keys, a revoked key, stale generation, wrong purpose, insufficient threshold, missing currentness evidence, or an unknown profile
- WHEN policy evaluation runs
- THEN the decision MUST fail closed with stable issue classes
- AND it MUST NOT claim revocation freshness beyond the supplied observation.

### Requirement: Consumer products retain authority

r[cairn.artifact_auth.repository_extraction.consumer_authority] Cairn and every downstream consumer MUST retain measurement, payload canonicalization, key storage, signing, trust-root selection, currentness collection, authorization, lifecycle, build, transport, registry, runtime, and release decisions in product-owned shells or policy layers.

#### Scenario: Cairn adapter supplies observations only

r[cairn.artifact_auth.repository_extraction.consumer_authority.cairn]
- GIVEN Cairn reads authenticated evidence, policy, and public-key inputs
- WHEN it delegates neutral statement and verification semantics to the standalone packages
- THEN Cairn MUST keep filesystem measurement, canonical receipt reconstruction, lifecycle profile selection, release-readiness composition, JSON presentation, and process exit behavior outside the standalone core.

#### Scenario: Shared core cannot promote product claims

r[cairn.artifact_auth.repository_extraction.consumer_authority.no_promotion]
- GIVEN a standalone authentication decision passes
- WHEN a consumer evaluates authorization, readiness, build, registry, runtime, or release policy
- THEN the shared decision MUST remain one bounded input and MUST NOT independently authorize or promote the product outcome.

### Requirement: Cutover uses immutable source and compatibility evidence

r[cairn.artifact_auth.repository_extraction.cutover] Cairn MUST consume one immutable reviewed Artifact workspace revision with aligned Cargo and Nix identities, MUST keep its resolved source package set limited to the two authentication packages, MUST preserve a bounded historical-source rollback, and MUST reject floating, workspace-relative, widened, or mismatched release dependencies.

#### Scenario: Exact unified source permits receiver migration

r[cairn.artifact_auth.repository_extraction.cutover.positive]
- GIVEN the unified revision is remotely available, Cargo and Nix agree, authentication crate bytes match the historical source, focused behavior tests pass, and only two authentication packages enter Cairn's graph
- WHEN cutover admission runs
- THEN Cairn may use the unified source while retaining all product authority and the historical rollback reference.

#### Scenario: Drift or graph widening blocks migration

r[cairn.artifact_auth.repository_extraction.cutover.negative]
- GIVEN source bytes, revision, repository, NAR hash, license, package profile, Cargo lock set, Nix lock, adapter behavior, or required non-claims drift
- WHEN cutover admission runs
- THEN Cairn keeps the migration blocked and does not treat equal package names as compatibility proof.

### Requirement: Conformance includes positive and negative mappings

r[cairn.artifact_auth.repository_extraction.conformance] The standalone repository MUST ship positive and negative canonical vectors and consumer mapping profiles for Cairn, Mantle, Molten, and Valence before downstream adoption changes can pin a release.

#### Scenario: Cross-consumer fixtures pass without semantic loss

r[cairn.artifact_auth.repository_extraction.conformance.positive]
- GIVEN representative valid statements and supplied policy observations from every consumer
- WHEN standalone and consumer mapping conformance runs
- THEN canonical identities, verified key identities, decisions, and bounded non-claims MUST match the reviewed mapping profile.

#### Scenario: Failure corpus rejects unsafe mappings

r[cairn.artifact_auth.repository_extraction.conformance.negative]
- GIVEN wrong domain or purpose, changed subject or parents, malformed digest or signature, unknown algorithm, revoked or stale key, duplicate-key threshold inflation, unknown profile, missing currentness, unsafe diagnostics, weakened non-claims, or a lossy product mapping
- WHEN conformance runs
- THEN the fixture MUST fail with its expected stable class and no consumer cutover may proceed.

### Requirement: Authentication claims remain bounded

r[cairn.artifact_auth.repository_extraction.non_claims] Standalone and consumer documentation, decisions, and receipts MUST state that accepted authentication proves only that supplied key material verifies the exact canonical statement and that supplied policy observations satisfy the selected bounded profile.

#### Scenario: Required non-claims are preserved

r[cairn.artifact_auth.repository_extraction.non_claims.preserved]
- GIVEN a passing cryptographic and policy decision
- WHEN documentation or receipt projection is produced
- THEN it MUST deny claims of artifact truth, producer authorization, trust-root correctness, revocation freshness, provenance correctness, lifecycle conformance, build correctness, semantic equivalence, deployment safety, and release eligibility.

#### Scenario: Overclaim wording is rejected

r[cairn.artifact_auth.repository_extraction.non_claims.overclaim]
- GIVEN a profile, mapping, or receipt describes signature validity as proof of truth, authority, correctness, readiness, or universal trust
- WHEN non-claim validation runs
- THEN the artifact MUST be rejected before release or downstream adoption.
