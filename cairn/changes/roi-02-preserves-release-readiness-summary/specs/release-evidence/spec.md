# Release Evidence Specification Delta

## ADDED Requirements

### Requirement: Release-readiness summarizes Preserves evidence
r[release_evidence.preserves_readiness_summary.output] The system MUST summarize internal Preserves evidence readiness in release-readiness receipts without replacing JSON public receipt semantics.

#### Scenario: Preserves readiness is reported without mutation
r[release_evidence.preserves_readiness_summary.fail_closed]
- GIVEN release-readiness runs in a Cairn checkout with Preserves corpus fixtures
- WHEN it evaluates Preserves readiness
- THEN it SHALL compose existing non-mutating rails and SHALL NOT run corpus update with `--execute`
- AND it SHALL emit deterministic issues when freshness cannot be proven.

#### Scenario: Preserves readiness summary is verified
r[release_evidence.preserves_readiness_summary.tests]
r[release_evidence.preserves_readiness_summary.validation]
- GIVEN the release-readiness Preserves summary
- WHEN focused tests and lifecycle validation run
- THEN schema expectations, failure issues, Cairn validation, and Tracey references SHALL pass.
