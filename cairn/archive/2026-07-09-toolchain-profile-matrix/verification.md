# Verification

## Baseline

Transcript: `/tmp/onix-toolchain-profile-baseline-20260709.txt`

- Proposal, design, and tasks gates passed before implementation.
- `nix run path:$PWD#cairn -- validate --root .` passed.
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-toolchain-profile nix develop -c cargo test -p cairn-core toolchain_profile_matrix -- --nocapture` passed with `0` matching pre-change tests.

## Focused implementation checks

Transcript: `/tmp/onix-toolchain-profile-focused-20260709.txt`

- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-toolchain-profile nix develop -c cargo fmt`
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-toolchain-profile nix develop -c cargo fmt --check`
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-toolchain-profile nix develop -c cargo test -p cairn-core toolchain_profile_matrix -- --nocapture`
- `nix run path:$PWD#cairn -- toolchain-profile verify --input fixtures/toolchain-profile-matrix/positive/profile-classes.json`
- Negative fixture matrix through `toolchain-profile verify`.

Results: formatting passed; focused core tests passed (`5` tests); valid profile-class fixture passed; negative fixtures for missing pins, missing commands, ambiguous profile, and weakened non-claims failed closed.

## Archive and final validation

Transcripts:

- `/tmp/onix-toolchain-profile-gate-sync-archive-20260709.txt`
- `/tmp/onix-toolchain-profile-post-archive-repair-20260709.txt`
- `/tmp/onix-toolchain-profile-final-validation-20260709.txt`

Results:

- Proposal, design, and tasks gates passed.
- Cairn validation passed before sync, after sync, after archive, after accepted-spec repair, and in final validation.
- Sync initially created an empty accepted spec wrapper; `cairn/specs/toolchain-profile-matrix/spec.md` was repaired from the archived spec delta.
- Final `cargo fmt --check`, focused core tests, scoped clippy for `cairn-core`/`cairn-cli`, positive/negative profile fixture matrix, Cairn validation, and traceability coverage all passed.

## Non-claims

This proves matrix declaration completeness, purpose classification, fixture behavior, and receipt shape. It does not prove command success, release eligibility, proof soundness, or compatibility without separate execution receipts.
