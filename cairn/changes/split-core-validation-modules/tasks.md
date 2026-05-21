## Phase 1: Source or test split

- [ ] [serial] I1 Inspect dependencies and choose the smallest safe module/test-target boundaries for `crates/cairn-core/src/verified/validation.rs` (~792 lines). r[cli_maintainability.core_validation_modules.focused_modules.visible_boundaries]
- [ ] [serial] I2 Move pure validation core helpers and issue construction into focused modules or test targets while keeping facade dispatch and bridge APIs narrow. r[cli_maintainability.core_validation_modules.focused_modules.visible_boundaries] r[cli_maintainability.core_validation_modules.focused_modules.narrow_bridges]
- [ ] [serial] I3 Preserve command behavior, deterministic receipts, issue codes, fixture output, and Tracey references. r[cli_maintainability.core_validation_modules.behavior_preserved.focused_checks]

## Phase 2: Verification

- [ ] [serial] V1 Run focused verification for pure validation core helpers and issue construction. r[cli_maintainability.core_validation_modules.behavior_preserved.focused_checks]
- [ ] [serial] V2 Run repository validation, Tracey coverage, formatting, linting, and diff checks. r[cli_maintainability.core_validation_modules.behavior_preserved.repository_checks]

## Verification Coverage

| Scenario | Verification |
| --- | --- |
| r[cli_maintainability.core_validation_modules.focused_modules.visible_boundaries] | I1, I2 |
| r[cli_maintainability.core_validation_modules.focused_modules.narrow_bridges] | I2 |
| r[cli_maintainability.core_validation_modules.behavior_preserved.focused_checks] | I3, V1 |
| r[cli_maintainability.core_validation_modules.behavior_preserved.repository_checks] | V2 |
