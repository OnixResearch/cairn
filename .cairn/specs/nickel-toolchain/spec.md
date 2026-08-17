# Nickel Toolchain Specification

## Purpose

Defines the `nickel-toolchain` capability.

## Requirements

### Requirement: Cairn pins Nickel 1.17 exactly

r[cairn.nickel_toolchain.cohort] Cairn MUST use Nickel `1.17.0` from upstream commit `1320a983e6c3d1e2fb53dd2464b084b4903b1426` for lifecycle policy and contracts.

#### Scenario: Policy tools resolve

- GIVEN Cairn builds policy checks and self-dogfood
- WHEN cohort admission runs
- THEN every Nickel command MUST use the exact reviewed evaluator

#### Scenario: An older or floating evaluator resolves

- GIVEN an ambient package, branch, or older pin supplies Nickel
- WHEN admission runs
- THEN admission MUST fail

### Requirement: Runtime remains evaluator-free

r[cairn.nickel_toolchain.runtime_boundary] Runtime Rust MUST continue to consume checked-in generated policy artifacts. It MUST NOT execute Nickel at runtime.

#### Scenario: Policy is loaded

- GIVEN the generated policy is fresh
- WHEN Cairn starts a runtime command
- THEN it MUST read the generated artifact
- AND it MUST NOT start a Nickel evaluator

### Requirement: Policy freshness and rejection remain checked

r[cairn.nickel_toolchain.compatibility] Cairn MUST regenerate policy with Nickel `1.17.0` and test valid, malformed, stale, unknown, import, contract, and type-mismatched fixtures.

#### Scenario: The compatibility matrix runs

- GIVEN valid and invalid lifecycle policy fixtures
- WHEN the new evaluator processes them
- THEN valid fixtures MUST retain supported output
- AND invalid fixtures MUST fail closed

### Requirement: Lockfiles are tool-generated

r[cairn.nickel_toolchain.lockfile] Cairn MUST change Nickel source inputs in `flake.nix`. Nix MUST generate any `flake.lock` change.

#### Scenario: The source pin changes

- GIVEN the exact upstream input is declared
- WHEN the lockfile updates
- THEN a Nix lock command MUST produce the change

### Requirement: The repository validation rail passes

r[cairn.nickel_toolchain.validation] The change MUST pass policy freshness, positive and negative fixtures, self-dogfood, formatting, Clippy, Cairn gates, and Nix checks.

#### Scenario: Validation completes

- GIVEN the pin, generated policy, fixtures, and evidence are current
- WHEN validation runs
- THEN every required check MUST pass or report one exact blocker
