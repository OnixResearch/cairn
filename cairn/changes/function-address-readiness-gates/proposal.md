## Why

Function-address evidence becomes useful to lifecycle users only when release-readiness can require or record it. Octet, Kamacite, Valence, and Mantle own production, receipt identity, linkage semantics, and bundle binding, but Cairn owns the final lifecycle/readiness policy over supplied receipts.

Cairn should add a readiness gate that consumes Valence/Mantle function-address evidence metadata, supports disabled/optional/required policy modes, and fails closed on stale or missing evidence when policy requires it. The gate must remain receipt-local: it should not parse Rust, recompute function addresses, or decide source-code correctness.

## What Changes

- Add release-readiness policy modes for function-address evidence: disabled, optional, and required.
- Validate supplied Valence/Mantle/Kamacite function-address evidence metadata, BLAKE3 digests, roles, schemas, claim scope, verification-role summary, and non-claims.
- Report function-address readiness contribution and graph summary context when supplied.
- Fail required mode for missing evidence, stale digests, malformed graph summaries, unsupported claim scopes, role promotion, weakened non-claims, or overclaims.
- Add positive and negative readiness fixtures and documentation that preserve stack ownership boundaries.

## Impact

- **Files**: Cairn policy schema/defaults, core readiness validation, CLI receipt rendering if needed, fixtures, docs, and lifecycle artifacts.
- **Testing**: focused core/CLI readiness tests, positive/negative fixture validation, policy freshness, release-readiness checks, Cairn validation/gates, and traceability coverage before archive.
