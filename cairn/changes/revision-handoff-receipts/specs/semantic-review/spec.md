# Revision Handoff Receipts Specification

## Purpose

Defines the `revision-handoff-receipts` capability for Cairn semantic trust/review workflows while preserving deterministic receipt and policy boundaries.

## Requirements

### Requirement: The system
r[cairn.semantic_review.revision_handoff_receipts.schema] The system MUST define a revision handoff receipt schema that links review feedback to a revision attempt.

#### Scenario: Schema Chain
r[cairn.semantic_review.revision_handoff_receipts.schema.schema.chain]
- GIVEN a revision handoff receipt
- WHEN Cairn validates it
- THEN it verifies the prior review receipt hash, revision artifact hashes, finding dispositions, and receipt hash

#### Scenario: Schema Dispositions
r[cairn.semantic_review.revision_handoff_receipts.schema.schema.dispositions]
- GIVEN prior review findings
- WHEN a revision handoff is validated
- THEN each prior finding has a policy-known disposition

### Requirement: The system
r[cairn.semantic_review.revision_handoff_receipts.validation] The system MUST validate revision handoff chains deterministically.

#### Scenario: Validation Addressed Evidence
r[cairn.semantic_review.revision_handoff_receipts.validation.validation.addressed_evidence]
- GIVEN a finding marked addressed
- WHEN Cairn validates the handoff
- THEN the finding includes evidence refs or artifact hashes required by policy

#### Scenario: Validation New Findings
r[cairn.semantic_review.revision_handoff_receipts.validation.validation.new_findings]
- GIVEN a follow-up review that introduces new findings
- WHEN Cairn validates the handoff
- THEN the receipt preserves those findings as new rather than overwriting prior findings
