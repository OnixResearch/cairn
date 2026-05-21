# Electrical Circuit Primitive Receipt Validation Specification

## Purpose

Defines deterministic local validation for `electrical.circuit-primitive-graph.v1` receipt artifacts without simulator, EDA-tool, host-path, or ambient-entropy trust.

## Requirements

### Requirement: Circuit primitive receipt schema
r[electrical_circuit_primitive_receipts.validator] `electrical.circuit-primitive-graph.v1` receipts MUST be validated by a deterministic Rust validator.

#### Scenario: Schema, topology, values, hashes, and non-claims are enforced
r[electrical_circuit_primitive_receipts.validator.core]

- GIVEN a JSON artifact submitted as a circuit primitive receipt
- WHEN the artifact is validated
- THEN the validator checks schema, reference-node membership, terminal-to-net bindings, dimensioned passive values, graph hash material, source artifact hash shape, and required non-claims.

### Requirement: CLI validation command
r[electrical_circuit_primitive_receipts.validator_cli] Cairn MUST expose `cairn evidence circuit-primitive validate <receipt.json>` for bounded local receipt validation.

#### Scenario: CLI emits deterministic report
r[electrical_circuit_primitive_receipts.validator.cli]

- GIVEN a bounded local circuit primitive receipt JSON file
- WHEN the CLI validates the receipt
- THEN it emits JSON containing validity, verdict, issues, computed graph hash, and a report receipt hash, and exits non-zero for invalid receipts.

### Requirement: Focused fixture coverage
r[electrical_circuit_primitive_receipts.fixtures] Circuit primitive receipt support MUST include positive and negative fixtures plus focused core and CLI tests.

#### Scenario: Positive fixture covers passive RC topology
r[electrical_circuit_primitive_receipts.fixtures.positive]

- GIVEN a valid passive two-terminal RC circuit primitive graph fixture
- WHEN the circuit primitive validator runs
- THEN the fixture validates successfully with no issues and stable hash fields.

#### Scenario: Negative fixtures prove fail-closed behavior
r[electrical_circuit_primitive_receipts.fixtures.negative]

- GIVEN fixtures with missing required terminal bindings or omitted safety/compliance non-claims
- WHEN the circuit primitive validator runs
- THEN the validator reports deterministic issue codes and rejects the receipt.

### Requirement: Policy registration
r[electrical_circuit_primitive_receipts.policy] The generated Cairn policy MUST list the circuit primitive receipt family with schema, command, hash field, hash material, and required non-claims.

#### Scenario: Policy consumers discover receipt family
r[electrical_circuit_primitive_receipts.policy.family]

- GIVEN the Cairn policy source includes reviewed receipt families
- WHEN policy is exported
- THEN the content-addressed evidence families include `circuit-primitive` with schema, validate command, graph hash field, canonical hash material description, and explicit circuit non-claims.
