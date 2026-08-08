# Radicle Cob Only Specification

## Purpose

Defines the `radicle-cob-only` capability.

## Requirements

### Requirement: Minimal COB dependency boundary
r[cairn.radicle_cob_only.dependencies] Cairn MUST implement its built-in signed Git collaboration provider with direct Radicle COB, cryptographic, object-identity, Git, and ref-format primitives without linking the umbrella Radicle standard library or requiring a Radicle node runtime.

#### Scenario: COB build excludes Radicle runtime
r[cairn.radicle_cob_only.dependencies.excludes_runtime]
- GIVEN the Cairn workspace dependency graph and supported-system Nix checks
- WHEN the COB provider is built and audited
- THEN no production dependency or command path includes the `radicle` umbrella crate or `radicle-node` executable
- AND direct COB format dependencies remain version-pinned by the generated Cargo lockfile

#### Scenario: Format-required primitives remain explicit
r[cairn.radicle_cob_only.dependencies.required_primitives]
- GIVEN signed COB entries use Git object IDs and Radicle extended signatures
- WHEN Cairn records dependency and receipt boundaries
- THEN Git SHA-1, Radicle crypto, OID, DAG, and ref-format use MUST be labeled format interoperability rather than Cairn-owned hash policy
- AND Cairn-owned semantic and receipt identities MUST remain BLAKE3

### Requirement: Explicit local Git COB store
r[cairn.radicle_cob_only.store] Cairn MUST operate the custom `xyz.onixresearch.cairn-change` object through one explicit Git repository and a deterministic bounded implementation of the direct `radicle-cob` storage traits.

#### Scenario: Signed object round trip preserves exact state
r[cairn.radicle_cob_only.store.roundtrip]
- GIVEN an initialized Git repository, explicit Ed25519 signer, valid root action, and immutable authority set
- WHEN Cairn creates, reloads, updates, and reloads the custom COB
- THEN every action blob, signed author, operation ID, parent relation, object ID, reduced state, conflict head, BLAKE3 state identity, and BLAKE3 content identity MUST match the expected direct COB history

#### Scenario: Unsafe refs and malformed entries fail closed
r[cairn.radicle_cob_only.store.negative]
- GIVEN malformed namespace components, symbolic or non-commit COB refs, duplicate matching change objects, invalid signatures, invalid manifests, malformed action JSON, unknown bases, or unauthorized actors
- WHEN Cairn enumerates or evaluates the explicit Git COB store
- THEN it MUST return deterministic failure without updating a COB ref or materializing partial trusted state

### Requirement: Immutable in-object authority
r[cairn.radicle_cob_only.authority] New Cairn change COB roots MUST bind a bounded immutable delegate public-key set into signed root state and MUST derive operation authority only from that set, the signed entry author, and the root creator.

#### Scenario: Creator and delegates are deterministic
r[cairn.radicle_cob_only.authority.allowed]
- GIVEN a signed root created by one actor with distinct explicitly listed delegate keys
- WHEN history is evaluated without a Radicle identity document
- THEN creator-or-delegate content updates and delegate-only approval transitions MUST be evaluated identically from the COB history alone

#### Scenario: Ambient or stale authority cannot authorize
r[cairn.radicle_cob_only.authority.denied]
- GIVEN an actor is absent from the immutable root delegates or a legacy operation requires authority available only from an external Radicle identity document
- WHEN the actor's operation is evaluated
- THEN Cairn MUST reject that operation branch
- AND CLI assertions, Git configuration, profile files, current clocks, or later authority files MUST NOT make it valid

#### Scenario: Authority set is bounded and canonical
r[cairn.radicle_cob_only.authority.bounded]
- GIVEN duplicate creator/delegate keys or more than the supported maximum delegate keys
- WHEN Cairn validates a new root
- THEN it MUST reject invalid authority without writing a COB reference
- AND valid delegate ordering MUST be canonical and deterministic.

#### Scenario: Legacy empty-authority hashes remain stable
r[cairn.radicle_cob_only.authority.legacy_hash]
- GIVEN an existing valid collaboration state whose root has no embedded delegate field
- WHEN Cairn decodes and hashes that semantic state
- THEN its prior canonical BLAKE3 content and state identities MUST remain unchanged
- AND the absence of embedded delegates MUST NOT be interpreted as external delegate authority

### Requirement: COB-only collaboration CLI
r[cairn.radicle_cob_only.cli] Cairn MUST expose preview-first local `cairn collaboration import|status|checkout|update|conflicts|resolve --backend radicle-cob.v1` commands and a `cairn cob` compatibility namespace using explicit Git repository and, for signed mutations, keystore inputs.

#### Scenario: Read and mutation context is explicit
r[cairn.radicle_cob_only.cli.explicit]
- GIVEN an operator invokes a COB status or mutation command
- WHEN arguments are parsed
- THEN status/conflict inspection MUST require `--git-repository PATH`
- AND import/update/resolve MUST additionally require `--keystore PATH`
- AND import MUST require an explicit workspace identity while optional delegates are explicit public keys
- AND signed mutation MUST require non-empty repository-local Git `user.name` and `user.email` commit metadata without treating those display fields as authority

#### Scenario: Preview and execution remain separated
r[cairn.radicle_cob_only.cli.preview]
- GIVEN valid explicit local COB inputs
- WHEN `--execute` is absent
- THEN Cairn MUST emit deterministic plans and receipts without writing Git objects, COB refs, locks, prepared files, or active-change files
- AND execution MUST recheck repository head, object state, signer, authority, policy, local artifacts, and the exact plan under cooperating locks

#### Scenario: Removed Radicle runtime arguments are rejected
r[cairn.radicle_cob_only.cli.reject_runtime]
- GIVEN an operator supplies `--rad-home`, `--rid`, control socket, peer ID, peer address, seed, follow, announce, or sync arguments
- WHEN Cairn parses a collaboration command
- THEN it MUST reject the unsupported runtime surface before mutation
- AND it MUST NOT infer a profile, node, repository ID, or peer from ambient state

### Requirement: No Radicle runtime or transport ownership
r[cairn.radicle_cob_only.no_runtime] Cairn MUST NOT expose Radicle node connection, discovery, seeding, follow, announce, fetch, push, synchronization, or convergence behavior as part of the COB-only provider.

#### Scenario: Synchronization is outside Cairn
r[cairn.radicle_cob_only.no_runtime.sync]
- GIVEN local COB refs need transfer between repositories
- WHEN an operator chooses a Git transport outside Cairn
- THEN Cairn MAY read the resulting local refs after transfer
- AND Cairn MUST NOT claim ownership or evidence of peer discovery, publication, remote authorization, availability, or convergence

#### Scenario: Runtime artifacts are absent
r[cairn.radicle_cob_only.no_runtime.artifacts]
- GIVEN the source tree, CLI usage, tests, and Nix checks
- WHEN the COB-only boundary is validated
- THEN node control clients, live-node process harnesses, peer convergence fixtures, and `radicle-node` package inputs MUST be absent from maintained product and validation paths

### Requirement: COB-only receipts and documentation
r[cairn.radicle_cob_only.docs] COB provider receipts and operator documentation MUST identify direct library versions, explicit Git/keystore context, signed author and immutable authority observations, object/ref/state/content identities, execution mode, blockers, and bounded non-claims.

#### Scenario: Local COB success is not network proof
r[cairn.radicle_cob_only.docs.non_claims]
- GIVEN a passing signed local COB import, update, resolution, status, or checkout receipt
- WHEN Cairn presents the result
- THEN the claim MUST be limited to the recorded local Git COB and filesystem observations
- AND it MUST NOT claim Radicle repository identity, identity-document authority, node participation, publication, synchronization, remote trust, peer availability, convergence, lifecycle correctness, or release eligibility

### Requirement: Exact Radicle COB format compatibility
r[cairn.radicle_cob_only.compatibility] Cairn MUST preserve the standard Radicle COB Git commit, manifest, action, parent-DAG, extended-signature, object-ID, type-ID, and namespaced-ref encodings required for interoperable object storage without claiming runtime or network compatibility.

#### Scenario: Legacy semantic identities remain stable
r[cairn.radicle_cob_only.compatibility.legacy]
- GIVEN a valid historical `cairn.radicle-change-cob.v1` state without embedded delegates
- WHEN the updated reducer decodes and hashes it
- THEN its canonical content and state identities MUST match the prior encoding exactly.

#### Scenario: Namespaced COB refs are standard
r[cairn.radicle_cob_only.compatibility.git_refs]
- GIVEN Cairn creates or updates a custom COB
- WHEN the explicit Git repository is inspected
- THEN the resulting reference MUST use the standard signed namespace and `refs/cobs/<type>/<object>` structure.

#### Scenario: Signed commits round-trip through direct traits
r[cairn.radicle_cob_only.compatibility.signatures]
- GIVEN an explicit valid Ed25519 keystore
- WHEN Cairn stores and reloads a COB operation
- THEN the standard extended signature and manifest MUST verify through direct `radicle-cob` traits.

#### Scenario: Malformed format evidence is rejected
r[cairn.radicle_cob_only.compatibility.malformed]
- GIVEN a malformed, unsigned, mismatched-author, or wrong-manifest COB history
- WHEN Cairn inspects the explicit store
- THEN inspection MUST fail closed instead of omitting or accepting the malformed branch.

### Requirement: Reproducible COB-only validation
r[cairn.radicle_cob_only.validation] The implementation MUST pass positive and negative direct-COB tests, a production dependency/runtime absence audit, workspace checks, lifecycle and traceability gates, release-readiness, self-dogfood, and the full supported-system flake check before archive.

#### Scenario: Direct crate boundary is executable evidence
r[cairn.radicle_cob_only.validation.direct]
- GIVEN the final source and generated lockfile
- WHEN the focused adapter suite and dependency audit run
- THEN signed create/read/update/conflict/resolution behavior MUST pass without a Radicle profile or node executable
- AND any production reference to umbrella Radicle, node control, peer transport, or ambient profile state MUST fail the audit
