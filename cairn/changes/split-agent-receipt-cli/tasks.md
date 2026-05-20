## Phase 1: Scope and Implementation

- [ ] [serial] I1 Extract agent receipt command-family implementation into `crates/cairn-cli/src/agent_receipt.rs`. r[cli_maintainability.agent_receipt_module.contracts.validation]
- [ ] [serial] I2 Wire dispatch and narrow bridge exports while preserving JSON and issue-code contracts. r[cli_maintainability.agent_receipt_module.shell.delegates]

## Phase 2: Verification

- [ ] [parallel] V1 Run focused agent receipt validation and bundle verification tests. r[cli_maintainability.agent_receipt_module.contracts.bundle]
- [ ] [serial] V2 Run full repository verification before commit. r[cli_maintainability.agent_receipt_module.shell.delegates]

## Verification Coverage

| Scenario | Verification |
| --- | --- |
| r[cli_maintainability.agent_receipt_module.contracts.validation] | I1 |
| r[cli_maintainability.agent_receipt_module.shell.delegates] | I2 |
| r[cli_maintainability.agent_receipt_module.contracts.bundle] | V1 |
| r[cli_maintainability.agent_receipt_module.shell.delegates] | V2 |
