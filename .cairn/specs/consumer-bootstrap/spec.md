# Consumer Bootstrap Specification

## Purpose

Defines the `consumer-bootstrap` capability.

## Requirements

### Requirement: Explicit policy selection
r[cairn.consumer_bootstrap.policy] The system MUST select a policy through an explicit option or checked project declaration and report its exact source and BLAKE3 identity.

#### Scenario: Clean setup supplies a usable policy
- GIVEN a clean root and an explicit pinned bootstrap cohort
- WHEN the operator applies the reviewed setup plan
- THEN ordinary validation resolves the installed policy without a Cairn source checkout

#### Scenario: An invalid explicit policy cannot fall back
- GIVEN an invalid explicit policy and another available default policy
- WHEN the operator requests validation
- THEN the command reports the selected policy error without substituting the default

### Requirement: Checked cohort and upgrade compatibility
r[cairn.consumer_bootstrap.compatibility] The system MUST admit setup and upgrade cohorts through a typed Nickel contract that binds tool, policy, export, profile, and compatibility identities.

#### Scenario: A declared upgrade preserves consumer policy
- GIVEN a compatible cohort and unchanged managed files
- WHEN the operator previews the upgrade
- THEN the plan preserves consumer-owned policy and reports exact source and destination identities

#### Scenario: Unsupported or stale cohorts remain blocked
- GIVEN an unsupported policy version or stale Nickel Export manifest
- WHEN the operator requests setup or upgrade
- THEN the system reports a compatibility blocker before publication

### Requirement: Ownership-aware publication and recovery
r[cairn.consumer_bootstrap.publication] The system MUST recheck reviewed plans under a project lock and preserve ownership, partial publication, durability uncertainty, and explicit recovery state.

#### Scenario: Unchanged plan publishes managed artifacts
- GIVEN a reviewed plan whose source, policy, destination, and ownership identities remain current
- WHEN the operator applies the exact plan identity
- THEN the shell publishes the managed files and reports their observed final identities

#### Scenario: Drift or interrupted publication cannot appear complete
- GIVEN destination drift or an interruption before the complete installation becomes visible
- WHEN the operator applies or checks the installation
- THEN the system blocks completion and preserves the last valid state or an explicit recovery state

### Requirement: Current project-local guidance
r[cairn.consumer_bootstrap.guidance] The system MUST compose existing generated-workflow freshness checks with doctor diagnostics for explicitly declared project-local guidance.

#### Scenario: Guidance matches the selected policy
- GIVEN current generated workflows for the selected policy and native layout
- WHEN the operator runs the setup check or doctor
- THEN the report identifies current guidance and its source manifest

#### Scenario: Modified guidance cannot be overwritten automatically
- GIVEN modified, unmanaged, missing-managed, or symlinked guidance
- WHEN the operator requests an upgrade
- THEN the system blocks replacement and reports the exact manual repair category

### Requirement: Capability and compatibility boundaries
r[cairn.consumer_bootstrap.boundary] The system MUST keep filesystem authority in the shell, retain low-level init compatibility, and prohibit hidden network, global-guidance, or runtime-evaluator fallback.

#### Scenario: Preview and normal checks remain bounded
- GIVEN explicit safe roots and declared inputs
- WHEN the operator previews setup or runs normal validation
- THEN preview performs no publication and normal validation does not evaluate Nickel or read global guidance

#### Scenario: Unsafe paths and legacy substitution fail closed
- GIVEN a parent escape, symlink substitution, or legacy-only lifecycle root
- WHEN the system admits the operation
- THEN it rejects the unsafe path or requires explicit legacy migration without overwriting content

### Requirement: Repeatable consumer acceptance
r[cairn.consumer_bootstrap.conformance] The system MUST retain positive and negative consumer fixtures for setup, upgrades, compatibility, publication, recovery, and guidance freshness.

#### Scenario: Fresh consumer reproduces the supported path
- GIVEN the pinned CLI, policy cohort, and maintained consumer fixture
- WHEN the fixture repeats setup and validation from an empty root
- THEN the expected semantic receipts agree and validation succeeds without ambient project state

#### Scenario: The policy-gap control remains observable
- GIVEN a low-level initialized root without the required policy declaration
- WHEN the fixture runs validation without an explicit policy
- THEN the command reports the policy gap and the explicit-policy control succeeds
