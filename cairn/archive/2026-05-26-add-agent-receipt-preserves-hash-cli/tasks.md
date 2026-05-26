# Tasks

## Implementation

- [x] [serial] I1 Add `agent-receipt preserves-hash <path>` dispatch and deterministic receipt builder. r[cairn.determinism.preserves_hash_cli.command]
- [x] [serial] I2 Reuse bounded agent receipt file loading and internal `agent_run_preserves_receipt_hash`. r[cairn.determinism.preserves_hash_cli.bounded_read]
- [x] [serial] I3 Preserve the existing JSON `receipt_hash` validation surface unchanged. r[cairn.determinism.preserves_hash_cli.no_schema_churn]

## Validation

- [x] [serial] V1 Add positive fixture, reproducibility, and non-object failure tests for the CLI receipt. r[cairn.determinism.preserves_hash_cli.validation]
- [x] [serial] V2 Run focused CLI/core tests, format, Cairn validation/gates, and Tracey coverage. r[cairn.determinism.preserves_hash_cli.validation]
