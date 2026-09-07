# Collaboration checkout root compatibility repair

## Scope

This batch gives two collaboration checkout implementation types local names.
It also shortens two nested test names.

The implementation uses `LocalSnapshot` and `Plan`.
`api/collaboration.rs` preserves both documented root types with narrow,
conditional `renamed_imports` allowances and specific reasons.

The aliases preserve the original type identities. Checkout planning, state
values, shell effects, and destination decisions did not change.

## Regression controls

`crates/cairn-core/tests/compatibility.rs` type-checks the preserved checkout
snapshot, plan, and planning-function root API.

The module tests remain the behavioral controls. They accept absent and exact
destinations, and reject divergent destinations.

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

The focused strict total fell from 7,921 to 7,915 overlapping findings.
Library findings fell from 3,684 to 3,682. Library-test findings fell from
4,237 to 4,233. The renamed types and test controls have no remaining
`path_segment_repetition` finding. The strict index has no finding for the
collaboration facade or compatibility test.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
