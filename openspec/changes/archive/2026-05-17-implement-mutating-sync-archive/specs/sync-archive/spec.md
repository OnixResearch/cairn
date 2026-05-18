# Sync Archive Execution Delta

## MODIFIED Requirements

### Requirement: Sync plan execution

r[sync_archive.execute.sync] The system MUST execute a sync plan only when the caller explicitly requests non-dry-run execution and the computed plan is unblocked.

#### Scenario: Sync applies delta specs when explicitly requested

- GIVEN a change with one or more delta spec files and no sync blockers
- WHEN the operator runs sync with the explicit execution flag
- THEN the system copies or merges the planned delta spec outputs into the accepted specs tree
- AND the JSON receipt reports `dry_run=false`, `mutated=true`, and the completed actions

#### Scenario: Sync remains dry-run by default

- GIVEN a change with one or more delta spec files
- WHEN the operator runs sync without the explicit execution flag
- THEN the system emits a dry-run plan
- AND no repository files are modified

### Requirement: Archive plan execution

r[sync_archive.execute.archive] The system MUST execute an archive plan only when the caller explicitly requests non-dry-run execution and the computed plan is unblocked.

#### Scenario: Archive moves completed change when explicitly requested

- GIVEN a completed change with synced delta specs and no archive blockers
- WHEN the operator runs archive with the explicit execution flag
- THEN the system moves the active change directory under `openspec/changes/archive/<date>-<change>`
- AND the JSON receipt reports `dry_run=false`, `mutated=true`, and the archive destination

#### Scenario: Blocked archive does not mutate

- GIVEN a change with incomplete tasks or missing required artifacts
- WHEN the operator runs archive with the explicit execution flag
- THEN the system refuses to mutate repository files
- AND the JSON receipt reports the blockers and `mutated=false`

### Requirement: Execution boundary

r[sync_archive.execute.boundary] The system MUST keep execution effects in the CLI shell layer while pure core code computes plans and typed execution receipts from shell-supplied observations.

#### Scenario: Core remains pure after execution support

- GIVEN sync/archive execution support is implemented
- WHEN Slotcar verifies the pure core boundary
- THEN no filesystem, environment, process, stdout/stderr, or JSON presentation effects are present under `crates/cairn-core/src/verified/`
