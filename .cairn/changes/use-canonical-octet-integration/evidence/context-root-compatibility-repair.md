# Context root compatibility repair

## Scope

This batch gives eleven context implementation types and three functions local
names. The root API preserves every documented context name.

The local type names are `ExplicitSelector`, `IdentityObservation`,
`LocatorObservation`, `ResolvedRoot`, `DoctorFinding`, `DirectReference`,
`Observations`, `SpecResult`, `AuthorityEffect`, `Boundary`, and `BuildResult`.
The local functions are `resolve_root`, `build`, and `qualified_fetch_command`.

`api/lifecycle.rs` now uses generic compatibility macros for lifecycle-store
identity and context aliases. Each alias entry has one literal reason and one
narrow conditional `renamed_imports` allowance. The facade is 196 lines and
does not add an API module.

The root aliases preserve the original type identities and function signatures.
Serde names, schema text, values, policy rules, state transitions, and shell
effects did not change.

## Regression controls

`crates/cairn-core/tests/lifecycle_wrapper_compatibility.rs` now has context
root API controls.

The positive control resolves an explicit store, indexes a distinct direct
reference, reads its accepted specification, and type-checks every preserved
context alias. The negative control rejects a self-reference and type-checks
its root diagnostic type.

The existing context unit tests retain positive root-resolution cases and
negative missing, stale, conflicting, duplicate, and self-reference cases.

## Verification

The local Rust-source output has a known hash mismatch. Validation ran in the
isolated remote vendor environment on `aspen1`. The remote environment uses the
pinned Octet input and vendored Cargo sources. No repository pin or lockfile
changed.

| Check | Result |
|---|---|
| `cargo fmt --all` | Passed |
| `cargo test -p cairn-core --all-targets --all-features` | Passed |
| `cargo test -p cairn-cli --test lifecycle_store_shell --all-features` | 9 passed |
| `cargo clippy -p cairn-core --all-targets --all-features -- -D warnings` | Passed |
| Focused strict helper for `cairn-core` | Compiled; expected nonzero result from remaining catalog findings |

The focused strict total fell from 7,978 to 7,950 overlapping findings.
Library findings fell from 3,710 to 3,696. Library-test findings fell from
4,268 to 4,254. The fourteen renamed declarations have no remaining
`path_segment_repetition` finding. The strict index has no finding for the
lifecycle facade or the root compatibility test.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
