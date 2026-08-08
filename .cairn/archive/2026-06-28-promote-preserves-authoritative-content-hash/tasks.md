# Tasks

## 1. Core content-hash binding

- [x] I1 [serial] [depends: add-preserves-receipt-field-drift-diagnostics] Add a `preserves_content_hash` helper that computes the canonical Preserves content hash over receipt content excluding the hash fields, mirroring the existing `receipt_hash` exclusion rule. r[cairn.determinism.preserves_authoritative_content_hash.field]

## 2. Receipt binding (additive, optional)

- [x] I2 [serial] Bind `preserves_content_hash` into agent-run receipts (agent-receipt preserves-hash output) and CLI verify-content-hash command. r[cairn.determinism.preserves_authoritative_content_hash.field]
- [x] I3 [serial] Ensure receipts without `preserves_content_hash` remain valid until a reviewed policy flag requires it; keep the JSON `receipt_hash` contract unchanged. r[cairn.determinism.preserves_authoritative_content_hash.compatibility]

## 3. Verification recheck

- [x] I4 [serial] Add `cairn receipt verify-content-hash --path <receipt.json>` recomputing canonical Preserves bytes and comparing to the stored field, failing closed on mismatch or tamper. r[cairn.determinism.preserves_authoritative_content_hash.verification]

## 4. Verification

- [x] V1 [serial] Add positive (field present and correct), negative (tampered field fails closed), compatibility (receipts without the field still validate), and boundary (key-reordered/float-equivalent inputs produce equal content hashes) tests. r[cairn.determinism.preserves_authoritative_content_hash.tests]
- [x] V2 [serial] Run focused gates: `validate`, `gate proposal|design|tasks`, `release-readiness`, `preserves-corpus-fresh`, Slotcar purity smoke, clippy, fmt, and a determinism replay case. r[cairn.determinism.preserves_authoritative_content_hash.validation]
