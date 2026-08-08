# Design: registry manifest binding rail

## Context

The evidence chain currently has three layers:

1. Evidence bundles and receipts prove local agent/replay/release/coverage artifacts.
2. Evidence indexes publish one bundle as a stable catalog artifact.
3. Evidence registries aggregate indexes and expose label selectors for downstream CI/agents.

Evidence manifests already provide an outer artifact index for bundle-era evidence, but registries are not yet first-class manifest artifacts. This leaves a review gap: downstream consumers can verify a registry, but cannot bind that registry and its selected/fetch-plan receipts into the same manifest envelope they already use for other evidence artifacts.

## Decisions

### 1. Add registry-aware manifest inputs

**Choice:** Extend `cairn evidence-manifest generate` with optional registry artifact inputs:

- `--registry PATH`
- `--registry-verification PATH`
- `--registry-selection PATH`
- `--registry-fetch-plan PATH`

**Rationale:** The manifest generator remains the single artifact-index shell. It does not rebuild registries or fetch bundles; it indexes already-produced artifacts and their hashes.

### 2. Verify registry artifacts during manifest verification

**Choice:** `cairn evidence-manifest verify` reuses the existing registry verification receipt semantics when a manifest includes a registry artifact.

**Rationale:** The verifier should not merely hash the registry bytes. It must prove the registry is internally valid and label-policy compliant using the same registry rail operators trust elsewhere.

### 3. Preserve metadata-only signing semantics

**Choice:** The manifest continues to hash optional signer/anchor metadata and artifact bytes, but it does not claim cryptographic signature validation.

**Rationale:** This matches existing signed-evidence-manifest semantics and avoids overclaiming.

### 4. Keep receipts portable and secret-safe

**Choice:** Manifest artifacts record stable roles, hashes, schema versions, and safe summaries. Receipts never include raw absolute roots, raw env values, credentials, or secrets.

**Rationale:** Registry manifests are intended for CI/agent portability across machines and mirrors.

## Risks / Trade-offs

- **Role sprawl:** Mitigated by a small fixed role set for registry artifacts.
- **Stale fixture hashes:** Policy/schema updates may require fixture and lifecycle golden regeneration.
- **Verification recursion:** Registry verification is bounded to registry/index metadata already handled by existing commands; no network fetch is introduced.
