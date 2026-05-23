# Review Rejection Taxonomy Specification

## Purpose

Defines the `review-rejection-taxonomy` capability for Cairn semantic trust/review workflows while preserving deterministic receipt and policy boundaries.

## Requirements

### Requirement: The system
r[cairn.semantic_review.review_rejection_taxonomy.policy] The system MUST define policy-owned review decision and rejection taxonomy vocabularies.

#### Scenario: Policy Categories
r[cairn.semantic_review.review_rejection_taxonomy.policy.policy.categories]
- GIVEN generated Cairn policy
- WHEN review taxonomy data is loaded
- THEN allowed categories include omission, systematic, incoherent, and infra

#### Scenario: Policy Decisions
r[cairn.semantic_review.review_rejection_taxonomy.policy.policy.decisions]
- GIVEN generated Cairn policy
- WHEN review decisions are loaded
- THEN allowed decisions include approved, needs_revision, escalate, and unknown

### Requirement: The system
r[cairn.semantic_review.review_rejection_taxonomy.finding_validation] The system MUST require categorized findings for non-approved semantic review receipts.

#### Scenario: Finding Validation Required
r[cairn.semantic_review.review_rejection_taxonomy.finding_validation.finding_validation.required]
- GIVEN a semantic review receipt with decision `needs_revision`
- WHEN Cairn validates the receipt
- THEN each finding has a policy-known category and stable finding id

#### Scenario: Finding Validation Approved
r[cairn.semantic_review.review_rejection_taxonomy.finding_validation.finding_validation.approved]
- GIVEN an approved semantic review receipt
- WHEN Cairn validates the receipt
- THEN the receipt may have an empty findings list and still remains valid
