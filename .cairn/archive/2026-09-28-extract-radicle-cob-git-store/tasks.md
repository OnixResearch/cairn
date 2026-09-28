## Source admission

- [x] [serial] Record the independent source commit, whole-archive BLAKE3, and original license texts. r[cairn.radicle_cob_store_extraction.pinned_vendor]
- [x] [serial] Implement the pinned import script, verify no-op reimport from another directory, and refuse an altered vendor snapshot. r[cairn.radicle_cob_store_extraction.pinned_vendor]
- [x] [serial] Add the in-repository Cargo package and lockfile entry without a sibling-path product dependency. r[cairn.radicle_cob_store_extraction.pinned_vendor]

## Domain cutover

- [x] [serial] Replace the duplicate store with direct `GitCobStore` and `GitCobStoreError` use in five domain modules, CLI, and interoperability tests. r[cairn.radicle_cob_store_extraction.cutover]
- [x] [serial] Preserve domain error variants, signer paths, reducers, authorization, locks, receipts, and signed/ref fixture interoperability. r[cairn.radicle_cob_store_extraction.cutover]

## Verification and lifecycle

- [x] [parallel] Document adoption provenance, owner, repeatable import path, bounded claims, and local-only behavior. r[cairn.radicle_cob_store_extraction.boundary]
- [x] [serial] Run focused adapter and CLI smoke, fmt, Clippy, lifecycle validation/traceability/gates, and relevant Nix checks; sync and archive only after passing evidence. r[cairn.radicle_cob_store_extraction.boundary]
