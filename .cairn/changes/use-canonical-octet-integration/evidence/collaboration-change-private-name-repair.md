# Collaboration change private name repair

## Scope

This batch renames three private admitted-value structs and one private
admission helper in `crates/cairn-core/src/verified/collaboration_change.rs`.

The implementation now uses `AdmittedInit`, `AdmittedOperation`,
`AdmittedState`, and `admit_operation`. No public type, function, root facade,
wire value, schema text, state transition, or policy rule changed.

## Regression controls

The existing collaboration-change tests retain positive neutral-schema and
transport-neutral action cases. They also retain malformed and legacy-revision
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
| `cargo clippy -p cairn-core --all-targets --all-features -- -D warnings` | Passed |
| Focused strict helper for `cairn-core` | Compiled; expected nonzero result from remaining catalog findings |

The focused strict total fell from 7,915 to 7,907 overlapping findings.
Library findings fell from 3,682 to 3,678. Library-test findings fell from
4,233 to 4,229. The renamed private declarations have no remaining
`path_segment_repetition` finding.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
