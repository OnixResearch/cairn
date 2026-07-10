## Why

Cairn readiness inputs may arrive from Valence, Mantle, and Kamacite with either `b3:<hex>` or bare BLAKE3 hex depending on the producer boundary. Cairn should accept only documented spellings and validate digest domains so readiness cannot pass because of arbitrary digest strings.

## What Changes

- Define supported digest spellings for function-address readiness inputs.
- Normalize allowed spellings in the CLI shell while preserving typed readiness fields.
- Reject malformed hashes, unsupported prefixes, uppercase variants where not allowed, and cross-field digest substitution.
- Add positive and negative readiness fixtures.

## Impact

Cairn remains a robust readiness gate while Valence owns the canonical evidence identity layer.
