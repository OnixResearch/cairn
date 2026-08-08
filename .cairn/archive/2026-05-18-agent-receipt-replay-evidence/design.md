# Design

## Receipt shape

Agent-run receipts gain a `replay_evidence` object with stable fields:

- `replay_receipt_hash`: BLAKE3 hash from a passing `cairn determinism replay` receipt.
- `replay_policy_hash`: BLAKE3 hash of the generated Cairn policy used for replay.
- `replay_selected_cases`: sorted replay case IDs covered by the replay evidence.
- `replay_verdict`: expected to be `pass`.
- `replay_artifact_path`: optional display-only local path for operator convenience.

The agent receipt's own `receipt_hash` continues to hash the full receipt with only `receipt_hash` excluded.

## Validation

`agent-receipt validate` remains bounded and deterministic. It loads the current generated policy, computes its canonical hash, and adds replay-evidence issues when:

- `replay_evidence` is missing or not an object,
- replay receipt hash is not BLAKE3-shaped,
- replay verdict is not `pass`,
- replay policy hash differs from current generated policy hash,
- required replay cases are absent from `replay_selected_cases`.

The initial required case set is small and policy-visible: `status`, `validate`, `agent_receipt_validate`, and `mcp_describe`. These prove the agent receipt used replay-checked core status/validation/MCP/agent-receipt surfaces without requiring the full replay suite for every agent action.

## Receipts and policy

The validation receipt exposes stable fields for `current_replay_policy_hash`, `required_replay_cases`, and `replay_evidence`. Public receipt schema docs and policy schema data are updated together.

## Tests

Tests cover:

- positive fixture with replay evidence validates,
- missing replay evidence fails deterministically,
- stale policy hash fails deterministically,
- insufficient replay coverage fails deterministically,
- CLI/MCP validation receipts remain stable and replay-covered.
