## MODIFIED Requirements

### Requirement: Steel lifecycle orchestration [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration]]
The system MUST support optional Steel-based lifecycle orchestration that composes existing Cairn lifecycle primitives without moving trusted validation, policy enforcement, effect execution, or receipt hashing out of Rust/Nickel-owned surfaces.

#### Scenario: Steel orchestration CLI is documented for operators [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.operator_docs]]
- GIVEN an operator reviews Cairn's public Steel orchestration documentation
- WHEN they inspect the `steel-orchestration receipt` and `steel-orchestration default-gate` examples
- THEN the documentation MUST show the disabled-by-default receipt behavior, default-gate failure under the default disabled policy, and selected-default gate shape with complete evidence hashes
- AND the documentation MUST state that Steel orchestration receipts are deterministic, redacted, and do not prove host effects executed
