# Review Ledger Analytics Specification

## Purpose

Defines the `review-ledger-analytics` capability for Cairn semantic trust/review workflows while preserving deterministic receipt and policy boundaries.

## Requirements

### Requirement: The system
r[cairn.semantic_review.review_ledger_analytics.aggregate] The system MUST aggregate semantic review receipts into deterministic trust-ledger analytics.

#### Scenario: Aggregate Counts
r[cairn.semantic_review.review_ledger_analytics.aggregate.aggregate.counts]
- GIVEN a set of valid semantic review receipts
- WHEN Cairn builds review ledger analytics
- THEN the receipt reports counts by review kind, decision, category, and source

#### Scenario: Aggregate Hashes
r[cairn.semantic_review.review_ledger_analytics.aggregate.aggregate.hashes]
- GIVEN multiple input review receipts
- WHEN Cairn builds analytics
- THEN the output includes sorted input receipt hashes and a stable analytics receipt hash

### Requirement: The system
r[cairn.semantic_review.review_ledger_analytics.input_validation] The system MUST fail closed on invalid ledger inputs.

#### Scenario: Input Validation Invalid Receipt
r[cairn.semantic_review.review_ledger_analytics.input_validation.input_validation.invalid_receipt]
- GIVEN a directory containing an invalid review receipt
- WHEN ledger analytics run
- THEN Cairn reports a deterministic invalid-input issue and does not include that receipt in approved aggregates

#### Scenario: Input Validation Sample Warning
r[cairn.semantic_review.review_ledger_analytics.input_validation.input_validation.sample_warning]
- GIVEN a category or gate kind with a policy-small sample count
- WHEN analytics are emitted
- THEN the receipt includes a warning rather than hiding the small sample
