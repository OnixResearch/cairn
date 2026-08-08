## Context

Rothrock’s weakest interface was the revision cycle after a rejection. Cairn should model the handoff from review findings to revision attempts so agents can prove which findings were addressed, which remain, and whether new findings appeared.

## Decisions

### 1. Hash-linked review chain

**Choice:** A revision handoff cites the previous review receipt hash and, when available, the follow-up review receipt hash.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

### 2. Disposition is explicit

**Choice:** Every prior finding must have a disposition with optional evidence refs.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

### 3. Recovery analytics ready

**Choice:** The schema should support later recovery-rate aggregation without rereading prose.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

## Risks / Trade-offs

- Agents may mark findings addressed without evidence; policy should require refs for addressed findings.
- Revision chains can become complex; first slice should support linear chains before branching.
