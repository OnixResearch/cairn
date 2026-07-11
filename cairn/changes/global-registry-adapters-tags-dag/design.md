## Context

Cairn's evidence registry is a portable, BLAKE3-bound catalog of already-verified evidence indexes. The requirement ID registry validates cross-repo ownership and aliases. The change dependency graph provides a deterministic local DAG, while active cross-repo dependency work validates explicitly presented upstream receipts. Valence separately owns shared evidence identity, typed durable-storage envelopes, and generic evidence graph vocabulary.

These boundaries should compose without collapsing into a single untyped database. The global registry is therefore a derived query and discovery layer over validated domain records. It must remain explicit, portable, backend-independent, and unable to strengthen the claims of imported artifacts.

## Decisions

### 1. “Global” means an explicit bounded scope

**Choice:** A registry belongs to one named installation, workspace, or trust domain and is selected explicitly by command input or a typed policy profile. It is not a process-global singleton, public network authority, or implicit source consulted by ordinary repository commands.

**Rationale:** Explicit scope makes the input auditable and prevents two machines with different local databases from producing unexplained lifecycle decisions. An XDG location may be a shell-level convenience, but commands and receipts must surface the selected registry identity.

### 2. Canonical artifacts remain authoritative

**Choice:** Repository lifecycle artifacts, verified evidence indexes, requirement registry inputs, and Valence-owned evidence identities remain the sources of truth. The registry stores normalized projections plus source identities and can be rebuilt from those inputs.

**Rationale:** A corrupt, deleted, stale, or migrated database must not erase or silently rewrite lifecycle history. Rebuildability also keeps redb from becoming a portable evidence format.

### 3. Use a typed canonical snapshot

**Choice:** Define a canonical registry snapshot containing schema identity, scope identity, source records, nodes, tag assignments, edges, adapter metadata, and a BLAKE3 snapshot hash. Every collection is normalized, uniquely keyed, and canonically sorted before hashing.

Candidate record shape:

```text
RegistryNode {
  stable_id,
  kind,
  schema,
  content_hash,
  payload_ref,
  source_ref,
  non_claims,
}

RegistryTagAssignment {
  assignment_id,
  subject_id,
  subject_kind,
  namespace,
  key,
  value,
  source_ref,
  non_claims,
}

RegistryEdge {
  stable_id,
  kind,
  from_id,
  to_id,
  vocabulary_owner,
  source_ref,
  status,
  non_claims,
}
```

**Rationale:** Typed records preserve domain boundaries and make snapshots testable without a database. Tag assignments are separate records so operational metadata can change without changing the imported artifact's identity.

### 4. Separate ingest adapters from storage adapters

**Choice:** Ingest adapters normalize supported external shapes into canonical candidate records. Storage adapters load and atomically persist already-validated snapshots. These are separate interfaces and capability sets.

Initial ingest families should cover existing Cairn evidence indexes and registry labels, requirement ID registries, local change dependency graphs, explicit cross-repo dependency summaries, and compatible Valence Evidence IR records. Initial storage families should cover in-memory testing, portable snapshot artifacts, and redb.

**Rationale:** Cairn, Octet, Valence, or legacy format compatibility is a semantic translation concern; redb is an I/O concern. Combining them would let a storage backend accidentally reinterpret evidence.

### 5. Adapters are compiled and policy-allowlisted

**Choice:** Adapter IDs, schema versions, read/write capabilities, migration support, and permitted record families are declared in Nickel-authored policy and consumed from generated policy JSON. Cairn does not load arbitrary executable plugins into the registry process.

**Rationale:** A closed, reviewed adapter vocabulary provides deterministic failure behavior and avoids granting ambient code execution through registry configuration.

### 6. Preserve the functional-core / imperative-shell boundary

**Choice:** `cairn-core` owns record parsing, normalization, identity calculation, candidate merge, conflict diagnostics, tag filtering, graph projection, cycle detection, topological ordering, and query result construction over in-memory snapshots. A shell-owned storage layer handles filesystem paths, redb transactions, locks, XDG resolution, import reads, export writes, and JSON presentation.

A write flow loads an observed snapshot, constructs and validates a candidate snapshot in pure code, then commits it only when the adapter can prove the expected prior snapshot identity still matches.

**Rationale:** All registry semantics remain testable without standing up redb, a filesystem, a daemon, a clock, or a network service.

### 7. Tags are provenance-bearing assignments

**Choice:** Tags use normalized namespace, key, and value fields and may target nodes or edges. Reserved policy/system namespaces are distinct from operator annotations. Every assignment records its source and its own content identity. A tag affects authority or gate selection only when an explicit policy profile says so.

Existing evidence registry labels can adapt into tag assignments while preserving the evidence index and bundle identities they annotate.

**Rationale:** Tags are useful for channel, platform, lifecycle state, owner, domain, and operator organization, but an arbitrary local annotation must not silently become trusted evidence or mutate content-addressed records.

### 8. Model a typed property graph, not one universal DAG

**Choice:** Registry edges form a typed property graph. Policy-owned graph profiles select allowed node kinds and edge kinds and declare whether the selected projection must be acyclic. Dependency, derivation, supersession, and selected containment projections may be DAG-constrained. Reference, alias, or related-to relationships may permit cycles outside those profiles.

**Rationale:** Requiring every relationship to be globally acyclic would reject legitimate mutual references and compatibility relationships. DAG guarantees are meaningful only for a declared semantic projection.

### 9. DAG validation fails closed with useful evidence

**Choice:** A DAG profile validates endpoint presence, edge vocabulary ownership, duplicate edges, unresolved external nodes, self-edges, cycles, and stable topological order. Cycle failures include a canonical bounded witness. Unresolved dependency nodes remain explicit blockers rather than disappearing from traversal results.

**Rationale:** Agents need deterministic explanations for blocked ordering. Silently dropping unresolved or cyclic edges would produce unsafe readiness conclusions.

### 10. Preserve edge-vocabulary ownership

**Choice:** Cairn owns lifecycle relationships such as local and cross-repo change dependencies. Valence retains ownership of generic evidence identity and provenance edge schemas. Ingest adapters preserve the source vocabulary owner and claim boundary instead of promoting imported records into stronger Cairn claims.

**Rationale:** A global query surface should connect the stack without transferring authority between layers.

### 11. Use redb as an optional embedded storage implementation

**Choice:** The redb adapter stores canonical records and transactional secondary indexes for node identity, tag lookup, forward/reverse adjacency, source identity, and snapshot metadata. Transactions update records and indexes atomically and reject an unexpected prior snapshot identity. Canonical output never depends on redb table or iteration order.

The redb file is local mutable state, not an evidence artifact. Portable export reconstitutes the canonical snapshot and verifies its BLAKE3 identity independently of redb.

**Rationale:** redb provides a daemon-free durable local backend suitable for incremental lookup, while the adapter boundary protects portability and determinism.

### 12. Make migrations and conflicts explicit

**Choice:** Storage schema upgrades validate typed envelope, codec, schema, and source snapshot identities before decoding or migration. A migration emits source and destination snapshot identities plus diagnostics and does not overwrite an unsupported or partially migrated store. Concurrent write conflicts fail with a stable expected-versus-observed snapshot diagnostic.

**Rationale:** Hidden in-place upgrades and last-writer-wins behavior could silently lose registry records or weaken evidence provenance.

### 13. Bind every query to a snapshot

**Choice:** Tag queries, exact-ID lookups, ancestor/descendant traversal, dependency closure, and topological-order queries run over one explicit snapshot. Receipts include adapter ID, snapshot hash, normalized selectors, selected stable IDs, graph profile when applicable, issue rows, and a BLAKE3 result or subgraph hash.

**Rationale:** Query results are reproducible only when callers know exactly which registry state was evaluated.

### 14. Export and verification are backend-independent

**Choice:** Cairn can export a canonical portable registry snapshot and verify or import it without the originating backend. Canonical JSON is the initial runtime interchange; a Preserves carrier may be added through the existing bounded evidence-envelope path without changing registry semantics. Portable artifacts exclude credentials, raw environment values, machine-local locks, and absolute host paths.

**Rationale:** Operators need offline transfer, review, backup, and backend migration without treating a database file as a stable protocol.

### 15. Roll out as read-only discovery before gate integration

**Choice:** The first usable slice imports, queries, traverses, exports, and verifies. Existing gates and mutations do not consult the registry implicitly. A later opt-in integration may present exact snapshot-bound candidates to existing domain validators, which must revalidate them under their current rules.

Candidate command family:

```text
cairn registry import ...
cairn registry query ...
cairn registry graph ...
cairn registry export ...
cairn registry verify ...
```

**Rationale:** Read-only discovery proves the model and adapter seam without making hidden cache state release-authoritative.

## Risks / Trade-offs

- A general registry can become an untyped dumping ground; closed record kinds, vocabulary ownership, and adapter allowlists mitigate that risk.
- Operational tags are mutable and may drift; separate assignment identities and source provenance make drift visible.
- Full-snapshot DAG checks may become expensive; correctness comes first, with incremental indexes treated as an optimization that must match the pure oracle.
- redb introduces file locking, corruption, migration, and dependency-maintenance concerns; portable export and adapter conformance prevent backend lock-in.
- Cross-repo graph visibility can look stronger than the imported evidence warrants; every receipt preserves non-claims and domain ownership.
- Explicit registry selection adds command verbosity, but prevents ambient state from changing lifecycle outcomes invisibly.
