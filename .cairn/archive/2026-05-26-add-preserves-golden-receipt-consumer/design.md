# Design

## Boundary

The first Preserves consumer lives beside the existing agent-run receipt hash helper. It accepts the same `serde_json::Value` receipt material, removes self-referential hash fields, converts the JSON-compatible receipt into a Preserves value, encodes packed bytes, and returns a BLAKE3 hex digest.

The helper is internal Rust API only. It does not alter validation semantics: `receipt_hash` remains the canonical JSON hash currently required by public agent-run receipts.

## Hash material

The Preserves consumer excludes:

- `receipt_hash`, the existing public JSON hash field.
- `preserves_receipt_hash`, reserved for future internal fixture or receipt material to avoid self-reference.

All other receipt fields remain in the hashed material.

## Verification

Tests must prove:

- The checked-in positive agent-run fixture maps to a stable golden Preserves hash.
- Object key ordering does not affect the Preserves consumer hash.
- Mutating receipt content changes the Preserves consumer hash.

## Risk Controls

Keeping this helper additive avoids downstream API churn while giving Cairn a real artifact-level regression target for Preserves packed-byte determinism.
