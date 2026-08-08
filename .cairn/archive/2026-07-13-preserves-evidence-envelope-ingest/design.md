## Design

Define an ingest manifest row with envelope path, schema label, producer repo, canonical digest, evidence role, claim scope, adapter ID, payload opacity mode, and required non-claims. The pure core validates parsed rows and measured envelope metadata. The CLI shell reads files, checks canonical-byte measurement, and emits deterministic receipts.

If `payload_opacity` is opaque, Cairn validates only carrier identity and declared role/non-claims. If an adapter ID is registered, Cairn may validate that bounded slice through existing adapter contracts.

## Non-claims

Ingest acceptance does not prove the producer's runtime semantics, transport behavior, or evidence truth. It only proves that Cairn accepted a canonical carrier for a declared role/scope under policy.
