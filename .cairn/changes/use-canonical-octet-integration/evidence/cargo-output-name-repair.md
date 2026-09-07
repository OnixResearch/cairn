# Cargo-output name repair

## Scope

This batch gives the cargo-output implementation neutral names.
`Coverage` and `coverage` replace the repeated implementation names.

`check_evidence` keeps `CargoCoverage` and `cargo_coverage` as individual,
conditional compatibility exports. Each export has its own
`renamed_imports` reason. The aliases preserve the original type and function
identities.

## Regression controls

`crates/cairn-core/tests/compatibility.rs` compiles the original public module
path. It accepts a complete libtest summary. It rejects malformed output.

The existing module test keeps the full positive and negative grammar matrix.

## Verification

The local Rust-source output has a known hash mismatch. Validation ran on
`aspen1` in `/var/tmp/cairn-octet-catalog-brittonr`. This user-owned copy uses
the pinned Cargo vendor configuration.

| Check | Result |
|---|---|
| `cargo fmt --all --check` | Passed |
| `cargo test -p cairn-core --all-targets --all-features` | Passed |
| `cargo check -p cairn-cli --all-targets --all-features` | Passed |
| `cargo clippy -p cairn-core --all-targets --all-features -- -D warnings` | Passed |
| `git diff --check` | Passed |

The direct pinned `cargo-octet` run reported 4,589 current source findings.
It has no `path_segment_repetition` finding for `CargoCoverage` or
`cargo_coverage`.

The Nix strict helper could not evaluate the copied sandbox. Its private Git
input waited on the remote SSH agent. This is an environment blocker, not a
strict acceptance result. Full strict acceptance remains required.

## Boundary

This repair does not change the libtest grammar or its fail-closed behavior.
It does not claim full Octet acceptance, release readiness, archive readiness,
or push readiness.
