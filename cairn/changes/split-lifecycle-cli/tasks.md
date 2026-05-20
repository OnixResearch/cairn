## Phase 1: Scope and Implementation

- [ ] [serial] I1 Extract sync/archive/gate/change-context command-family implementation into a focused lifecycle module. r[cli_maintainability.lifecycle_module.contracts.mutation]
- [ ] [serial] I2 Preserve helper boundaries for mutation manifests, archive-date validation, and gate receipts with narrow exports only. r[cli_maintainability.lifecycle_module.shell.delegates]

## Phase 2: Verification

- [ ] [parallel] V1 Run focused sync/archive/gate tests, including dry-run and temp-fixture execute paths. r[cli_maintainability.lifecycle_module.contracts.mutation]
- [ ] [serial] V2 Run full repository verification before commit. r[cli_maintainability.lifecycle_module.shell.delegates]

## Verification Coverage

| Scenario | Verification |
| --- | --- |
| r[cli_maintainability.lifecycle_module.contracts.mutation] | I1 |
| r[cli_maintainability.lifecycle_module.shell.delegates] | I2 |
| r[cli_maintainability.lifecycle_module.contracts.mutation] | V1 |
| r[cli_maintainability.lifecycle_module.shell.delegates] | V2 |
