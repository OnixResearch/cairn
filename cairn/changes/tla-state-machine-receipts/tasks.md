## Tasks

- [ ] [serial] r[cairn.lifecycle_orchestration.state_machine_receipts.model] Define the pure state-machine receipt model for lifecycle transition evidence.
- [ ] [depends:state-machine-receipt-model] r[cairn.lifecycle_orchestration.state_machine_receipts.validation] Validate model refs, transition profiles, property labels, checker identity, result status, diagnostics, hashes, roles, and non-claims.
- [ ] [depends:state-machine-receipt-validation] r[cairn.lifecycle_orchestration.state_machine_receipts.cli] Add a read-only CLI validation path for already-produced state-machine receipts.
- [ ] [depends:state-machine-receipt-validation] r[cairn.lifecycle_orchestration.state_machine_receipts.fixtures.positive] Add positive fixtures for passing lifecycle FSM and bounded simulation receipts.
- [ ] [depends:state-machine-receipt-validation] r[cairn.lifecycle_orchestration.state_machine_receipts.fixtures.negative] Add negative fixtures for stale model hash, unknown checker, missing property label, unchecked status, role promotion, and missing non-claims.
- [ ] [depends:state-machine-receipt-fixtures] r[cairn.lifecycle_orchestration.state_machine_receipts.final_validation] Run focused receipt tests, CLI smoke, core purity smoke, Cairn gates, and validation.