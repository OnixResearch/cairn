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

### Requirement: Function-address gate policy
r[cairn.function_address_gates.policy] Cairn release-readiness policy SHOULD support disabled, optional, and required modes for Valence-verified and Mantle-bound function-address evidence.

#### Scenario: Disabled mode does not evaluate function evidence
r[cairn.function_address_gates.policy.disabled]
- GIVEN function-address gate mode is disabled
- WHEN release-readiness runs
- THEN Cairn MUST omit function-address evidence as a gating requirement and MUST NOT claim function-address evidence was verified.

#### Scenario: Optional absent mode records disposition
r[cairn.function_address_gates.policy.optional_absent]
- GIVEN function-address gate mode is optional and no function-address receipt is supplied
- WHEN release-readiness runs
- THEN Cairn MUST record an absent or skipped function-address disposition without failing readiness solely for that absence.

#### Scenario: Required missing mode fails
r[cairn.function_address_gates.policy.required_missing]
- GIVEN function-address gate mode is required
- WHEN the Valence receipt, Kamacite receipt identity, or Mantle bundle linkage metadata required by policy is missing
- THEN release-readiness MUST fail with deterministic diagnostics naming the missing evidence.

### Requirement: Function-address receipt validation
r[cairn.function_address_gates.receipt_validation] Cairn MUST validate supplied function-address readiness metadata, including Valence receipt hash, Mantle bundle linkage metadata, Kamacite receipt identity when present, artifact BLAKE3 hashes, expected roles, expected schemas, claim scope, verification-role summary, and required non-claims before counting function-address evidence as present.

#### Scenario: Complete receipt wiring passes
r[cairn.function_address_gates.receipt_validation.complete]
- GIVEN a passing Valence function-address receipt, matching Mantle bundle linkage metadata, matching artifact hashes, supported claim scope, verification-role summary, and required non-claims
- WHEN Cairn evaluates function-address readiness
- THEN the readiness contribution MUST be passing and MUST include the Valence receipt hash and supplied Kamacite receipt identity.

#### Scenario: Digest mismatch fails required mode
r[cairn.function_address_gates.receipt_validation.digest_mismatch]
- GIVEN the Valence receipt hash, sidecar digest, Kamacite receipt identity, or artifact BLAKE3 hash does not match declared metadata
- WHEN Cairn evaluates required function-address readiness
- THEN the readiness contribution MUST fail with a deterministic digest-mismatch diagnostic.

#### Scenario: Failed upstream verdict fails required mode
r[cairn.function_address_gates.receipt_validation.failed_upstream]
- GIVEN the supplied Valence or Mantle function-address evidence verdict is failing
- WHEN Cairn evaluates required function-address readiness
- THEN the readiness contribution MUST fail with a deterministic upstream-verdict diagnostic.

### Requirement: Function-address graph summary
r[cairn.function_address_gates.graph_summary] Cairn MAY include a compact function-address graph summary in release-readiness receipts without reimplementing Valence stack semantics.

#### Scenario: Graph summary is included when supplied
r[cairn.function_address_gates.graph_summary.included]
- GIVEN a Valence function-address graph report summary with function count, source count, dependency-edge count, recorded-only count, property count, failed-link count, and report hash
- WHEN Cairn emits release-readiness output
- THEN Cairn MAY include those fields as context bound to the Valence report hash.

#### Scenario: Malformed graph summary fails required mode
r[cairn.function_address_gates.graph_summary.malformed]
- GIVEN function-address evidence is required and the supplied graph summary is malformed or not bound to the Valence report hash
- WHEN Cairn evaluates readiness
- THEN the readiness contribution MUST fail with a deterministic malformed-summary diagnostic.

### Requirement: Function-address gate boundary
r[cairn.function_address_gates.boundary] Cairn function-address gates MUST preserve verification-role boundaries. Function-address evidence MAY satisfy a required-presence policy but MUST NOT count as property proof unless a separate eligible proof receipt is linked under existing verification-role rules.

#### Scenario: Recorded-only evidence is not property proof
r[cairn.function_address_gates.boundary.recorded_only]
- GIVEN a Valence function-address report marks the evidence as recorded-only or boundary evidence
- WHEN Cairn evaluates release readiness
- THEN Cairn MUST NOT count that evidence as verified property evidence.

#### Scenario: Separate proof remains separate
r[cairn.function_address_gates.boundary.proof_separate]
- GIVEN a function-address graph links to a separate proof receipt
- WHEN Cairn evaluates release readiness
- THEN the proof receipt MUST be evaluated under existing proof/readiness policy and MUST NOT promote the function-address receipt itself.

### Requirement: Function-address positive fixture coverage
r[cairn.function_address_gates.fixtures.positive] Cairn MUST include positive fixtures for disabled mode, optional absent mode, optional valid function-address evidence, and required valid function-address evidence.

#### Scenario: Positive fixtures cover policy modes
r[cairn.function_address_gates.fixtures.positive.modes]
- GIVEN positive function-address readiness fixtures for disabled, optional absent, optional valid, and required valid modes
- WHEN the fixture suite runs
- THEN disabled mode MUST omit the gate, optional absent MUST record skipped context, optional valid MUST record passing context, and required valid MUST pass readiness.

### Requirement: Function-address negative fixture coverage
r[cairn.function_address_gates.fixtures.negative] Cairn MUST include negative fixtures for required missing evidence, stale digests, malformed graph summaries, unsupported claim scopes, role promotion, weakened non-claims, overclaims, and failed upstream verdicts.

#### Scenario: Negative fixtures fail required mode
r[cairn.function_address_gates.fixtures.negative.required]
- GIVEN invalid required-mode function-address readiness fixtures
- WHEN the fixture suite runs
- THEN each fixture MUST fail closed with deterministic diagnostics naming the invalid evidence class.

### Requirement: Function-address documentation and non-claims
r[cairn.function_address_gates.docs] Cairn documentation and release-readiness receipts MUST state that function-address gate success proves lifecycle policy conformance over supplied receipts only and does not prove source-code correctness, Rust semantic correctness, compiler correctness, build correctness, Valence verifier soundness, Mantle build correctness, or release eligibility outside the configured policy.

#### Scenario: Boundary is visible
r[cairn.function_address_gates.docs.boundary]
- GIVEN a release-readiness receipt or operator documentation describes function-address gate success
- WHEN the supported claim is stated
- THEN it MUST say Cairn consumed Valence/Mantle/Kamacite receipts for lifecycle policy conformance and did not prove source-code correctness or verifier soundness.

### Requirement: Function-address final validation evidence
r[cairn.function_address_gates.final_validation] The change MUST include positive and negative fixtures, focused readiness tests, policy freshness checks, Cairn validation/gates, traceability coverage, and release-readiness evidence before archive.

#### Scenario: Validation is reviewable
r[cairn.function_address_gates.final_validation.reviewable]
- GIVEN the function-address readiness gate implementation is complete
- WHEN focused validation runs
- THEN valid fixtures MUST pass, invalid fixtures MUST fail closed, policy freshness MUST pass, Cairn validation/gates MUST pass, and saved evidence MUST identify the readiness receipt hashes.

### Requirement: Supported digest spellings are producer-specific
r[cairn.stack_provenance_gates.function_address_digest_normalization.contract] Cairn MUST document supported digest spellings for Valence, Mantle, Kamacite, and graph function-address readiness fields.

#### Scenario: Supported spelling is normalized
r[cairn.stack_provenance_gates.function_address_digest_normalization.positive]
- GIVEN a readiness input field uses its documented digest spelling
- WHEN Cairn parses the function-address readiness inputs
- THEN Cairn MUST normalize the digest deterministically before readiness evaluation.

### Requirement: Digest spelling normalization stays outside role validation
r[cairn.stack_provenance_gates.function_address_digest_normalization.domain] Cairn MUST validate readiness field roles and identity domains separately from digest spelling normalization.

#### Scenario: Cross-field substitution fails
r[cairn.stack_provenance_gates.function_address_digest_normalization.negative]
- GIVEN a syntactically valid BLAKE3 digest appears in the wrong readiness field
- WHEN function-address readiness evaluation runs
- THEN Cairn MUST fail or report the evidence with a deterministic role/domain issue.

### Requirement: Malformed digest spellings fail closed
r[cairn.stack_provenance_gates.function_address_digest_normalization.shell] Cairn MUST reject malformed function-address readiness digests, including unsupported prefixes, wrong length, non-hex content, and unsupported case variants.

#### Scenario: Smoke validates normalized inputs
r[cairn.stack_provenance_gates.function_address_digest_normalization.validation]
- GIVEN the stack smoke uses documented producer spellings
- WHEN Cairn release-readiness runs
- THEN readiness MUST accept valid normalized inputs and reject invalid optional-present or required evidence.
