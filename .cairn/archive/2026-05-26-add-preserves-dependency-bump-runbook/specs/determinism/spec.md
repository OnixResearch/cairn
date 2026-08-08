# Determinism Specification Delta

## ADDED Requirements

### Requirement: Preserves dependency bumps have a maintainer runbook
r[cairn.determinism.preserves_dependency_bump_runbook] Cairn MUST document the maintainer workflow for reviewing Preserves dependency bumps that may affect canonical receipt hashes.

#### Scenario: Runbook starts with fail-closed drift detection
r[cairn.determinism.preserves_dependency_bump_runbook.workflow]
- GIVEN a maintainer changes the `preserves` dependency or related serde conversion behavior
- WHEN they follow the Preserves dependency-bump runbook
- THEN the workflow SHALL run the dry-run corpus update rail and CI drift guards before any corpus mutation
- AND the no-drift path SHALL preserve checked-in golden hashes unchanged

#### Scenario: Intentional drift is explicit and reviewable
r[cairn.determinism.preserves_dependency_bump_runbook.intentional_drift]
- GIVEN Preserves canonical bytes intentionally change after review
- WHEN the maintainer refreshes corpus golden hashes
- THEN the runbook SHALL require `--execute`, review of the corpus diff, focused Preserves checks, workspace verification, and no public/stored JSON receipt schema change

#### Scenario: Dependency bump runbook is discoverable
r[cairn.determinism.preserves_dependency_bump_runbook.discoverability]
- GIVEN maintainers already consult README and Preserves receipt workflow docs
- WHEN the dependency-bump runbook is added
- THEN those surfaces SHALL link to the runbook

#### Scenario: Dependency bump runbook is validated
r[cairn.determinism.preserves_dependency_bump_runbook.validation]
- GIVEN the Preserves dependency-bump runbook
- WHEN Cairn validates lifecycle specs, Tracey coverage, and focused Preserves checks
- THEN the runbook contract SHALL have durable implementation and verification references
