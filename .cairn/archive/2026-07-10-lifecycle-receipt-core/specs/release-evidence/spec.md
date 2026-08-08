# Release Evidence Specification

## Purpose

Adds a reusable lifecycle receipt envelope core owned by Cairn.

## Requirements

### Requirement: Lifecycle receipt core contract
r[cairn.release_evidence.lifecycle_receipt_core.contract] Cairn MUST define pure lifecycle receipt core contracts for verdicts, diagnostic rows, evidence summaries, command identity, hash fields, receipt hash profiles, and non-claim boundaries.

#### Scenario: Valid receipt envelope validates
r[cairn.release_evidence.lifecycle_receipt_core.fixtures.positive]
- GIVEN a receipt envelope has a supported verdict, required diagnostic shape, evidence summary, command identity, hash fields, receipt hash profile, and non-claim boundary
- WHEN lifecycle receipt core validation runs
- THEN validation MUST accept the envelope and preserve the receipt family identity.

#### Scenario: Invalid receipt envelope fails closed
r[cairn.release_evidence.lifecycle_receipt_core.fixtures.negative]
- GIVEN a receipt envelope has a malformed verdict, missing hash, stale receipt hash, missing diagnostic field, weakened non-claim, or release/lifecycle overclaim
- WHEN lifecycle receipt core validation runs
- THEN validation MUST fail with deterministic diagnostics naming the invalid receipt field.

### Requirement: Receipt hash profiles are explicit
r[cairn.release_evidence.lifecycle_receipt_core.hash_profiles] Cairn MUST require named receipt hash profiles for shared envelope validation.

#### Scenario: Self-hash field exclusion is profile-bound
r[cairn.release_evidence.lifecycle_receipt_core.hash_profiles.self_hash]
- GIVEN a receipt contains its own receipt hash field
- WHEN receipt hash material is computed
- THEN only fields declared by the selected receipt hash profile MUST be excluded from hash material.

### Requirement: Lifecycle receipt validation is pure
r[cairn.release_evidence.lifecycle_receipt_core.validation] Cairn MUST implement lifecycle receipt envelope validation as pure deterministic logic over in-memory receipt values, hash profiles, and policy metadata.

#### Scenario: Shell owns receipt collection
r[cairn.release_evidence.lifecycle_receipt_core.validation.shell]
- GIVEN a CLI collects repository snapshots, policy files, command inputs, or receipt JSON from disk
- WHEN lifecycle receipt core validation is invoked
- THEN filesystem access, JSON presentation, stdout/stderr, process exits, and mutation planning MUST remain outside the receipt core.

### Requirement: Compatibility adapters preserve receipt families
r[cairn.release_evidence.lifecycle_receipt_core.adapters] Cairn SHOULD expose adapters for existing Cairn receipt families and downstream receipt-shaped evidence without changing their public claim boundaries.

#### Scenario: Existing receipt family adapts
r[cairn.release_evidence.lifecycle_receipt_core.adapters.family]
- GIVEN an existing Cairn gate, validate, sync, archive, runtime-evidence, or release-evidence receipt uses its current public JSON shape
- WHEN the compatibility adapter validates it
- THEN the adapter SHOULD preserve the public receipt shape while routing shared envelope checks through the lifecycle receipt core.

### Requirement: Lifecycle receipt core remains bounded
r[cairn.release_evidence.lifecycle_receipt_core.docs] Passing lifecycle receipt core validation MUST NOT prove release eligibility, lifecycle readiness, policy satisfaction, evidence truth, source-code correctness, verifier soundness, or downstream behavioral correctness.

#### Scenario: Boundary is visible
r[cairn.release_evidence.lifecycle_receipt_core.final_validation]
- GIVEN a lifecycle receipt core validation report passes
- WHEN an operator reads the supported claim
- THEN the report MUST state that validation proves only receipt envelope syntax, hash consistency, diagnostics shape, and non-claim presence.
