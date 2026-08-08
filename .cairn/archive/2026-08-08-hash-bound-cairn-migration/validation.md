# Validation

Date: 2026-08-08

The following checks passed for the hash-bound migration change:

- `nix develop -c cargo fmt --check`
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo test --workspace`
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo clippy --workspace --all-targets -- -D warnings`
- `nix run path:$PWD#cairn -- policy export --check`
- `nix run path:$PWD#cairn -- validate --root .`
- `nix run path:$PWD#cairn -- traceability coverage --root .` with `2223/2223` referenced requirements
- `nix run path:$PWD#cairn -- release-readiness --root .`
- `nix flake check path:$PWD -L`

The focused migration tests cover valid execution and failures for missing or stale plan hashes, changed source bytes, destination races, and legacy-only runtime commands.

The archive command moved the completed package to `.cairn/archive/2026-08-08-hash-bound-cairn-migration/`. Post-archive validation reported zero active changes.
