## Phase 1: Receipt validator implementation

- [x] [serial] I1 Add a pure Rust validator for `electrical.circuit-primitive-graph.v1` receipts. r[electrical_circuit_primitive_receipts.validator.core]
- [x] [serial] I2 Add the `cairn evidence circuit-primitive validate <receipt.json>` CLI shell. r[electrical_circuit_primitive_receipts.validator.cli]
- [x] [parallel] H1 Add positive and negative circuit primitive receipt fixtures. r[electrical_circuit_primitive_receipts.fixtures.positive] r[electrical_circuit_primitive_receipts.fixtures.negative]
- [x] [serial] I3 Register the circuit primitive receipt family in generated Cairn policy metadata. r[electrical_circuit_primitive_receipts.policy.family]
- [x] [serial] V1 Run focused validator, CLI fixture, policy freshness, Cairn validation, Tracey, Clippy, and diff checks. r[electrical_circuit_primitive_receipts.validator.core] r[electrical_circuit_primitive_receipts.validator.cli] r[electrical_circuit_primitive_receipts.fixtures.negative] r[electrical_circuit_primitive_receipts.policy.family]

## Verification Coverage

| Scenario | Verification |
| --- | --- |
| r[electrical_circuit_primitive_receipts.validator.core] | V1 |
| r[electrical_circuit_primitive_receipts.validator.cli] | V1 |
| r[electrical_circuit_primitive_receipts.fixtures.positive] | V1 |
| r[electrical_circuit_primitive_receipts.fixtures.negative] | V1 |
| r[electrical_circuit_primitive_receipts.policy.family] | V1 |
