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
