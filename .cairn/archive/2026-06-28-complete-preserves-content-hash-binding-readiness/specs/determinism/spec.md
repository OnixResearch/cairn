# Determinism Specification Delta

## ADDED Requirements

### Requirement: Preserves content-hash binding readiness
r[cairn.determinism.preserves_content_hash_binding_readiness.binding] Cairn SHALL add `preserves_content_hash` to durable replay, evidence-bundle, and release-evidence receipt outputs that claim receipt content identity while preserving the existing JSON `receipt_hash` field.

#### Scenario: Policy declares the additive hash field
r[cairn.determinism.preserves_content_hash_binding_readiness.policy]
- GIVEN a target receipt command emits `preserves_content_hash`
- WHEN generated policy receipt schemas are inspected
- THEN the command SHALL list `preserves_content_hash` as a top-level key and hash field.

#### Scenario: MCP smoke validates fresh agent receipt evidence
r[cairn.determinism.preserves_content_hash_binding_readiness.mcp_smoke]
- GIVEN the positive agent receipt fixture and current generated policy
- WHEN `cairn agent-receipt smoke-mcp --root .` runs
- THEN fixture replay policy evidence SHALL match current policy and the MCP smoke receipt SHALL pass.

#### Scenario: Tracey coverage remains clean
r[cairn.determinism.preserves_content_hash_binding_readiness.tracey]
- GIVEN the accepted Preserves requirements and implementation references
- WHEN `cairn tracey coverage --root .` runs
- THEN there SHALL be no missing accepted requirements and no dangling implementation references.

#### Scenario: Readiness verification passes
r[cairn.determinism.preserves_content_hash_binding_readiness.validation]
- GIVEN the binding, fixture, and Tracey repairs
- WHEN focused validation runs
- THEN Cairn validation, gates, focused cargo checks, Tracey coverage, MCP smoke, and release-readiness SHALL pass.
