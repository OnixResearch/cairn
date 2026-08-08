## Phase 1: Contracts and pure core

- [x] [serial] Define Nickel contracts and generated policy rows for registry scopes, canonical records, adapter profiles, tag namespaces, graph profiles, receipt schemas, and bounded non-claims. r[cairn.global_registry.scope] r[cairn.global_registry.snapshot] r[cairn.global_registry.adapters]
- [x] [depends:registry-contracts] Implement pure registry record parsing, normalization, BLAKE3 identity, candidate merge, conflict detection, and snapshot construction over in-memory inputs. r[cairn.global_registry.snapshot] r[cairn.global_registry.fcis]
- [x] [depends:registry-core] Implement pure tag assignment validation, provenance classification, reserved-namespace enforcement, deterministic filtering, and tag result identity. r[cairn.global_registry.tags]
- [x] [depends:registry-core] Implement typed edge validation, policy-selected graph projections, canonical cycle witnesses, unresolved-node blockers, dependency closure, and stable topological ordering. r[cairn.global_registry.graph] r[cairn.global_registry.dag_profiles]

## Phase 2: Adapter and storage shell

- [x] [depends:registry-core] Define separate shell-owned ingest and storage adapter ports with policy-allowlisted IDs, explicit capabilities, expected-snapshot conflict handling, and deterministic error mapping. r[cairn.global_registry.adapters] r[cairn.global_registry.fcis]
- [x] [depends:registry-store-port] Add in-memory and portable-snapshot storage adapters plus a shared adapter conformance harness. r[cairn.global_registry.storage] r[cairn.global_registry.portability]
- [x] [depends:registry-store-port] Add the optional redb storage adapter with transactional record/index updates, forward and reverse adjacency indexes, tag indexes, migration checks, and backend-independent snapshot export. r[cairn.global_registry.storage] r[cairn.global_registry.portability]
- [x] [depends:registry-adapters] Add ingest adapters for verified evidence indexes and labels, requirement ID registries, local change dependency graphs, explicit cross-repo dependency summaries, and compatible Valence records without changing source-owned identities or claims. r[cairn.global_registry.adapters] r[cairn.global_registry.graph]

## Phase 3: Operator surfaces

- [x] [depends:registry-adapters] Add explicit import, exact-ID and tag query, typed graph traversal, export, and verification commands whose receipts bind adapter, snapshot, selector, result, and subgraph identities. r[cairn.global_registry.query] r[cairn.global_registry.portability]
- [x] [depends:registry-cli] Preserve current root-local validation, gate, sync, archive, release-readiness, evidence-registry, requirement-registry, and change-graph behavior without implicit global-registry reads. r[cairn.global_registry.scope] r[cairn.global_registry.integration_boundary]
- [x] [depends:registry-cli] Document explicit registry selection, tag namespaces, graph profiles, redb's local-cache boundary, portable recovery, migration, and every supported non-claim. r[cairn.global_registry.docs]

## Phase 4: Verification and lifecycle completion

- [x] [depends:registry-core] Add positive tests for deterministic rebuilds, backend parity, valid tags, acyclic graph projections, stable topological order, successful redb transactions, portable export/import, and Valence/Cairn vocabulary preservation. r[cairn.global_registry.validation]
- [x] [depends:registry-core] Add negative tests for malformed records, duplicate identities, conflicting sources, reserved-tag misuse, unknown adapters, dangling endpoints, self-edges, cycles, stale expected snapshots, tampered exports, unsupported migrations, interrupted writes, and claim promotion. r[cairn.global_registry.validation]
- [x] [depends:registry-validation] Run policy freshness, focused adapter and graph tests, core purity smoke, formatting, workspace tests, Clippy, Cairn validation and gates, traceability coverage, release readiness, and Nix flake checks. r[cairn.global_registry.validation]
- [x] [depends:registry-final-validation] Sync the accepted specification, archive the completed change with deterministic evidence, and verify the accepted requirement IDs are present before commit. r[cairn.global_registry.validation]
