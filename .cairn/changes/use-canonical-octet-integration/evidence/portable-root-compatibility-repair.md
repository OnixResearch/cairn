# Portable collaboration root compatibility repair

## Scope

This batch gives four portable collaboration implementation items local names.
The root API preserves every documented portable collaboration name.

The implementation uses `Envelope`, `Failure`, `build`, and `validate`.
`api/collaboration.rs` reexports each documented root name with one narrow,
conditional `renamed_imports` allowance and a specific reason.

The aliases preserve type identities and function signatures. Serde names,
schema text, envelope bytes, hash values, validation rules, and CLI behavior did
not change.

## Regression controls

`crates/cairn-core/tests/compatibility.rs` type-checks both preserved root types
and both preserved root functions.

The module unit tests remain the behavioral controls. They accept valid and
conflicted envelopes, and reject tampered labels, hashes, and state.

## Verification

The local Rust-source output has a known hash mismatch. Validation ran in the
isolated remote vendor environment on `aspen1`. The remote environment uses the
pinned Octet input and vendored Cargo sources. No repository pin or lockfile
changed.

| Check | Result |
|---|---|
| `cargo fmt --all` | Passed |
| `cargo test -p cairn-core --all-targets --all-features` | Passed |
| `cargo check -p cairn-cli --all-targets --all-features` | Passed |
| `cargo clippy -p cairn-core --all-targets --all-features -- -D warnings` | Passed |
| Focused strict helper for `cairn-core` | Compiled; expected nonzero result from remaining catalog findings |

A broad `cargo test -p cairn-cli --all-targets --all-features` run compiled the
changed caller surface, then stopped at
`bootstrap::publication::tests::permission_failure_and_unknown_durability_do_not_complete`.
That test is outside this patch and expected an environment-specific permission
failure. It needs separate diagnosis. The compile-only CLI control passed.

The focused strict total fell from 7,941 to 7,933 overlapping findings.
Library findings fell from 3,693 to 3,689. Library-test findings fell from
4,248 to 4,244. The four renamed public declarations have no remaining
`path_segment_repetition` finding. The strict index has no finding for the
portable API facade or compatibility test.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
