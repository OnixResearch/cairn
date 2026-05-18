## MODIFIED Requirements

### Requirement: Cairn validation parity
r[cairn.supersede.validate_ears] The system MUST validate OpenSpec-compatible requirement markdown for EARS-style testability without requiring the upstream OpenSpec CLI.

#### Scenario: Accept structured requirements
r[cairn.supersede.validate_ears.accept]
- GIVEN an OpenSpec-compatible repository whose accepted specs and active delta specs contain requirements with normative strength words
- AND each scenario contains GIVEN, WHEN, and THEN clauses
- WHEN the operator runs `cairn validate --root <path>`
- THEN the CLI emits a deterministic JSON receipt with `valid=true`
- AND the spec issue collection is empty

#### Scenario: Reject weak requirement prose
r[cairn.supersede.validate_ears.weak_requirement]
- GIVEN an OpenSpec-compatible spec requirement without a MUST, SHALL, SHOULD, or MAY strength word
- WHEN the operator runs `cairn validate --root <path>`
- THEN the CLI emits a deterministic JSON receipt with `valid=false`
- AND the issue collection identifies the weak requirement location

#### Scenario: Reject incomplete EARS scenario
r[cairn.supersede.validate_ears.incomplete_scenario]
- GIVEN an OpenSpec-compatible spec scenario missing a GIVEN, WHEN, or THEN clause
- WHEN the operator runs `cairn validate --root <path>`
- THEN the CLI emits a deterministic JSON receipt with `valid=false`
- AND the issue collection identifies the incomplete scenario location

#### Scenario: Preserve FCIS validation boundary
r[cairn.supersede.validate_ears.boundary]
- GIVEN EARS-style validation support
- WHEN `cairn-core` evaluates requirement and scenario quality
- THEN it consumes only caller-provided markdown strings and performs no filesystem, environment, process, time, stdout/stderr, or JSON presentation effects
