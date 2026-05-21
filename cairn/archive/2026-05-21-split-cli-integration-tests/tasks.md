## Phase 1: Source or test split

- [x] [serial] I1 Inspect dependencies and choose the smallest safe module/test-target boundaries for `crates/cairn-cli/tests/cli.rs` (~1,671 lines). r[cli_maintainability.cli_integration_tests.focused_modules.visible_boundaries]
- [x] [serial] I2 Move remaining monolithic CLI integration-test families into focused modules or test targets while keeping facade dispatch and bridge APIs narrow. r[cli_maintainability.cli_integration_tests.focused_modules.visible_boundaries] r[cli_maintainability.cli_integration_tests.focused_modules.narrow_bridges]
- [x] [serial] I3 Preserve command behavior, deterministic receipts, issue codes, fixture output, and Tracey references. r[cli_maintainability.cli_integration_tests.behavior_preserved.focused_checks]

## Phase 2: Verification

- [x] [serial] V1 Run focused verification for remaining monolithic CLI integration-test families. r[cli_maintainability.cli_integration_tests.behavior_preserved.focused_checks]
- [x] [serial] V2 Run repository validation, Tracey coverage, formatting, linting, and diff checks. r[cli_maintainability.cli_integration_tests.behavior_preserved.repository_checks]

## Verification Coverage

| Scenario | Verification |
| --- | --- |
| r[cli_maintainability.cli_integration_tests.focused_modules.visible_boundaries] | I1, I2 |
| r[cli_maintainability.cli_integration_tests.focused_modules.narrow_bridges] | I2 |
| r[cli_maintainability.cli_integration_tests.behavior_preserved.focused_checks] | I3, V1 |
| r[cli_maintainability.cli_integration_tests.behavior_preserved.repository_checks] | V2 |
