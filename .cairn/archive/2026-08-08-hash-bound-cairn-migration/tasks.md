## Phase 1: Plan binding

- [x] [serial] Add a pure stable migration plan projection and BLAKE3 `plan_hash`, with positive stability and negative drift tests. r[cairn.lifecycle_directory.legacy_migration.plan_binding]
- [x] [serial] Require a matching `--plan-hash` for execution and reject missing or stale hashes before mutation. r[cairn.lifecycle_directory.legacy_migration.plan_binding.execute]

## Phase 2: Publication and diagnostics

- [x] [serial] Recheck source bytes and destination absence, then publish with no-replace semantics and negative race fixtures. r[cairn.lifecycle_directory.legacy_migration.no_replace]
- [x] [serial] Reject normal commands on legacy-only lifecycle roots without reading legacy content as runtime state. r[cairn.lifecycle_directory.legacy_diagnostic]

## Phase 3: Documentation and completion

- [x] [serial] Update CLI usage and operator documentation for hash-bound migration execution. r[cairn.lifecycle_directory.legacy_migration.plan_binding]
- [x] [serial] Run focused and full checks, sync the accepted specification, archive the change, and record final evidence. r[cairn.lifecycle_directory.legacy_migration.plan_binding]
