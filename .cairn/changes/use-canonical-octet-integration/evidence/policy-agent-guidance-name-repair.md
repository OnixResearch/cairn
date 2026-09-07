# Policy agent-guidance private name repair

## Scope

`verified/policy/agent_guidance.rs` contained the private helper
`agent_guidance_policy` inside the `agent_guidance` module. The helper now has
the local name `parse`. `policy/parser.rs` imports and calls that private helper
by its new name.

The helper remains `pub(super)`. This change does not affect a public API,
policy JSON field, Serde spelling, receipt, or behavior.

## Verification

The same pinned remote vendor environment used for the preceding compatibility
repair ran these checks:

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | Passed |
| `cargo test -p cairn-core --all-targets --all-features` | 644 unit, 4 compatibility, and 2 interface tests passed |
| Focused strict helper for `cairn-core` | Compiled; expected nonzero result from remaining catalog findings |

The focused strict total fell from 8,088 to 8,086 overlapping findings. Library
findings fell from 3,759 to 3,758. Library-test findings fell from 4,329 to
4,328. The policy module has no remaining `path_segment_repetition` finding.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
