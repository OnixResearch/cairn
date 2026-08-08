## ADDED Requirements

### Requirement: Octet Evidence File Format Contract

r[content_addressed_rust_evidence.file_formats] The system MUST distinguish human review artifacts from machine receipt artifacts for Octet content-addressed Rust evidence and define the supported format role for each artifact class.

#### Scenario: Markdown remains human review surface

r[content_addressed_rust_evidence.file_formats.markdown_review]

- GIVEN a Cairn or Octet change package, design note, task list, or evidence narrative for content-addressed Rust evidence
- WHEN the artifact is intended for human review and lifecycle context
- THEN Markdown MUST remain an accepted review format
- AND the artifact MUST state caveats in prose when it discusses object identity, determinism, or replay boundaries.

#### Scenario: JSON remains current machine receipt surface

r[content_addressed_rust_evidence.file_formats.json_receipts]

- GIVEN an Octet function object, object bundle, corpus receipt, corpus check report, or deterministic invalid-input report
- WHEN the artifact is emitted for machine replay, CI gating, fixture comparison, or bundle inclusion
- THEN deterministic JSON MUST remain an accepted v1 receipt encoding
- AND the receipt MUST carry explicit schema/version identity and deterministic hash boundaries.

### Requirement: Receipt Contract Ownership

r[content_addressed_rust_evidence.receipt_contract_ownership] The system MUST treat emitted Octet evidence receipt compatibility as Rust DTO-owned runtime behavior with generated or registered policy contract metadata, not as an ad hoc file-extension convention.

#### Scenario: Rust DTO owns emitted receipt shape

r[content_addressed_rust_evidence.receipt_contract_ownership.rust_dto]

- GIVEN an emitted Octet evidence receipt family that is consumed by Cairn, CI, or downstream reviewers
- WHEN the receipt shape is validated for compatibility
- THEN the Rust DTO and serializer behavior MUST be the runtime source of truth
- AND compatibility changes MUST be versioned rather than inferred only from `.json` file names.

#### Scenario: Nickel metadata tracks compatibility policy

r[content_addressed_rust_evidence.receipt_contract_ownership.nickel_metadata]

- GIVEN an Octet evidence receipt family that becomes compatibility-relevant across tools
- WHEN Cairn policy describes the family
- THEN the receipt family MUST be registered as Rust-derived or otherwise explicitly classified in Nickel contract metadata
- AND hand-authored JSON schema MUST NOT become the normative compatibility source when it can drift from the Rust DTO.

### Requirement: Canonical Serialization and Future Formats

r[content_addressed_rust_evidence.canonical_serialization_formats] The system MUST preserve deterministic canonical serialization rules for current JSON receipts while allowing future additive encodings only when they retain schema identity, hash boundaries, and reviewable summaries.

#### Scenario: JSON hash material is canonical and bounded

r[content_addressed_rust_evidence.canonical_serialization_formats.json_hashing]

- GIVEN a deterministic JSON Octet receipt with object, corpus, bundle, or check hash fields
- WHEN hash material is computed or validated
- THEN self-referential hash fields and ambient host entropy MUST be excluded from hash material
- AND sorted deterministic content MUST be used for portable evidence hashes.

#### Scenario: Future binary format is additive

r[content_addressed_rust_evidence.canonical_serialization_formats.future_binary]

- GIVEN a future canonical binary or packed evidence bundle format for Octet receipts
- WHEN the format is introduced
- THEN it MUST be additive to the v1 JSON review surface unless a later migration change explicitly replaces that surface
- AND it MUST provide schema identity, compatibility rules, deterministic hashes, and human-reviewable summaries.

### Requirement: File Format Non-Claims

r[content_addressed_rust_evidence.file_format_non_claims] The system MUST require Octet/Cairn file-format receipts to preserve explicit evidence-only boundaries and reject correctness or semantic-equivalence claims derived only from artifact format or hash agreement.

#### Scenario: File format does not prove correctness

r[content_addressed_rust_evidence.file_format_non_claims.no_correctness]

- GIVEN a Markdown narrative, JSON receipt, or future binary bundle whose hashes validate
- WHEN Cairn or a downstream consumer reports the artifact status
- THEN the report MUST state or preserve that hash agreement is evidence/provenance identity, not a behavioral correctness proof.

#### Scenario: File format does not prove semantic equivalence

r[content_addressed_rust_evidence.file_format_non_claims.no_semantic_equivalence]

- GIVEN two receipts or bundles that share matching object/corpus hash fields
- WHEN the artifacts are compared
- THEN the comparison MUST NOT claim Rust semantic equivalence solely from matching file-format encodings or hashes.
