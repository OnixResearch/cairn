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

### Requirement: Preserves receipt canonicalization

Cairn SHALL keep Preserves receipt canonicalization additive and internal until a specific public/stored surface is explicitly promoted.

#### Scenario: Agent-run fixture has an internal Preserves hash consumer

r[cairn.determinism.preserves_receipt_consumer.agent_run_internal_hash]

- GIVEN an agent-run receipt represented as JSON-compatible receipt material
- WHEN Cairn computes its internal Preserves receipt hash
- THEN the hash material SHALL exclude self-referential receipt hash fields
- AND the existing public JSON `receipt_hash` contract SHALL remain unchanged

#### Scenario: Positive agent-run fixture has a golden Preserves hash

r[cairn.determinism.preserves_receipt_consumer.golden_fixture]

- GIVEN the checked-in positive agent-run receipt fixture
- WHEN Cairn computes its internal Preserves receipt hash
- THEN the result SHALL match a stable golden BLAKE3 hex digest

#### Scenario: Consumer Preserves hash is stable and content-sensitive

r[cairn.determinism.preserves_receipt_consumer.stable_hashing]

- GIVEN equivalent agent-run receipt material with different JSON object key ordering
- WHEN Cairn computes internal Preserves receipt hashes
- THEN the hashes SHALL match
- AND a semantic content change SHALL produce a different internal Preserves hash

#### Scenario: Preserves receipt consumer verification is focused

r[cairn.determinism.preserves_receipt_consumer.validation]

- GIVEN the Preserves receipt consumer implementation
- WHEN focused tests and Cairn lifecycle validation run
- THEN the verification SHALL prove the golden fixture, stable hashing, content drift, lifecycle gates, and Tracey references

#### Scenario: Agent receipt Preserves hash CLI emits deterministic inspection receipts

r[cairn.determinism.preserves_hash_cli.command]

- GIVEN a path to a JSON agent-run receipt file
- WHEN an operator runs `cairn agent-receipt preserves-hash <path>`
- THEN Cairn SHALL emit a deterministic JSON inspection receipt containing the internal Preserves hash
- AND the command SHALL NOT mutate the input file

#### Scenario: Preserves hash CLI uses bounded receipt loading

r[cairn.determinism.preserves_hash_cli.bounded_read]

- GIVEN an agent-run receipt file path
- WHEN Cairn loads the file for Preserves hash inspection
- THEN it SHALL use the same bounded read limit as agent receipt validation
- AND the receipt SHALL disclose the bounded byte limit

#### Scenario: Preserves hash CLI does not change public receipt validation

r[cairn.determinism.preserves_hash_cli.no_schema_churn]

- GIVEN existing agent-run receipt validation and public JSON `receipt_hash` fields
- WHEN the Preserves hash inspection CLI is added
- THEN existing validation semantics and public/stored receipt fields SHALL remain unchanged

#### Scenario: Preserves hash CLI has positive and negative verification

r[cairn.determinism.preserves_hash_cli.validation]

- GIVEN the Preserves hash inspection CLI
- WHEN focused tests run
- THEN they SHALL prove the positive fixture golden hash, reproducible receipt output, and structured failure for non-object JSON input

#### Scenario: Agent receipt Preserves hash CLI is policy-owned replay evidence

r[cairn.determinism.preserves_hash_cli.replay_policy]
r[cairn.determinism.preserves_hash_cli.surface_inventory]
r[cairn.determinism.preserves_hash_cli.replay_execution]
r[cairn.determinism.preserves_hash_cli.replay_validation]

- GIVEN the `agent-receipt preserves-hash` operator command exists
- WHEN Cairn exports and validates its deterministic replay policy
- THEN `determinism_replay_policy.cases` SHALL include an `agent_receipt_preserves_hash` case for the checked-in positive agent receipt fixture
- AND the case SHALL be included in the `full` replay group without expanding existing `agent` or `release` required preflight groups
- AND `determinism_surface_inventory` SHALL list the command as a covered CLI surface
- AND `cairn determinism replay --case agent_receipt_preserves_hash` SHALL produce a passing deterministic replay receipt
- AND `cairn determinism coverage-audit` SHALL report the surface as covered

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
