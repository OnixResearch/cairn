# Sync Archive Delta: Portable Mutation Evidence

## ADDED Requirements

### Requirement: Lifecycle mutation manifests
r[sync_archive.execute.mutation_manifest] The system MUST emit deterministic before/after mutation manifests for explicit lifecycle executions that mutate repository files.

#### Scenario: Sync execution reports touched file hashes
r[sync_archive.execute.mutation_manifest.sync]
- GIVEN a sync plan is executed with the explicit execution flag
- WHEN the system merges delta specs into accepted specs
- THEN the JSON receipt MUST include a before/after manifest for every planned touched file using root-relative paths and BLAKE3 content hashes
- AND the manifest hash MUST be stable for identical pre-state, delta content, and execution result across checkout roots

#### Scenario: Archive execution reports moved tree hashes
r[sync_archive.execute.mutation_manifest.archive]
- GIVEN an archive plan is executed with the explicit execution flag
- WHEN the system moves a completed change into the archive tree
- THEN the JSON receipt MUST include a before/after manifest for the active change tree and archive destination tree using root-relative paths and BLAKE3 content hashes
- AND the receipt MUST report `mutated=false` if no filesystem move occurred

#### Scenario: Migration execution reports copied file hashes
r[sync_archive.execute.mutation_manifest.migration]
- GIVEN a migration plan is executed with the explicit execution flag
- WHEN legacy lifecycle files are copied into the native Cairn layout
- THEN the JSON receipt MUST include source and destination before/after manifests using root-relative paths where possible and explicit external-input metadata otherwise
- AND any conflict or partial failure MUST be reflected in deterministic blocker or issue fields
