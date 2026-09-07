# Agent-guidance compatibility repair

## Scope

This batch removes the local `agent_guidance` path-name repetitions without
changing Cairn's documented root API, Serde wire spelling, or receipt bytes.

The implementation types and functions now use names that describe their local
role. `src/api/guidance.rs` preserves each existing root name with an explicit
public compatibility reexport. Each alias has its own narrow Octet allowance and
reason. The allowance applies only when the Octet Dylint library is active.
`cairn-core/Cargo.toml` declares that existing `dylint_lib = "octet"` cfg so the
compiler can check the conditional attribute without accepting arbitrary cfgs.

The parser and policy model use the local `Policy` and `validate_policy` names
directly. No consumer API, field order, enum spelling, schema text, hash
framing, or validation policy changed.

## Regression controls

`crates/cairn-core/tests/compatibility.rs` uses every preserved root type and
function. It includes these controls:

- A positive receipt test binds the local and portable result bytes to BLAKE3
  `43fdaba898cb81cb15b94158581b48b04e071e4263b73fde9e94f599846bd2bd`.
- A negative declaration test binds invalid-input issue bytes to BLAKE3
  `bb0ec131cab10d1fc09c4fd65d2a4ec42da60dc1155bb599b18a9646a98310d6`.
- A negative observation test rejects stale and missing required sources.
- A positive and negative Serde test preserves status bytes and rejects raw,
  malformed, short, and unknown portable values.

## Verification

The local Nix Rust-source output has a known hash mismatch. Validation therefore
ran in an isolated remote clone on `aspen1`, with the exact local pinned Octet,
Mantle, and Cargo vendor sources transferred by their immutable content
identities. The remote clone has an untracked input override only for that
validation environment. No repository pin or lockfile changed.

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | Passed |
| `cargo test -p cairn-core --all-targets --all-features` | 644 unit, 4 compatibility, and 2 interface tests passed |
| `cargo test -p cairn-cli --test agent_guidance --all-features` | 8 passed |
| `cargo clippy -p cairn-core --all-targets --all-features -- -D warnings` | Passed |
| Focused strict helper for `cairn-core` | Compiled; expected nonzero result from remaining catalog findings |

The strict baseline had 3,786 library and 4,356 library-test findings, for
8,142 overlapping findings. This batch has 3,759 library and 4,329
library-test findings, for 8,088 overlapping findings. It removes all 54
`path_segment_repetition` findings in
`crates/cairn-core/src/verified/agent_guidance.rs`. Two repetitions remain in
the separate `verified/policy/agent_guidance.rs` module and remain outside this
batch.

The final strict log contains no finding for `tests/compatibility.rs`.

## Boundary

The strict catalog remains non-green. This record does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.
