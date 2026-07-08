# Kani Receipt Gates Specification

## Purpose

Defines the `kani-receipt-gates` capability.

## Requirements

### Requirement: Kani receipt profile
r[cairn.kani_receipt_gates.receipt_profile] Cairn MUST define a Kani receipt profile that binds requirement id, source scope, content hash, policy hash, verifier identity, toolchain identity, property status, contract or unwind state, stub classification, and non-claims.

#### Scenario: Complete Kani receipt profile passes
r[cairn.kani_receipt_gates.receipt_profile.complete]
- GIVEN a Kani receipt contains all required profile fields
- WHEN Cairn validates the receipt
- THEN the profile fields MUST be preserved in deterministic validation output.

#### Scenario: Missing Kani receipt field fails
r[cairn.kani_receipt_gates.receipt_profile.missing]
- GIVEN a Kani receipt lacks a required profile field
- WHEN Cairn validates the receipt
- THEN validation MUST fail with a deterministic missing-field diagnostic.

### Requirement: Kani receipt eligibility
r[cairn.kani_receipt_gates.eligibility] Cairn MUST accept Kani receipts as behavioral verification artifacts only when the receipt is current, passing, scoped to the requirement, supported by appropriate contract or unwind evidence, and carries required non-claims.

#### Scenario: Scoped passing receipt is eligible
r[cairn.kani_receipt_gates.eligibility.scoped]
- GIVEN a Kani receipt passes for the requirement id and current source hash
- WHEN a Cairn gate evaluates the matching requirement
- THEN the receipt MAY satisfy the scoped behavioral artifact requirement.

#### Scenario: Unsupported unbounded claim fails
r[cairn.kani_receipt_gates.eligibility.unbounded]
- GIVEN a Kani receipt lacks passing unwind or contract evidence for an unbounded claim
- WHEN a Cairn gate evaluates the claim
- THEN the gate MUST reject the receipt.

### Requirement: Default gate no-execution boundary
r[cairn.kani_receipt_gates.default_no_execution] Cairn gates MUST verify saved Kani receipts by default and MUST NOT run Kani inline unless a separate explicit probe-execution policy enables it.

#### Scenario: Gate verifies saved receipt
r[cairn.kani_receipt_gates.default_no_execution.saved]
- GIVEN a lifecycle change references a saved Kani receipt
- WHEN Cairn runs a gate
- THEN the gate MUST validate the receipt metadata without invoking Kani.

#### Scenario: Missing saved receipt fails
r[cairn.kani_receipt_gates.default_no_execution.missing]
- GIVEN a lifecycle change requires Kani evidence but no saved receipt exists
- WHEN Cairn runs a gate
- THEN the gate MUST fail with a deterministic missing-receipt diagnostic.

### Requirement: Kani receipt positive fixtures
r[cairn.kani_receipt_gates.positive_fixtures] Cairn MUST include positive fixtures for scoped passing bounded Kani receipts and contract-backed Kani receipts.

#### Scenario: Positive fixtures pass
r[cairn.kani_receipt_gates.positive_fixtures.valid]
- GIVEN valid bounded and contract-backed Kani receipt fixtures
- WHEN Cairn fixture tests run
- THEN both fixtures MUST validate deterministically.

### Requirement: Kani receipt negative fixtures
r[cairn.kani_receipt_gates.negative_fixtures] Cairn MUST include negative fixtures for stale content, failed verdict, missing identity, unsupported unbounded claim, unverified-stub promotion, and weakened non-claims.

#### Scenario: Stale receipt fails
r[cairn.kani_receipt_gates.negative_fixtures.stale]
- GIVEN a Kani receipt content hash does not match current source identity
- WHEN Cairn validates the receipt
- THEN validation MUST fail with a stale-content diagnostic.

#### Scenario: Unverified-stub promotion fails
r[cairn.kani_receipt_gates.negative_fixtures.stub]
- GIVEN a Kani receipt uses an unverified stub but claims property evidence
- WHEN Cairn validates the receipt
- THEN validation MUST fail closed.

### Requirement: Kani gate diagnostics
r[cairn.kani_receipt_gates.gate_diagnostics] Cairn MUST emit deterministic diagnostics that name why a Kani receipt is ineligible.

#### Scenario: Diagnostic names failed condition
r[cairn.kani_receipt_gates.gate_diagnostics.named]
- GIVEN Kani receipt validation fails
- WHEN Cairn emits the gate result
- THEN the diagnostic MUST identify the failed eligibility condition.

### Requirement: Kani receipt operator documentation
r[cairn.kani_receipt_gates.operator_docs] Cairn documentation MUST explain that Kani runs are probe or CI steps and gates verify saved receipts by default.

#### Scenario: Operator docs show receipt workflow
r[cairn.kani_receipt_gates.operator_docs.visible]
- GIVEN an operator reads Cairn Kani receipt documentation
- WHEN the workflow is described
- THEN the documentation MUST say to run Kani before gates and save the receipt for gate verification.
