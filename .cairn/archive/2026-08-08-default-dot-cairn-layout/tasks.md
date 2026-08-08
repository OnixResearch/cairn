## Phase 1: Lifecycle contract

- [x] [serial] Change the pure native layout helpers to `.cairn/` and add the old `cairn/` directory as an explicit migration source. r[cairn.lifecycle_directory.hidden_default] r[cairn.lifecycle_directory.legacy_migration]
- [x] [serial] Preserve the stable native layout receipt name while exposing exact hidden paths in actions and content records. r[cairn.lifecycle_directory.receipt_compatibility]

## Phase 2: Shell and repository migration

- [x] [serial] Update CLI, MCP, collaboration, compaction, lifecycle-store, policy, fixture, and golden paths. r[cairn.lifecycle_directory.hidden_default] r[cairn.lifecycle_directory.legacy_migration]
- [x] [serial] Move maintained repository lifecycle content and fixture lifecycle roots to `.cairn/`. r[cairn.lifecycle_directory.hidden_default]
- [x] [serial] Update current specs, documentation, examples, and the repository-maintained workflow skill. r[cairn.lifecycle_directory.hidden_default] r[cairn.lifecycle_directory.legacy_migration]

## Phase 3: Verification and completion

- [x] [serial] Add positive tests for initialization and migration, plus negative tests for fallback and destination conflicts. r[cairn.lifecycle_directory.hidden_default] r[cairn.lifecycle_directory.legacy_migration]
- [x] [serial] Run focused lifecycle tests, policy freshness, validation, traceability, and full repository checks. r[cairn.lifecycle_directory.hidden_default] r[cairn.lifecycle_directory.legacy_migration] r[cairn.lifecycle_directory.receipt_compatibility]
- [x] [serial] Sync the accepted specification, archive the change, and record final evidence. r[cairn.lifecycle_directory.hidden_default] r[cairn.lifecycle_directory.legacy_migration] r[cairn.lifecycle_directory.receipt_compatibility]
