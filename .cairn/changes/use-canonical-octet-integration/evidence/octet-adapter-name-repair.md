# Octet adapter name repair

## Scope

This batch gives the Octet adapter neutral implementation names.
`Status`, `Admission`, and `admit_json` replace repeated names.

`check_evidence` keeps `OctetStatus`, `OctetAdmission`, and
`admit_octet_json` as individual conditional compatibility exports. The
aliases preserve the public enum, DTO, and function identities.

## Regression controls

`crates/cairn-core/tests/compatibility.rs` compiles the original function and
DTO path. It keeps the `passed` enum wire value. It rejects an unknown wire
value.

The module test keeps the original accepted receipt and rejected input matrix.

## Verification

The local Rust-source output has a known hash mismatch. Validation ran on
`aspen1` in `/var/tmp/cairn-octet-catalog-brittonr`. The copy uses the pinned
Cargo vendor configuration.

| Check | Result |
|---|---|
| `cargo fmt --all --check` | Passed |
| `cargo test -p cairn-core --all-targets --all-features` | Passed |
| `cargo check -p cairn-cli --all-targets --all-features` | Passed |
| `cargo clippy -p cairn-core --all-targets --all-features -- -D warnings` | Passed |
| `git diff --check` | Passed |

The direct pinned `cargo-octet` check reported 4,583 current source findings.
The preceding direct run reported 4,589 findings. The repaired adapter has no
remaining `path_segment_repetition` finding.

The Nix strict helper cannot evaluate the copied sandbox. Its private Git
input waits on the remote SSH agent. This is an environment blocker. It is not
strict acceptance evidence.

## Boundary

This repair does not change receipt admission, status wire values, or
fail-closed behavior. It does not claim full Octet acceptance, release
readiness, archive readiness, or push readiness.
