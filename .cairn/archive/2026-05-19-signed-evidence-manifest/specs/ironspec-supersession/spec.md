## ADDED Requirements

### Requirement: Signed Evidence Manifest Verification

Cairn SHALL verify a persisted evidence manifest that indexes agent and CI evidence artifacts with deterministic pass/fail behavior. r[cairn.supersede.signed_evidence_manifest.verify]

#### Scenario: Valid manifest verifies artifact index

- GIVEN a manifest that lists a valid agent evidence bundle and referenced evidence artifacts with matching stable hashes
- WHEN the operator runs `cairn evidence-manifest verify <manifest.json> --root .`
- THEN Cairn SHALL emit a deterministic receipt with `valid=true`, `verdict="pass"`, stable artifact roles, stable artifact hashes, and its own `receipt_hash`.

#### Scenario: Invalid artifact hash is rejected

- GIVEN a manifest whose declared artifact hash does not match the referenced artifact content
- WHEN the operator verifies the manifest
- THEN Cairn SHALL emit a deterministic failure receipt with a stable issue code and SHALL exit nonzero.

#### Scenario: Referenced evidence chain is invalid

- GIVEN a manifest whose referenced bundle, replay receipt, replay verification receipt, agent receipt, or release-readiness receipt fails its authoritative verifier
- WHEN the operator verifies the manifest
- THEN Cairn SHALL emit a deterministic failure receipt that names only stable roles/classes and SHALL exit nonzero.

### Requirement: Signed Evidence Manifest Public Schema

Cairn SHALL publish a stable public receipt schema for evidence manifest verification. r[cairn.supersede.signed_evidence_manifest.schema]

#### Scenario: Stable manifest verification fields are policy-owned

- GIVEN generated Cairn policy
- WHEN public receipt schemas are inspected
- THEN the `evidence-manifest verify` receipt schema SHALL include stable top-level fields for command, root, valid, verdict, issues, artifacts, manifest hash, optional signer metadata hash, optional anchor metadata hash, and receipt hash.

### Requirement: Evidence Manifest Validator Composition

Cairn SHALL compose existing deterministic validators when verifying manifest artifact roles. r[cairn.supersede.signed_evidence_manifest.chain]

#### Scenario: Bundle and replay validators remain authoritative

- GIVEN a manifest that references an agent evidence bundle and replay verification artifacts
- WHEN Cairn verifies the manifest
- THEN Cairn SHALL use existing bundle and replay receipt validation semantics rather than accepting wrapper-provided claims.

### Requirement: Public Metadata Only

Evidence manifest signer and anchor metadata SHALL be optional, deterministic, JSON-object-shaped, and secret-free. r[cairn.supersede.signed_evidence_manifest.metadata]

#### Scenario: Metadata is hashed but not cryptographically trusted

- GIVEN signer or anchor metadata in a manifest
- WHEN Cairn verifies the manifest
- THEN Cairn SHALL include stable metadata hashes in the verification receipt and SHALL NOT claim remote anchoring or signature cryptographic validity.

### Requirement: Manifest Documentation

Cairn SHALL document the evidence manifest format, CLI usage, stable receipt fields, and non-goals. r[cairn.supersede.signed_evidence_manifest.docs]

#### Scenario: Operators can build manifests without hidden wrapper conventions

- GIVEN the README and agent receipt documentation
- WHEN an operator follows the documented evidence flow
- THEN the operator can save replay, replay-verification, agent, bundle, and optional release-readiness artifacts and verify them through the manifest command.

### Requirement: Manifest Verification Tests

Cairn SHALL test valid and invalid evidence manifest verification behavior. r[cairn.supersede.signed_evidence_manifest.tests]

#### Scenario: Failure receipts do not leak host paths or secrets

- GIVEN malformed or tampered manifest evidence with path-like and secret-like input values
- WHEN Cairn emits a failure receipt
- THEN the receipt SHALL avoid raw absolute paths, checkout roots, raw environment values, and credentials.

### Requirement: Manifest Change Lifecycle

Cairn SHALL sync and archive the signed evidence manifest change after verification. r[cairn.supersede.signed_evidence_manifest.archive]

#### Scenario: Archived manifest change retains review evidence

- GIVEN implementation and verification are complete
- WHEN the change is synced and archived
- THEN the accepted spec and archive SHALL retain traceable requirements and verification references.

### Requirement: Manifest Change Gates

Cairn SHALL pass proposal, design, and tasks gates for the signed evidence manifest change. r[cairn.supersede.signed_evidence_manifest.gates]

#### Scenario: Change artifacts are gate-ready

- GIVEN proposal, design, tasks, and spec delta artifacts
- WHEN Cairn gates each stage
- THEN each gate receipt SHALL be valid.
