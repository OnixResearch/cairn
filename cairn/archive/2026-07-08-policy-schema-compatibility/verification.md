# Verification

## Baseline

- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-policy-schema-baseline nix develop -c cargo test -p cairn-core verified::policy -- --nocapture`
- Result: passed before implementation (`13` policy tests passed).

## Final checks

Transcript: `/tmp/onix-policy-schema-required-checks-20260708.txt`

- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-policy-schema-after nix develop -c cargo fmt --check`
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-policy-schema-after nix develop -c cargo test -p cairn-core verified::policy -- --nocapture`
- `nix run path:$PWD#cairn -- validate --root .`
- Stale downstream policy diagnostic from `basalt` cwd using the updated Cairn binary.

Results:

- Formatting passed.
- Focused policy tests passed (`18` policy tests passed, including positive and negative compatibility fixtures).
- Cairn validation passed.
- The stale Basalt generated policy now fails with `diagnostic=policy-refresh-required`, names `missing_field=steel_orchestration_policy`, and prints the refresh command.

## Post-archive repair and coverage

Transcript: `/tmp/onix-policy-schema-final-validation-20260708.txt`

After archive, the accepted spec was checked to ensure the new requirement IDs were present. Final checks passed:

- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-policy-schema-after nix develop -c cargo fmt --check`
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-policy-schema-after nix develop -c cargo test -p cairn-core verified::policy -- --nocapture`
- `nix run path:$PWD#cairn -- validate --root .`
- `nix run path:$PWD#cairn -- traceability coverage --root . --json`

## Non-claims

This evidence proves policy schema metadata parsing, compatibility classification, and stale-policy diagnostics for the focused fixtures. It does not prove that downstream repos have refreshed their checked-in generated policy JSON.
