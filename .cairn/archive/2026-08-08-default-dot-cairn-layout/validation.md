# Validation

Date: 2026-08-08

The following checks passed for the hidden lifecycle-directory change:

- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target/post-move nix develop -c cargo test --workspace`
- `nix develop -c cargo fmt --check`
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target/post-move nix develop -c cargo clippy --workspace --all-targets -- -D warnings`
- `nix run path:$PWD#cairn -- policy export --check`
- `nix run path:$PWD#cairn -- validate --root .`
- `nix run path:$PWD#cairn -- tracey coverage --root .` with `2216/2216` referenced requirements
- `nix run path:$PWD#cairn -- release-readiness --root .`
- `nix build path:$PWD#checks.x86_64-linux.self-dogfood --no-link -L`
- `nix flake check path:$PWD -L`

The archive command moved the completed package to `.cairn/archive/2026-08-08-default-dot-cairn-layout/`. Post-archive validation reported zero active changes.
