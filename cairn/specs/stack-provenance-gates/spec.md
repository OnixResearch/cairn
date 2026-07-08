# Stack Provenance Gates Specification

## Purpose

Defines the `stack-provenance-gates` capability.

## Requirements

### Requirement: Stack provenance gate policy
r[cairn.stack_provenance_gates.policy] Cairn release-readiness policy SHOULD support disabled, optional, and required modes for Valence-verified stack provenance evidence.

#### Scenario: Disabled mode does not evaluate stack provenance
r[cairn.stack_provenance_gates.policy.disabled]
- GIVEN stack provenance gate mode is disabled
- WHEN release-readiness runs
- THEN Cairn MUST omit stack provenance as a gating requirement and MUST NOT claim stack provenance was verified.

#### Scenario: Optional absent mode records disposition
r[cairn.stack_provenance_gates.policy.optional_absent]
- GIVEN stack provenance gate mode is optional and no Valence receipt is supplied
- WHEN release-readiness runs
- THEN Cairn MUST record an absent or skipped stack provenance disposition without failing readiness solely for that absence.

#### Scenario: Required mode fails missing evidence
r[cairn.stack_provenance_gates.policy.required_missing]
- GIVEN stack provenance gate mode is required
- WHEN the Valence receipt or Mantle bundle linkage is missing
- THEN release-readiness MUST fail with deterministic diagnostics naming the missing evidence.

### Requirement: Stack provenance receipt validation
r[cairn.stack_provenance_gates.receipt_validation] Cairn MUST validate Valence stack-provenance receipt metadata, Mantle bundle linkage metadata, artifact BLAKE3 hashes, claim scope, verification role summary, and required non-claims before counting stack provenance as present.

#### Scenario: Complete receipt wiring passes
r[cairn.stack_provenance_gates.receipt_validation.complete]
- GIVEN a passing Valence verification receipt, matching Mantle bundle linkage metadata, matching artifact hashes, supported claim scope, and required non-claims
- WHEN Cairn evaluates stack provenance readiness
- THEN the readiness contribution MUST be passing and MUST include the Valence receipt hash.

#### Scenario: Stale policy fails required mode
r[cairn.stack_provenance_gates.receipt_validation.stale_policy]
- GIVEN the Valence receipt policy or schema hash is stale against the configured accepted policy
- WHEN Cairn evaluates required stack provenance readiness
- THEN the readiness contribution MUST fail with a deterministic stale-policy diagnostic.

#### Scenario: Digest mismatch fails required mode
r[cairn.stack_provenance_gates.receipt_validation.digest_mismatch]
- GIVEN the Valence receipt hash, sidecar digest, or artifact BLAKE3 hash does not match declared metadata
- WHEN Cairn evaluates required stack provenance readiness
- THEN the readiness contribution MUST fail with a deterministic digest-mismatch diagnostic.

#### Scenario: Unsupported claim scope fails required mode
r[cairn.stack_provenance_gates.receipt_validation.claim_scope]
- GIVEN stack provenance metadata declares a claim scope outside the policy-owned allowed set
- WHEN Cairn evaluates required stack provenance readiness
- THEN the readiness contribution MUST fail with a deterministic unsupported-claim-scope diagnostic.

#### Scenario: Manual-review artifacts are not counted as verified
r[cairn.stack_provenance_gates.receipt_validation.manual_review]
- GIVEN a Valence report marks artifacts as manual-review or recorded-only evidence
- WHEN Cairn evaluates stack provenance readiness
- THEN Cairn MUST NOT count those artifacts as verified property evidence.

### Requirement: Stack provenance graph summary
r[cairn.stack_provenance_gates.graph_summary] Cairn MAY include a compact Valence provenance graph summary in release-readiness and evidence bundle verification receipts without reimplementing Valence stack semantics.

#### Scenario: Graph summary is included when supplied
r[cairn.stack_provenance_gates.graph_summary.included]
- GIVEN a Valence graph report summary with artifact count, edge count, manual-review node count, unresolved-link count, failed-link count, and report hash
- WHEN Cairn emits release-readiness output
- THEN Cairn MAY include those fields as context bound to the Valence report hash.

#### Scenario: Malformed graph summary fails required mode
r[cairn.stack_provenance_gates.graph_summary.malformed]
- GIVEN stack provenance is required and the supplied graph summary is malformed or not bound to the Valence report hash
- WHEN Cairn evaluates readiness
- THEN the readiness contribution MUST fail with a deterministic malformed-summary diagnostic.

### Requirement: Stack provenance gate boundary
r[cairn.stack_provenance_gates.boundary] Cairn stack provenance gates MUST state that Cairn verifies lifecycle and release policy conformance over supplied receipts and does not prove source-code correctness, Valence verifier soundness, Mantle build correctness, or Trellis/Octet proof validity by itself.

#### Scenario: Boundary is visible
r[cairn.stack_provenance_gates.boundary.visible]
- GIVEN a release-readiness receipt or operator documentation describes stack provenance gate success
- WHEN the supported claim is stated
- THEN it MUST say Cairn consumed Valence/Mantle receipts for lifecycle policy conformance and did not prove source-code correctness or verifier soundness.

#### Scenario: Weakened boundary fails required mode
r[cairn.stack_provenance_gates.boundary.weakened]
- GIVEN required stack provenance evidence weakens or omits the Cairn non-claim boundary
- WHEN Cairn evaluates readiness
- THEN the readiness contribution MUST fail with a deterministic non-claim diagnostic.

### Requirement: Stack provenance fixture matrix
r[cairn.stack_provenance_gates.fixture_matrix] Cairn MUST include positive and negative fixtures for disabled, optional, and required stack provenance gate behavior.

#### Scenario: Positive fixtures cover gate modes
r[cairn.stack_provenance_gates.fixture_matrix.positive]
- GIVEN disabled, optional-absent, optional-present, and required-complete fixture inputs
- WHEN the fixture suite runs
- THEN disabled mode MUST omit stack provenance, optional absent MUST record absent disposition, optional present MUST verify receipt wiring, and required complete MUST pass.

#### Scenario: Negative fixtures cover required-mode failures
r[cairn.stack_provenance_gates.fixture_matrix.negative]
- GIVEN fixtures with missing receipt, stale policy, digest mismatch, unsupported claim scope, missing Mantle linkage, manual-review artifacts counted as verified, weakened non-claims, malformed graph summary, or failed Valence verdict
- WHEN the fixture suite runs in required mode
- THEN each fixture MUST fail closed with deterministic diagnostics.
