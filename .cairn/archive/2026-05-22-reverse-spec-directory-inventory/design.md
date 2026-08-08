## Context

Reverse-spec discovery is useful only if it can quickly inventory an existing brownfield tree without unsafe traversal or noisy build artifacts.

## Decisions

### 1. CLI-owned filesystem traversal

**Choice:** Keep directory walking in `cairn-cli`; pass only bounded text snapshots into `cairn-core`.

**Rationale:** Filesystem access stays in the imperative shell while the core planner remains deterministic.

### 2. Deterministic safe inventory

**Choice:** Sort expanded relative paths, ignore hidden/build/vendor directories, and read only known text-like source/doc/test extensions.

**Rationale:** Stable receipts matter more than exhaustive archaeology, and skipped paths should be visible for review.

## Non-goals

- No accepted-spec mutation.
- No LLM inference, network lookup, or semantic proof of candidate completeness.
