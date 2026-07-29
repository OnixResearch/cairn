# Tasks: Type lifecycle evidence identities

## Phase 1: Inventory and shared foundation

- [x] [serial] I1 Inventory semantic IDs, refs, BLAKE3 roles, imported digests, paths, wire values, and display text in selected `cairn-core` modules. r[cairn.nominal_evidence_identities.inventory]
- [x] [serial] V1 Capture baseline stable CLI JSON, receipt hashes, replay receipts, archive roots, collaboration state, and stack-readiness outputs. r[cairn.nominal_evidence_identities.compatibility]
- [x] [serial] I2 Add typed Nickel domain catalog entries and deterministic generated projections for core and Octet use. r[cairn.nominal_evidence_identities.policy]
- [x] [serial] I3 Add private `DomainId<Role>` and `Blake3Digest<Role>` families with checked construction, borrowed views, explicit accessors, and typed diagnostics. r[cairn.nominal_evidence_identities.types]
- [x] [parallel] V2 Add positive and negative constructor tests for bounds, grammar, prefixes, case, algorithms, and malformed values. r[cairn.nominal_evidence_identities.validation]
- [x] [serial] I4 Add explicit wire-to-core adapters and closed tagged enums for heterogeneous identity records. r[cairn.nominal_evidence_identities.wire_boundary]

## Phase 2: Readiness cohort

- [x] [serial] I5 Migrate stack provenance, evidence-chain, formal-proof, and function-address readiness identities to exact types. r[cairn.nominal_evidence_identities.readiness]
- [x] [parallel] V3 Add wrong-source, wrong-binary, wrong-sidecar, wrong-Valence, wrong-Mantle, wrong-policy, wrong-graph, and role-promotion tests. r[cairn.nominal_evidence_identities.readiness.validation]
- [x] [parallel] V4 Prove readiness fixture JSON, chain identities, and readiness receipt hashes remain stable. r[cairn.nominal_evidence_identities.compatibility]

## Phase 3: Evidence-chain cohort

- [x] [serial] I6 Migrate agent receipts, replay, release readiness, global registries, evidence manifests, publication, import, republish, index, and selection identities. r[cairn.nominal_evidence_identities.receipt_chains]
- [x] [parallel] V5 Add stale policy, wrong receipt, wrong chain, wrong registry, missing parent, tampered member, and evidence-role negative tests. r[cairn.nominal_evidence_identities.receipt_chains.validation]
- [x] [parallel] V6 Prove existing evidence bundles and replay receipts remain readable and hash-stable. r[cairn.nominal_evidence_identities.compatibility]

## Phase 4: Lifecycle mutation cohort

- [x] [serial] I7 Migrate sync, archive, compaction, mutation-manifest, collaboration, change, workspace, operation, state, content-root, pack, and ledger identities. r[cairn.nominal_evidence_identities.lifecycle]
- [x] [parallel] V7 Add stale plan, wrong archive, wrong pack, wrong state, wrong actor, wrong operation, conflicting workspace, and chain-tamper tests. r[cairn.nominal_evidence_identities.lifecycle.validation]
- [x] [parallel] V8 Prove sync and archive plans, compaction receipts, and collaboration envelopes retain stable serialized compatibility. r[cairn.nominal_evidence_identities.compatibility]

## Phase 5: Compiler enforcement and closeout

- [x] [parallel] V9 Add compile-pass same-role fixtures and compile-fail fixtures for every maintained identity family. r[cairn.nominal_evidence_identities.compile_time]
- [x] [serial] I8 Enable future Octet nominal-domain checks for migrated verified-core scopes. r[cairn.nominal_evidence_identities.octet]
- [x] [serial] I9 Document identity ownership, wire compatibility, consumer migration, FCIS boundaries, and evidence non-claims. r[cairn.nominal_evidence_identities.docs]
- [x] [serial] V10 Run policy freshness, formatting, workspace tests, Clippy, core purity smoke, traceability, Cairn validation, change gates, release readiness, and relevant Nix checks. r[cairn.nominal_evidence_identities.final_checks]

## Verification coverage

- `Scenario: Inventory excludes display and shell values` -> I1
- `Scenario: Domain catalog is deterministic` -> I2, V10
- `Scenario: Same-role identity is accepted` -> I3, V2, V9
- `Scenario: Cross-role identity does not compile` -> I3, V9
- `Scenario: Wire record admits typed core values` -> I4, V2
- `Scenario: Readiness roles remain distinct` -> I5, V3
- `Scenario: Readiness identities remain stable` -> V1, V4
- `Scenario: Receipt chain roles remain distinct` -> I6, V5
- `Scenario: Existing evidence bundles remain readable` -> V6
- `Scenario: Lifecycle mutation roles remain distinct` -> I7, V7
- `Scenario: Archive and collaboration formats remain stable` -> V8
- `Scenario: Octet rejects raw-domain regression` -> I8, V10
- `Scenario: Core remains effect-free` -> I9, V10
- `Scenario: Typed evidence claims remain bounded` -> I9, V10
