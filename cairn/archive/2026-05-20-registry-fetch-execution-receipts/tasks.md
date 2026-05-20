## Phase 1: Scope and Implementation

- [x] [serial] I1 Design the fetch execution command shape, receipt schema, and linkage to existing registry fetch-plan receipts. r[evidence_registry_execution.fetch.receipt.materializes]
- [x] [serial] I2 Implement local/offline artifact materialization with safe relative paths and hash verification. r[evidence_registry_execution.fetch.safety.unsafe_path]

## Phase 2: Verification

- [x] [parallel] I3 Emit deterministic failure receipts for missing artifacts, unverified plans, hash mismatches, and unsafe paths. r[evidence_registry_execution.fetch.receipt.hash_mismatch]
- [x] [parallel] V1 Add positive and negative fixture-backed CLI tests with no network dependency. r[evidence_registry_execution.fetch.safety.no_network]
- [x] [serial] V2 Run policy/schema updates if needed, focused registry tests, full workspace tests, Cairn validate, Tracey, and flake check. r[evidence_registry_execution.fetch.safety.no_network]

## Verification Coverage

| Scenario | Verification |
| --- | --- |
| r[evidence_registry_execution.fetch.receipt.materializes] | I1 |
| r[evidence_registry_execution.fetch.safety.unsafe_path] | I2 |
| r[evidence_registry_execution.fetch.receipt.hash_mismatch] | I3 |
| r[evidence_registry_execution.fetch.safety.no_network] | V1 |
| r[evidence_registry_execution.fetch.safety.no_network] | V2 |
