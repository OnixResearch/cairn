# Tasks: Split evidence registry CLI source

## Phase 1: Source split

- [x] [serial] r[cairn.supersede.split_evidence_registry_cli_source.focused_modules] Inspect `evidence_registry.rs` dependencies and choose submodule boundaries.
- [x] [serial] r[cairn.supersede.split_evidence_registry_cli_source.focused_modules] Extract registry build/verify, mirror verification, consume/select/fetch-plan, and fetch-execute implementation into focused modules.
- [x] [serial] r[cairn.supersede.split_evidence_registry_cli_source.behavior_preserved] Preserve public CLI dispatch, option parsing, JSON receipt shapes, issue codes, and deterministic hashes.

## Phase 2: Verification and archive

- [x] [serial] r[cairn.supersede.split_evidence_registry_cli_source.behavior_preserved] Run focused registry/evidence CLI tests and command-family compilation checks.
- [x] [serial] r[cairn.supersede.split_evidence_registry_cli_source.behavior_preserved] Run full repository validation: fmt, clippy, tests, Cairn validate, Tracey coverage, and diff checks.
- [x] [serial] r[cairn.supersede.split_evidence_registry_cli_source.focused_modules] Archive, commit, push, and leave the repository clean.
