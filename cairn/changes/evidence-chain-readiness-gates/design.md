## Design

Cairn adds a pure evidence-chain readiness evaluator over typed input records. The evaluator checks generic chain linkage and profile registration, then reports optional/required readiness disposition. Profile-specific gates can add extra constraints, but the base evaluator never parses profile payloads or promotes evidence truth.

### Decisions

- **Generic gate first.** Readiness validates evidence kind, profile version, canonical envelope identity, Valence validation identity, Mantle binding identity, source/binary linkage, policy hashes, claim scope, projections, and non-claims.
- **Profile registry is explicit.** Function-address is the first registered evidence-chain profile; unknown profiles fail closed in required mode.
- **Payloads remain opaque.** Cairn does not parse Rust functions, proof terms, Preserves internals, or release binaries.
- **Policy modes stay gradual.** Optional mode skips absent evidence, checks present evidence, and reports invalid present evidence; required mode fails on missing or invalid evidence.
- **Chain root is deterministic.** Cairn emits an evidence-chain readiness hash that commits to typed upstream links and gate policy identity.
- **No release eligibility promotion.** Passing a profile gate means lifecycle policy conformance for that evidence kind only.

### Validation shape

Positive fixtures cover optional absent evidence, optional valid present evidence, required function-address evidence, and a non-function-address stub. Negative fixtures cover unknown profile, missing canonical envelope hash, stale Valence/Mantle hash, malformed digest, source/binary mismatch, unsupported claim scope, stale policy hash, projection drift, and weakened non-claims.
