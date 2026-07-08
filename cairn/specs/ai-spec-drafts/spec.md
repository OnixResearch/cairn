# Ai Spec Drafts Specification

## Purpose

Defines the `ai-spec-drafts` capability.

## Requirements

### Requirement: AI-assisted draft provenance
r[cairn.ai_spec_drafts.provenance] Cairn MAY record AI-assisted draft provenance for lifecycle specs, contracts, harnesses, and evidence notes, including artifact kind, assistant class, redaction class, target requirement, validation command, review decision, and draft status.

#### Scenario: Draft provenance is recorded
r[cairn.ai_spec_drafts.provenance.recorded]
- GIVEN an agent drafts a lifecycle artifact or Kani contract with AI assistance
- WHEN the draft provenance is saved
- THEN Cairn MAY preserve the draft metadata without requiring raw prompts.

#### Scenario: Missing target is rejected when provenance is required
r[cairn.ai_spec_drafts.provenance.target]
- GIVEN a policy requires draft provenance for an AI-assisted verification artifact
- WHEN the provenance lacks a target requirement or property id
- THEN validation MUST fail closed.

### Requirement: AI draft provenance is not verification
r[cairn.ai_spec_drafts.not_verification] Cairn MUST NOT treat AI-assisted draft provenance by itself as verification evidence for a requirement.

#### Scenario: AI-only proof claim fails
r[cairn.ai_spec_drafts.not_verification.ai_only]
- GIVEN a lifecycle task claims a requirement is verified only because an AI assistant drafted a contract or proof
- WHEN Cairn validates the task evidence
- THEN validation MUST reject the claim.

#### Scenario: Checked AI draft can support evidence
r[cairn.ai_spec_drafts.not_verification.checked]
- GIVEN an AI-drafted Kani contract has a passing Kani receipt for the target property
- WHEN Cairn validates the task evidence
- THEN the passing receipt MAY support the requirement while the AI provenance remains draft metadata.

### Requirement: AI draft acceptance paths
r[cairn.ai_spec_drafts.acceptance_paths] Cairn MUST define accepted completion paths for AI-assisted drafts through machine-check receipts, human-review decisions, or explicit docs-only rationale.

#### Scenario: Machine-check path completes
r[cairn.ai_spec_drafts.acceptance_paths.machine]
- GIVEN an AI-drafted artifact has a matching passing machine-check receipt
- WHEN Cairn validates the lifecycle task
- THEN the task MAY be considered complete for the scoped machine-checked claim.

#### Scenario: Review path completes
r[cairn.ai_spec_drafts.acceptance_paths.review]
- GIVEN an AI-drafted lifecycle spec has a required human-review acceptance record
- WHEN Cairn validates the lifecycle task
- THEN the task MAY be considered complete for the reviewed documentation claim.

### Requirement: AI draft positive fixtures
r[cairn.ai_spec_drafts.positive_fixtures] Cairn MUST include positive fixtures for an AI-drafted Kani contract with a passing receipt and an AI-drafted Cairn spec with human-review acceptance.

#### Scenario: Positive AI draft fixtures pass
r[cairn.ai_spec_drafts.positive_fixtures.valid]
- GIVEN valid AI-draft fixtures include accepted machine-check or review evidence
- WHEN fixture tests run
- THEN validation MUST accept the fixtures deterministically.

### Requirement: AI draft negative fixtures
r[cairn.ai_spec_drafts.negative_fixtures] Cairn MUST include negative fixtures for AI-only proof claims, missing validation status, missing target requirement, and rejected draft reuse.

#### Scenario: Rejected draft cannot be reused
r[cairn.ai_spec_drafts.negative_fixtures.rejected]
- GIVEN a draft has been rejected or superseded
- WHEN a lifecycle task cites it as accepted evidence
- THEN validation MUST fail closed.

#### Scenario: Missing validation status fails
r[cairn.ai_spec_drafts.negative_fixtures.status]
- GIVEN AI-assisted draft provenance lacks draft status or acceptance status
- WHEN Cairn validates the artifact
- THEN validation MUST fail closed.

### Requirement: AI draft gate diagnostics
r[cairn.ai_spec_drafts.gate_diagnostics] Cairn MUST emit deterministic diagnostics when AI-assisted draft provenance is presented as unsupported verification evidence.

#### Scenario: Diagnostic names unsupported AI evidence
r[cairn.ai_spec_drafts.gate_diagnostics.named]
- GIVEN a gate rejects AI-only evidence
- WHEN Cairn emits the gate result
- THEN the diagnostic MUST say that AI draft provenance is not verification evidence.

### Requirement: AI draft operator documentation
r[cairn.ai_spec_drafts.operator_docs] Cairn documentation MUST explain how to record AI-assisted draft provenance without storing raw prompts and how to pair drafts with machine checks or human review.

#### Scenario: Operator docs show safe provenance
r[cairn.ai_spec_drafts.operator_docs.visible]
- GIVEN an operator reads the AI-assisted draft guidance
- WHEN draft provenance is described
- THEN the guidance MUST avoid raw prompt storage and MUST require a separate accepted evidence path.
