# Content Addressed Rust Evidence Delta

## ADDED Requirements

### Requirement: Octet function object receipt contract
r[content_addressed_rust_evidence.function_object] The system MUST define a deterministic receipt contract for Octet-emitted Rust function objects that identifies the selected Rust definition, canonical object hash, source-token body hash, signature hash, normalization version, toolchain identity, and explicit caveats.

#### Scenario: Source-token function object is accepted with caveats
r[content_addressed_rust_evidence.function_object.source_token]

- GIVEN an Octet function object with schema `octet.function-object.v1`
- WHEN Cairn validates the object receipt
- THEN the receipt includes the object hash, body hash, signature hash, normalization metadata, and caveats declaring source-token rather than semantic compiler identity.

#### Scenario: Function object does not imply correctness
r[content_addressed_rust_evidence.function_object.no_correctness_claim]

- GIVEN a function object with matching hashes
- WHEN Cairn renders or validates the associated receipt
- THEN the receipt states that hash agreement is review/provenance evidence and not a proof of behavioral correctness.

### Requirement: Canonical object hashing
r[content_addressed_rust_evidence.canonical_hash] The system MUST require Octet content-addressed Rust evidence objects to compute BLAKE3 hashes over deterministic canonical content that excludes self-referential hash fields and ambient entropy.

#### Scenario: Canonical hash is portable
r[content_addressed_rust_evidence.canonical_hash.portable]

- GIVEN the same logical function object emitted under two repository roots
- WHEN canonical object hashing is computed
- THEN the hash input excludes display-only root paths and produces the same object hash.

#### Scenario: Ambient entropy is rejected
r[content_addressed_rust_evidence.canonical_hash.no_entropy]

- GIVEN an object receipt containing timestamps, absolute temporary paths, or environment-derived values in hash material
- WHEN Cairn validates the receipt
- THEN validation fails or reports the fields as non-hash display metadata.

### Requirement: Name pointer movement receipts
r[content_addressed_rust_evidence.name_pointer] The system MUST model mutable Rust name bindings as pointers to immutable object hashes and require deterministic receipts for pointer creation, movement, rollback, and drift checks.

#### Scenario: Pointer movement is reviewable
r[content_addressed_rust_evidence.name_pointer.move]

- GIVEN a name pointer from `crate::module::function` to an old object hash
- WHEN the pointer is moved to a new object hash
- THEN the receipt records the old hash, new hash, reason, diff summary, and rollback handle.

#### Scenario: Pointer drift fails closed
r[content_addressed_rust_evidence.name_pointer.drift]

- GIVEN a reviewed pointer and source that no longer matches its referenced object
- WHEN Cairn consumes an Octet check receipt
- THEN the receipt marks the pointer invalid until an explicit update receipt is present.

### Requirement: Dependency and effect caveat vocabulary
r[content_addressed_rust_evidence.caveats] The system MUST define a closed, machine-readable caveat vocabulary for dependency identity status, macro/generated-code status, and coarse effect/capability summaries.

#### Scenario: Source-scanned dependencies remain caveated
r[content_addressed_rust_evidence.caveats.dependency_status]

- GIVEN a function object whose dependencies were extracted by source-token scanning
- WHEN Cairn validates the receipt
- THEN the dependency status is `source-scanner-token-set` or an equivalent closed value and does not claim HIR-resolved dependency identity.

#### Scenario: Unknown effects block purity claims
r[content_addressed_rust_evidence.caveats.effects_fail_closed]

- GIVEN a function object with unknown, macro-hidden, or unresolved effects
- WHEN a downstream consumer asks whether the object is pure-cache eligible
- THEN the receipt reports the object as not eligible unless a stronger reviewed effect status is present.

### Requirement: Compiler IR evidence boundary
r[content_addressed_rust_evidence.compiler_ir] The system MUST specify future HIR, THIR, and MIR evidence as compiler-version-coupled evidence layers that supplement but do not replace the source-token function object contract.

#### Scenario: HIR evidence reports resolved dependencies
r[content_addressed_rust_evidence.compiler_ir.hir_dependencies]

- GIVEN an Octet object with HIR evidence
- WHEN Cairn validates the receipt
- THEN the receipt records the rustc/toolchain identity, extractor version, resolved dependency summary, and any unresolved or macro-origin caveats.

#### Scenario: MIR evidence remains toolchain-coupled
r[content_addressed_rust_evidence.compiler_ir.mir_coupled]

- GIVEN an Octet object with MIR-derived call, panic, drop, async, or unsafe summaries
- WHEN Cairn validates the receipt
- THEN the receipt identifies the exact rustc/toolchain and does not claim stability across compiler versions.

### Requirement: Valence export bundle receipt
r[content_addressed_rust_evidence.export_bundle] The system MUST define a deterministic export-bundle receipt for downstream consumers that includes object files, name pointers, review manifests, caveat summaries, and the Octet command/replay metadata used to produce them.

#### Scenario: Bundle consistency is replayable
r[content_addressed_rust_evidence.export_bundle.replay]

- GIVEN an Octet export bundle
- WHEN Cairn validates the bundle receipt
- THEN the receipt verifies sorted object membership, pointer references to present object hashes, canonical bundle hash, and replay command metadata.

#### Scenario: Bundle mutation is explicit
r[content_addressed_rust_evidence.export_bundle.mutation_manifest]

- GIVEN an Octet operation that updates inline markers or pointer manifests
- WHEN Cairn records the operation
- THEN the receipt includes a deterministic mutation manifest with before and after hashes for touched files.

### Requirement: Downstream pilot evidence boundaries
r[content_addressed_rust_evidence.downstream_pilots] The system MUST define bounded interop receipt requirements for Crunch/Mantle evidence objects and Aspen CI/job objects without claiming downstream build or distributed-execution correctness from Octet hashes alone.

#### Scenario: Crunch evidence object is bounded
r[content_addressed_rust_evidence.downstream_pilots.crunch]

- GIVEN a Crunch/Mantle checker or parity evidence row referenced by an Octet evidence object
- WHEN Cairn validates the pilot receipt
- THEN the receipt records checker hash, input evidence hash, bounded claim class, and explicit non-claim that build correctness is not proven by object identity.

#### Scenario: Aspen CI object is bounded
r[content_addressed_rust_evidence.downstream_pilots.aspen]

- GIVEN an Aspen CI job, dogfood stage, or executor identity referenced by an Octet evidence object
- WHEN Cairn validates the pilot receipt
- THEN the receipt records config/source/executor identity hashes, replay/cache key material, and explicit non-claim that distributed execution correctness is proven by object identity.
