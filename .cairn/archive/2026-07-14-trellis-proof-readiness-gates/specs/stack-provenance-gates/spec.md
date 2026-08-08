# Stack Provenance Gates Specification

## Purpose

Gate readiness over Trellis proof evidence-chain profiles.

## Requirements

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
