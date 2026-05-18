## Context

The CLI currently detects `cairn/` when present and otherwise selects legacy `openspec/`. That fallback was useful during migration but now contradicts the native lifecycle direction.

## Decisions

### 1. Runtime commands select native layout only

**Choice:** Change layout detection for normal commands to always select `LifecycleLayout::Native`.

**Rationale:** Operators should get deterministic native receipts and must use `migrate openspec` explicitly for old repositories.

**Alternative rejected:** Keep fallback but warn in `doctor`; this preserves ambiguity and lets old fixtures continue to mask missing native content.

### 2. Keep legacy enum for migration source

**Choice:** Keep `LifecycleLayout::LegacyOpenSpec` only for migration planning/copying and tests that directly verify migration input/output paths.

**Rationale:** Removing the enum would make migration code less explicit and risk changing import behavior.

### 3. Convert behavioral fixtures to native layout

**Choice:** Copy canonical fixture lifecycle content from `openspec/` to `cairn/` for normal command tests, while migration tests keep legacy-only temp copies.

**Rationale:** Fixtures should represent supported runtime layout; migration remains separately tested.

## Risks / Trade-offs

- Existing users with only `openspec/` must run migration before normal commands see content.
- Upstream `openspec validate` still reads the repo's `openspec/` directory for change management; Cairn runtime tests should not rely on that fallback.