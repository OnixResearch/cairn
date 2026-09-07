# Portable collaboration private name repair

## Scope

This batch renames three private implementation items and three nested test
controls in `crates/cairn-core/src/verified/collaboration_portable.rs`.

The implementation now uses `derive_content_root`, `hash_envelope`, and
`EnvelopeIdentity`. The tests use short names that describe round trips and
tampering without repeating module path words.

No public type, function, root facade, wire value, schema text, hash value, or
validation rule changed.

## Regression controls

The existing portable collaboration suite includes both outcomes. Positive
controls preserve exact state and hashes, including conflicted state without a
content-root claim. Negative controls reject label, hash, and state tampering.

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

The focused strict total fell from 7,950 to 7,941 overlapping findings.
Library findings fell from 3,696 to 3,693. Library-test findings fell from
4,254 to 4,248. The renamed private declarations and test controls have no
remaining `path_segment_repetition` finding.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
