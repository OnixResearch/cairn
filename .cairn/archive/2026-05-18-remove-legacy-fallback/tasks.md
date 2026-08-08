## Phase 1: Spec and detection

- [x] [serial] r[cairn.supersede.native_layout] r[cairn.supersede.native_layout.native_only] Specify native-only runtime selection and migration-only legacy handling.
- [x] [serial] r[cairn.supersede.native_layout.native_only] Make normal command layout detection select native `cairn/` only.

## Phase 2: Fixtures and tests

- [x] [parallel] r[cairn.supersede.native_layout.native_only] Convert normal runtime fixtures/goldens/self-dogfood checks to native layout.
- [x] [parallel] r[cairn.supersede.migrate_openspec] Preserve explicit legacy migration tests without implicit fallback.
- [x] [parallel] r[cairn.supersede.native_layout] Update README and Hermes skill source.

## Phase 3: Verification

- [x] [serial] [depends:phase-2] Run full verification, sync, archive, and commit the clean worktree.
