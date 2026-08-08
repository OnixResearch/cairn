# Tasks

## Phase 1: Implementation

- [x] [serial] I1 Inspect `release_receipts.rs` families and select cohesive submodule boundaries. r[cli_maintainability.release_receipts_submodules.focused_modules.visible_boundaries]
- [x] [serial] I2 Move tests into focused submodules with a local support module and narrow helper visibility. r[cli_maintainability.release_receipts_submodules.focused_modules.narrow_bridges]

## Phase 2: Verification

- [x] [serial] V1 Run focused verification for the reorganized `release_receipts` target. r[cli_maintainability.release_receipts_submodules.behavior_preserved.focused_checks]
- [x] [serial] V2 Run repository validation, Tracey coverage, formatting, linting, and diff checks. r[cli_maintainability.release_receipts_submodules.behavior_preserved.repository_checks]

## Verification Coverage

- `cargo test -p cairn-cli --test release_receipts -- --nocapture`
- `cargo test --workspace`
- `cargo clippy -p cairn-core -p cairn-cli --all-targets -- -D warnings`
- `cairn validate --root .`
- `cairn tracey coverage --root .`
- `nix flake check path:$PWD -L`
- `git diff --check`
