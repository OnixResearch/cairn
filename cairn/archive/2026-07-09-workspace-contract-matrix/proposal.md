## Why

The OnixResearch workspace has several strong pairwise contracts, but there is no single lifecycle artifact that answers which repo provides which cross-repo contract, which consumer validates it, and which command proves the edge is still current. That makes broad stack changes hard to review and lets dependency/contract drift hide in repo-local docs.

This change adds a Cairn-owned workspace contract matrix that records provider, consumer, artifact, schema, identity, validation command, and non-claim boundaries for cross-repo coupling.

## What Changes

- Add a workspace contract matrix schema and validation command.
- Model provider/consumer edges such as Trellis→UCAN, UCAN→Basalt, Octet→Valence, Valence→Mantle, Mantle→Cairn, and stack inputs to Molten.
- Require each edge to name a positive fixture, negative fixture or failure class, validation command, artifact identity, owner, and non-claim boundary.
- Emit a BLAKE3-bound matrix receipt that CI or release readiness can consume.

## Impact

- **Cairn** provides the lifecycle/governance view of cross-repo contract health.
- **Provider repos** keep owning their local DTOs, proofs, and receipts.
- **Consumer repos** can cite matrix rows to show which upstream contract they validated.
- **Testing** needs positive/negative matrix fixtures and deterministic validation receipts.
