# Release Evidence Specification

## Purpose

Commit reproducible positive and negative stack smoke fixtures for function-address evidence exchange.

## Requirements

### Requirement: Stack smoke fixture is reproducible
r[cairn.release_evidence.function_address_stack_smoke.fixture] Cairn MUST provide or reference a reproducible function-address stack smoke fixture that exercises Octet, Kamacite, Valence, Mantle, and Cairn evidence exchange.

#### Scenario: Positive smoke passes
r[cairn.release_evidence.function_address_stack_smoke.expected_outputs]
- GIVEN the committed smoke fixture source crate and expected artifacts
- WHEN the documented stack smoke command runs
- THEN Octet, Kamacite, Valence, Mantle, and Cairn MUST produce matching positive evidence and Cairn readiness MUST pass for complete required evidence.

### Requirement: Smoke uses first-class commands
r[cairn.release_evidence.function_address_stack_smoke.first_class_commands] The committed stack smoke MUST use first-class repo commands for Octet export, Kamacite receipt projection, Valence validation, Mantle binding, and Cairn readiness once those commands exist.

#### Scenario: No temporary adapter required
r[cairn.release_evidence.function_address_stack_smoke.validation]
- GIVEN all first-class function-address commands have landed
- WHEN the stack smoke runs
- THEN it MUST NOT compile or run smoke-specific adapter code outside the owned repo commands.

### Requirement: Negative smoke cases exercise boundaries
r[cairn.release_evidence.function_address_stack_smoke.negative] The stack smoke fixture MUST include negative cases for stale function address, invalid span, stale receipt identity, wrong Valence role, wrong Mantle claim scope, missing required Cairn evidence, and overclaiming non-claim text.

#### Scenario: Smoke docs preserve non-claims
r[cairn.release_evidence.function_address_stack_smoke.docs]
- GIVEN the stack smoke passes
- WHEN an operator reads the smoke documentation
- THEN the docs MUST state that the smoke proves evidence exchange and readiness policy conformance only, not Rust semantic correctness, compiler correctness, whole-program safety, verifier soundness, or release eligibility.
