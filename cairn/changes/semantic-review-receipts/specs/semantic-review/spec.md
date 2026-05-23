# Semantic Review Receipts Specification

## Purpose

Defines the `semantic-review-receipts` capability for Cairn semantic trust/review workflows while preserving deterministic receipt and policy boundaries.

## Requirements

### Requirement: The system
r[cairn.semantic_review.semantic_review_receipts.schema] The system MUST define a stable semantic review receipt schema for externally produced review outcomes.

#### Scenario: Schema Fields
r[cairn.semantic_review.semantic_review_receipts.schema.schema.fields]
- GIVEN a semantic review receipt with review kind, decision, reviewed inputs, findings, reviewer metadata, and receipt hash
- WHEN Cairn validates the receipt
- THEN it accepts only policy-known fields and emits deterministic issues for missing or malformed fields

#### Scenario: Schema Hash Binding
r[cairn.semantic_review.semantic_review_receipts.schema.schema.hash_binding]
- GIVEN reviewed artifact paths and content hashes in a receipt
- WHEN Cairn validates the receipt against a repository root
- THEN it verifies the referenced root-relative artifacts match their declared hashes

### Requirement: The system
r[cairn.semantic_review.semantic_review_receipts.cli] The system MUST expose semantic review receipt validation through a deterministic CLI command.

#### Scenario: Cli Valid
r[cairn.semantic_review.semantic_review_receipts.cli.cli.valid]
- GIVEN a valid semantic review receipt
- WHEN the operator runs `cairn review validate <receipt>`
- THEN Cairn emits a valid deterministic validation receipt

#### Scenario: Cli Invalid
r[cairn.semantic_review.semantic_review_receipts.cli.cli.invalid]
- GIVEN a malformed semantic review receipt
- WHEN the operator validates it
- THEN Cairn emits stable issue codes without network, LLM, or reviewer execution

### Requirement: The system
r[cairn.semantic_review.semantic_review_receipts.docs] The system MUST document semantic review receipts as externally produced evidence that Cairn validates structurally.

#### Scenario: Docs Non Claims
r[cairn.semantic_review.semantic_review_receipts.docs.docs.non_claims]
- GIVEN an operator reads the semantic review documentation
- WHEN they inspect the command contract
- THEN it states Cairn does not perform the semantic review, call remote models, or prove reviewed work is correct
