# Determinism Specification

## Purpose

Defines the `determinism` capability.

## Requirements

### Requirement: Preserves receipt canonicalization seam

r[cairn.determinism.preserves_receipt_canonicalization.additive_internal_seam]
Cairn MUST provide an additive internal helper for encoding existing receipt-shaped values into Preserves canonical machine bytes without replacing existing JSON receipt surfaces.

#### Scenario: Encode receipt material through the additive seam

r[cairn.determinism.preserves_receipt_canonicalization.additive_internal_seam.encode]
- GIVEN a JSON-compatible receipt value
- WHEN Cairn computes Preserves canonical receipt bytes
- THEN the helper returns deterministic packed bytes
- AND existing JSON receipt schemas and public CLI outputs remain unchanged.

### Requirement: Stable Preserves receipt hashing

r[cairn.determinism.preserves_receipt_canonicalization.stable_hashing]
Cairn MUST compute a BLAKE3 hash from Preserves canonical receipt bytes that is stable for semantically equivalent receipt objects and changes when receipt content changes.

#### Scenario: Equivalent object order has the same hash

r[cairn.determinism.preserves_receipt_canonicalization.stable_hashing.object_order]
- GIVEN two receipt objects with the same keys and values in different source orders
- WHEN Cairn computes their Preserves canonical receipt hashes
- THEN both hashes are identical.

#### Scenario: Changed receipt content has a different hash

r[cairn.determinism.preserves_receipt_canonicalization.stable_hashing.content_change]
- GIVEN two receipt objects with different receipt content
- WHEN Cairn computes their Preserves canonical receipt hashes
- THEN the hashes differ.

### Requirement: Focused validation evidence

r[cairn.determinism.preserves_receipt_canonicalization.validation]
Cairn MUST verify the Preserves receipt canonicalization seam with focused unit tests and Cairn lifecycle gates before landing the spike.

#### Scenario: Validate the spike before landing

r[cairn.determinism.preserves_receipt_canonicalization.validation.focused]
- GIVEN the Preserves receipt canonicalization change package
- WHEN focused Rust tests and Cairn gates run
- THEN they pass with deterministic evidence.
