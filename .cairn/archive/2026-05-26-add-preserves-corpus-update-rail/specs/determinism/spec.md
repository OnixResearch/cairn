# Determinism Delta

## MODIFIED Requirements

### Requirement: Preserves receipt canonicalization

Cairn SHALL provide a narrow internal Preserves canonicalization seam for deterministic receipt-shaped artifacts without replacing public JSON receipt/config surfaces.

#### Scenario: Preserves corpus has an explicit update rail

r[cairn.determinism.preserves_corpus_update_rail.command]

- GIVEN the checked-in Preserves canonicalization corpus
- WHEN an operator runs the corpus update rail without an execute flag
- THEN Cairn SHALL recompute expected hashes and report stale cases without mutating the corpus file.

#### Scenario: Preserves corpus update rail recomputes hashes through the core seam

r[cairn.determinism.preserves_corpus_update_rail.hash_recompute]

- GIVEN a corpus case with JSON `value` material
- WHEN the update rail evaluates the case
- THEN Cairn SHALL compute the replacement `expected_hash` through the existing internal Preserves canonical hash helper.

#### Scenario: Preserves corpus update rail emits deterministic receipts

r[cairn.determinism.preserves_corpus_update_rail.receipt]

- GIVEN valid or malformed corpus input
- WHEN the update rail completes
- THEN Cairn SHALL emit deterministic receipt fields for case count, stale count, mutation status, case summaries, and issue codes.

#### Scenario: Preserves corpus update rail validates stale and malformed cases

r[cairn.determinism.preserves_corpus_update_rail.validation]

- GIVEN focused CLI tests for stale, clean, update, and malformed corpus files
- WHEN Cairn runs the Preserves corpus update validation rail
- THEN stale dry-runs SHALL not mutate, explicit updates SHALL repair expected hashes, clean corpora SHALL remain no-op, and malformed corpora SHALL fail closed.
