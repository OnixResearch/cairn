# Tasks: Preserves corpus update rail

## Implementation

- [x] I1 [serial] Add `cairn preserves-corpus update --path <corpus.json> [--execute]` with dry-run default and explicit mutation semantics. r[cairn.determinism.preserves_corpus_update_rail.command]
- [x] I2 [serial] Recompute hashes through the existing core Preserves canonical hash helper while preserving corpus values and metadata. r[cairn.determinism.preserves_corpus_update_rail.hash_recompute]
- [x] I3 [serial] Emit deterministic receipts with stale case summaries and malformed-corpus issues. r[cairn.determinism.preserves_corpus_update_rail.receipt]

## Validation

- [x] V1 [serial] Add focused CLI tests for stale dry-run detection, explicit repair, no-op clean corpus, and malformed corpus rejection. r[cairn.determinism.preserves_corpus_update_rail.validation]
- [x] V2 [serial] Run focused Preserves corpus/update tests plus Cairn validate, Tracey, clippy, workspace tests, and diff checks. r[cairn.determinism.preserves_corpus_update_rail.validation]
