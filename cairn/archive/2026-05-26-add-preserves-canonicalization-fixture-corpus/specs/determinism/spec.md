# Determinism Delta

## MODIFIED Requirements

### Requirement: Preserves receipt canonicalization

Cairn SHALL keep Preserves receipt canonicalization additive and internal until a specific public/stored receipt surface is specified.

#### Scenario: Preserves canonicalization has a checked-in fixture corpus

r[cairn.determinism.preserves_canonicalization_corpus.fixture_corpus]

- GIVEN Cairn depends on Preserves packed encoding for internal deterministic receipt hashing
- WHEN maintainers review the Preserves canonicalization corpus
- THEN the corpus SHALL contain stable case identifiers, JSON values, and expected BLAKE3 hashes of canonical Preserves bytes
- AND the corpus SHALL include primitive, Unicode, array, nested-object, and receipt-shaped cases
- AND the corpus SHALL be checked into source control rather than generated only at test time

#### Scenario: Preserves corpus golden hashes are verified

r[cairn.determinism.preserves_canonicalization_corpus.golden_hashes]

- GIVEN the checked-in Preserves canonicalization corpus
- WHEN focused core tests recompute canonical Preserves hashes
- THEN every case SHALL match its expected hash
- AND object-key ordering equivalent material SHALL produce the same canonical bytes and hash

#### Scenario: Preserves corpus validation is content-sensitive and internal

r[cairn.determinism.preserves_canonicalization_corpus.validation]

- GIVEN Preserves canonicalization remains an internal deterministic evidence seam
- WHEN corpus validation runs
- THEN changed fixture content SHALL produce a different hash from the original case
- AND public/stored JSON receipt schemas and operator CLI output SHALL remain unchanged
