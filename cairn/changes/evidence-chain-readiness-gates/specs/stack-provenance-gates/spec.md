# Stack Provenance Gates Specification

## Purpose

Add generic evidence-chain readiness gates for stack evidence profiles.

## Requirements

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
