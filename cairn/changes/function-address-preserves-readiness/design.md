## Design

The pure readiness evaluator gains typed fields for canonical Kamacite Preserves receipt identity and schema metadata. The CLI shell reads Valence, Mantle, Kamacite, graph, and optional compatibility projection inputs, normalizes documented digest spellings, and passes typed values into the evaluator. Required mode fails when canonical Preserves identity is absent or stale; optional mode skips absent evidence but reports invalid present evidence.

### Decisions

1. **Kamacite Preserves identity is first-class.** Readiness inputs include canonical Preserves receipt hash and schema version rather than treating JSON as authoritative.
2. **Cairn does not parse function records.** Cairn validates lifecycle input contracts, linkage, roles, claim scopes, hashes, and non-claims only.
3. **Compatibility is bounded.** JSON wrappers remain accepted only when they bind to canonical Preserves identity and declare non-authoritative status.
4. **Required mode is strict.** Required policy fails on missing Preserves identity, stale Valence/Mantle linkage, malformed digest, unsupported schema, or weakened boundary.
5. **Lifecycle-only boundary remains.** Passing readiness means function-address evidence is present and well-formed for policy, not that Rust code is semantically correct or release-eligible.

### Validation shape

Positive fixtures cover optional absent evidence, optional present Preserves-backed evidence, and required complete Preserves-backed evidence. Negative fixtures cover missing canonical Preserves hash, stale Valence or Mantle hash, unsupported schema version, malformed digest, projection drift, wrong claim scope, role mismatch, and overclaim text.
