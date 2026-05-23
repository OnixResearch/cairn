## Context

Semantic review criteria should not be hard-coded into Rust or scattered across prompts. Cairn needs policy-owned review profiles for plan, design, code, and diff review gates so teams can evolve criteria while keeping receipts structurally stable.

## Decisions

### 1. Nickel-first configuration

**Choice:** Profiles live in Nickel policy and export to JSON consumed by Rust.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

### 2. Profiles define structure, not prompt magic

**Choice:** Profiles specify required criteria ids and receipt fields, not hidden LLM prompts.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

### 3. Reviewer source is declared

**Choice:** Receipts identify whether the reviewer was manual, agentic, static, or imported.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

## Risks / Trade-offs

- Too much profile flexibility may weaken cross-project analytics.
- Policy changes will require fixture/golden refreshes.
