# Verification

## Baseline

Transcript: `/tmp/onix-hash-policy-baseline-20260708.txt`

- Proposal, design, and tasks gates passed before implementation.
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-hash-policy nix develop -c cargo test -p cairn-core hash_algorithm -- --nocapture` passed with `0` matching tests before implementation.

## Final focused checks

Transcript: `/tmp/onix-hash-policy-checks-20260708.txt`

- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-hash-policy nix develop -c cargo fmt --check`
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-hash-policy nix develop -c cargo test -p cairn-core hash_algorithm_policy -- --nocapture`
- `nix run path:$PWD#cairn -- hash-policy verify --input fixtures/hash-algorithm-policy/positive/declared-sha256-exception.json`
- `nix run path:$PWD#cairn -- hash-policy verify --input fixtures/hash-algorithm-policy/negative/undeclared-sha256.json`

Results:

- Formatting passed.
- Core hash policy tests passed (`6` positive/negative tests).
- Positive CLI fixture passed.
- Negative CLI fixture failed closed and emitted `hash_policy.undeclared_hash`.

## Final validation and traceability

Transcript: `/tmp/onix-hash-policy-final-validation-20260708.txt`

- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-hash-policy nix develop -c cargo fmt --check`
- `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target-hash-policy nix develop -c cargo test -p cairn-core hash_algorithm_policy -- --nocapture`
- `nix run path:$PWD#cairn -- validate --root .`
- `nix run path:$PWD#cairn -- traceability coverage --root . --json`

Results: formatting, focused tests, Cairn validation, and traceability coverage all passed after archive and accepted-spec repair.

## Non-claims

This proves the bounded hash-policy contract, fixture matrix, pure validation, and thin CLI manifest shell. It does not prove artifact truth, producer correctness, algorithm cryptographic strength, or semantic correctness of artifacts referenced by declared exceptions.
