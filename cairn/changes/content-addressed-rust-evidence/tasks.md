## Phase 1: Contract package

- [x] [serial] I1 Define the native Cairn proposal/design/spec package for content-addressed Rust evidence. r[content_addressed_rust_evidence.function_object] r[content_addressed_rust_evidence.canonical_hash]
- [ ] [serial] V1 Run Cairn validation and proposal/design/tasks gates for this change package. r[content_addressed_rust_evidence.function_object.source_token]

## Phase 2: Function object receipt implementation

- [ ] [serial] I2 Add Cairn receipt model support for Octet function-object v1 validation, including canonical hash material, normalization metadata, and required non-correctness claims. r[content_addressed_rust_evidence.function_object] r[content_addressed_rust_evidence.canonical_hash]
- [ ] [parallel] H1 Add deterministic fixture objects for portable canonical hashing and ambient-entropy rejection. command=`nix run path:$PWD#cairn -- validate --root .`; input=`fixtures/content-addressed-rust-evidence/function-object-v1/`; asserts=`portable object hashes match and entropy-bearing hash material fails closed`; evidence=`target/content-addressed-rust-evidence/function-object-v1-validation.json` r[content_addressed_rust_evidence.canonical_hash.portable] r[content_addressed_rust_evidence.canonical_hash.no_entropy]
- [ ] [serial] V2 Verify function-object receipt validation with positive and negative fixtures. r[content_addressed_rust_evidence.function_object.no_correctness_claim]

## Phase 3: Pointer and caveat contracts

- [ ] [serial] I3 Add receipt validation for name-pointer creation, movement, rollback, drift, and deterministic mutation manifests. r[content_addressed_rust_evidence.name_pointer] r[content_addressed_rust_evidence.export_bundle.mutation_manifest]
- [ ] [parallel] I4 Add closed caveat vocabularies for dependency identity status, macro/generated-code status, and coarse effect/capability summaries. r[content_addressed_rust_evidence.caveats]
- [ ] [parallel] H2 Add pointer and caveat fixture receipts. command=`nix run path:$PWD#cairn -- validate --root .`; input=`fixtures/content-addressed-rust-evidence/pointers-and-caveats/`; asserts=`pointer move records old/new/rollback fields and unknown effects block pure-cache eligibility`; evidence=`target/content-addressed-rust-evidence/pointers-and-caveats-validation.json` r[content_addressed_rust_evidence.name_pointer.move] r[content_addressed_rust_evidence.name_pointer.drift] r[content_addressed_rust_evidence.caveats.effects_fail_closed]
- [ ] [serial] V3 Verify pointer/caveat receipt behavior and mutation-manifest determinism. r[content_addressed_rust_evidence.caveats.dependency_status]

## Phase 4: IR and bundle contracts

- [ ] [serial] I5 Add supplemental compiler-IR evidence validation fields for HIR and MIR summaries, with exact rustc/toolchain and extractor identity. r[content_addressed_rust_evidence.compiler_ir]
- [ ] [serial] I6 Add export-bundle receipt validation for sorted object membership, pointer references, bundle hash, replay metadata, and mutation manifests. r[content_addressed_rust_evidence.export_bundle]
- [ ] [parallel] H3 Add IR and bundle fixture receipts. command=`nix run path:$PWD#cairn -- validate --root .`; input=`fixtures/content-addressed-rust-evidence/ir-and-bundles/`; asserts=`HIR dependencies are toolchain-scoped, MIR summaries are compiler-coupled, and bundle membership is replayable`; evidence=`target/content-addressed-rust-evidence/ir-and-bundles-validation.json` r[content_addressed_rust_evidence.compiler_ir.hir_dependencies] r[content_addressed_rust_evidence.compiler_ir.mir_coupled] r[content_addressed_rust_evidence.export_bundle.replay]
- [ ] [serial] V4 Verify compiler-IR and export-bundle receipt behavior with positive and negative fixtures. r[content_addressed_rust_evidence.export_bundle.mutation_manifest]

## Phase 5: Downstream pilots

- [ ] [serial] I7 Add bounded Crunch/Mantle evidence-object pilot validation for checker hash, input evidence hash, and bounded claim class. r[content_addressed_rust_evidence.downstream_pilots.crunch]
- [ ] [serial] I8 Add bounded Aspen CI/job-object pilot validation for config/source/executor identity and replay/cache key material. r[content_addressed_rust_evidence.downstream_pilots.aspen]
- [ ] [parallel] H4 Add downstream pilot fixture receipts. command=`nix run path:$PWD#cairn -- validate --root .`; input=`fixtures/content-addressed-rust-evidence/downstream-pilots/`; asserts=`Crunch and Aspen pilot receipts record identity material and explicit non-claims`; evidence=`target/content-addressed-rust-evidence/downstream-pilots-validation.json` r[content_addressed_rust_evidence.downstream_pilots]
- [ ] [serial] V5 Run full validation, Tracey coverage, and archive readiness after implementation tasks complete. r[content_addressed_rust_evidence.downstream_pilots.crunch] r[content_addressed_rust_evidence.downstream_pilots.aspen]

## Verification Coverage

| Scenario | Verification |
| --- | --- |
| r[content_addressed_rust_evidence.function_object.source_token] | V1, V2 |
| r[content_addressed_rust_evidence.function_object.no_correctness_claim] | V2 |
| r[content_addressed_rust_evidence.canonical_hash.portable] | H1, V2 |
| r[content_addressed_rust_evidence.canonical_hash.no_entropy] | H1, V2 |
| r[content_addressed_rust_evidence.name_pointer.move] | H2, V3 |
| r[content_addressed_rust_evidence.name_pointer.drift] | H2, V3 |
| r[content_addressed_rust_evidence.caveats.dependency_status] | V3 |
| r[content_addressed_rust_evidence.caveats.effects_fail_closed] | H2, V3 |
| r[content_addressed_rust_evidence.compiler_ir.hir_dependencies] | H3, V4 |
| r[content_addressed_rust_evidence.compiler_ir.mir_coupled] | H3, V4 |
| r[content_addressed_rust_evidence.export_bundle.replay] | H3, V4 |
| r[content_addressed_rust_evidence.export_bundle.mutation_manifest] | I3, V4 |
| r[content_addressed_rust_evidence.downstream_pilots.crunch] | I7, H4, V5 |
| r[content_addressed_rust_evidence.downstream_pilots.aspen] | I8, H4, V5 |
