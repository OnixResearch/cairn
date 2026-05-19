# Design

## Policy shape

`determinism_replay_policy` gains `groups`, a list of objects with:

- `id`: stable group identifier.
- `description`: operator-facing purpose.
- `cases`: stable list of case IDs owned by the policy.

Initial groups:

- `smoke`: cheap status/validate preflight.
- `agent`: cases needed by current agent receipt replay-evidence expectations.
- `release`: release-readiness plus agent/smoke evidence.
- `full`: all policy cases.

## CLI behavior

`cairn determinism replay` accepts optional `--group <id>` in addition to existing `--case <id>`.

- No selector: run all policy cases, preserving current behavior.
- `--case`: run exactly the selected case.
- `--group`: run the policy group's case list, sorted by existing case order.
- `--case` and `--group` together fail before replay with a deterministic selection receipt.
- Unknown groups fail like unknown cases: machine-readable receipt, nonzero exit, stable mismatch class, and reproduction command.

## Receipt shape

Replay receipts add stable top-level fields:

- `available_groups`
- `selected_groups`

Existing `available_cases`, `selected_cases`, `summary`, and mismatch fields remain stable.

## Verification

Add CLI integration tests for group selection, unknown group failures, selector conflicts, schema policy, docs, and replay stability. Regenerate policy JSON and policy-hash-sensitive golden receipts.
