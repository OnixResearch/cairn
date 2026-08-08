## Phase 1: Rename contract

- [x] [serial] Update accepted/spec delta contract for canonical Cairn identity. r[cairn.supersede.rename_cairn] r[cairn.supersede.native_layout.init_cairn]
- [x] [serial] Run proposal, design, and tasks gates for the rename package. r[cairn.supersede.rename_cairn.classification]

## Phase 2: Implementation surfaces

- [x] [serial] Rename Rust package/crate/binary surfaces, public help text, README/docs, repo-maintained Hermes skill source, and Nix package/app/check attributes to canonical `cairn`. r[cairn.supersede.rename_cairn.cli] r[cairn.supersede.rename_cairn.packaging]
- [x] [serial] Implement explicit legacy `ironspec` compatibility aliases and canonical-over-legacy lifecycle layout precedence. r[cairn.supersede.rename_cairn.legacy_alias] r[cairn.supersede.native_layout.prefer_cairn]
- [x] [serial] Add deterministic migration support from legacy `ironspec/` lifecycle content to canonical `cairn/` content. r[cairn.supersede.migrate_openspec.ironspec_dry_run] r[cairn.supersede.migrate_openspec.ironspec_execute]

## Phase 3: Evidence and archive

- [x] [serial] Add and save deterministic rename classification evidence that fails on `needs-fix` legacy-name occurrences. r[cairn.supersede.rename_cairn.classification]
- [x] [serial] Update fixtures/goldens and run focused CLI tests, `cargo test --workspace`, `cairn validate --root .`, and `nix flake check path:$PWD -L`. r[cairn.supersede.rename_cairn]
- [x] [serial] Sync accepted specs, archive the change, and commit the completed rename. r[cairn.supersede.rename_cairn]
