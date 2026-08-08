# Add Preserves operator docs

## Why

Cairn now has an internal Preserves canonicalization seam, a read-only agent receipt hash CLI, a checked-in corpus, an explicit corpus update rail, and CI drift guards. Operators need one short source of truth that explains how these pieces relate without implying Preserves replaces public JSON receipts.

## What Changes

- Add an operator-facing Preserves receipt/hash workflow document.
- Link it from existing agent-run receipt docs and README command examples.
- Document JSON `receipt_hash` vs internal Preserves hash, read-only inspection, corpus dry-run/update semantics, and CI guard expectations during Preserves dependency bumps.

## Impact

- **Files**: docs plus native Cairn determinism docs spec/archive files.
- **Testing**: Cairn validate/gates, Tracey coverage via durable source refs, docs text inspection, and focused Preserves corpus checks.
