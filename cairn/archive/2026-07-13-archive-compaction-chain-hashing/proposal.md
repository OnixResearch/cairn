## Why

Cairn archive directories grow monotonically as completed changes, receipts, and evidence accumulate. Operators need a way to compact old archived changes without turning lifecycle history into unverifiable prose or relying only on Git history.

A linear hash chain is useful as an audit spine, but it is not enough by itself: operators also need content-addressed manifests that prove which exact archived files were represented and allow partial verification or restoration. This change defines archive compaction around Merkle-style archive roots plus chain-hashed compaction receipts.

## What Changes

- Add a deterministic archive compaction model for old `cairn/archive/` entries.
- Define lossless archive packs backed by BLAKE3 file identities, stable manifests, and restore metadata.
- Define compaction receipts that bind prior chain hash, pre-state root, post-state root, packed archive root, policy hash, plan hash, mutation manifest hash, and explicit lossiness.
- Add verification and unpacking requirements so compacted archives can be audited and restored before operators trust or prune source trees.
- Deny lossy summary compaction by default unless a future reviewed policy profile explicitly enables it with non-claim metadata.

## Impact

- **Storage:** Completed archived changes can be represented by compact content-addressed packs instead of fully expanded trees.
- **Auditability:** Chain hashes prove compaction ordering while Merkle roots prove the exact archived content represented by each pack.
- **Safety:** Compaction remains dry-run-first, reversible for the lossless profile, and guarded by mutation manifests plus verification receipts.
