# Collaboration update root compatibility repair

## Scope

This batch gives seven collaboration-update implementation names local names.
The root API preserves every documented update name.

The implementation uses `Snapshot`, `Plan`, `CobPlan`, `build_plan`, `apply`,
`plan_hash`, and `cob_plan_hash`. `api/collaboration.rs` reexports each root
name with one narrow, conditional `renamed_imports` allowance and a specific
reason.

The root aliases preserve type identities and function signatures. Plan values,
hash values, state transitions, policy rules, and shell effects did not change.

## Regression controls

`crates/cairn-core/tests/collaboration_update_compatibility.rs` includes both
controls. Its positive control creates and applies a root no-op update plan. Its
negative control rejects an unauthorized actor. The test also type-checks the
preserved COB plan and hash root API.

The module unit tests retain sorted add, change, and remove positive cases, plus
byte-equivalent, identity, policy, actor, protocol, and conflict negative cases.

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

The focused strict total fell from 7,873 to 7,859 overlapping findings.
Library findings fell from 3,662 to 3,655. Library-test findings fell from
4,211 to 4,204. The seven renamed public declarations have no remaining
`path_segment_repetition` finding. The strict index has no finding for the
collaboration facade or update compatibility test.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
