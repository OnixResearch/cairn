# Lifecycle opaque-wrapper name repair

## Scope

The `lifecycle_store_identity` functional core now uses these local opaque type
names:

- `Id`
- `RemoteHint`
- `NonClaim`
- `Digest`
- `CheckoutPath`

The public lifecycle facade preserves the five documented `LifecycleStore...`
root type names. Each compatibility reexport has one conditional
`renamed_imports` allowance with a specific reason. The values remain opaque,
serialize-only wrappers. No field, wire spelling, state transition, policy rule,
or shell effect changed.

## Regression controls

`lifecycle_wrapper_compatibility.rs` has both controls:

- The positive control admits an identity, registers it into a locator state,
  and type-checks every preserved root wrapper name.
- The negative control rejects an escaping store identifier and checks the
  reported `store_id` diagnostic field.

The existing lifecycle-shell integration suite continues to use the preserved
root types through the public facade.

## Verification

The pinned remote vendor environment ran these checks:

| Check | Result |
|---|---|
| `cargo fmt --all` followed by local whitespace check | Passed |
| `cargo test -p cairn-core --all-targets --all-features` | 644 unit, 4 prior compatibility, 2 interface, and 2 new wrapper controls passed |
| `cargo test -p cairn-cli --test lifecycle_store_shell --all-features` | 9 passed |
| `cargo clippy -p cairn-core --all-targets --all-features -- -D warnings` | Passed |
| Focused strict helper for `cairn-core` | Compiled; expected nonzero result from remaining catalog findings |

The focused strict total fell from 8,074 to 8,064 overlapping findings. Library
findings fell from 3,752 to 3,747. Library-test findings fell from 4,322 to
4,317. The five wrapper declarations have no remaining
`path_segment_repetition` finding. The new compatibility test has no strict
finding.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
