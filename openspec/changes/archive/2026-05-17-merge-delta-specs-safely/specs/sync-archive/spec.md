## MODIFIED Requirements

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
