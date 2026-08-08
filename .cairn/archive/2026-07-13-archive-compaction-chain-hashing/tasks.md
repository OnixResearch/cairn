## Tasks

- [x] [serial] Define archive compaction planning inputs, candidate selection, retention metadata, and dry-run receipt fields. r[cairn.sync_archive.compaction.plan]
- [x] [serial] Implement pure manifest normalization and BLAKE3 archive/pack root calculation. r[cairn.sync_archive.compaction.manifest_roots]
- [x] [serial] Implement pure compaction receipt hash and append-only chain hash validation. r[cairn.sync_archive.compaction.receipt_chain]
- [x] [serial] Add lossless pack execution, pointer replacement, mutation manifests, and unpack planning in the CLI shell. r[cairn.sync_archive.compaction.lossless_pack]
- [x] [serial] Add read-only compaction verification for ledgers, chains, manifests, pack objects, and pointers. r[cairn.sync_archive.compaction.verify]
- [x] [serial] Reject lossy summary compaction until an explicit reviewed policy profile enables it. r[cairn.sync_archive.compaction.lossiness_policy]
- [x] [serial] Preserve the functional-core / imperative-shell boundary for planning, hashing, execution, verification, and unpacking. r[cairn.sync_archive.compaction.boundary]
- [x] [serial] Add positive tests for deterministic plans, successful lossless pack verification, chain append, and exact-byte unpacking. r[cairn.sync_archive.compaction.tests]
- [x] [serial] Add negative tests for tampered objects, missing predecessors, stale plans, unsafe paths, duplicate manifest entries, unsupported lossy requests, and interrupted executions. r[cairn.sync_archive.compaction.tests]
- [x] [serial] Document the compaction commands, receipt fields, chain/Merkle distinction, restore workflow, and non-claims. r[cairn.sync_archive.compaction.docs]
- [x] [serial] Run focused compact tests, Cairn validation/gates, policy freshness checks, and release-readiness evidence before archive. r[cairn.sync_archive.compaction.final_validation]
