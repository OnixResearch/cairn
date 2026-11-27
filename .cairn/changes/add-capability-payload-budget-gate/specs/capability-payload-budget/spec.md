# Capability payload budget gate requirements

## ADDED Requirements

### Requirement: Default-off capability-payload budget gate for cross-repo plumbing changes

`capability_payload_budget` SHALL default to disabled (`enabled = false`) and SHALL impose zero additional required fields on single-repo changes that do not declare cross-repo dependencies or cross-repo receipt schemas. When opted in by repository policy, `cairn validate` and `cairn gate proposal` SHALL verify that multi-repo or cross-repo plumbing changes declare a concrete downstream capability payload and respect `max_cross_repo_targets` (unless an explicit scope justification is provided).

#### Scenario: Multi-repo plumbing change without a named capability payload is flagged

- **GIVEN** a repository policy with `capability_payload_budget.enabled = true`, `max_cross_repo_targets = 2`, and `enforcement = "error"`
- **WHEN** `cairn validate` evaluates a change declaring 3 cross-repo targets without a named downstream capability payload or scope justification
- **THEN** validation SHALL fail closed with a structured `capability_payload_budget` diagnostic identifying the missing payload and exceeded cross-repo target count

#### Scenario: Single-repo change incurs zero ceremony overhead

- **GIVEN** the same repository policy with `capability_payload_budget.enabled = true`
- **WHEN** `cairn validate` evaluates an ordinary single-repo change with no cross-repo dependency or receipt-schema declarations
- **THEN** the `capability_payload_budget` check SHALL pass cleanly without requiring extra proposal sections
