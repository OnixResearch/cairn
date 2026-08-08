# Stack Provenance Gates Specification

## Purpose

Requires measured-byte identity, trusted producer authorization, current policy identity, and mandatory Onix release-profile gates for stack provenance evidence.

## Requirements

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
