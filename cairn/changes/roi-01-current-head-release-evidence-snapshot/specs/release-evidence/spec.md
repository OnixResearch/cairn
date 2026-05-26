# Release Evidence Specification Delta

## ADDED Requirements

### Requirement: Current-HEAD release evidence snapshot
r[release_evidence.current_head_snapshot.generate] The system MUST support generating a local current-HEAD release evidence snapshot using existing release-evidence rails.

#### Scenario: Generate current-head evidence into target artifacts
r[release_evidence.current_head_snapshot.check_index]
- GIVEN a clean synced Cairn checkout
- WHEN the operator generates a release-evidence index into `target/release-evidence-current-head/`
- THEN the generated index SHALL bind the live HEAD and local artifacts without committing generated receipts
- AND the existing check-index rail SHALL validate the index in current-head mode.

#### Scenario: Snapshot verification is durable and non-self-referential
r[release_evidence.current_head_snapshot.validation]
r[release_evidence.current_head_snapshot.docs_boundary]
- GIVEN current-HEAD evidence changes after any subsequent commit
- WHEN the operator records or reports the snapshot
- THEN the evidence SHALL distinguish verified code commit from evidence-recording commit
- AND release-readiness, Cairn validation, Tracey coverage, and diff checks SHALL pass.
