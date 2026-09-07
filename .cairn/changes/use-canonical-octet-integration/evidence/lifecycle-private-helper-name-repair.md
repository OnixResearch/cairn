# Lifecycle private helper name repair

## Scope

This batch renames ten private helpers and six nested test controls in
`crates/cairn-core/src/verified/lifecycle_store_identity.rs`.

The new helper names describe local roles without repeating lifecycle-store
path words. The changes include `validate_schema`, `parse_id`, `assemble`,
`parse_digest`, `local_state_digest`, and `portable_state_digest`.

All changed items are private to the functional core or its nested test module.
The public facade, Serde names, schema text, hash framing, policy decisions,
state transitions, and shell effects did not change.

## Regression controls

Existing lifecycle controls exercise both outcomes:

- A positive control admits an identity, registers it, and applies its plan.
- Negative controls reject unsafe identifiers, duplicate identifiers, duplicate
  paths, stale plans, and changed digests.

The public lifecycle facade continues to expose the preserved wrapper names.

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

The focused strict total fell from 8,064 to 8,038 overlapping findings.
Library findings fell from 3,747 to 3,737. Library-test findings fell from
4,317 to 4,301. The renamed helper and test declarations have no remaining
`path_segment_repetition` finding.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
