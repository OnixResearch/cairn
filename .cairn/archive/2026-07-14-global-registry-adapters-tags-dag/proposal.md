## Why

Cairn already has several registry-shaped surfaces: portable evidence registries, a cross-repo requirement ID registry, local change dependency graphs, policy-owned labels, and active work for explicit cross-repo evidence dependencies. Those surfaces are deterministic in isolation, but operators and agents do not have one installation- or workspace-scoped catalog for discovering verified records, filtering them by operational metadata, or traversing typed relationships across repositories.

A backend-specific database added directly to a command would create hidden ambient state and make lifecycle results depend on one machine. Cairn instead needs an explicit, rebuildable registry model whose canonical snapshot and query semantics are independent of storage. Storage and ingest adapters can then support local memory, portable artifacts, and an embedded redb database without making any backend the source of lifecycle truth.

## What Changes

- Add an explicitly selected global Cairn registry for one installation, workspace, or declared trust domain.
- Keep repository artifacts, verified evidence indexes, accepted requirement registries, and Valence-owned identities authoritative; the global registry is a derived catalog that can be rebuilt and exported.
- Define canonical typed registry nodes, tag assignments, edges, source records, snapshots, and BLAKE3 identities.
- Separate ingest adapters, which normalize supported Cairn and stack artifacts, from storage adapters, which persist already-validated registry snapshots.
- Provide storage-adapter conformance semantics for in-memory, portable snapshot, and optional embedded redb implementations.
- Add provenance-bearing namespaced tags for nodes and edges without changing the identity of the tagged artifact.
- Add a typed graph model with policy-owned DAG projections for acyclic relationships such as dependencies, derivations, and supersession while allowing non-DAG relationships outside those projections.
- Add explicit import, query, graph, export, and verification receipts bound to an exact registry snapshot hash.
- Preserve current root-local lifecycle behavior: validation, gates, sync, archive, and release readiness do not silently consult global state.

## Impact

- **Architecture**: New global-registry pure core, shell-owned adapter ports, policy contracts, and an optional redb storage implementation.
- **Compatibility**: Existing `evidence registry`, `requirement-registry`, and `change graph` contracts remain authoritative and become supported ingest sources rather than being replaced.
- **Evidence boundary**: Registry membership, tags, and graph reachability prove cataloged identity and declared linkage only; they do not prove evidence truth, requirement satisfaction, implementation correctness, trust, or release eligibility.
- **Testing**: Positive and negative core tests, adapter conformance suites, backend-parity fixtures, graph cycle and unresolved-node cases, tag provenance cases, transaction/migration failures, CLI receipts, Cairn gates, and release-readiness checks.
