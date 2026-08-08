# Determinism Specification Delta

## ADDED Requirements

### Requirement: Preserves authoritative content hash
r[cairn.determinism.preserves_authoritative_content_hash.field] Receipt material that claims content identity SHALL bind a `preserves_content_hash` computed from canonical Preserves bytes over the receipt content excluding the hash fields, additive alongside the existing JSON `receipt_hash`.

#### Scenario: Content hash is additive and optional
r[cairn.determinism.preserves_authoritative_content_hash.compatibility]
- GIVEN a receipt produced before this change
- WHEN it is validated against current policy
- THEN it SHALL remain valid without a `preserves_content_hash` field
- AND the JSON `receipt_hash` public and stored contract SHALL remain unchanged.

#### Scenario: Content hash is recomputable and tamper-evident
r[cairn.determinism.preserves_authoritative_content_hash.verification]
- GIVEN a receipt with a stored `preserves_content_hash`
- WHEN an operator runs `cairn receipt verify-content-hash <path>`
- THEN Cairn SHALL recompute canonical Preserves bytes over the receipt content excluding the hash fields
- AND SHALL fail closed when the stored field does not match the recomputed hash.

#### Scenario: Canonical content identity is serialization-independent
r[cairn.determinism.preserves_authoritative_content_hash.field]
- GIVEN two receipts whose JSON differs only by object key ordering or float formatting that serde_json would render differently
- WHEN their `preserves_content_hash` values are computed
- THEN the hashes SHALL be equal
- AND the canonical Preserves bytes SHALL be byte-stable across runs and machines.

#### Scenario: Promotion has positive, negative, compatibility, and boundary coverage
r[cairn.determinism.preserves_authoritative_content_hash.tests]
r[cairn.determinism.preserves_authoritative_content_hash.validation]
- GIVEN receipts with and without the content-hash field and tampered variants
- WHEN focused tests and Cairn lifecycle validation run
- THEN present-and-correct fields pass, tampered fields fail closed, pre-change receipts remain valid, serialization-equivalent inputs produce equal hashes, Tracey coverage is satisfied, and the determinism replay case passes.
