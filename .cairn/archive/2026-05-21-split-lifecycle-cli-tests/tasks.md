## Phase 1: Scope and Implementation

- [x] [serial] I1 Create `crates/cairn-cli/tests/lifecycle.rs` and move the lifecycle/init/change/status/validate/gate/sync/archive/migration test cluster into it. r[cli_maintainability.lifecycle_tests.coverage]
- [x] [serial] I2 Keep only minimal local helpers or narrow shared imports needed by the moved lifecycle tests. r[cli_maintainability.lifecycle_tests.behavior_preserving]

## Phase 2: Verification

- [x] [serial] V1 Run the lifecycle integration test target and the remaining CLI integration target. r[cli_maintainability.lifecycle_tests.coverage]
- [x] [serial] V2 Run workspace verification, Cairn validate, Tracey coverage, and diff checks. r[cli_maintainability.lifecycle_tests.behavior_preserving]

## Verification Coverage

| Scenario | Verification |
| --- | --- |
| r[cli_maintainability.lifecycle_tests.coverage] | I1, V1 |
| r[cli_maintainability.lifecycle_tests.behavior_preserving] | I2, V2 |
