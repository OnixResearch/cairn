# Add Preserves canonicalization fixture corpus

## Why

Cairn now uses Preserves packed encoding as an internal receipt-hash seam for agent receipt inspection. The current tests prove object-order stability and one agent receipt golden, but they do not pin a small representative corpus of primitive, nested, Unicode, numeric, and receipt-shaped values. A fixture corpus makes future Preserves dependency upgrades or serde conversion changes fail loudly before they alter deterministic evidence hashes.

## What changes

- Add a checked-in Preserves canonicalization fixture corpus with stable case ids, JSON values, and expected BLAKE3 hashes of packed Preserves bytes.
- Add focused core tests that recompute every corpus hash, prove key-order-equivalent material hashes the same, and prove content changes alter hashes.
- Keep the corpus internal to deterministic evidence tests; no public receipt schema, CLI output, or stored JSON surface changes.
- Add live Tracey refs for corpus coverage and validation.

## Non-goals

- Do not replace JSON public/stored receipts with Preserves.
- Do not add a new public CLI command for the corpus.
- Do not widen policy-owned deterministic replay surfaces; the existing Preserves hash CLI remains the operator surface.
