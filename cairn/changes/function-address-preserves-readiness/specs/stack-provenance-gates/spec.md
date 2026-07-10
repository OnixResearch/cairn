# Stack Provenance Gates Specification

## Purpose

Gate function-address readiness over canonical Kamacite Preserves receipt identity.

## Requirements

### Requirement: Preserves receipt identity is a first-class readiness input
r[cairn.stack_provenance_gates.function_address_preserves_readiness.contract] Cairn MUST support function-address readiness inputs that include canonical Kamacite Preserves receipt hash, schema version, Valence receipt linkage, Mantle binding linkage, roles, claim scope, and non-claims.

#### Scenario: Required Preserves-backed evidence passes
r[cairn.stack_provenance_gates.function_address_preserves_readiness.positive]
- GIVEN function-address policy mode is required and complete Valence, Mantle, Kamacite Preserves, and graph inputs are valid
- WHEN Cairn release-readiness runs
- THEN readiness MUST pass the function-address gate with a deterministic receipt hash.

### Requirement: Readiness core validates linkage without parsing functions
r[cairn.stack_provenance_gates.function_address_preserves_readiness.core] Cairn MUST validate Preserves-backed function-address readiness through typed linkage, role, claim-scope, schema, digest, and non-claim checks without parsing Rust functions or Preserves function records in the readiness core.

#### Scenario: Core stays lifecycle-only
r[cairn.stack_provenance_gates.function_address_preserves_readiness.validation]
- GIVEN Cairn evaluates Preserves-backed function-address evidence
- WHEN the readiness gate passes
- THEN the result MUST claim only lifecycle policy conformance and MUST NOT claim Rust semantic correctness, compiler correctness, whole-program safety, verifier soundness, or release eligibility.

### Requirement: JSON wrappers are compatibility-only
r[cairn.stack_provenance_gates.function_address_preserves_readiness.json_projection] Cairn MAY accept JSON compatibility wrappers only when they declare non-authoritative status and bind to the canonical Kamacite Preserves receipt identity.

#### Scenario: Invalid Preserves-backed evidence fails closed
r[cairn.stack_provenance_gates.function_address_preserves_readiness.negative]
- GIVEN function-address readiness input has missing canonical Preserves hash, stale Valence or Mantle linkage, unsupported schema, malformed digest, projection drift, wrong claim scope, role mismatch, or overclaiming text
- WHEN Cairn evaluates the gate
- THEN readiness MUST fail or report invalid present evidence with deterministic function-address issues.
