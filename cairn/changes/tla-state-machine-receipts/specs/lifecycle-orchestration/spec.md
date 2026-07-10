# Lifecycle Orchestration Specification

## Purpose

Add a bounded state-machine receipt profile for Cairn lifecycle transition evidence.

## Requirements

### Requirement: State-machine receipt model
r[cairn.lifecycle_orchestration.state_machine_receipts.model] Cairn MUST define a pure state-machine receipt model that records lifecycle model artifact identity, transition relation profile, state/event vocabulary, checked property labels, checker identity, result status, diagnostics, input/output hashes, verification role, and explicit non-claims.

#### Scenario: Passing lifecycle FSM receipt validates
r[cairn.lifecycle_orchestration.state_machine_receipts.fixtures.positive]
- GIVEN an external model-check or simulation run emits a receipt for a Cairn lifecycle transition model with matching artifact hashes, checker identity, property labels, diagnostics, and non-claims
- WHEN Cairn validates the state-machine receipt
- THEN validation MUST accept the receipt as bounded lifecycle transition evidence.

### Requirement: Receipt validation fails closed
r[cairn.lifecycle_orchestration.state_machine_receipts.validation] Cairn MUST reject stale, unchecked, unknown, malformed, or overclaiming state-machine receipts.

#### Scenario: Invalid state-machine receipt is rejected
r[cairn.lifecycle_orchestration.state_machine_receipts.fixtures.negative]
- GIVEN a state-machine receipt has a stale model hash, unknown checker identity, missing property label, unchecked status, promoted verification role, missing diagnostic, or missing non-claim boundary
- WHEN receipt validation runs
- THEN validation MUST fail with deterministic diagnostics and MUST NOT count the receipt as lifecycle readiness evidence.

### Requirement: State-machine receipt validation is shell-separated
r[cairn.lifecycle_orchestration.state_machine_receipts.cli] Cairn MUST keep TLA+ parsing, model checking, simulation, code generation, filesystem reads, stdout/stderr, JSON rendering, and process exits outside the pure receipt core.

#### Scenario: CLI validates an existing receipt
- GIVEN an operator provides an already-produced state-machine receipt file
- WHEN Cairn CLI validates it
- THEN the CLI MAY read and render the receipt, but core validation MUST operate only on parsed in-memory receipt values.

### Requirement: State-machine receipts remain bounded witnesses
r[cairn.lifecycle_orchestration.state_machine_receipts.final_validation] Cairn MUST state that accepted state-machine receipts are bounded witnesses for named lifecycle transition properties only.

#### Scenario: Model evidence does not replace lifecycle gates
- GIVEN a state-machine receipt passes validation
- WHEN Cairn release-readiness or lifecycle gates consume it
- THEN the receipt MUST NOT prove Cairn implementation correctness, repository correctness, source-code correctness, exhaustive lifecycle safety, release eligibility, verifier soundness, or downstream behavioral correctness.