# Design: Preserves corpus policy inventory

Extend `cairn-policy/default.ncl` with a deterministic surface for the corpus freshness rail. Add a replay case for `preserves_corpus_freshness` that checks `fixtures/preserves-canonicalization/corpus.json` in dry-run mode and records deterministic receipt fields.

Execution should reuse existing CLI/core functions where practical; if replay remains in-process, avoid shelling out and avoid mutation.
