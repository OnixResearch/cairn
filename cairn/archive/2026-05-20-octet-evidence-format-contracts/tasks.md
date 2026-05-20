## Phase 1: Contract Package

- [x] [serial] I1 Define the file-format proposal/design/spec package for Octet content-addressed Rust evidence. r[content_addressed_rust_evidence.file_formats]
- [x] [serial] V1 Run Cairn validation and proposal/design/tasks gates for this change package. r[content_addressed_rust_evidence.file_formats.markdown_review] r[content_addressed_rust_evidence.file_formats.json_receipts]

## Phase 2: Future Implementation

- [x] [serial] I2 Register compatibility-relevant Octet receipt families as Rust-derived or explicitly classified policy contract metadata when Cairn consumes them directly. command=`nix develop -c cargo run -p cairn-cli --bin cairn -- policy export --check`; input=`cairn-policy/default.ncl`; asserts=`function-object, name-pointer, export-bundle, and downstream-pilot are registered as rust-dto-derived deterministic-json-v1 Cairn-consumed Octet receipt contracts`; evidence=`target/content-addressed-rust-evidence/file-format-contracts.json` r[content_addressed_rust_evidence.receipt_contract_ownership.nickel_metadata]
- [x] [serial] I3 Add fixture-backed validators or import gates only after Octet stabilizes a receipt family as a Cairn-consumed contract. command=`nix run path:$PWD#cairn -- evidence downstream-pilot validate fixtures/content-addressed-rust-evidence/downstream-pilots/crunch-mantle-positive.json`; input=`fixtures/content-addressed-rust-evidence/`; asserts=`Cairn-consumed receipt families use Rust validators and fixture-backed gates rather than hand-authored JSON schema as the runtime source`; evidence=`target/content-addressed-rust-evidence/file-format-contracts.json` r[content_addressed_rust_evidence.receipt_contract_ownership.rust_dto]
- [x] [parallel] H1 Add deterministic evidence showing JSON hash material excludes self-referential fields and ambient host entropy for each Cairn-consumed Octet receipt family. command=`nix run path:$PWD#cairn -- validate --root .`; input=`fixtures/content-addressed-rust-evidence/`; asserts=`JSON receipt hash material is canonical and bounded for function-object, name-pointer, export-bundle, and downstream-pilot receipts`; evidence=`target/content-addressed-rust-evidence/file-format-contracts.json` r[content_addressed_rust_evidence.canonical_serialization_formats.json_hashing]
- [x] [parallel] H2 Add future-format migration evidence only when a binary or packed encoding is introduced. command=`nix run path:$PWD#cairn -- validate --root .`; input=`cairn/changes/octet-evidence-format-contracts/specs/content-addressed-rust-evidence/spec.md`; asserts=`no binary/packed format exists now; any future format remains additive or requires an explicit migration change`; evidence=`target/content-addressed-rust-evidence/future-format-contracts.json` r[content_addressed_rust_evidence.canonical_serialization_formats.future_binary]

## Phase 3: Boundary Verification

- [x] [serial] V2 Verify that any Cairn-consumed Octet receipt validator preserves no-correctness and no-semantic-equivalence boundary fields. command=`nix develop -c cargo test -p cairn-cli evidence_validator -- --nocapture`; asserts=`receipt validators preserve evidence-only boundaries and reject correctness/semantic/distributed overclaims`; evidence=`target/content-addressed-rust-evidence/file-format-contracts.json` r[content_addressed_rust_evidence.file_format_non_claims.no_correctness] r[content_addressed_rust_evidence.file_format_non_claims.no_semantic_equivalence]

## Verification Coverage

| Scenario | Verification |
| --- | --- |
| r[content_addressed_rust_evidence.file_formats.markdown_review] | V1 |
| r[content_addressed_rust_evidence.file_formats.json_receipts] | V1 |
| r[content_addressed_rust_evidence.receipt_contract_ownership.rust_dto] | I3 |
| r[content_addressed_rust_evidence.receipt_contract_ownership.nickel_metadata] | I2 |
| r[content_addressed_rust_evidence.canonical_serialization_formats.json_hashing] | H1 |
| r[content_addressed_rust_evidence.canonical_serialization_formats.future_binary] | H2 |
| r[content_addressed_rust_evidence.file_format_non_claims.no_correctness] | V2 |
| r[content_addressed_rust_evidence.file_format_non_claims.no_semantic_equivalence] | V2 |
