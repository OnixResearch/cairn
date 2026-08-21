# Gate Interchange Specification

## Purpose

Defines the `gate-interchange` capability.

## Requirements

### Requirement: Versioned gate interchange contract
r[cairn.gate_interchange.contract] Cairn MUST publish a versioned, host-independent gate interchange contract with bounded request, result, tool-cohort, native-observation, finding, and diagnostic values.

#### Scenario: Consumer pins the contract
r[cairn.gate_interchange.contract.scenario.pinned]
- GIVEN a consumer that pins one immutable Cairn revision and declared schema versions
- WHEN the consumer decodes a conforming gate request or result
- THEN the contract MUST expose the same bounded typed value without requiring Cairn CLI or host effects.

#### Scenario: Schema differs
r[cairn.gate_interchange.contract.scenario.schema]
- GIVEN a request or result with an unsupported schema identifier
- WHEN contract admission runs
- THEN admission MUST reject the value before any external effect.

### Requirement: Exact gate request identity
r[cairn.gate_interchange.request] Each gate request MUST bind the exact candidate, candidate root, pipeline revision, policy, ordered evidence set, provider profile, effect, idempotency key, native gate target, and tool cohort through deterministic BLAKE3 framing.

#### Scenario: Complete request
r[cairn.gate_interchange.request.scenario.valid]
- GIVEN a bounded request with sorted unique evidence identities and supported schemas
- WHEN request admission runs
- THEN Cairn MUST recompute and accept its exact request identity.

#### Scenario: Evidence order or content changes
r[cairn.gate_interchange.request.scenario.evidence]
- GIVEN a request whose evidence order or evidence content differs
- WHEN request admission runs
- THEN Cairn MUST reject the old request identity or the non-canonical evidence order.

### Requirement: Exact gate result admission
r[cairn.gate_interchange.result] Each admitted gate result MUST bind the exact request, repeated applicability facts, tool cohort, native input, native policy, native receipt, findings, verdict, and result identity.

#### Scenario: Passing observation matches
r[cairn.gate_interchange.result.scenario.pass]
- GIVEN a passing native observation with no findings whose target and policy match an admitted request
- WHEN result admission runs
- THEN Cairn MUST admit one passing result scoped to that exact request.

#### Scenario: Candidate or cohort differs
r[cairn.gate_interchange.result.scenario.substitution]
- GIVEN a result whose candidate applicability or tool cohort differs from the request
- WHEN result admission runs
- THEN Cairn MUST reject the result even if its native verdict is passing.

#### Scenario: Verdict contradicts findings
r[cairn.gate_interchange.result.scenario.contradiction]
- GIVEN a result marked passing with findings or marked blocked without findings
- WHEN result admission runs
- THEN Cairn MUST reject the contradictory result.

### Requirement: Unknown execution remains outside result production
r[cairn.gate_interchange.reconciliation] A missing, malformed, timed-out, cancelled, or ambiguously terminated gate execution MUST NOT produce a passing or blocked gate interchange result.

#### Scenario: Process result is missing
r[cairn.gate_interchange.reconciliation.scenario.unknown]
- GIVEN a consumer shell that did not capture one complete admitted native gate observation
- WHEN outcome classification runs
- THEN the shell MUST preserve timeout, cancellation, provider failure, or unknown without fabricating a contract result.

### Requirement: Gate interchange conformance
r[cairn.gate_interchange.conformance] Maintained conformance MUST include deterministic producer fixtures, positive and negative Rust tests, typed Nickel positive and negative fixtures, host-independent compilation, and fixture freshness checks.

#### Scenario: Fixture drifts
r[cairn.gate_interchange.conformance.scenario.drift]
- GIVEN a checked-in producer fixture that differs from the current contract output
- WHEN conformance checks run
- THEN the checks MUST fail.

#### Scenario: Invalid consumer input
r[cairn.gate_interchange.conformance.scenario.negative]
- GIVEN an unsupported schema, invalid identity, unordered evidence, applicability substitution, cohort substitution, or contradictory verdict
- WHEN conformance checks run
- THEN at least one maintained negative case MUST reject it.

### Requirement: Gate interchange boundary
r[cairn.gate_interchange.boundary] Gate interchange documentation and results MUST preserve Cairn lifecycle authority, consumer shell authority, exact applicability scope, and explicit non-claims.

#### Scenario: Bound result passes
r[cairn.gate_interchange.boundary.scenario.scope]
- GIVEN an admitted passing result
- WHEN a consumer records or evaluates it
- THEN the result MUST NOT claim candidate truth, source correctness, test completeness, process reliability, merge authority, release execution, deployment success, or future applicability.
