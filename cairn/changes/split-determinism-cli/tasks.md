## Phase 1: Scope and Implementation

- [ ] [serial] I1 Extract determinism/replay command-family implementation into `crates/cairn-cli/src/determinism.rs`. r[cli_maintainability.determinism_module.behavior.replay]
- [ ] [serial] I2 Wire top-level dispatch through the new module and keep only narrow crate-visible bridge exports. r[cli_maintainability.determinism_module.shell.delegates]

## Phase 2: Verification

- [ ] [parallel] V1 Run focused determinism CLI tests and compare existing receipt/failure contracts. r[cli_maintainability.determinism_module.behavior.verify_receipt]
- [ ] [serial] V2 Run fmt, clippy, workspace tests, Cairn validate, Tracey coverage, diff checks, and flake check before commit. r[cli_maintainability.determinism_module.shell.delegates]

## Verification Coverage

| Scenario | Verification |
| --- | --- |
| r[cli_maintainability.determinism_module.behavior.replay] | I1 |
| r[cli_maintainability.determinism_module.shell.delegates] | I2 |
| r[cli_maintainability.determinism_module.behavior.verify_receipt] | V1 |
| r[cli_maintainability.determinism_module.shell.delegates] | V2 |
