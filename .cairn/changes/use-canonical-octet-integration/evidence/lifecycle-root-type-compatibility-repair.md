# Lifecycle root type compatibility repair

## Scope

This batch gives fourteen lifecycle implementation types local names. The root
API keeps every documented `LifecycleStore...` type name.

The local names are `Projection`, `Admitted`, `Diagnostic`, `Admission`,
`MappingProjection`, `LocatorProjection`, `Mapping`, `LocatorState`,
`LocatorAdmission`, `LocatorOperation`, `LocatorLocalFacts`,
`LocatorPortableFacts`, `LocatorPlan`, and `LocatorApplication`.

Each facade reexport has one conditional `renamed_imports` allowance and a
specific compatibility reason. Each reexport preserves the original type
identity. Serde names, schema text, field order, hash framing, policy rules,
state transitions, and shell effects did not change.

## Regression controls

`crates/cairn-core/tests/lifecycle_wrapper_compatibility.rs` now type-checks all
five opaque wrapper aliases and all fourteen root type aliases.

Its positive control admits an identity, builds a locator plan, checks both
hashes, admits a locator projection, applies the plan, and inspects its mapping.
Its negative control rejects an unsafe identifier and type-checks its diagnostic.

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

The focused strict total fell from 8,038 to 8,010 overlapping findings.
Library findings fell from 3,737 to 3,723. Library-test findings fell from
4,301 to 4,287. The fourteen renamed implementation declarations have no
remaining `path_segment_repetition` finding. The strict index has no finding
for the lifecycle API facade or the compatibility test.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
