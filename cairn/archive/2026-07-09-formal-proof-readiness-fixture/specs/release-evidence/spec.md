# Release Evidence Specification Delta

## ADDED Requirements

### Requirement: Formal proof readiness fixture contract
r[cairn.release_evidence.formal_proof_readiness_fixture.contract] Cairn MUST define a frozen formal-proof readiness fixture contract for Valence stack readiness inputs derived from Octet formal-proof exports.

#### Scenario: Fixture contract names upstream evidence and Cairn receipt fields
r[cairn.release_evidence.formal_proof_readiness_fixture.contract.fields]
- GIVEN a formal-proof readiness fixture is declared
- WHEN Cairn validates the fixture contract
- THEN the fixture MUST name the Valence input identity, upstream Octet proof identity, verification-role summary, policy hash, input hash, readiness receipt hash, and required non-claim boundary.

### Requirement: Positive formal-proof readiness fixture
r[cairn.release_evidence.formal_proof_readiness_fixture.positive] Cairn MUST accept a readiness fixture with one eligible property record backed by Valence stack evidence and an Octet-owned formal-proof receipt while preserving recorded-only/manual-review context.

#### Scenario: Only eligible property evidence counts as verified
r[cairn.release_evidence.formal_proof_readiness_fixture.positive.property_count]
- GIVEN a Valence readiness input contains one eligible property record and additional recorded-only or manual-review records
- WHEN Cairn validates the readiness fixture
- THEN the readiness receipt MUST count only the eligible property record as verified and MUST report recorded-only or manual-review records separately.

### Requirement: Negative formal-proof readiness fixtures
r[cairn.release_evidence.formal_proof_readiness_fixture.negative] Cairn MUST reject readiness fixtures with stale Valence input, digest drift, role promotion, failed upstream proof status, missing hashes, missing non-claims, or overclaim text.

#### Scenario: Recorded-only evidence cannot satisfy readiness proof count
r[cairn.release_evidence.formal_proof_readiness_fixture.negative.recorded_only]
- GIVEN a readiness input counts recorded-only, boundary, imported Trellis, or manual-review evidence as verified property evidence
- WHEN Cairn validates the readiness fixture
- THEN validation MUST fail with a deterministic verification-role-promotion diagnostic.

#### Scenario: Stale upstream input fails closed
r[cairn.release_evidence.formal_proof_readiness_fixture.negative.stale]
- GIVEN the Valence graph input hash or sidecar digest does not match the readiness fixture declaration
- WHEN Cairn validates the readiness fixture
- THEN validation MUST fail with a deterministic stale-input diagnostic.

### Requirement: Formal-proof readiness validation core
r[cairn.release_evidence.formal_proof_readiness_fixture.validation] Cairn MUST validate formal-proof readiness fixtures through a pure deterministic core over parsed fixture values and policy metadata.

#### Scenario: Pure core owns readiness fixture decisions
r[cairn.release_evidence.formal_proof_readiness_fixture.validation.core]
- GIVEN parsed readiness fixture values and policy metadata
- WHEN the readiness fixture validator runs
- THEN it MUST return pass/fail diagnostics without reading files, invoking processes, inspecting environment, or contacting sibling repositories.

### Requirement: Readiness ownership documentation
r[cairn.release_evidence.formal_proof_readiness_fixture.docs] Cairn documentation MUST state that formal-proof readiness fixtures prove Cairn receipt handling under policy only.

#### Scenario: Readiness non-claim is visible
r[cairn.release_evidence.formal_proof_readiness_fixture.docs.non_claims]
- GIVEN a formal-proof readiness fixture passes
- WHEN an operator reads the supported claim
- THEN the report MUST state that Cairn does not prove upstream source correctness, verifier soundness, Valence correctness, Octet correctness, whole-program correctness, semantic equivalence, or release eligibility outside the configured readiness policy.

### Requirement: Final validation evidence
r[cairn.release_evidence.formal_proof_readiness_fixture.final_validation] The change MUST include positive and negative fixtures, focused core tests, policy freshness checks, Cairn validation/gates, and release-readiness evidence before archive.

#### Scenario: Readiness fixture validation is reviewable
r[cairn.release_evidence.formal_proof_readiness_fixture.final_validation.fixtures]
- GIVEN valid and invalid formal-proof readiness fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass, invalid fixtures MUST fail closed, and saved evidence MUST bind fixture, policy, input, and receipt identities.
