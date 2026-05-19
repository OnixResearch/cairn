## Phase 1: Contract Package

- [x] [serial] I1 Define the file-format proposal/design/spec package for Octet content-addressed Rust evidence. r[content_addressed_rust_evidence.file_formats]
- [x] [serial] V1 Run Cairn validation and proposal/design/tasks gates for this change package. r[content_addressed_rust_evidence.file_formats.markdown_review] r[content_addressed_rust_evidence.file_formats.json_receipts]

## Phase 2: Future Implementation

- [ ] [serial] I2 Register compatibility-relevant Octet receipt families as Rust-derived or explicitly classified policy contract metadata when Cairn consumes them directly. r[content_addressed_rust_evidence.receipt_contract_ownership.nickel_metadata]
- [ ] [serial] I3 Add fixture-backed validators or import gates only after Octet stabilizes a receipt family as a Cairn-consumed contract. r[content_addressed_rust_evidence.receipt_contract_ownership.rust_dto]
- [ ] [parallel] H1 Add deterministic evidence showing JSON hash material excludes self-referential fields and ambient host entropy for each Cairn-consumed Octet receipt family. command=`nix run path:$PWD#cairn -- validate --root .`; input=`fixtures/content-addressed-rust-evidence/file-formats/`; asserts=`JSON receipt hash material is canonical and bounded`; evidence=`target/content-addressed-rust-evidence/file-format-contracts.json` r[content_addressed_rust_evidence.canonical_serialization_formats.json_hashing]
- [ ] [parallel] H2 Add future-format migration evidence only when a binary or packed encoding is introduced. command=`nix run path:$PWD#cairn -- validate --root .`; input=`fixtures/content-addressed-rust-evidence/future-formats/`; asserts=`future format remains additive or has an explicit migration change`; evidence=`target/content-addressed-rust-evidence/future-format-contracts.json` r[content_addressed_rust_evidence.canonical_serialization_formats.future_binary]

## Phase 3: Boundary Verification

- [ ] [serial] V2 Verify that any Cairn-consumed Octet receipt validator preserves no-correctness and no-semantic-equivalence boundary fields. r[content_addressed_rust_evidence.file_format_non_claims.no_correctness] r[content_addressed_rust_evidence.file_format_non_claims.no_semantic_equivalence]

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
