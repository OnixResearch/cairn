# Global Registry Specification Delta

## ADDED Requirements

### Requirement: Registry consumes lifecycle namespaces through a typed adapter

r[cairn.global_registry.lifecycle_namespaces] Cairn MUST provide an explicit ingest adapter for admitted lifecycle-namespace snapshots. The adapter MUST preserve source vocabulary and typed namespace values.

#### Scenario: Namespace snapshot projects deterministically

r[cairn.global_registry.lifecycle_namespaces.valid]
- GIVEN an admitted lifecycle-namespace snapshot and a selected compatible registry snapshot
- WHEN the namespace ingest adapter builds candidate records
- THEN equivalent inputs MUST produce the same namespace-assignment records and candidate snapshot identity
- AND each record MUST preserve subject, source, catalog, authority-class, value-variant, and non-claim data.

#### Scenario: Invalid namespace projection is rejected

r[cairn.global_registry.lifecycle_namespaces.invalid]
- GIVEN a namespace source is stale, unchecked, malformed, type-invalid, collision-prone, or missing required provenance
- WHEN the ingest adapter evaluates it
- THEN Cairn MUST reject the candidate records with deterministic diagnostics
- AND Cairn MUST prevent every storage adapter from committing the candidate snapshot.

### Requirement: Registry namespace queries preserve domain authority

r[cairn.global_registry.lifecycle_namespace_queries] Cairn MUST bind namespace queries to one explicit registry snapshot and typed selector. Query success MUST remain cataloged metadata and linkage evidence only.

#### Scenario: Typed namespace selector is deterministic

r[cairn.global_registry.lifecycle_namespace_queries.valid]
- GIVEN one registry snapshot contains namespace assignments with typed values
- WHEN equivalent namespace selectors arrive in different input orders
- THEN Cairn MUST return the same canonical subject IDs and result hash
- AND the receipt MUST bind snapshot, namespace, value variant, subject kind, and result identities.

#### Scenario: Query result does not satisfy a target

r[cairn.global_registry.lifecycle_namespace_queries.boundary]
- GIVEN a registry query finds assignments that resemble repository targets or completion state
- WHEN Cairn evaluates a lifecycle gate or central target set
- THEN the query result MUST NOT satisfy the gate or target
- AND explicit repository-owned evidence MUST remain necessary.

### Requirement: Namespace persistence reuses existing storage adapters

r[cairn.global_registry.lifecycle_namespace_storage] Cairn MUST use the existing global-registry storage contract for namespace records. The optional `redb` adapter MUST remain a rebuildable derived index.

#### Scenario: Explicit redb profile persists one derived snapshot

r[cairn.global_registry.lifecycle_namespace_storage.redb]
- GIVEN an admitted namespace snapshot, an explicit `redb` profile, and the expected prior snapshot identity
- WHEN the existing storage adapter commits the candidate snapshot
- THEN records and namespace indexes MUST become visible in one transaction
- AND reload MUST reproduce the same canonical snapshot identity as memory and portable JSON adapters.

#### Scenario: Database absence does not remove lifecycle authority

r[cairn.global_registry.lifecycle_namespace_storage.rebuild]
- GIVEN a `redb` file is absent, corrupt, stale, or uses an unsupported storage schema
- WHEN the operator selects verification or rebuild from canonical repository artifacts
- THEN Cairn MUST preserve or reconstruct the canonical namespace snapshot without database-owned meaning
- AND Cairn MUST NOT add a separate namespace database, hidden migration, or last-writer-wins replacement path.

### Requirement: Legacy tags have a bounded compatibility path

r[cairn.global_registry.legacy_tag_compatibility] Cairn MUST isolate legacy tag parsing from namespace-native records. New namespace sources MUST NOT produce legacy tag assignments.

#### Scenario: Legacy snapshot remains readable

r[cairn.global_registry.legacy_tag_compatibility.valid]
- GIVEN a valid version-one registry snapshot contains provenance-bearing tag assignments
- WHEN Cairn reads it through the compatibility adapter
- THEN Cairn MUST preserve the original snapshot and record identities
- AND namespace queries MUST label converted values as legacy text.

#### Scenario: Compatibility collision fails closed

r[cairn.global_registry.legacy_tag_compatibility.collision]
- GIVEN two legacy namespace-and-key pairs map to one canonical namespace ID or conflict with a native definition
- WHEN compatibility conversion runs
- THEN Cairn MUST emit a deterministic collision diagnostic
- AND it MUST NOT merge, overwrite, infer a type, or commit the conflicting records.
