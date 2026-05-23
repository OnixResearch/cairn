## Context

Rothrock’s analysis shows the most actionable distinction is not pass/fail alone but whether a rejection is an omission, systematic error, incoherence, or infrastructure failure. Cairn needs policy-owned labels so review receipts can be aggregated without ad hoc prose parsing.

## Decisions

### 1. Four initial categories

**Choice:** Use `omission`, `systematic`, `incoherent`, and `infra` as the initial policy-owned categories.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

### 2. Decision and category are separate

**Choice:** A receipt decision such as `needs_revision` can contain multiple categorized findings.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

### 3. Policy owns vocabulary

**Choice:** Nickel policy defines allowed decisions, categories, severities, and finding scopes; Rust consumes generated policy.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

## Risks / Trade-offs

- Categories can become too coarse for later research; keep additive metadata fields.
- LLM-produced labels can be wrong; Cairn validates vocabulary, not truth.
