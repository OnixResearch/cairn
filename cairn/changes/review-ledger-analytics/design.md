## Context

Cairn can store deterministic receipts, but it lacks a trust-ledger summary over semantic review outcomes. Rothrock’s analysis becomes useful because gate outcomes can be aggregated by gate, decision, error type, and recovery. Cairn should produce local deterministic analytics over checked-in or supplied review receipts.

## Decisions

### 1. Local files only

**Choice:** The first implementation reads explicit local receipt files/directories; no log scraping, network, or hidden session database access.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

### 2. Stable aggregate schema

**Choice:** Counts and percentages are sorted and rounded deterministically.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

### 3. Composable receipts

**Choice:** Ledger analytics should cite input receipt hashes and produce its own receipt hash.

**Rationale:** This keeps the slice reviewable, deterministic, and compatible with Cairn's receipt-first lifecycle.

## Risks / Trade-offs

- Small sample sizes can be overinterpreted; receipt should include counts and warnings.
- Input receipt discovery must avoid hidden/generated noise unless explicitly included.
