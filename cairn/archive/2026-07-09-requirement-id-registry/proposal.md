## Why

Requirement IDs flow through Cairn specs, Octet provenance, Valence Evidence IR, Mantle release bundles, and downstream evidence reports. Today those IDs are mostly plain strings. A shared registry would make ownership, namespace rules, allowed consumers, aliases, and deprecation explicit.

## What Changes

- Add a requirement ID registry contract for cross-repo `r[...]` IDs.
- Record owning repo, namespace, accepted spec, aliases, consumer repos, and status.
- Validate provenance/evidence references against known owners and lifecycle state.
- Include negative fixtures for unknown IDs, wrong owner, stale alias, and overbroad namespace use.

## Impact

- Cross-repo provenance can resolve requirement IDs without guessing owners.
- Repos can evolve namespaces while preserving explicit aliases.
- Cairn release readiness can reject dangling or stale stack requirement refs.
