## ADDED Requirements

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
