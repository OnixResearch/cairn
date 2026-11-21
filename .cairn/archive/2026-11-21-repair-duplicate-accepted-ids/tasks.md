# Tasks

- [x] [serial] [task:core] Record accepted duplicate IDs instead of blocking preflight, and refuse only a merged document that keeps or creates a duplicate. r[cairn.sync_archive.delta_merge.identity]
- [x] [serial] [task:repair] Apply a MODIFIED operation that retargets one duplicated accepted block by its unique heading before the remaining operations, with stable diagnostics for ambiguity and introduced duplicates. r[cairn.sync_archive.delta_merge.identity.duplicate.repair]
- [x] [serial] [task:tests] Add core positives for repair and replay plus negatives for a retained duplicate, ambiguous heading targeting, an introduced duplicate, and an unrelated delta against a still-duplicated document. r[cairn.sync_archive.delta_merge.identity.duplicate.repair]
- [x] [parallel] [task:fixtures] Add a CLI fixture modeled on the Terrane fetch-add and constant-folding duplicates; sync it, re-sync it as already applied, and validate the result. r[cairn.sync_archive.delta_merge.identity.duplicate.repair]
- [x] [serial] [task:docs] Document the repair shape, its refusals, and its unchanged receipt binding. r[cairn.sync_archive.delta_merge.identity.duplicate.repair]
- [x] [serial] [task:verification] Run focused core and CLI tests, policy freshness, validate, gates, sync, archive, post-archive validation and coverage, and the full flake check. r[cairn.sync_archive.delta_merge.identity]
