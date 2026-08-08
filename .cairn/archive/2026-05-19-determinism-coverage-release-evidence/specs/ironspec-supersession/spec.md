## ADDED Requirements

### Requirement: Determinism Coverage Release Evidence

Cairn MUST include deterministic surface coverage-audit evidence in release-readiness so agents and CI can prove that public deterministic surfaces had reviewed replay coverage at release time. r[cairn.supersede.determinism_coverage_release_evidence.release]

#### Scenario: Release-readiness includes coverage audit evidence

- GIVEN a workspace with generated policy and deterministic surface inventory
- WHEN an operator runs `cairn release-readiness --root .`
- THEN the release-readiness receipt includes stable coverage-audit validity and receipt hash fields
- AND the receipt remains deterministic, path-safe, and secret-safe

#### Scenario: Coverage audit failure blocks release-readiness

- GIVEN a policy state where deterministic surface coverage audit would fail
- WHEN release-readiness evaluates deterministic evidence
- THEN release-readiness reports a deterministic coverage-audit issue
- AND the failure does not expose raw checkout paths, environment values, or secrets

### Requirement: Determinism Coverage Bundle And Manifest Evidence

Cairn SHOULD allow agent evidence bundles and evidence manifests to carry a coverage-audit artifact so durable CI evidence chains can prove the audit artifact that was published. r[cairn.supersede.determinism_coverage_release_evidence.bundle_manifest]

#### Scenario: Bundle verifies coverage audit artifact linkage

- GIVEN an agent evidence bundle with a coverage-audit artifact path and expected receipt hash
- WHEN an operator runs `cairn agent-receipt verify-bundle <bundle.json> --root .`
- THEN the bundle verification receipt validates the coverage audit artifact, records its receipt hash, and fails deterministically when the artifact is missing, failing, malformed, or hash-mismatched

#### Scenario: Evidence manifest verifies coverage audit artifact role

- GIVEN an evidence manifest that includes a coverage-audit artifact role
- WHEN an operator runs `cairn evidence-manifest verify <manifest.json> --root .`
- THEN the manifest verification receipt validates the coverage audit artifact hash and records the coverage-audit receipt hash
- AND generated manifests can include the coverage-audit role using canonical role ordering

### Requirement: Deterministic Coverage Evidence Failures

Coverage-audit evidence failures MUST use stable issue codes/classes and MUST NOT include raw artifact paths, absolute checkout roots, raw environment values, credentials, or secrets. r[cairn.supersede.determinism_coverage_release_evidence.failures]

Cairn MUST document the coverage-audit evidence order for agent/operator workflows so wrappers run coverage audit before replay, bundle, manifest, and release-readiness evidence is trusted. r[cairn.supersede.determinism_coverage_release_evidence.docs]

#### Scenario: Secret-safe coverage evidence errors

- GIVEN coverage-audit evidence paths or metadata containing secret-looking strings
- WHEN release-readiness, bundle verification, or manifest verification reports invalid coverage evidence
- THEN the emitted receipt uses deterministic issue codes and classes only
- AND the emitted receipt does not include the secret-looking input text

#### Scenario: Deterministic coverage evidence gates

- GIVEN the coverage evidence implementation is complete
- WHEN project gates, Tracey coverage, tests, coverage audit, and release-readiness run
- THEN all new requirements are referenced by implementation or verification evidence. r[cairn.supersede.determinism_coverage_release_evidence.gates]

#### Scenario: Coverage evidence archive

- GIVEN implementation and verification tasks are complete
- WHEN the change is synced and archived
- THEN the archive preserves the accepted requirement evidence for later review. r[cairn.supersede.determinism_coverage_release_evidence.archive]
