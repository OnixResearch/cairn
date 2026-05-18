## ADDED Requirements

### Requirement: Sync dry-run plan
r[sync_archive.plan.sync] The system MUST compute a deterministic sync dry-run plan for a named change without mutating repository files.

#### Scenario: Sync plan reports delta specs
r[sync_archive.plan.sync.delta_specs]
- GIVEN a change with delta specs
- WHEN the operator runs a sync dry-run
- THEN the output reports the change name, dry-run status, mutation status, planned actions, and delta spec paths

### Requirement: Archive dry-run plan
r[sync_archive.plan.archive] The system MUST compute a deterministic archive dry-run plan for a named change without mutating repository files.

#### Scenario: Archive plan reports blockers
r[sync_archive.plan.archive.blockers]
- GIVEN a change that is not ready to archive
- WHEN the operator runs an archive dry-run
- THEN the output reports the change name, dry-run status, mutation status, and blocking reasons

### Requirement: Plan core purity
r[sync_archive.plan.purity] Sync and archive planning MUST remain in the pure core boundary and consume shell-supplied change/policy snapshots.

#### Scenario: Planning has no shell effects
r[sync_archive.plan.purity.no_effects]
- GIVEN sync/archive planning inputs
- WHEN the core plan builder runs
- THEN it does not read files, inspect environment variables, spawn processes, or render JSON
