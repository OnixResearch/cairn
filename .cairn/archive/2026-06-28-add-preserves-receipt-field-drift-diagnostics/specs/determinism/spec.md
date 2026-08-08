# Determinism Specification Delta

## ADDED Requirements

### Requirement: Preserves receipt field drift diagnostics
r[cairn.determinism.preserves_receipt_field_drift.command] Cairn MUST provide a read-only command that maps a Preserves hash delta between two receipt snapshots to sorted receipt field paths and their source requirement IDs.

#### Scenario: Diff command is read-only and deterministic
r[cairn.determinism.preserves_receipt_field_drift.read_only]
- GIVEN two stored receipt JSON snapshots
- WHEN an operator runs `cairn receipt-preserves-diff <baseline.json> <candidate.json>`
- THEN Cairn SHALL emit deterministic JSON with both input hashes, both Preserves hashes, a sorted drift list of field paths and drift classes, policy/policy hashes, and its own `receipt_hash`
- AND it SHALL NOT mutate either input file.

#### Scenario: Source-ID mapping comes from reviewed policy
r[cairn.determinism.preserves_receipt_field_drift.source_id_mapping]
- GIVEN a field path differs between baseline and candidate
- WHEN the drift entry is emitted
- THEN the entry SHALL cite a source requirement ID loaded from generated policy
- AND unmapped paths SHALL be reported with `source_id: null`.

#### Scenario: Drift is ordering-insensitive and byte-aware
r[cairn.determinism.preserves_receipt_field_drift.deterministic]
- GIVEN two receipts that differ only by object key order
- WHEN the diff is computed
- THEN the drift list SHALL be empty for reordered keys
- AND a `canonical_bytes_differ` boolean SHALL reflect the canonical Preserves byte comparison.

#### Scenario: Diagnostics have positive, negative, malformed, and boundary coverage
r[cairn.determinism.preserves_receipt_field_drift.tests]
r[cairn.determinism.preserves_receipt_field_drift.validation]
- GIVEN identical, drifting, malformed, and boundary fixtures
- WHEN focused CLI tests and Cairn lifecycle validation run
- THEN identical inputs yield empty drift, drifting inputs cite correct field paths and source IDs, malformed inputs fail closed, boundary cases (float change, key reordering, scalar/object type change) behave deterministically, Tracey coverage is satisfied, and the determinism replay case passes.
