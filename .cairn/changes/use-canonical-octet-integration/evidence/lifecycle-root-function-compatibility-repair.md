# Lifecycle root function compatibility repair

## Scope

This batch gives eleven lifecycle implementation functions local names. The
root API keeps every documented lifecycle-store function name.

The local names are `admit`, `material`, `source_digest`, `empty_locator`,
`locator_projection`, `admit_locator`, `plan_registration`,
`plan_unregistration`, `apply_locator_plan`, `plan_hash`, and `portable_hash`.

`api/lifecycle.rs` uses one small macro for the compatibility reexports. Every
macro entry has its own literal reason and narrow conditional
`renamed_imports` allowance. `rustfmt::skip` preserves the compact invocation
list. It does not suppress an Octet lint.

The facade remains 300 lines, which is the configured file limit. The batch
does not add a module or exceed the API module-file limit. Root callers retain
their function names, signatures, values, Serde names, schema text, hash
framing, policy rules, state transitions, and shell effects.

## Regression controls

`crates/cairn-core/tests/lifecycle_wrapper_compatibility.rs` uses every root
lifecycle-store function. Its positive control admits an identity, checks its
material, builds and hashes a registration plan, reads a locator projection,
re-admits it, applies the plan, and plans unregistration.

Its negative control rejects an unsafe identifier and checks the root
diagnostic type. The same test also type-checks every preserved root type.

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

The focused strict total fell from 8,010 to 7,988 overlapping findings.
Library findings fell from 3,723 to 3,712. Library-test findings fell from
4,287 to 4,276. The eleven renamed function declarations have no remaining
`path_segment_repetition` finding. The strict index has no finding for the
lifecycle facade or the root compatibility test.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
