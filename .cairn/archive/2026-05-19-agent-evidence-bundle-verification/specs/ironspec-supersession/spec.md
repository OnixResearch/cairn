## ADDED Requirements

### Requirement: Agent Evidence Bundle Verification Command r[cairn.supersede.agent_evidence_bundle_verification.cli]

Cairn SHALL provide a read-only `cairn agent-receipt verify-bundle <bundle.json> --root .` command for validating an end-to-end agent evidence bundle.

#### Scenario: Bundle command accepts wrapper evidence

r[cairn.supersede.agent_evidence_bundle_verification.cli]

- GIVEN a JSON bundle that references an agent receipt, replay receipt, replay verification receipt, and optional release-readiness receipt
- WHEN an operator runs `cairn agent-receipt verify-bundle <bundle.json> --root .`
- THEN Cairn SHALL load the referenced artifacts without mutating the workspace
- AND Cairn SHALL emit a deterministic bundle verification receipt.

### Requirement: Agent Evidence Bundle Validation r[cairn.supersede.agent_evidence_bundle_verification.validation]

Cairn SHALL validate that bundle artifacts agree about the current replay policy, agent replay evidence, saved replay receipt hash, replay verification receipt, required replay group coverage, and environment attestation hash.

#### Scenario: Valid bundle passes

r[cairn.supersede.agent_evidence_bundle_verification.validation]

- GIVEN an agent receipt with passing replay evidence for the policy-owned `agent` group
- AND a saved replay receipt whose receipt hash and environment attestation hash verify
- AND a replay verification receipt that reports `valid=true` for the saved replay receipt
- WHEN Cairn verifies the bundle
- THEN the bundle receipt SHALL report `valid=true` and `verdict=pass`.

#### Scenario: Stale or mismatched evidence fails deterministically

r[cairn.supersede.agent_evidence_bundle_verification.validation]
r[cairn.supersede.agent_evidence_bundle_verification.secret_safe]

- GIVEN a bundle with stale policy hashes, mismatched replay hashes, missing group coverage, failed replay verification evidence, or mismatched environment attestation evidence
- WHEN Cairn verifies the bundle
- THEN the bundle receipt SHALL report `valid=false`
- AND SHALL include stable issue codes for each mismatch class.

### Requirement: Agent Evidence Bundle Receipt Schema r[cairn.supersede.agent_evidence_bundle_verification.receipt]

Cairn SHALL publish a stable schema for bundle verification receipts so CI and wrappers can consume one deterministic gate artifact.

#### Scenario: Bundle receipt exposes stable fields

r[cairn.supersede.agent_evidence_bundle_verification.receipt]

- GIVEN any bundle verification result
- WHEN Cairn emits the bundle verification receipt
- THEN the receipt SHALL include stable fields for command, validity, verdict, issues, current replay policy hash, required replay group, agent receipt hash, replay receipt hash, replay verification receipt hash, environment attestation hash, optional release-readiness receipt hash, and receipt hash.

### Requirement: Agent Evidence Bundle Secret-Safe Failures r[cairn.supersede.agent_evidence_bundle_verification.secret_safe]

Cairn SHALL keep bundle verification failure receipts secret-safe and path-safe.

#### Scenario: Bundle failure omits raw paths and secrets

r[cairn.supersede.agent_evidence_bundle_verification.secret_safe]

- GIVEN a bundle that references invalid or tampered evidence artifacts
- WHEN Cairn emits a failure receipt
- THEN the receipt SHALL identify artifact classes and stable issue codes
- AND SHALL NOT include raw environment values, credentials, absolute checkout roots, or host-specific paths.

### Requirement: Agent Evidence Bundle Documentation r[cairn.supersede.agent_evidence_bundle_verification.docs]

Cairn SHALL document how wrappers create and verify end-to-end evidence bundles.

#### Scenario: Wrapper flow is documented

r[cairn.supersede.agent_evidence_bundle_verification.docs]

- GIVEN a wrapper that runs agent receipt validation, replay, replay receipt verification, and optional release-readiness
- WHEN the wrapper consults Cairn documentation
- THEN the documentation SHALL show how to assemble a bundle and run `agent-receipt verify-bundle` before publishing evidence.

### Requirement: Agent Evidence Bundle Public Schema r[cairn.supersede.agent_evidence_bundle_verification.schema]

Cairn SHALL include bundle verification receipt fields in the policy-owned public receipt schema catalog.

#### Scenario: Schema catalog lists bundle verification receipt

r[cairn.supersede.agent_evidence_bundle_verification.schema]

- GIVEN the generated Cairn policy bundle
- WHEN a consumer inspects public receipt schemas
- THEN the catalog SHALL include the stable fields for `agent-receipt verify-bundle` receipts.

### Requirement: Agent Evidence Bundle Test Coverage r[cairn.supersede.agent_evidence_bundle_verification.tests]

Cairn SHALL test valid and invalid bundle verification flows.

#### Scenario: Tests cover bundle verifier behavior

r[cairn.supersede.agent_evidence_bundle_verification.tests]

- GIVEN Cairn's CLI test suite
- WHEN the tests run
- THEN they SHALL cover passing bundles, tampered replay receipts, failed replay verification receipts, group coverage mismatch, optional release-readiness mismatch, deterministic issue codes, and path/secret safety.

### Requirement: Agent Evidence Bundle Archive Evidence r[cairn.supersede.agent_evidence_bundle_verification.archive]

Cairn SHALL archive the completed bundle verification change only after validation and coverage gates pass.

#### Scenario: Archive captures completed change

r[cairn.supersede.agent_evidence_bundle_verification.archive]

- GIVEN the bundle verifier implementation is complete
- WHEN the change is synced and archived
- THEN the accepted specs SHALL retain the bundle verifier requirements
- AND Tracey coverage SHALL pass for the new requirement IDs.
