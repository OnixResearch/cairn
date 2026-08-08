## Context

Cairn validates local requirement references, but stack evidence crosses repo boundaries. A Valence provenance link or Mantle release receipt can cite a `r[...]` ID whose owner is not the current repo. That needs a bounded registry rather than free-form strings.

## Design

Author a Nickel registry with rows for requirement namespace, owner repo, accepted spec path, status, aliases, and allowed consumer roles. The CLI shell loads registries and evidence references; the pure core checks ownership, status, alias resolution, and dangling references.

Registry validation emits a receipt with registry hash, row count, dangling refs, stale aliases, and receipt hash. It does not prove requirement satisfaction.

## Risks

- Registry centralization could become a bottleneck. Mitigate with repo-owned registry fragments merged into a stack view.
- Alias support could hide stale IDs. Mitigate with explicit status and expiry guidance.
