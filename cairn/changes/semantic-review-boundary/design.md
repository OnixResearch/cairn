## Context

Adding semantic review support risks muddying Cairn’s deterministic contract. Cairn should explicitly define the boundary: external humans or agents produce semantic judgments, while Cairn validates, hashes, aggregates, and gates their evidence.

## Decisions

### 1. Cairn validates evidence, not truth

**Choice:** The product claim is structural trust evidence, not guaranteed semantic correctness.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

### 2. No ambient authority

**Choice:** Review validation does not read credentials, call reviewer APIs, or inspect hidden logs unless explicit local files are provided.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

### 3. FCIS preserved

**Choice:** Receipt parsing and validation are pure core logic; file loading and path checks remain CLI shell logic.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

## Risks / Trade-offs

- Users may ask Cairn to become an LLM reviewer; defer execution adapters until receipt validation is stable.
- Boundary docs must be prominent to avoid marketing overclaims.
