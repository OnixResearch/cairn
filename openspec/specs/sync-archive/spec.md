# Sync Archive Specification

## Purpose

Defines deterministic dry-run planning for `cairn sync` and `cairn archive`.

## Requirements

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


### Requirement: Sync plan execution

r[sync_archive.execute.sync] The system MUST execute a sync plan only when the caller explicitly requests non-dry-run execution, the computed plan is unblocked, and delta specs are merged into accepted specs without replacing accepted-spec wrapper content.

#### Scenario: Sync applies delta specs when explicitly requested

r[sync_archive.execute.sync.applies]
- GIVEN a change with one or more delta spec files and no sync blockers
- WHEN the operator runs sync with the explicit execution flag
- THEN the system merges each planned delta spec into the accepted specs tree
- AND the JSON receipt reports `dry_run=false`, `mutated=true`, and the completed actions

#### Scenario: Sync remains dry-run by default

r[sync_archive.execute.sync.default_dry_run]
- GIVEN a change with one or more delta spec files
- WHEN the operator runs sync without the explicit execution flag
- THEN the system emits a dry-run plan
- AND no repository files are modified

#### Scenario: Existing accepted spec wrapper is preserved

r[sync_archive.execute.sync.merge_preserves_wrapper]
- GIVEN an accepted spec with a title, purpose, and requirements wrapper
- AND a delta spec containing only delta requirement sections
- WHEN the operator runs sync with the explicit execution flag
- THEN the accepted spec still contains its title, purpose, and requirements wrapper
- AND the new or modified requirement content appears under the accepted requirements section

#### Scenario: New accepted spec is created from delta sections

r[sync_archive.execute.sync.merge_creates_spec]
- GIVEN a change with a delta spec for a domain that does not exist in the accepted specs tree
- WHEN the operator runs sync with the explicit execution flag
- THEN the system creates an accepted spec document with a stable title, purpose, and requirements wrapper
- AND the delta requirement content appears under the accepted requirements section

### Requirement: Archive plan execution
r[sync_archive.execute.archive] The system MUST execute an archive plan only when the caller explicitly requests non-dry-run execution and the computed plan is unblocked.

#### Scenario: Archive moves completed change when explicitly requested
r[sync_archive.execute.archive.moves]
- GIVEN a completed change with synced delta specs and no archive blockers
- WHEN the operator runs archive with the explicit execution flag
- THEN the system moves the active change directory under `openspec/changes/archive/<date>-<change>`
- AND the JSON receipt reports `dry_run=false`, `mutated=true`, and the archive destination

#### Scenario: Blocked archive does not mutate
r[sync_archive.execute.archive.blocked_noop]
- GIVEN a change with incomplete tasks or missing required artifacts
- WHEN the operator runs archive with the explicit execution flag
- THEN the system refuses to mutate repository files
- AND the JSON receipt reports the blockers and `mutated=false`

### Requirement: Execution boundary
r[sync_archive.execute.boundary] The system MUST keep execution effects in the CLI shell layer while pure core code computes plans from shell-supplied observations.

#### Scenario: Core remains pure after execution support
r[sync_archive.execute.boundary.pure_core]
- GIVEN sync/archive execution support is implemented
- WHEN Slotcar verifies the pure core boundary
- THEN no filesystem, environment, process, stdout/stderr, or JSON presentation effects are present under `crates/cairn-core/src/verified/`
