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

### Requirement: Split circuit primitive content-addressed core validation uses focused boundaries

r[electrical_circuit_primitive_receipts.circuit_primitive_core_split.focused_modules] The system MUST address `crates/cairn-core/src/verified/content_addressed/circuit_primitive.rs` (~566 lines) with focused boundaries that reduce review surface while preserving existing behavior.

#### Scenario: Focused boundaries are visible

r[electrical_circuit_primitive_receipts.circuit_primitive_core_split.focused_modules.visible_boundaries]
- GIVEN the ROI slice is implemented
- WHEN a reviewer inspects the affected source, tests, docs, or evidence
- THEN topology, component, unit, hash, issue construction, and test-fixture validation logic are separated into named pure modules
- AND the original target no longer remains the sole large review surface for that family

#### Scenario: Bridge APIs stay narrow

r[electrical_circuit_primitive_receipts.circuit_primitive_core_split.focused_modules.narrow_bridges]
- GIVEN another module or operator workflow still consumes the affected behavior
- WHEN the implementation exposes cross-module functions, shared helpers, or evidence pointers
- THEN only the narrow bridge surface needed by those consumers is exposed
- AND broad crate-root shims, unrelated helper moves, and tag or receipt churn are avoided

### Requirement: Split circuit primitive content-addressed core validation remains behavior preserving

r[electrical_circuit_primitive_receipts.circuit_primitive_core_split.behavior_preserved] The system MUST preserve existing public behavior, deterministic evidence, fixture expectations, and issue codes while draining this ROI slice.

#### Scenario: Focused checks pass

r[electrical_circuit_primitive_receipts.circuit_primitive_core_split.behavior_preserved.focused_checks]
- GIVEN the ROI slice is implemented
- WHEN the operator runs content-addressed circuit primitive positive and negative validation tests plus core purity smoke
- THEN the focused checks pass without intentional receipt, issue-code, policy, fixture, or hash drift

#### Scenario: Repository checks pass

r[electrical_circuit_primitive_receipts.circuit_primitive_core_split.behavior_preserved.repository_checks]
- GIVEN the ROI slice is implemented
- WHEN the operator runs native Cairn validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the repository remains ready for commit
