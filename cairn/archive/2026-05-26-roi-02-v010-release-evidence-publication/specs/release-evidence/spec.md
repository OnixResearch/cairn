# Release Evidence Specification Delta

## ADDED Requirements

### Requirement: v0.1.0 release evidence publication snapshot
r[release_evidence.v010_publication.index] The system MUST support generating and verifying a local release-evidence index for the v0.1.0 release payload without mutating the release tag.

#### Scenario: v0.1.0 evidence index is verified
r[release_evidence.v010_publication.publish]
- GIVEN a clean checkout at the v0.1.0 payload commit
- WHEN the operator generates, verifies, and materializes release evidence
- THEN the index and publication receipts SHALL be valid and bind to the evaluated payload commit.

#### Scenario: v0.1.0 evidence summary is auditable
r[release_evidence.v010_publication.docs]
r[release_evidence.v010_publication.validation]
- GIVEN generated release evidence artifacts exist under `target/`
- WHEN the operator records checked-in evidence
- THEN only a compact audited summary SHALL be committed and it SHALL distinguish local/trusted evidence from remote publication, signature, credential, or production-readiness claims.
