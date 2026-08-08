# Determinism Delta

## MODIFIED Requirements

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
