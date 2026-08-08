# Tasks

- [x] [serial] Split agent receipt pure-core implementation into focused modules while preserving the `verified::agent_receipt` facade. r[cli_maintainability.agent_receipt_core_modules.focused_modules]
- [x] [serial] Keep public API, receipt hash material, schema constants, issue codes/messages, and deterministic fixture behavior unchanged. r[cli_maintainability.agent_receipt_core_modules.behavior_preserved]
- [x] [serial] Add durable Tracey implementation and verification refs for the split. r[cli_maintainability.agent_receipt_core_modules.focused_modules.narrow_bridges]
- [x] [serial] Run focused agent receipt core tests and repository validation gates. r[cli_maintainability.agent_receipt_core_modules.behavior_preserved.focused_checks]
- [x] [serial] Run full workspace checks, Tracey coverage, and flake confidence before archive/commit. r[cli_maintainability.agent_receipt_core_modules.behavior_preserved.repository_checks]
