## Why

The smoke path proved Cairn can gate function-address readiness, but it used wrapper JSON around Mantle, Valence, Kamacite, and graph artifacts. Cairn should consume stable versioned readiness inputs directly and fail closed on malformed or weakened schema fields.

## What Changes

- Define Cairn-owned input contracts for Valence, Mantle, Kamacite, and graph function-address readiness artifacts.
- Prefer direct Mantle binding receipts once Mantle owns a stable schema.
- Validate required hashes, roles, schema versions, claim scopes, disposition fields, and non-claim boundaries.
- Add positive and negative readiness fixtures for optional and required policies.

## Impact

Cairn release readiness no longer depends on smoke-specific wrappers while preserving lifecycle-only gating boundaries.
