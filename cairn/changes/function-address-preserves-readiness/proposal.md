## Why

Cairn is the final lifecycle/readiness gate for stack evidence. If Kamacite Preserves receipts become the canonical function-address interchange form, Cairn readiness should recognize Preserves-backed receipt identity directly instead of relying on JSON wrappers from the first smoke.

## What Changes

- Extend function-address readiness inputs to include canonical Kamacite Preserves receipt identity and schema metadata.
- Validate Preserves receipt hash presence, Valence/Mantle linkage, claim scope, roles, schema versions, digest spelling, and non-claims.
- Keep JSON wrappers as compatibility only when bound to canonical Preserves identity.
- Add positive and negative release-readiness fixtures for optional and required Preserves-backed modes.

## Impact

Cairn can gate lifecycle readiness over durable Preserves-backed function-address evidence while preserving its non-claim boundary and optional adoption path.
