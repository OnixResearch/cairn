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
