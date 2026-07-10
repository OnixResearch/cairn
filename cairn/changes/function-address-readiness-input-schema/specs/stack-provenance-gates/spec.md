# Stack Provenance Gates Specification

## Purpose

Tighten Cairn function-address readiness input contracts for direct Valence, Mantle, Kamacite, and graph evidence consumption.

## Requirements

### Requirement: Function-address readiness inputs are versioned contracts
r[cairn.stack_provenance_gates.function_address_input_schema.contract] Cairn MUST define versioned input contracts for function-address readiness evidence from Valence, Mantle, Kamacite, and graph summaries.

#### Scenario: Direct Mantle binding receipt is accepted
r[cairn.stack_provenance_gates.function_address_input_schema.mantle_direct]
- GIVEN Mantle emits a versioned function-address release binding receipt with required fields
- WHEN Cairn release-readiness consumes the receipt
- THEN Cairn MUST validate it directly without smoke-specific wrapper glue.

### Requirement: Readiness validation fails closed on malformed evidence
r[cairn.stack_provenance_gates.function_address_input_schema.validation] Cairn MUST reject or mark failed function-address evidence with missing required fields, unsupported versions, malformed BLAKE3 hashes, wrong roles, wrong schemas, unsupported claim scopes, stale receipt identities, or weakened non-claim boundaries.

#### Scenario: Required mode rejects bad input
r[cairn.stack_provenance_gates.function_address_input_schema.negative]
- GIVEN function-address policy mode is required
- WHEN one required readiness input is missing, malformed, stale, or overclaiming
- THEN release readiness MUST fail with a deterministic function-address issue.

### Requirement: Optional mode remains gradual
r[cairn.stack_provenance_gates.function_address_input_schema.positive] Cairn MUST keep optional function-address policy gradual: absent optional evidence is skipped, valid present evidence is checked, and invalid present evidence is reported.

#### Scenario: Full validation suite proves schema behavior
r[cairn.stack_provenance_gates.function_address_input_schema.final_validation]
- GIVEN positive and negative readiness fixtures exercise direct input contracts
- WHEN cargo tests and lifecycle validation run
- THEN Cairn MUST preserve lifecycle-only readiness gating without claiming Rust semantic correctness or release eligibility.
