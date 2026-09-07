# Collaboration update private name repair

## Scope

This batch renames four private helpers or identity types and one nested test
control in `crates/cairn-core/src/verified/collaboration_update.rs`.

The implementation now uses `hash_plan`, `artifact_actions`,
`derive_operation_id`, and `OperationIdentity`. No public type, function, root
facade, plan value, hash value, state transition, or policy rule changed.

## Regression controls

The existing update suite retains positive sorted add, change, and remove cases.
It also retains byte-equivalent, wrong-identity, policy, actor, protocol, and
conflict negative cases.

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

The focused strict total fell from 7,882 to 7,873 overlapping findings.
Library findings fell from 3,666 to 3,662. Library-test findings fell from
4,216 to 4,211. The renamed private declarations and test control have no
remaining `path_segment_repetition` finding.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
