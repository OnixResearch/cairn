# Design: Promote Preserves to authoritative content hash

## Context

The Preserves canonicalization seam (`canonical_preserves_bytes` /
`canonical_preserves_hash` in `crates/cairn-core/src/verified/evidence.rs`)
already produces byte-stable canonical Preserves bytes and BLAKE3 hashes for
receipt-shaped values. The corpus, golden consumer, and CI guards prove
byte-stable material, receipt-verification compatibility, and positive/negative
fixture equivalence. The `add-preserves-receipt-field-drift-diagnostics` change
adds the diagnostic mapping to source IDs, satisfying the last clause of the
`format_strategy.preserves_candidate` gate.

## Decisions

### 1. Additive, not replacement

**Choice:** Introduce `preserves_content_hash` as a new field computed from
canonical Preserves bytes, stored alongside the existing JSON `receipt_hash`.
`receipt_hash` remains the public, stored, compatibility and verification
field.

**Rationale:** Every existing verifier and stored receipt depends on the JSON
`receipt_hash` contract. Replacing it would break stored receipts and every
downstream verifier. An additive field lets the canonical content identity
coexist with the compatibility surface, and lets the field roll out
incrementally.

### 2. Optional until proven, then required by policy

**Choice:** `preserves_content_hash` is optional in this change. Receipts
without it remain valid. A later reviewed policy flag makes it required once
the field is proven stable across the evidence, replay, and release-evidence
paths.

**Rationale:** Avoids a flag-day cutover. Producers add the field; consumers
verify it when present; a policy flip later enforces presence.

### 3. Verification recheck

**Choice:** `cairn receipt verify-content-hash <path>` recomputes canonical
Preserves bytes from the receipt content (excluding the hash fields, mirroring
how `receipt_hash` excludes itself) and compares to the stored
`preserves_content_hash`, failing closed on mismatch or tamper.

**Rationale:** Gives consumers a deterministic recheck independent of the
producer, mirroring the existing `agent-receipt validate` and
`determinism verify-receipt` precedent.

### 4. Content scope mirrors receipt_hash exclusion

**Choice:** The content hashed for `preserves_content_hash` excludes the
hash fields themselves (`receipt_hash`, `preserves_content_hash`) using the
same exclusion rule the JSON `receipt_hash` already applies.

**Rationale:** A receipt must not hash its own hash. Reusing the existing
exclusion rule keeps both hashes over the same content scope.

### 5. Why the ROI-02 non-goal is relaxed

**Choice:** ROI-02 deferred promotion because the diagnostic-mapping clause was
unmet. With `add-preserves-receipt-field-drift-diagnostics` landed, that clause
is satisfied and the full gate is proven. The non-goal is relaxed only for the
additive content-hash field, not as a replacement of `receipt_hash`.

## Risks / Trade-offs

- Two hashes on the same content scope is redundant until the field is
  required. Acceptable: the redundancy is the compatibility bridge, and the
  policy flip retires the JSON-only identity later.
- Producers that do not emit the field remain valid during the optional window,
  so content-identity claims are only as strong as adoption. Mitigated by
  binding the field into Cairn's own agent-run, evidence, and replay receipts
  in this change.
- Canonical Preserves bytes depend on the `preserves` crate version. The
  existing `preserves-corpus` CI guard and dependency-bump runbook already
  cover this; promotion inherits that guard.
