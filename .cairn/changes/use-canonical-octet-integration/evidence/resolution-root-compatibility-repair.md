# Collaboration resolution root compatibility repair

## Scope

This batch gives four resolution types and six resolution functions local names.
The root API preserves every documented resolution name.

The implementation uses `Inspection`, `Snapshot`, `Plan`, `CobPlan`, `inspect`,
`inspect_value`, `plan`, `apply`, `hash_plan`, and `hash_cob_plan`.

`api/collaboration.rs` now uses one compact compatibility macro for the
collaboration change, update, and resolution alias sets. Every alias entry has a
literal reason and a narrow conditional `renamed_imports` allowance. This
refactor does not suppress an Octet lint.

The root aliases preserve type identities and function signatures. Conflict
inspection, resolution plan values, hashes, state transitions, and policy rules
did not change.

## Regression controls

`crates/cairn-core/tests/collaboration_update_compatibility.rs` type-checks all
preserved resolution root types and functions. The same file has a positive
no-op update control and a negative unauthorized-actor control.

The existing resolution suite retains positive inspection and complete-selection
cases. It also retains incomplete, duplicate, stale, unauthorized, and drifted
input negative cases.

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

The focused strict total fell from 7,859 to 7,839 overlapping findings.
Library findings fell from 3,655 to 3,645. Library-test findings fell from
4,204 to 4,194. The ten renamed public declarations have no remaining
`path_segment_repetition` finding. The strict index has no finding for the
collaboration facade or compatibility test.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
