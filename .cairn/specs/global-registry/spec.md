# Global Registry Specification

## Purpose

Defines the `global-registry` capability.

## Requirements

### Requirement: Global registry scope is explicit and bounded

r[cairn.global_registry.scope] Cairn MUST support a global registry only as an explicitly selected installation, workspace, or declared trust-domain catalog and MUST NOT silently consult it from ordinary root-local lifecycle commands.

#### Scenario: Operator selects a registry

r[cairn.global_registry.scope.explicit]
- GIVEN an operator wants to import or query records across repositories
- WHEN the operator invokes a global-registry command
- THEN the command MUST identify the selected registry scope and adapter in its receipt
- AND the command MUST bind its result to the exact observed registry snapshot.

#### Scenario: Root-local operation remains independent

r[cairn.global_registry.scope.local_independence]
- GIVEN no global registry exists or the configured registry is unavailable
- WHEN an operator runs root-local validation, gates, sync, archive, change graph, or release readiness without an explicit registry input
- THEN Cairn MUST preserve the existing root-local behavior
- AND it MUST NOT infer lifecycle truth from ambient user or machine state.

### Requirement: Registry snapshots are canonical and rebuildable

r[cairn.global_registry.snapshot] Cairn MUST represent registry state as a canonical typed snapshot containing scope, source, node, tag-assignment, edge, adapter, schema, non-claim, and BLAKE3 identity data.

#### Scenario: Equivalent inputs produce one snapshot identity

r[cairn.global_registry.snapshot.deterministic]
- GIVEN equivalent validated source records are presented in different filesystem, adapter, or iteration orders
- WHEN Cairn normalizes and builds registry snapshots
- THEN the snapshots MUST contain the same canonical record order and BLAKE3 snapshot hash.

#### Scenario: Conflicting records fail closed

r[cairn.global_registry.snapshot.conflict]
- GIVEN candidate records contain duplicate stable IDs with incompatible kind, schema, payload identity, source identity, or non-claim data
- WHEN Cairn constructs a candidate snapshot
- THEN construction MUST fail with deterministic conflict diagnostics
- AND no storage adapter may commit the candidate snapshot.

#### Scenario: Registry can be rebuilt from authoritative sources

r[cairn.global_registry.snapshot.rebuild]
- GIVEN a registry store is absent or discarded and the same authoritative source artifacts remain available
- WHEN Cairn reimports and validates those source artifacts
- THEN it MUST be able to reconstruct the same canonical snapshot identity without relying on backend-private IDs.

### Requirement: Ingest and storage adapters are separate contracts

r[cairn.global_registry.adapters] Cairn MUST define separate adapter contracts for semantic ingestion and snapshot storage, with policy-owned adapter IDs, schema versions, capabilities, record families, and deterministic failure classes.

#### Scenario: Ingest adapter preserves source semantics

r[cairn.global_registry.adapters.ingest]
- GIVEN a supported Cairn or Valence source artifact passes its domain validator
- WHEN an ingest adapter normalizes the artifact into registry records
- THEN the records MUST preserve the source-owned stable IDs, vocabulary owner, roles, hashes, and non-claims
- AND ingestion MUST NOT promote the artifact's evidence or verification claim.

#### Scenario: Storage adapter cannot reinterpret records

r[cairn.global_registry.adapters.storage]
- GIVEN two conforming storage adapters receive the same validated candidate snapshot
- WHEN each adapter persists and reloads the snapshot
- THEN both MUST return canonically equivalent registry records and the same snapshot identity.

#### Scenario: Unknown adapter is rejected

r[cairn.global_registry.adapters.unknown]
- GIVEN configuration names an adapter ID or capability absent from generated policy
- WHEN Cairn resolves the requested adapter
- THEN it MUST reject the request before reading or mutating registry data
- AND it MUST emit a deterministic unsupported-adapter diagnostic.

### Requirement: Registry semantics preserve the functional-core boundary

r[cairn.global_registry.fcis] Cairn MUST implement record validation, normalization, merge, conflict detection, tag filtering, graph projection, cycle detection, topological ordering, and query result construction as pure deterministic logic over in-memory values.

#### Scenario: Shell owns adapter effects

r[cairn.global_registry.fcis.shell]
- GIVEN registry inputs require filesystem reads, path resolution, redb transactions, locking, import, export, or JSON presentation
- WHEN a registry command runs
- THEN those effects MUST remain in the imperative shell
- AND the pure core MUST remain independent of filesystems, databases, clocks, processes, networks, and ambient environment state.

#### Scenario: Candidate is validated before commit

r[cairn.global_registry.fcis.commit]
- GIVEN a storage adapter has loaded an observed snapshot and a command proposes a mutation
- WHEN Cairn prepares the write
- THEN the pure core MUST construct and validate the complete candidate snapshot before the shell commits it
- AND the adapter MUST reject the commit if the expected prior snapshot identity is stale.

### Requirement: Registry provides conforming memory, portable, and redb storage

r[cairn.global_registry.storage] Cairn MUST provide an in-memory conformance adapter, a portable snapshot adapter, and an optional compiled redb adapter that implement the same canonical snapshot semantics.

#### Scenario: In-memory adapter supports deterministic tests

r[cairn.global_registry.storage.memory]
- GIVEN a test supplies a valid or invalid in-memory registry candidate
- WHEN the conformance harness exercises load, candidate validation, commit, query, and export behavior
- THEN the adapter MUST produce the expected canonical result or deterministic failure without external services.

#### Scenario: redb transaction commits atomically

r[cairn.global_registry.storage.redb]
- GIVEN a valid candidate snapshot and the expected prior snapshot identity match the current redb store
- WHEN the redb adapter commits records and secondary indexes
- THEN node, tag, forward-edge, reverse-edge, source, and snapshot indexes MUST become visible atomically
- AND reloading the store MUST reproduce the candidate snapshot identity.

#### Scenario: redb conflict or migration failure preserves prior state

r[cairn.global_registry.storage.redb_failure]
- GIVEN a stale expected snapshot, unsupported storage schema, malformed durable envelope, interrupted candidate, or incompatible migration
- WHEN the redb adapter attempts to open or commit the store
- THEN it MUST fail with a deterministic diagnostic
- AND it MUST preserve the last valid committed snapshot.

### Requirement: Tags are typed provenance-bearing assignments

r[cairn.global_registry.tags] Cairn MUST model tags as separately identified assignments with subject, namespace, key, value, source provenance, and non-claims rather than mutating the identity of tagged nodes or edges.

#### Scenario: Existing evidence label becomes a tag assignment

r[cairn.global_registry.tags.evidence_label]
- GIVEN a verified evidence registry entry carries channel, release, platform, or compatibility labels
- WHEN Cairn imports the entry into the global registry
- THEN it MUST preserve the evidence index and bundle identities
- AND it MUST represent each imported label through a deterministic provenance-bearing tag assignment.

#### Scenario: Operator tag does not grant authority

r[cairn.global_registry.tags.authority]
- GIVEN an operator assigns a local organizational tag to a node or edge
- WHEN a query or lifecycle integration evaluates the tagged subject
- THEN the tag MUST remain non-authoritative unless an explicit policy profile permits that namespace and use
- AND the assignment MUST NOT strengthen the subject's verification role, trust, or release status.

#### Scenario: Reserved namespace misuse fails closed

r[cairn.global_registry.tags.reserved]
- GIVEN an untrusted or incompatible source attempts to assign a reserved policy or system tag
- WHEN tag validation runs
- THEN Cairn MUST reject the assignment with a deterministic namespace or provenance diagnostic.

### Requirement: Registry graph preserves typed edge ownership

r[cairn.global_registry.graph] Cairn MUST represent relationships as typed directed edges with stable endpoints, vocabulary ownership, source provenance, status, and non-claims.

#### Scenario: Cairn lifecycle edge is imported

r[cairn.global_registry.graph.cairn]
- GIVEN a valid local change dependency or explicit cross-repo dependency summary
- WHEN a Cairn ingest adapter imports the relationship
- THEN the edge MUST preserve Cairn lifecycle vocabulary, dependency direction, source receipt identity, and blocker status.

#### Scenario: Valence evidence edge is imported

r[cairn.global_registry.graph.valence]
- GIVEN a compatible Valence Evidence IR graph record
- WHEN the Valence ingest adapter imports the relationship
- THEN the edge MUST preserve Valence vocabulary ownership and evidence-role boundary
- AND Cairn MUST NOT reinterpret it as lifecycle readiness or requirement satisfaction.

#### Scenario: Missing endpoint remains visible

r[cairn.global_registry.graph.unresolved]
- GIVEN an imported dependency names an upstream stable ID that is not present in the selected snapshot
- WHEN graph validation or traversal runs
- THEN Cairn MUST report an explicit unresolved endpoint or blocker
- AND it MUST NOT silently remove the edge or treat the dependency as satisfied.

### Requirement: DAG guarantees are profile-scoped

r[cairn.global_registry.dag_profiles] Cairn MUST apply acyclicity only through policy-owned graph profiles that declare allowed node kinds, edge kinds, endpoint rules, and whether the selected projection is a DAG.

#### Scenario: Dependency projection is acyclic

r[cairn.global_registry.dag_profiles.acyclic]
- GIVEN a graph profile selects valid dependency edges whose endpoints are present and acyclic
- WHEN Cairn validates the projection
- THEN it MUST emit deterministic dependency closure, reverse blockers, and stable topological order.

#### Scenario: Cycle fails with canonical witness

r[cairn.global_registry.dag_profiles.cycle]
- GIVEN a DAG-constrained projection contains a self-edge or multi-node cycle
- WHEN graph validation runs
- THEN Cairn MUST reject the projection
- AND it MUST emit a deterministic bounded cycle witness without reporting cyclic nodes as ready.

#### Scenario: Non-DAG relation may contain a cycle

r[cairn.global_registry.dag_profiles.non_dag]
- GIVEN a reference, alias, or related-to profile explicitly permits cycles
- WHEN the relationship graph contains a cycle
- THEN Cairn MAY preserve and query that cycle
- AND it MUST NOT mislabel the profile as a validated DAG.

### Requirement: Registry queries emit snapshot-bound receipts

r[cairn.global_registry.query] Cairn MUST provide deterministic exact-ID, tag, and typed graph queries whose receipts bind the selected adapter, snapshot hash, normalized selectors, graph profile where applicable, result identities, diagnostics, and BLAKE3 result or subgraph hash.

#### Scenario: Tag query is deterministic

r[cairn.global_registry.query.tags]
- GIVEN one registry snapshot and equivalent tag selectors in different input orders
- WHEN Cairn evaluates the selectors
- THEN it MUST return the same canonically ordered stable IDs and result hash.

#### Scenario: Graph traversal is deterministic

r[cairn.global_registry.query.graph]
- GIVEN one registry snapshot, start node, direction, edge-kind set, and graph profile
- WHEN Cairn evaluates ancestors, descendants, dependency closure, or topological order
- THEN it MUST return a canonically ordered result bound to the snapshot and selected subgraph.

#### Scenario: Ambiguous or stale query fails closed

r[cairn.global_registry.query.failure]
- GIVEN a query names an unknown stable ID, unsupported selector, ambiguous scope, stale expected snapshot, or invalid graph profile
- WHEN query validation runs
- THEN Cairn MUST emit deterministic diagnostics
- AND it MUST NOT silently broaden the query or select a different snapshot.

### Requirement: Registry export and verification are backend-independent

r[cairn.global_registry.portability] Cairn MUST export, import, and verify a canonical portable registry snapshot without requiring the originating storage backend or exposing backend-private state.

#### Scenario: redb snapshot exports portably

r[cairn.global_registry.portability.export]
- GIVEN a valid registry snapshot is stored in redb
- WHEN an operator exports the snapshot
- THEN Cairn MUST emit a canonical portable artifact with the same snapshot identity
- AND the export MUST omit redb table layout, locks, absolute host paths, credentials, raw environment values, and backend-private identifiers.

#### Scenario: Portable snapshot verifies without redb

r[cairn.global_registry.portability.verify]
- GIVEN a portable registry snapshot and its declared BLAKE3 identity
- WHEN verification runs through a different conforming environment or adapter
- THEN Cairn MUST recompute record and snapshot identities and return the same verdict without opening the original database.

#### Scenario: Tampered portable snapshot is rejected

r[cairn.global_registry.portability.tamper]
- GIVEN a portable snapshot has a modified record, tag, edge, source, schema, non-claim, or snapshot hash
- WHEN Cairn verifies or imports it
- THEN verification MUST fail with deterministic diagnostics
- AND no target storage adapter may commit the tampered candidate.

### Requirement: Registry integration remains explicit and domain-validated

r[cairn.global_registry.integration_boundary] Cairn MUST require explicit snapshot-bound input before registry records influence another lifecycle surface, and the consuming domain validator MUST revalidate the selected records under its existing rules.

#### Scenario: Registry candidate does not satisfy dependency by presence

r[cairn.global_registry.integration_boundary.dependency]
- GIVEN a registry contains a record that appears to match a cross-repo dependency
- WHEN an operator presents that record to dependency validation
- THEN registry membership alone MUST NOT satisfy the dependency
- AND the existing dependency validator MUST check receipt kind, subject, status, policy, content identity, and every declared constraint.

#### Scenario: Registry query does not grant release readiness

r[cairn.global_registry.integration_boundary.readiness]
- GIVEN a tag or graph query returns complete and acyclic results
- WHEN an operator evaluates release readiness
- THEN the query receipt MUST remain identity and linkage evidence only
- AND release readiness MUST continue to apply its own explicit evidence and policy checks.

### Requirement: Shared durable identity ownership remains with Valence

r[cairn.global_registry.valence_boundary] Cairn MUST preserve Valence ownership of generic evidence identity, typed durable-storage envelope semantics, and Valence graph vocabulary while owning Cairn lifecycle registry records, policies, queries, and receipts.

#### Scenario: Compatible durable envelope is consumed

r[cairn.global_registry.valence_boundary.envelope]
- GIVEN a registry record uses a supported Valence-owned durable envelope or artifact-reference profile
- WHEN Cairn validates and stores the record
- THEN Cairn MUST validate the declared schema, value type, codec, content identity, migration history, and non-claims through the supported compatibility boundary
- AND it MUST NOT fork the shared identity vocabulary into a conflicting Cairn-owned definition.

### Requirement: Registry behavior has positive and negative verification

r[cairn.global_registry.validation] Cairn MUST verify registry core, adapters, tags, graph profiles, queries, portability, migrations, and integration boundaries with positive and negative tests.

#### Scenario: Positive registry suite passes

r[cairn.global_registry.validation.positive]
- GIVEN deterministic source fixtures, valid tags, supported adapters, acyclic DAG projections, matching prior snapshots, and untampered portable exports
- WHEN focused core, adapter-conformance, backend-parity, CLI, and receipt tests run
- THEN every supported backend MUST produce equivalent canonical snapshots, query results, graph order, and expected passing receipts.

#### Scenario: Negative registry suite fails correctly

r[cairn.global_registry.validation.negative]
- GIVEN malformed or conflicting records, unknown adapters, reserved-tag misuse, dangling endpoints, self-edges, cycles, stale transactions, incompatible migrations, interrupted writes, tampered exports, weakened non-claims, or claim promotion
- WHEN focused tests run
- THEN Cairn MUST emit the expected deterministic diagnostics
- AND it MUST not panic, silently broaden claims, corrupt the prior store, or commit an invalid snapshot.

### Requirement: Registry documentation states operational and claim boundaries

r[cairn.global_registry.docs] Cairn documentation MUST explain explicit registry selection, authoritative source artifacts, adapter separation, tag provenance, graph profiles, redb recovery and migration, portable export, query receipts, and bounded non-claims.

#### Scenario: Operator understands registry limits

r[cairn.global_registry.docs.non_claims]
- GIVEN an operator reads global-registry documentation or a passing registry receipt
- WHEN the supported claim is described
- THEN it MUST state that registry membership, storage, tags, graph reachability, acyclicity, and query success prove only cataloged identity, declared metadata, and bounded linkage
- AND they MUST NOT be presented as evidence truth, requirement satisfaction, implementation correctness, verifier soundness, social trust, runtime authority, lifecycle readiness, or release eligibility.
