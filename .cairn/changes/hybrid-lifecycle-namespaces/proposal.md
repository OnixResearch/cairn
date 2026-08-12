## Why

Lifecycle stores can centralize planning, but they do not provide a typed classification vocabulary. Global-registry tags use string fields and remain derived records.

A shared store cannot reject value-type errors or distinguish classifications from lifecycle state. It also cannot bind coordinated completion to repository-owned evidence.

Cairn needs lifecycle namespaces for central planning across repositories. Repository validation, implementation evidence, and release authority must remain local.

## What Changes

- Add a typed lifecycle-namespace catalog with Nickel source and a checked JSON projection.
- Replace new domain uses of tags with namespace definitions, values, and assignments.
- Use nominal Rust newtypes for IDs, hashes, subject references, and repository references.
- Use typestate only for admission stages that must occur in order.
- Add typed value kinds, subject applicability, cardinality, authority classes, and namespace constraints.
- Keep lifecycle state, graph relations, authorization, evidence satisfaction, and release readiness outside namespace assignments.
- Add explicit repository targets for central changes and BLAKE3-bound repository completion evidence.
- Add deterministic namespace inspection and query receipts for CLI and MCP clients.
- Add an explicit global-registry adapter for namespace projections and import-only compatibility for legacy tag records.
- Add positive and negative fixtures for type admission, stale projections, target evidence, compatibility, and claim boundaries.

## Impact

- **Policy**: Add Nickel contracts, namespace value kinds, named bounds, compatibility rules, and required non-claims.
- **Core**: Add pure newtypes, typestate admission, assignment validation, target-evidence binding, queries, and receipt identities.
- **CLI**: Add namespace export, inspection, query, and explicit repository-evidence inputs.
- **MCP**: Add read-only namespace inspection and query tools with strict schemas.
- **Registry**: Add namespace records and a compatibility adapter for legacy tag assignments.
- **Documentation**: Add a central-store namespace guide and migration notes.
- **Testing**: Add focused policy, core, CLI, MCP, registry, mutation-preflight, and receipt tests.

## Out of Scope

- Network discovery, cloning, fetching, pushing, or repository synchronization.
- One atomic transaction across several repositories.
- Central-store authority over repository implementation, validation, or release decisions.
- Encoding dependencies, lifecycle state, authorization, or evidence truth as namespace values.
- Generating Rust types from operator-defined namespace catalogs.
- Treating global-registry membership or namespace query results as gate satisfaction.
