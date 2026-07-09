## Why

Cairn already has Preserves canonicalization history, but stack adoption needs a current ingest contract for external `.preserves` evidence envelopes. Cairn should be able to validate a Preserves envelope as an imported evidence carrier while keeping lifecycle semantics in Cairn specs, policy, and receipts.

## What Changes

- Add a Preserves evidence envelope ingest contract.
- Validate schema label, canonical bytes, BLAKE3 identity, declared producer, evidence role, claim scope, and non-claims.
- Treat imported Preserves payload semantics as opaque unless a Cairn-owned adapter is registered.
- Add positive and negative fixtures for canonical envelopes, non-canonical bytes, unknown schemas, stale digests, unsupported roles, and semantic overclaims.

## Impact

- Molten/Aspen `.preserves` evidence can enter Cairn gates without format guessing.
- Cairn stays the lifecycle/gate authority and does not become a general Preserves runtime.
