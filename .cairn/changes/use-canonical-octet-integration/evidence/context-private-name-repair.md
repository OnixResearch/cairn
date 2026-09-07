# Context private name repair

## Scope

This batch renames two private helpers and six nested test controls in
`crates/cairn-core/src/verified/lifecycle_store_context.rs`.

The implementation now uses `resolve_locator` and `boundary`. The nested tests
use names that describe selectors, entries, references, and direct accepted
specifications without repeating module path words.

No public type, function, root facade, wire value, schema text, policy decision,
or shell effect changed.

## Regression controls

The existing context suite includes both outcomes. Positive controls resolve
explicit, native, and registered roots. Negative controls reject missing,
stale, conflicting, duplicate, and self-referential input.

The lifecycle shell suite continues to consume the unchanged root facade.

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

The focused strict total fell from 7,988 to 7,978 overlapping findings.
Library findings fell from 3,712 to 3,710. Library-test findings fell from
4,276 to 4,268. The renamed helper and test declarations have no remaining
`path_segment_repetition` finding.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
