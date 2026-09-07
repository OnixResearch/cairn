# Collaboration import root compatibility repair

## Scope

This batch gives four collaboration import implementation names local names.
It also renames one private identity type and two nested test controls.

The implementation uses `Snapshot`, `plan`, `initialize`, `operation_id`, and
`OperationIdentity`. `api/collaboration.rs` preserves every documented root
name with one narrow, conditional `renamed_imports` allowance and a specific
reason.

The root aliases preserve type identities and function signatures. Import
values, operation identities, artifact admission, schema text, and validation
rules did not change.

## Regression controls

`crates/cairn-core/tests/compatibility.rs` type-checks the preserved snapshot,
plan, initialize, and operation-ID root APIs. Its typed initialize wrapper calls
the root function with a concrete `String` actor.

The module unit tests remain the behavioral controls. They accept exact import
artifacts and stable operation identities, and reject unsafe identifiers and
oversized artifacts.

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

The focused strict total fell from 7,933 to 7,921 overlapping findings.
Library findings fell from 3,689 to 3,684. Library-test findings fell from
4,244 to 4,237. The six renamed declarations and test controls have no
remaining `path_segment_repetition` finding. The strict index has no finding
for the collaboration facade or compatibility test.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
