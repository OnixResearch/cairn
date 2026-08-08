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

### Requirement: Function-address readiness inputs are versioned contracts
r[cairn.stack_provenance_gates.function_address_input_schema.contract] Cairn MUST define versioned input contracts for function-address readiness evidence from Valence, Mantle, Kamacite, and graph summaries.

#### Scenario: Direct Mantle binding receipt is accepted
r[cairn.stack_provenance_gates.function_address_input_schema.mantle_direct]
- GIVEN Mantle emits a versioned function-address release binding receipt with required fields
- WHEN Cairn release-readiness consumes the receipt
- THEN Cairn MUST validate it directly without smoke-specific wrapper glue.

### Requirement: Readiness validation fails closed on malformed evidence
r[cairn.stack_provenance_gates.function_address_input_schema.validation] Cairn MUST reject or mark failed function-address evidence with missing required fields, unsupported versions, malformed BLAKE3 hashes, wrong roles, wrong schemas, unsupported claim scopes, stale receipt identities, or weakened non-claim boundaries.

#### Scenario: Required mode rejects bad input
r[cairn.stack_provenance_gates.function_address_input_schema.negative]
- GIVEN function-address policy mode is required
- WHEN one required readiness input is missing, malformed, stale, or overclaiming
- THEN release readiness MUST fail with a deterministic function-address issue.

### Requirement: Optional mode remains gradual
r[cairn.stack_provenance_gates.function_address_input_schema.positive] Cairn MUST keep optional function-address policy gradual: absent optional evidence is skipped, valid present evidence is checked, and invalid present evidence is reported.

#### Scenario: Full validation suite proves schema behavior
r[cairn.stack_provenance_gates.function_address_input_schema.final_validation]
- GIVEN positive and negative readiness fixtures exercise direct input contracts
- WHEN cargo tests and lifecycle validation run
- THEN Cairn MUST preserve lifecycle-only readiness gating without claiming Rust semantic correctness or release eligibility.

### Requirement: Cairn supports generic evidence-chain readiness inputs
r[cairn.stack_provenance_gates.evidence_chain_readiness.contract] Cairn MUST support typed evidence-chain readiness inputs with evidence kind, profile version, canonical envelope hash, Valence validation hash, Mantle binding hash, source/binary linkage, policy hashes, claim scope, compatibility projections, and non-claim boundaries.

#### Scenario: Function-address uses the generic gate
r[cairn.stack_provenance_gates.evidence_chain_readiness.profile_registry]
- GIVEN function-address is registered as an evidence-chain profile
- WHEN Cairn evaluates complete function-address chain evidence
- THEN Cairn MUST evaluate it through the generic evidence-chain readiness gate.

### Requirement: Readiness validation checks typed chain linkage
r[cairn.stack_provenance_gates.evidence_chain_readiness.core] Cairn MUST validate canonical envelope identity, Valence validation identity, Mantle binding identity, source/binary linkage, policy identity, claim scope, projections, and non-claims without parsing profile payload internals.

#### Scenario: Chain hash commits to upstream links
r[cairn.stack_provenance_gates.evidence_chain_readiness.chain_hash]
- GIVEN evidence-chain readiness passes
- WHEN Cairn emits a readiness receipt
- THEN the receipt hash MUST commit to typed upstream links and Cairn gate policy identity.

### Requirement: Profile registration fails closed
r[cairn.stack_provenance_gates.evidence_chain_readiness.profile_registry] Cairn MUST reject unknown or unsupported evidence-chain profiles in required mode and report invalid present evidence in optional mode.

#### Scenario: Unknown profile is rejected
r[cairn.stack_provenance_gates.evidence_chain_readiness.negative]
- GIVEN readiness input declares an unknown evidence kind or unsupported profile version
- WHEN the profile is required
- THEN readiness MUST fail with deterministic unsupported-profile issues.

### Requirement: Optional and required modes stay gradual
r[cairn.stack_provenance_gates.evidence_chain_readiness.policy_modes] Cairn MUST preserve optional and required policy modes for evidence-chain profiles: optional absent evidence is skipped, optional present evidence is checked, and required missing or invalid evidence fails.

#### Scenario: Optional absent evidence is skipped
r[cairn.stack_provenance_gates.evidence_chain_readiness.positive]
- GIVEN an evidence-chain profile is optional and no evidence is provided
- WHEN readiness runs
- THEN Cairn MUST emit a skipped disposition without treating the profile as failed.

### Requirement: Generic gate preserves non-claim boundaries
r[cairn.stack_provenance_gates.evidence_chain_readiness.validation] Cairn MUST state that evidence-chain readiness proves lifecycle policy conformance only and does not prove evidence truth, Rust semantic correctness, compiler correctness, whole-program safety, verifier soundness, runtime authority, or release eligibility.

#### Scenario: Overclaiming evidence fails closed
- GIVEN evidence-chain input weakens non-claims, has stale Valence or Mantle identity, malformed digest, source/binary mismatch, unsupported claim scope, stale policy hash, or projection drift
- WHEN Cairn evaluates readiness
- THEN readiness MUST fail or report invalid present evidence with deterministic issues.

### Requirement: Preserves receipt identity is a first-class readiness input
r[cairn.stack_provenance_gates.function_address_preserves_readiness.contract] Cairn MUST support function-address readiness inputs that include canonical Kamacite Preserves receipt hash, schema version, Valence receipt linkage, Mantle binding linkage, roles, claim scope, and non-claims.

#### Scenario: Required Preserves-backed evidence passes
r[cairn.stack_provenance_gates.function_address_preserves_readiness.positive]
- GIVEN function-address policy mode is required and complete Valence, Mantle, Kamacite Preserves, and graph inputs are valid
- WHEN Cairn release-readiness runs
- THEN readiness MUST pass the function-address gate with a deterministic receipt hash.

### Requirement: Readiness core validates linkage without parsing functions
r[cairn.stack_provenance_gates.function_address_preserves_readiness.core] Cairn MUST validate Preserves-backed function-address readiness through typed linkage, role, claim-scope, schema, digest, and non-claim checks without parsing Rust functions or Preserves function records in the readiness core.

#### Scenario: Core stays lifecycle-only
r[cairn.stack_provenance_gates.function_address_preserves_readiness.validation]
- GIVEN Cairn evaluates Preserves-backed function-address evidence
- WHEN the readiness gate passes
- THEN the result MUST claim only lifecycle policy conformance and MUST NOT claim Rust semantic correctness, compiler correctness, whole-program safety, verifier soundness, or release eligibility.

### Requirement: JSON wrappers are compatibility-only
r[cairn.stack_provenance_gates.function_address_preserves_readiness.json_projection] Cairn MAY accept JSON compatibility wrappers only when they declare non-authoritative status and bind to the canonical Kamacite Preserves receipt identity.

#### Scenario: Invalid Preserves-backed evidence fails closed
r[cairn.stack_provenance_gates.function_address_preserves_readiness.negative]
- GIVEN function-address readiness input has missing canonical Preserves hash, stale Valence or Mantle linkage, unsupported schema, malformed digest, projection drift, wrong claim scope, role mismatch, or overclaiming text
- WHEN Cairn evaluates the gate
- THEN readiness MUST fail or report invalid present evidence with deterministic function-address issues.

### Requirement: Readiness inputs bind measured bytes
r[cairn.stack_provenance_gates.authenticated_inputs.measurement] Cairn MUST measure explicitly supplied artifact, sidecar, receipt, graph, and policy-export bytes before counting their identities in stack-provenance or function-address readiness.

#### Scenario: Matching measured evidence passes
r[cairn.stack_provenance_gates.authenticated_inputs.fixtures.positive]
- GIVEN declared identities, typed roles, schemas, and parent links match the shell-measured bytes
- WHEN pure readiness evaluation runs
- THEN the identity contribution MAY pass subject to signature, policy, and release-profile requirements.

#### Scenario: Fabricated digest fails
r[cairn.stack_provenance_gates.authenticated_inputs.fixtures.negative]
- GIVEN an input contains a lexically valid digest that was not derived from the referenced bytes
- WHEN readiness evaluation compares declared and measured identities
- THEN the input MUST fail with a deterministic measured-digest mismatch diagnostic.

### Requirement: Canonical receipt identities are recomputed
r[cairn.stack_provenance_gates.authenticated_inputs.receipt_identity] Cairn MUST derive imported receipt identity from canonical receipt bytes under an accepted versioned canonicalization profile and MUST treat a supplied receipt-hash field only as an assertion to compare.

#### Scenario: Tampered receipt is rejected
- GIVEN receipt content changes while its supplied receipt-hash field remains unchanged
- WHEN Cairn recomputes canonical receipt identity
- THEN validation MUST fail before the receipt contributes to readiness.

#### Scenario: Unsupported canonicalization profile is rejected
- GIVEN a receipt uses an unknown, vulnerable, or disallowed canonicalization profile
- WHEN Cairn evaluates the receipt
- THEN validation MUST fail with an unsupported-profile diagnostic unless an explicit accepted migration receipt is supplied.

### Requirement: Pure readiness core consumes typed observations
r[cairn.stack_provenance_gates.authenticated_inputs.core] Cairn MUST evaluate measured identity, role, schema, linkage, policy, and signature observations in pure deterministic core logic.

#### Scenario: Shell and core authority stay separate
- GIVEN the CLI loads evidence files and verifies a configured signature
- WHEN readiness is evaluated
- THEN file I/O, canonical byte measurement, key loading, and cryptographic operations MUST remain in the shell while the core evaluates bounded observations without ambient I/O.

### Requirement: Onix release evidence requires trusted producer signatures
r[cairn.stack_provenance_gates.authenticated_inputs.signature_policy] Evidence counted by the Onix release profile MUST have a verified producer signature that binds the configured producer, key identity, purpose, canonical subject bytes, subject identity, and required parent identities under an explicit trust-root policy.

#### Scenario: Authorized signature passes
- GIVEN a current authorized producer key signs the canonical subject bytes for the configured purpose and parent identities
- WHEN signature and readiness policy evaluation run
- THEN the signature contribution MUST pass and preserve producer and trust-root identity in the readiness receipt.

#### Scenario: Missing or unauthorized signature fails
- GIVEN a required receipt is unsigned, malformed, signed for another purpose, signed by an unauthorized or revoked key, or bound to another subject
- WHEN Onix release readiness runs
- THEN readiness MUST fail with a deterministic signature-policy diagnostic.

### Requirement: Accepted policy identity is generated from Nickel
r[cairn.stack_provenance_gates.authenticated_inputs.policy_identity] Cairn MUST derive accepted stack-provenance and function-address policy identities from canonical exports of reviewed Nickel policy material and MUST reject placeholder identities.

#### Scenario: Current policy identity passes
- GIVEN measured evidence names the identity generated from the current reviewed policy export
- WHEN policy freshness evaluation runs
- THEN the policy contribution MUST pass.

#### Scenario: Placeholder or stale identity fails
- GIVEN configured or supplied policy identity is a placeholder, malformed, absent, or stale
- WHEN readiness evaluation runs
- THEN the evidence MUST fail before it can satisfy a required gate.

### Requirement: Onix release profile requires authenticated evidence gates
r[cairn.stack_provenance_gates.authenticated_inputs.required_release_profile] The Onix release profile MUST require authenticated stack-provenance and function-address gates and MUST NOT produce release-ready status when either required gate is absent, skipped, optional, invalid, or unauthenticated.

#### Scenario: Optional diagnostic profile remains non-release
- GIVEN a diagnostic profile configures evidence as optional or disabled
- WHEN readiness runs without complete authenticated evidence
- THEN it MAY emit diagnostic context but MUST NOT emit Onix release-ready status.

#### Scenario: Complete authenticated release profile passes
- GIVEN all required evidence has matching measured bytes, canonical receipt identity, current policy identity, authorized signatures, valid roles, and valid links
- WHEN Onix release readiness runs
- THEN the authenticated evidence contributions MAY pass while preserving bounded non-claims.

### Requirement: Cross-repository canonicalization dependency is explicit
r[cairn.stack_provenance_gates.authenticated_inputs.cross_repo_dependency] Cairn MUST require evidence that imported Valence receipts use the archived corrected Preserves integrity profile before accepting them in authenticated readiness.

#### Scenario: Vulnerable Valence profile is blocked
- GIVEN a Valence receipt predates or does not satisfy the corrected Preserves integrity profile
- WHEN Cairn evaluates it for authenticated readiness
- THEN Cairn MUST reject it or require an explicit accepted migration receipt.

### Requirement: Authenticated input boundary is documented
r[cairn.stack_provenance_gates.authenticated_inputs.docs] Cairn documentation and receipts MUST state that authenticated readiness proves measured-byte linkage, configured producer authorization, and lifecycle-policy conformance only.

#### Scenario: Stronger claims remain excluded
- GIVEN authenticated readiness passes
- WHEN the result is rendered
- THEN it MUST NOT claim source correctness, verifier soundness, build correctness, semantic equivalence, runtime authority, or universal release fitness.

### Requirement: Authenticated readiness verification rail
r[cairn.stack_provenance_gates.authenticated_inputs.final_validation] The change MUST include positive and negative evidence for measurement, canonical receipt identity, policy freshness, signature policy, and required release modes.

#### Scenario: Fabricated evidence cannot pass fixtures
- GIVEN fabricated-hash and tampered-byte fixtures that previously satisfied lexical checks
- WHEN the corrected fixture rail runs
- THEN every fabricated fixture MUST fail and complete authenticated fixtures MUST pass.

### Requirement: Flake-check CI is checked in
r[cairn.stack_provenance_gates.authenticated_inputs.flake_check_ci] Cairn MUST include a checked-in CI workflow for this remediation whose verification command is `nix flake check`.

#### Scenario: CI uses the scoped verification rail
- GIVEN a change is evaluated by checked-in CI
- WHEN the remediation workflow runs
- THEN it MUST execute `nix flake check` without requiring a separate expanded CI command matrix in this change.

### Requirement: Trellis proof evidence is a readiness profile
r[cairn.stack_provenance_gates.trellis_proof_readiness.profile] Cairn MUST support Trellis proof evidence as a profile of the generic evidence-chain readiness gate.

#### Scenario: Required accepted proof evidence passes
r[cairn.stack_provenance_gates.trellis_proof_readiness.positive]
- GIVEN policy requires accepted Trellis formal-proof evidence for a declared requirement or property ID
- WHEN Cairn receives valid Kamacite, Valence, Mantle, and graph inputs with accepted proof role
- THEN readiness MUST pass the Trellis proof evidence gate.

### Requirement: Readiness validates typed proof chain links
r[cairn.stack_provenance_gates.trellis_proof_readiness.links] Cairn MUST validate canonical Kamacite envelope hash, Valence validation hash, Mantle binding hash, source/binary linkage, policy hashes, proof role, claim scope, and non-claims as distinct typed domains.

#### Scenario: Stale or substituted link fails
r[cairn.stack_provenance_gates.trellis_proof_readiness.negative]
- GIVEN a Trellis proof readiness input has missing canonical hash, stale Valence or Mantle hash, malformed digest, source/binary mismatch, projection drift, or cross-domain substitution
- WHEN readiness evaluation runs
- THEN the gate MUST fail or report invalid present evidence with deterministic issues.

### Requirement: Proof readiness modes preserve role boundaries
r[cairn.stack_provenance_gates.trellis_proof_readiness.policy_modes] Cairn MUST support optional, required-recorded, and required-formal-proof policy modes for Trellis proof evidence.

#### Scenario: Recorded-only cannot satisfy formal-proof mode
- GIVEN policy requires accepted formal-proof evidence
- WHEN only recorded-only Trellis proof evidence is present
- THEN readiness MUST fail with a role-mismatch issue.

### Requirement: Assumption and scope policy is explicit
r[cairn.stack_provenance_gates.trellis_proof_readiness.assumptions] Cairn MUST validate declared requirement/property scope and assumption policy before accepting Trellis proof evidence for required modes.

#### Scenario: Disallowed assumption fails required mode
- GIVEN Trellis proof evidence depends on an assumption forbidden by Cairn policy
- WHEN the proof profile is required
- THEN readiness MUST fail with an assumption-policy issue.

### Requirement: Trellis proof readiness does not prove release correctness
r[cairn.stack_provenance_gates.trellis_proof_readiness.validation] Cairn MUST state that passing Trellis proof readiness proves only lifecycle policy conformance for declared proof evidence and does not prove verifier soundness, compiler correctness, whole-program safety, Rust semantic correctness, downstream system correctness, or release eligibility.

#### Scenario: Overclaiming proof evidence fails closed
- GIVEN Trellis proof readiness input claims release eligibility or whole downstream correctness from proof evidence alone
- WHEN Cairn evaluates the gate
- THEN readiness MUST fail with deterministic non-claim boundary issues.
