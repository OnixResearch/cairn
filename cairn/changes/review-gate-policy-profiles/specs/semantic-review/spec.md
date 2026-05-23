# Review Gate Policy Profiles Specification

## Purpose

Defines the `review-gate-policy-profiles` capability for Cairn semantic trust/review workflows while preserving deterministic receipt and policy boundaries.

## Requirements

### Requirement: The system
r[cairn.semantic_review.review_gate_policy_profiles.profiles] The system MUST define policy-owned semantic review gate profiles.

#### Scenario: Profiles Kinds
r[cairn.semantic_review.review_gate_policy_profiles.profiles.profiles.kinds]
- GIVEN generated Cairn policy
- WHEN semantic review profiles are loaded
- THEN plan, design, code, and diff profiles are present with stable ids

#### Scenario: Profiles Criteria
r[cairn.semantic_review.review_gate_policy_profiles.profiles.profiles.criteria]
- GIVEN a review profile
- WHEN Cairn validates a receipt for that profile
- THEN required criteria ids and receipt fields are enforced

### Requirement: The system
r[cairn.semantic_review.review_gate_policy_profiles.sources] The system MUST validate reviewer source metadata through policy.

#### Scenario: Sources Allowed
r[cairn.semantic_review.review_gate_policy_profiles.sources.sources.allowed]
- GIVEN a semantic review receipt
- WHEN reviewer metadata is checked
- THEN the reviewer source is one of the policy-owned allowed sources

#### Scenario: Sources No Secret
r[cairn.semantic_review.review_gate_policy_profiles.sources.sources.no_secret]
- GIVEN reviewer metadata
- WHEN Cairn serializes validation receipts
- THEN it does not require or expose credentials, prompts with secrets, or remote tokens
