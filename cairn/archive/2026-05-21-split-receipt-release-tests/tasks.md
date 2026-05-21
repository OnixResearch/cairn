## Phase 1: Test split

- [x] [serial] I1 Inspect dependencies and choose the narrow receipt/release test family to move out of `crates/cairn-cli/tests/cli.rs`. r[cli_maintainability.receipt_release_tests.focused_modules.visible_boundaries]
- [x] [serial] I2 Move public receipt schema, agent receipt replay, release-readiness bundle, and signed evidence manifest tests into a focused integration target with narrow local helpers. r[cli_maintainability.receipt_release_tests.focused_modules.visible_boundaries] r[cli_maintainability.receipt_release_tests.focused_modules.narrow_bridges]
- [x] [serial] I3 Preserve command behavior, deterministic receipts, issue codes, fixture output, policy hashes, and Tracey references. r[cli_maintainability.receipt_release_tests.behavior_preserved.focused_checks]

## Phase 2: Verification

- [x] [serial] V1 Run focused verification for the new receipt/release target plus the remaining `cli` target. r[cli_maintainability.receipt_release_tests.behavior_preserved.focused_checks]
- [x] [serial] V2 Run repository validation, Tracey coverage, formatting, linting, and diff checks. r[cli_maintainability.receipt_release_tests.behavior_preserved.repository_checks]

## Verification Coverage

| Scenario | Verification |
| --- | --- |
| r[cli_maintainability.receipt_release_tests.focused_modules.visible_boundaries] | I1, I2 |
| r[cli_maintainability.receipt_release_tests.focused_modules.narrow_bridges] | I2 |
| r[cli_maintainability.receipt_release_tests.behavior_preserved.focused_checks] | I3, V1 |
| r[cli_maintainability.receipt_release_tests.behavior_preserved.repository_checks] | V2 |
