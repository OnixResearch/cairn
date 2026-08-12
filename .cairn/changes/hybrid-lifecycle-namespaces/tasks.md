## Phase 1: Contracts and domain model

- [ ] [serial] Review published Onix components for compatible nominal identity, typed-value, and receipt contracts. Record each reuse or bounded rejection. r[cairn.lifecycle_namespaces.newtypes] r[cairn.lifecycle_namespaces.receipts]
- [ ] [depends:component-review] Add Nickel contracts for namespace catalogs, values, subjects, cardinality, authority, targets, receipts, bounds, and non-claims. r[cairn.lifecycle_namespaces.catalog] r[cairn.lifecycle_namespaces.values] r[cairn.lifecycle_namespaces.hybrid_targets]
- [ ] [depends:namespace-contracts] Add private-field newtypes for protocol IDs and hashes with canonical parsing, display, serialization, and negative substitution tests. r[cairn.lifecycle_namespaces.newtypes]
- [ ] [depends:namespace-contracts] Add the closed `NamespaceValue` sum and pure definition, assignment, cardinality, applicability, constraint, and reserved-domain validation. r[cairn.lifecycle_namespaces.values] r[cairn.lifecycle_namespaces.assignments]
- [ ] [depends:namespace-types] Add bounded typestate transitions for unchecked catalogs, unchecked assignments, observed repository evidence, and their admitted or bound states. r[cairn.lifecycle_namespaces.typestate]
- [ ] [depends:namespace-types] Add pure repository-target admission, evidence matching, blocker diagnostics, and central completion decisions. r[cairn.lifecycle_namespaces.hybrid_targets] r[cairn.lifecycle_namespaces.repository_evidence]
- [ ] [depends:namespace-types] Add pure namespace snapshot construction, canonical BLAKE3 identities, typed query evaluation, and portable receipt material. r[cairn.lifecycle_namespaces.query] r[cairn.lifecycle_namespaces.receipts]

## Phase 2: Imperative shell and adapters

- [ ] [depends:namespace-core] Add preview-first namespace configuration export, freshness checks, bounded reads, locking, rechecks, and atomic projection replacement. r[cairn.lifecycle_namespaces.catalog] r[cairn.lifecycle_namespaces.fcis]
- [ ] [depends:namespace-core] Add read-only namespace list, show, validate, and typed query CLI commands for explicit roots and lifecycle-store IDs. r[cairn.lifecycle_namespaces.query]
- [ ] [depends:namespace-cli] Add strict read-only MCP tools with CLI-equivalent namespace results and deterministic receipts. r[cairn.lifecycle_namespaces.query] r[cairn.lifecycle_namespaces.verification]
- [ ] [depends:target-core] Add explicit repository-evidence loading to central validation, sync, archive, and release-readiness preflight without sibling or network discovery. r[cairn.lifecycle_namespaces.repository_evidence] r[cairn.lifecycle_namespaces.hybrid_targets]
- [ ] [depends:namespace-core] Add namespace-native global-registry records, ingest policy, typed selectors, and snapshot-bound query receipts. r[cairn.lifecycle_namespaces.registry_projection] r[cairn.global_registry.lifecycle_namespaces] r[cairn.global_registry.lifecycle_namespace_queries]
- [ ] [depends:namespace-registry] Add import-only version-one tag compatibility with preserved source identity, legacy-text values, and collision rejection. r[cairn.lifecycle_namespaces.compatibility] r[cairn.global_registry.legacy_tag_compatibility]

## Phase 3: Verification and documentation

- [ ] [depends:namespace-shell] Add positive policy, core, CLI, MCP, target-evidence, registry, compatibility, receipt, and mutation-preflight fixtures. r[cairn.lifecycle_namespaces.verification.positive]
- [ ] [depends:namespace-shell] Add negative fixtures for invalid types, stale inputs, ambiguous evidence, ambient lookup, collisions, overclaims, and partial mutation. r[cairn.lifecycle_namespaces.verification.negative]
- [ ] [depends:namespace-tests] Add compile-fail coverage for forbidden typestate transitions where direct compile-time enforcement applies. r[cairn.lifecycle_namespaces.typestate.invalid] r[cairn.lifecycle_namespaces.verification]
- [ ] [depends:namespace-shell] Document central-store setup, namespace authoring, query commands, repository evidence, legacy compatibility, repair actions, and all non-claims. r[cairn.lifecycle_namespaces.verification] r[cairn.global_registry.lifecycle_namespace_queries.boundary]

## Phase 4: Final validation and lifecycle completion

- [ ] [depends:namespace-tests] Run policy freshness, focused tests, purity smoke, formatting, workspace tests, Clippy, lifecycle gates, traceability, readiness, and Nix checks. r[cairn.lifecycle_namespaces.verification]
- [ ] [depends:namespace-final-validation] Sync accepted specifications, archive the completed change, and confirm every new requirement ID exists in accepted specs before commit. r[cairn.lifecycle_namespaces.verification]
