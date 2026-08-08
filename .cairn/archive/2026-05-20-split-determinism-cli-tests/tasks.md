## Phase 1: Scope and Implementation

- [x] [serial] I1 Create `crates/cairn-cli/tests/determinism.rs` and move the determinism/replay test cluster into it. r[cli_maintainability.determinism_tests.coverage.replay]
- [x] [serial] I2 Keep only minimal local helpers or narrow shared imports needed for the moved tests. r[cli_maintainability.determinism_tests.behavior_preserving.focused_full]

## Phase 2: Verification

- [x] [parallel] V1 Run the determinism integration test target and relevant focused filters. r[cli_maintainability.determinism_tests.coverage.replay]
- [x] [serial] V2 Run full workspace verification and confirm `tests/cli.rs` shrank without losing coverage. r[cli_maintainability.determinism_tests.behavior_preserving.focused_full]

## Verification Coverage

| Scenario | Verification |
| --- | --- |
| r[cli_maintainability.determinism_tests.coverage.replay] | I1 |
| r[cli_maintainability.determinism_tests.behavior_preserving.focused_full] | I2 |
| r[cli_maintainability.determinism_tests.coverage.replay] | V1 |
| r[cli_maintainability.determinism_tests.behavior_preserving.focused_full] | V2 |
