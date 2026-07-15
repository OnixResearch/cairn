# Global registry

Cairn's global registry is an explicitly selected, rebuildable discovery catalog for one installation, workspace, or declared trust domain. It is not process-global authority, a public network service, or an ambient input to repository validation.

Repository lifecycle artifacts, verified evidence indexes, requirement registries, change graphs, cross-repository dependency receipts, and Valence-owned Evidence IR records remain authoritative in their own domains. Registry state is a derived projection and can be discarded and rebuilt from those inputs.

## Canonical snapshot

A portable `cairn.global-registry-snapshot.v1` contains:

- an explicit `scope_id`;
- source records with source family, vocabulary owner, safe source reference, and BLAKE3 identity;
- typed nodes with source-owned stable IDs and content hashes;
- separately identified provenance-bearing tag assignments;
- typed directed edges with vocabulary owner, status, endpoints, and non-claims;
- compiled adapter metadata; and
- a canonical BLAKE3 `snapshot_hash`.

Collections are sorted and deduplicated before hashing. Conflicting stable IDs, malformed hashes, absent source provenance, unsafe references, empty non-claims, missing endpoints, self-edges, reserved-tag misuse, and unsupported vocabulary ownership fail before storage commit.

## Adapter separation

Ingest adapters translate reviewed source shapes into candidate canonical records. Storage adapters receive only a complete validated snapshot. Storage never reinterprets source semantics.

Policy allowlists these compiled ingest adapters:

- `cairn.canonical-records.v1`;
- `cairn.evidence-index.v1`;
- `cairn.requirement-registry.v1`;
- `cairn.change-graph.v1`;
- `cairn.cross-repo-dependencies.v1`; and
- `valence.evidence-ir.v1`.

Cairn adapters preserve Cairn lifecycle vocabulary. The Valence adapter preserves Valence identity and graph vocabulary and does not convert evidence roles into lifecycle readiness.

Storage adapters are:

- `memory.v1`, a non-persistent conformance adapter;
- `portable-json.v1`, the canonical offline interchange and recovery format; and
- `redb.v1`, an optional `redb-registry` feature providing a transactional local cache.

Every commit compares the expected prior snapshot hash with the observed state. A stale write fails without replacing the prior snapshot. Portable writes use atomic replacement. redb updates snapshot, node, source, tag, forward-adjacency, reverse-adjacency, schema, and metadata tables in one transaction. Unsupported storage schemas and interrupted transactions do not overwrite the last canonical snapshot.

A redb file is local mutable cache state, not an evidence artifact or stable protocol. Export reconstructs the portable snapshot and verifies its identity independently of table iteration order.

## Commands

Import remains dry-run unless `--execute` is present:

```console
cairn registry import \
  --scope workspace:onix \
  --adapter portable-json.v1 \
  --store registry.json \
  --ingest cairn.canonical-records.v1 \
  --input candidates.json \
  --expected-snapshot <prior-blake3> \
  --execute
```

The receipt exposes observed, expected, and candidate snapshot identities, validation issues, dry-run/mutation state, selected ingest and storage adapters, and the exact scope. If `--expected-snapshot` is omitted, the shell binds the observed hash as the expected value and the adapter rechecks it at commit.

Queries always identify a registry, scope, adapter, and snapshot:

```console
cairn registry query --scope workspace:onix --adapter portable-json.v1 \
  --registry registry.json --id change:alpha

cairn registry query --scope workspace:onix --adapter portable-json.v1 \
  --registry registry.json --tag operator:channel=dev
```

Multiple tag selectors are intersected after canonical sorting. Exact-ID, selector, selected-ID, result, and receipt hashes bind the response to one snapshot.

Graph traversal uses policy-owned profiles:

```console
cairn registry graph --scope workspace:onix --adapter portable-json.v1 \
  --registry registry.json --profile change-dependencies \
  --start change:alpha --direction descendants
```

Dependency, requirement, and evidence-derivation profiles require DAGs. They reject unresolved endpoints, duplicate edges, self-edges, and cycles with a canonical bounded witness and stable topological order. The `references` profile permits cycles and never claims to be a DAG.

Portable recovery and verification are backend-independent:

```console
cairn registry export --scope workspace:onix --adapter redb.v1 \
  --registry registry.redb --output registry.json --execute
cairn registry verify --registry registry.json
```

Portable artifacts exclude table layout, locks, credentials, raw environment values, absolute host paths, and backend-private identifiers.

## Tags and authority

Tags do not mutate node or edge identity. `operator` tags are annotations only. Imported `evidence` tags preserve source labels. `cairn.policy` and `cairn.system` are reserved and require Cairn policy provenance. No tag affects a gate or grants authority unless a future explicit policy integration presents a snapshot-bound result to the existing domain validator.

## Integration boundary

`status`, `validate`, proposal/design/tasks gates, sync, archive, release readiness, evidence-registry, requirement-registry, and change-graph commands do not read global registry files, XDG state, or `CAIRN_GLOBAL_REGISTRY`. Registry membership cannot satisfy a cross-repository dependency. A future consuming command must accept explicit snapshot-bound input and rerun the existing domain validator over selected records.

## Non-claims

Registry membership proves cataloged identity only. Tags, graph reachability, acyclicity, storage success, and query success prove declared metadata and bounded linkage only. They do not prove evidence truth, requirement satisfaction, implementation correctness, verifier soundness, social trust, runtime authority, lifecycle readiness, or release eligibility.
