## ADDED Requirements

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
