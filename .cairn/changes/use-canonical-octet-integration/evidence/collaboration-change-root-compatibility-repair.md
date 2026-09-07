# Collaboration change root compatibility repair

## Scope

This batch gives seven collaboration-change implementation names local names.
The root API preserves every documented collaboration-change name.

The implementation uses `Evidence`, `Error`, `Init`, `Action`, `Operation`,
`State`, and `action_json`. `api/collaboration.rs` reexports every preserved
name with one narrow, conditional `renamed_imports` allowance and a specific
reason.

Internal verified modules now use the local names through their qualified
implementation paths. Root callers retain the documented names. Type identities,
function signatures, Serde names, schema text, values, state transitions, and
policy rules did not change.

## Regression controls

`crates/cairn-core/tests/compatibility.rs` type-checks the preserved operation,
state, error, init, action, evidence, and action-JSON root APIs.

The existing collaboration-change suite retains positive neutral-schema and
transport-neutral action cases. It also retains malformed and legacy-revision
negative cases.

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

The focused strict total fell from 7,907 to 7,891 overlapping findings.
Library findings fell from 3,678 to 3,670. Library-test findings fell from
4,229 to 4,221. The seven renamed declarations and their qualified internal
callers have no remaining `path_segment_repetition` finding. The strict index
has no finding for the collaboration facade or compatibility test.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
