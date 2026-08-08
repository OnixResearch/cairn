## ADDED Requirements

### Requirement: Evidence Bundle Verify Command

Cairn MUST provide a first-class evidence bundle verification command that validates a directory produced by `cairn evidence export`. r[cairn.supersede.evidence_bundle_verify_command.command]

#### Scenario: Verify exported bundle

Given a directory produced by `cairn evidence export --root . --out <dir>`
When an operator runs `cairn evidence verify --root . --bundle <dir>`
Then Cairn MUST emit a deterministic verification receipt with `valid=true`, an empty `issues` array, the exported artifact roles, the export receipt hash, manifest hash, manifest verification receipt hash, and a receipt hash.

### Requirement: Evidence Bundle Artifact Verification

Cairn MUST rehash exported artifacts, reject unsafe artifact paths, require canonical exported roles, and compose existing evidence validators. r[cairn.supersede.evidence_bundle_verify_command.artifacts]

#### Scenario: Artifact hashes and nested verifiers match

Given an exported evidence bundle
When Cairn verifies the bundle
Then every declared artifact hash MUST match the file content
And required artifact roles MUST be present exactly once
And replay verification, bundle verification, manifest verification, coverage audit, and release-readiness receipts MUST be valid.

### Requirement: Evidence Bundle Verify Failure Semantics and Documentation

Evidence bundle verification failures MUST be deterministic, path-safe, secret-safe, and documented as the preferred consumer-side workflow. r[cairn.supersede.evidence_bundle_verify_command.failures]

#### Scenario: Safe failure receipt

Given an evidence bundle with a missing or modified artifact
When Cairn verifies the bundle
Then Cairn MUST emit `valid=false`
And the receipt MUST include stable issue codes without leaking absolute bundle paths or secret values.

#### Scenario: Consumer documentation

Given the export/verify workflow is available
When an agent or CI needs to publish and consume evidence
Then the docs MUST describe `cairn evidence export --root . --out <dir>` for producers and `cairn evidence verify --root . --bundle <dir>` for reviewers or CI consumers.

r[cairn.supersede.evidence_bundle_verify_command.command]
r[cairn.supersede.evidence_bundle_verify_command.artifacts]
r[cairn.supersede.evidence_bundle_verify_command.failures]
r[cairn.supersede.evidence_bundle_verify_command.docs]
r[cairn.supersede.evidence_bundle_verify_command.gates]
r[cairn.supersede.evidence_bundle_verify_command.archive]
