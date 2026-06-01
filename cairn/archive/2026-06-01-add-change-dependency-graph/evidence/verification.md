# Verification: add-change-dependency-graph

## Baseline

- `nix run path:$PWD#cairn -- mcp --call cairn_drain_plan '{"root":"."}'`
- `nix run path:$PWD#cairn -- validate --root .`
- `nix run path:$PWD#cairn -- gate proposal add-change-dependency-graph --root .`
- `nix run path:$PWD#cairn -- gate design add-change-dependency-graph --root .`
- `nix run path:$PWD#cairn -- gate tasks add-change-dependency-graph --root .`

Result: baseline commands completed in pueue task 172.

## Focused implementation rails

- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo test -p cairn-core change_dependencies --lib`
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo test -p cairn-cli --test lifecycle --test lifecycle_mutations --test mcp_cli`

Result: final run passed in pueue task 205.

## Closeout rails

- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo fmt --check`
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop -c cargo clippy -p cairn-core -p cairn-cli --all-targets -- -D warnings`
- `nix run path:$PWD#cairn -- policy export --check`
- `nix run path:$PWD#cairn -- validate --root .`
- `nix run path:$PWD#cairn -- gate proposal add-change-dependency-graph --root .`
- `nix run path:$PWD#cairn -- gate design add-change-dependency-graph --root .`
- `nix run path:$PWD#cairn -- gate tasks add-change-dependency-graph --root .`
- `nix run path:$PWD#cairn -- tracey coverage --root .`

Result: final bundle passed in pueue task 205 after `sync --execute` promoted the accepted dependency graph spec. Tracey reported `tracey bootstrap coverage ok: 921/921 referenced`.

## Notes

Remote builder warnings for `ssh-ng://root@10.10.10.1` appeared during Nix commands and were non-blocking; Nix continued locally and the requested checks passed.
