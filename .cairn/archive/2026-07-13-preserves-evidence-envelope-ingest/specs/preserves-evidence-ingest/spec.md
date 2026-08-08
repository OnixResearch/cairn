# Preserves Evidence Ingest Specification

## Purpose

Defines Cairn ingest behavior for external Preserves evidence envelopes.

## Requirements

### Requirement: Preserves evidence ingest contract
r[cairn.preserves_evidence_ingest.contract] Cairn MUST support an ingest contract for external Preserves evidence envelopes that records schema label, producer repo, canonical digest, evidence role, claim scope, adapter ID, payload opacity mode, and non-claims.

#### Scenario: Canonical Preserves envelope passes
r[cairn.preserves_evidence_ingest.fixtures.positive]
- GIVEN an envelope row has canonical Preserves bytes, a supported schema label, BLAKE3 identity, producer repo, evidence role, claim scope, opacity mode, and required non-claims
- WHEN ingest validation runs
- THEN validation MUST pass and preserve the envelope identity.

#### Scenario: Invalid Preserves envelope fails
r[cairn.preserves_evidence_ingest.fixtures.negative]
- GIVEN an envelope row has non-canonical bytes, unknown schema, stale digest, unsupported role, missing non-claims, or semantic overclaim
- WHEN ingest validation runs
- THEN validation MUST fail with deterministic diagnostics.

### Requirement: Opaque payload boundary
r[cairn.preserves_evidence_ingest.validation] Cairn MUST treat imported Preserves payload semantics as opaque unless a Cairn-owned adapter is registered for the declared schema and role.

#### Scenario: Opaque payload stays bounded
r[cairn.preserves_evidence_ingest.validation.opaque]
- GIVEN an imported Preserves envelope is declared opaque
- WHEN ingest validation passes
- THEN the receipt MUST state that Cairn validates carrier identity and role/scope only.

### Requirement: Ingest docs
r[cairn.preserves_evidence_ingest.docs] Documentation MUST state that Preserves ingest acceptance is not evidence truth, runtime correctness, transport correctness, or producer authorization proof.

#### Scenario: Non-claim is visible
r[cairn.preserves_evidence_ingest.docs.non_claims]
- GIVEN a Preserves ingest receipt passes
- WHEN the supported claim is rendered
- THEN it MUST state the bounded lifecycle claim and the producer-semantic non-claims.

### Requirement: Final validation
r[cairn.preserves_evidence_ingest.final_validation] The change MUST include positive and negative fixtures plus focused validation evidence before archive.

#### Scenario: Fixture suite covers ingest health
r[cairn.preserves_evidence_ingest.final_validation.fixtures]
- GIVEN valid and invalid Preserves ingest fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass and invalid fixtures MUST fail closed.
