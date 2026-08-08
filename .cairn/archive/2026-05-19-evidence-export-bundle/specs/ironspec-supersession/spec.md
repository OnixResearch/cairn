## ADDED Requirements

### Requirement: Evidence Export Bundle Command

Cairn MUST provide a first-class evidence export command that materializes the canonical local evidence chain into a caller-provided output directory. r[cairn.supersede.evidence_export_bundle.command]

#### Scenario: Export creates complete evidence set

Given a repository with valid Cairn policy and agent receipt fixtures
When an operator runs `cairn evidence export --root . --out <dir>`
Then Cairn MUST write coverage audit, replay, replay verification, release-readiness, agent receipt, agent evidence bundle, bundle verification, evidence manifest, manifest verification, and export receipt artifacts under `<dir>`.

#### Scenario: Export composes existing validators

Given generated export artifacts
When Cairn builds the bundle, manifest, and export receipt
Then the export MUST use the same validation semantics as `determinism verify-receipt`, `agent-receipt verify-bundle`, and `evidence-manifest verify` rather than accepting wrapper-only assertions.

### Requirement: Evidence Export Artifact Manifest

Cairn MUST emit a deterministic export receipt that lists generated artifacts by stable role, relative path, and content hash. r[cairn.supersede.evidence_export_bundle.artifacts]

#### Scenario: Stable artifact roles and hashes

Given an evidence export completed successfully
When the export receipt is inspected
Then it MUST include stable artifact role entries sorted by role with relative paths and BLAKE3 content hashes, plus the evidence manifest hash, manifest verification receipt hash, and export receipt hash.

#### Scenario: Deterministic repeated exports

Given two evidence exports for equivalent repository content into different output directories
When host-specific root and output prefixes are normalized
Then the logical export receipt hashes and artifact role/hash mappings MUST match.

### Requirement: Evidence Export Failure Semantics and Documentation

Evidence export failures MUST be deterministic, path-safe, and documented as the preferred publication flow. r[cairn.supersede.evidence_export_bundle.failures]

#### Scenario: Safe failure receipt

Given export cannot read a required artifact, generate a receipt, or validate the generated evidence chain
When the command reports failure
Then the failure receipt MUST use stable issue codes/classes and MUST NOT include raw absolute artifact paths, raw environment values, credentials, or secrets.

#### Scenario: Documentation describes canonical publication flow

Given agent/operator documentation describes local evidence collection
When the export command is available
Then the docs MUST prefer `cairn evidence export --root . --out <dir>` for publication while preserving the lower-level receipt commands for debugging.

r[cairn.supersede.evidence_export_bundle.docs]
r[cairn.supersede.evidence_export_bundle.gates]
r[cairn.supersede.evidence_export_bundle.archive]
