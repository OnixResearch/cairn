# Determinism Delta

## MODIFIED Requirements

### Requirement: Preserves receipt canonicalization

Cairn SHALL keep Preserves receipt canonicalization additive and internal until a specific public/stored surface is explicitly promoted.

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
