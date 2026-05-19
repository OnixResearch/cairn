## ADDED Requirements

### Requirement: Evidence Manifest Generation Command r[cairn.supersede.evidence_manifest_generate.cli]

Cairn SHALL provide a deterministic `cairn evidence-manifest generate` command for producing manifest JSON from role-specific evidence artifacts.

#### Scenario: Command writes canonical manifest JSON

r[cairn.supersede.evidence_manifest_generate.cli]

- GIVEN valid paths for an agent evidence bundle, agent receipt, replay receipt, and replay verification receipt
- WHEN the operator runs `cairn evidence-manifest generate --bundle bundle.json --agent-receipt agent.json --replay replay.json --replay-verification replay-verify.json --out manifest.json --root .`
- THEN Cairn SHALL write a manifest JSON artifact that `cairn evidence-manifest verify manifest.json --root .` can consume.

### Requirement: Canonical Manifest Artifact Ordering r[cairn.supersede.evidence_manifest_generate.canonical]

Generated evidence manifests SHALL use stable artifact roles, stable artifact hashes, and deterministic artifact ordering independent of CLI argument order.

#### Scenario: Artifact roles are ordered and hashed

r[cairn.supersede.evidence_manifest_generate.canonical]

- GIVEN the same artifact inputs in any CLI argument order
- WHEN Cairn generates a manifest
- THEN artifact entries SHALL be emitted in stable role order
- AND each entry SHALL include the stable role, wrapper-provided path, and BLAKE3 artifact hash.

### Requirement: Public Metadata Generation r[cairn.supersede.evidence_manifest_generate.metadata]

Cairn SHALL support optional signer and anchor metadata JSON objects when generating evidence manifests.

#### Scenario: Metadata is public and object-shaped

r[cairn.supersede.evidence_manifest_generate.metadata]

- GIVEN optional signer or anchor metadata file arguments
- WHEN Cairn generates a manifest
- THEN Cairn SHALL require each metadata file to parse as a JSON object
- AND SHALL include the metadata object in the generated manifest without claiming cryptographic signature or remote-anchor validity.

### Requirement: Generation Failure Receipts r[cairn.supersede.evidence_manifest_generate.failures]

Cairn SHALL emit deterministic failure receipts for invalid manifest generation inputs.

#### Scenario: Missing or invalid inputs fail safely

r[cairn.supersede.evidence_manifest_generate.failures]

- GIVEN missing required artifact flags, unreadable artifacts, invalid metadata JSON, or non-object metadata JSON
- WHEN Cairn attempts to generate a manifest
- THEN Cairn SHALL emit a deterministic failure receipt and exit nonzero
- AND the receipt SHALL avoid raw absolute paths, checkout roots, environment values, credentials, and host-specific values.

### Requirement: Manifest Generation Documentation r[cairn.supersede.evidence_manifest_generate.docs]

Cairn SHALL document the manifest generation command, stable roles, and generate-to-verify flow.

#### Scenario: Operator flow includes generation

r[cairn.supersede.evidence_manifest_generate.docs]

- GIVEN the README and agent receipt documentation
- WHEN an operator follows the documented evidence flow
- THEN the operator can generate a manifest from saved artifacts and verify the generated manifest before publication.

### Requirement: Manifest Generation Test Coverage r[cairn.supersede.evidence_manifest_generate.tests]

Cairn SHALL test valid and invalid evidence manifest generation behavior.

#### Scenario: Tests cover round-trip and failures

r[cairn.supersede.evidence_manifest_generate.tests]

- GIVEN Cairn's CLI test suite
- WHEN manifest generation tests run
- THEN they SHALL cover generate-to-verify success, canonical role order, metadata inclusion, missing input failures, invalid metadata failures, and path/secret-safe receipts.

### Requirement: Manifest Generation Change Gates r[cairn.supersede.evidence_manifest_generate.gates]

Cairn SHALL pass proposal, design, and tasks gates for the evidence manifest generation change.

#### Scenario: Change artifacts are gate-ready

r[cairn.supersede.evidence_manifest_generate.gates]

- GIVEN proposal, design, tasks, and spec delta artifacts
- WHEN Cairn gates each stage
- THEN each gate receipt SHALL be valid.

### Requirement: Manifest Generation Change Lifecycle r[cairn.supersede.evidence_manifest_generate.archive]

Cairn SHALL sync and archive the evidence manifest generation change after verification.

#### Scenario: Archived change retains review evidence

r[cairn.supersede.evidence_manifest_generate.archive]

- GIVEN implementation and verification are complete
- WHEN the change is synced and archived
- THEN the accepted spec and archive SHALL retain traceable requirements and verification references.
