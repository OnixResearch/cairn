## Why

Cairn should be able to gate lifecycle or release readiness on Trellis proof evidence without parsing proof payloads or confusing reference evidence with accepted formal proof. Trellis proof IR gives the stack typed proof metadata; Cairn needs policy gates over the chain links, roles, assumptions, and non-claims.

## What Changes

- Add Trellis proof evidence as a supported evidence-chain readiness profile.
- Validate canonical Kamacite envelope hash, Valence proof validation hash, Mantle binding hash, proof role, property/requirement scope, assumption policy, source/binary linkage, policy hashes, claim scope, and non-claims.
- Preserve optional, required-recorded, and required-formal-proof policy modes.
- Add positive and negative readiness fixtures.

## Impact

Cairn can require or observe Trellis verified-logic evidence for readiness while preserving strict boundaries around proof scope, assumptions, verifier soundness, and release eligibility.
