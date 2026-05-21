## ADDED Requirements

### Requirement: Evidence registry CLI tests are isolated

r[cairn.supersede.split_evidence_registry_cli_tests.focused_target] The Cairn CLI test suite MUST keep the portable evidence registry chain coverage in a dedicated integration-test target while preserving the existing CLI behavior proof.

#### Scenario: Focused evidence registry test target

r[cairn.supersede.split_evidence_registry_cli_tests.focused_target.run]

- GIVEN the evidence export/import/republish/index/registry/mirror/select/fetch-plan/fetch-execute chain is covered by integration tests
- WHEN maintainers need to verify evidence registry behavior
- THEN they can run a focused evidence-registry integration-test target without running the whole catch-all CLI test file

### Requirement: Behavior-preserving evidence registry test split

r[cairn.supersede.split_evidence_registry_cli_tests.behavior_preserving] The evidence registry test split MUST preserve the existing positive and negative assertions for public CLI behavior, deterministic receipts, and failure cases.

#### Scenario: Moved assertions remain intact

r[cairn.supersede.split_evidence_registry_cli_tests.behavior_preserving.assertions]

- GIVEN the test topology is refactored
- WHEN the moved evidence-registry tests run
- THEN they preserve the existing positive and negative assertions for receipts, hashes, safe paths, missing artifacts, and hash mismatches

### Requirement: Catch-all CLI test file shrinks

r[cairn.supersede.split_evidence_registry_cli_tests.cli_shrinks] The catch-all CLI integration-test file SHOULD no longer own the full portable evidence registry chain proof after the split.

#### Scenario: Registry chain proof moves out of cli.rs

r[cairn.supersede.split_evidence_registry_cli_tests.cli_shrinks.registry_chain]

- GIVEN the registry/evidence mega test has a dedicated target
- WHEN maintainers inspect `crates/cairn-cli/tests/cli.rs`
- THEN the catch-all file no longer owns the full portable evidence registry chain proof
