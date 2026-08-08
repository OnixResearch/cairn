# Design

## Policy ownership

The new replay case id is `agent_receipt_preserves_hash`. It uses command kind `agent_receipt_preserves_hash`, fixture path `fixtures/agent-receipts/positive.json`, repeated comparison, and hashes the stable `agent_receipt_preserves_hash`, `input_hash`, and `receipt_hash` fields.

The surface inventory id matches the replay case and records the operator command as a CLI surface with receipt command `agent-receipt preserves-hash`.

## Replay execution

Replay does not shell out. It dispatches a new `ReplayCaseKind::AgentReceiptPreservesHash`, reads the same bounded fixture path, calls the same `agent_preserves_hash_receipt` builder used by the CLI, removes any existing `receipt_hash`, then re-adds a portable receipt hash for replay normalization.

## Group coverage

The case joins `full` because it is deterministic and policy-owned, but it does not join `agent` or `release`: those groups are required by existing agent receipts and release preflights, and expanding them would retroactively invalidate checked-in evidence that was produced before this optional inspection surface existed.

## Tests

- Replay default case list includes the new case and total count increments.
- `--case agent_receipt_preserves_hash` passes and emits the expected golden Preserves hash.
- Coverage-audit reports the new surface as covered.
- Policy export/check and Tracey prove policy/spec/test wiring.
