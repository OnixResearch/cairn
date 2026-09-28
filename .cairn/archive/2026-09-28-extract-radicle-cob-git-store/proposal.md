## Why

Cairn's signed Radicle COB adapter contained generic Git storage and explicit signer-loading mechanics in its domain crate. Reusing that implementation as product-neutral storage requires a separate immutable source boundary while Cairn keeps lifecycle authority.

## What Changes

- Import the independently committed `onix-radicle-cob-git-store` source archive by pinned Git revision and BLAKE3 digest into a local Cargo path package; retain original dual-license texts.
- Remove Cairn's duplicate `store.rs`, migrate all COB domain modules, CLI, and interoperability tests to direct `GitCobStore` and `GitCobStoreError` types, and translate explicit signer errors into the existing domain errors.
- Preserve Cairn reducers, locks, actor authorization, CRUD flows, receipts, fixture producer independence, and local-only/no-transport claims.
- Record exact adoption provenance and a repeatable fail-closed import command.

## Impact

Cairn remains the current consumer and lifecycle-policy owner. The mechanism's source has no published remote; repeatability depends on the recorded immutable local Git revision and verified source archive until a distribution channel is supplied. The import does not confer release readiness, network convergence, or cross-repository source availability.
