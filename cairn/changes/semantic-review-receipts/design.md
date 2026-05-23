## Context

Cairn has deterministic lifecycle gates, but it lacks a first-class artifact for semantic plan/design/code review outcomes. Rothrock-style review gates produce trust evidence about omissions, systematic mistakes, and incoherence; Cairn should be able to validate and preserve those review outcomes without performing the review itself.

## Decisions

### 1. Validated evidence, not built-in judgment

**Choice:** Cairn will validate receipt structure, hashes, labels, and references. It will not claim to deterministically judge architecture quality or call an LLM.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

### 2. Review kinds are explicit

**Choice:** The schema will support `plan`, `design`, `code`, and `diff` review kinds so downstream analytics can compare gate performance.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

### 3. Receipts compose with existing evidence

**Choice:** Review receipt hashes should be usable by evidence bundles, release evidence, and future ledger analytics.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

## Risks / Trade-offs

- Overclaiming semantic correctness if docs blur validation with review.
- Receipt schema churn if reviewer outputs are not bounded early.
