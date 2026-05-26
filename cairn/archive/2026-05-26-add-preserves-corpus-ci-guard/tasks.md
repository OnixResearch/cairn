# Tasks

## 1. CI guard implementation

- [x] I1 [serial] Add a positive Nix check for a clean Preserves corpus dry-run. r[cairn.determinism.preserves_corpus_ci_guard.clean_check]
- [x] I2 [serial] Add a negative Nix check proving stale corpus hashes fail closed. r[cairn.determinism.preserves_corpus_ci_guard.stale_detection]

## 2. Validation

- [x] V1 [serial] Run the focused Preserves corpus Nix checks. r[cairn.determinism.preserves_corpus_ci_guard.validation]
- [x] V2 [serial] Run Cairn validate/gates/Tracey and relevant Rust checks. r[cairn.determinism.preserves_corpus_ci_guard.validation]
