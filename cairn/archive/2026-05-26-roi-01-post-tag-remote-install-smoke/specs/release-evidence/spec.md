# Release Evidence Specification Delta

## ADDED Requirements

### Requirement: Post-tag remote install smoke
r[release_evidence.post_tag_remote_install_smoke.build] The system MUST support a post-tag smoke workflow that verifies the tagged remote flake package can build outside the local checkout.

#### Scenario: Remote tag package builds
r[release_evidence.post_tag_remote_install_smoke.lifecycle]
- GIVEN a published Cairn release tag
- WHEN an operator builds `github:OnixResearch/cairn/<tag>#cairn` and runs the resulting binary against a temporary lifecycle root
- THEN `init`, `status`, and `validate` SHALL complete successfully without requiring repository-local target artifacts.

#### Scenario: Smoke evidence stays bounded
r[release_evidence.post_tag_remote_install_smoke.docs]
r[release_evidence.post_tag_remote_install_smoke.validation]
- GIVEN the post-tag smoke completes
- WHEN the operator records evidence
- THEN the report SHALL name the tag, commands, verdict, and non-claims without preserving credentials, absolute temp paths, or claiming remote production readiness.
