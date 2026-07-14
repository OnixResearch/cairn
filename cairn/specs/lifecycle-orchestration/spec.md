# Lifecycle Orchestration Specification

## Purpose

Defines the `lifecycle-orchestration` capability.

## Requirements

### Requirement: Steel lifecycle orchestration [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration]]
The system MUST support optional Steel-based lifecycle orchestration that composes existing Cairn lifecycle primitives without moving trusted validation, policy enforcement, effect execution, or receipt hashing out of Rust/Nickel-owned surfaces.

#### Scenario: Steel scripts produce typed lifecycle action envelopes [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.typed_action_envelopes]]
- GIVEN a reviewed Steel lifecycle script is selected
- WHEN Cairn evaluates the script for a lifecycle workflow
- THEN the script MUST produce typed Cairn action envelopes or call explicitly exposed host functions
- AND Rust MUST validate every requested action before any effect is executed

#### Scenario: Rust owns authority and effects [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.rust_owned_authority]]
- GIVEN a Steel script requests filesystem, process, network, lifecycle mutation, or evidence-writing behavior
- WHEN Cairn handles the request
- THEN Rust MUST authorize the request through policy and runtime authority before execution
- AND Steel MUST NOT receive ambient authority to mutate repository state or execute host effects directly

#### Scenario: Policy declares Steel orchestration surfaces [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.policy_declared_surface]]
- GIVEN Steel lifecycle orchestration is enabled
- WHEN Cairn loads orchestration policy
- THEN policy MUST declare allowed script profiles, host functions, deterministic budgets, redaction classes, fallback modes, and receipt schema versions
- AND malformed or missing policy declarations MUST fail closed with deterministic diagnostics

#### Scenario: Steel orchestration receipts are deterministic and redacted [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.deterministic_receipts]]
- GIVEN a Steel orchestration attempt completes, is denied, or falls back
- WHEN Cairn emits the orchestration receipt
- THEN the receipt MUST include stable script identity, profile identity, requested capabilities, authorized action summary, denial or fallback reason when present, output hashes, and receipt schema version
- AND the receipt MUST NOT include credentials, raw host secrets, private provider payloads, or unredacted environment values

#### Scenario: Cairn works without Steel [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.optional_fallback]]
- GIVEN Steel is unavailable, disabled, or denied by policy
- WHEN an operator runs existing Cairn lifecycle commands
- THEN Cairn MUST preserve the existing Rust/Nickel lifecycle behavior
- AND any command that attempted Steel orchestration MUST produce a deterministic disabled, denied, or fallback receipt instead of silently changing behavior

#### Scenario: Steel does not replace core lifecycle primitives [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.no_core_replacement]]
- GIVEN Cairn validates specs, gates proposals, syncs or archives changes, verifies Tracey coverage, exports policy, or emits release evidence
- WHEN Steel orchestration is used to compose those operations
- THEN the underlying primitive MUST remain the existing Rust/Nickel-owned implementation
- AND Steel MUST NOT become the canonical implementation of validation, mutation manifests, receipt hashing, policy export, Tracey coverage, Nix verification, or release-readiness checks

#### Scenario: Steel default promotion is evidence-gated [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.evidence_gated_default]]
- GIVEN Cairn supports Steel lifecycle orchestration
- WHEN a workflow is promoted from disabled, comparison, or opt-in mode toward default Steel orchestration
- THEN policy MUST name the workflow profile, fallback mode, required comparison evidence, and required receipt checks
- AND promotion MUST fail closed unless deterministic receipts prove equivalent requested actions, stable denial and fallback behavior, CI reproducibility, and operator-visible debugging evidence for the promoted workflow

#### Scenario: Comparison mode cannot execute effects [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.comparison_mode_no_effects]]
- GIVEN Steel is configured in comparison mode for a lifecycle workflow
- WHEN Cairn evaluates the Steel script beside the existing Rust/Nickel workflow
- THEN the Steel path MUST produce plan and comparison receipts without executing filesystem, process, network, lifecycle mutation, or evidence-writing effects
- AND the Rust/Nickel workflow MUST remain the only effecting path for that run

#### Scenario: Steel orchestration CLI is documented for operators [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.operator_docs]]
- GIVEN an operator reviews Cairn's public Steel orchestration documentation
- WHEN they inspect the `steel-orchestration receipt` and `steel-orchestration default-gate` examples
- THEN the documentation MUST show the disabled-by-default receipt behavior, default-gate failure under the default disabled policy, and selected-default gate shape with complete evidence hashes
- AND the documentation MUST state that Steel orchestration receipts are deterministic, redacted, and do not prove host effects executed

### Requirement: Steel orchestration CLI boundary split
r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.boundary_split] The system SHOULD keep Steel orchestration CLI dispatch, policy/profile selection, script evaluation, and deterministic receipt construction in focused Rust modules without changing operator-visible behavior.

#### Scenario: Dispatch stays a thin CLI shell
r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.boundary_split.dispatch]
- GIVEN an operator runs `cairn steel-orchestration receipt` or `default-gate`
- WHEN the CLI dispatches the subcommand
- THEN the dispatch layer SHALL only parse command shape, load policy inputs, and print deterministic receipt JSON from narrower builders.

#### Scenario: Script evaluation remains Rust-owned
r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.boundary_split.evaluation]
- GIVEN a reviewed Steel script envelope is provided, missing, malformed, or denied
- WHEN Cairn evaluates the request
- THEN Rust SHALL hash, validate, redact, and classify the requested actions without granting Steel ambient host authority.

#### Scenario: Receipt builders preserve JSON contracts
r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.boundary_split.receipts]
- GIVEN receipt and default-gate builders receive the same policy/profile/script inputs as before
- WHEN they materialize receipt JSON
- THEN the existing fields, verdicts, fallback reasons, evidence checks, and receipt hash behavior SHALL be preserved.

#### Scenario: Split is behavior-preserving
r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.boundary_split.tests]
- GIVEN the Steel orchestration source has been split into focused modules
- WHEN focused CLI and module tests run
- THEN disabled, planned, denied, fallback, redaction, and default-gate cases SHALL continue to pass without executing host effects.

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
