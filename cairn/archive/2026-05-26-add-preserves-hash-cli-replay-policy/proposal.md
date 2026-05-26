# Add Preserves hash CLI replay policy

## Why

`cairn agent-receipt preserves-hash <path>` is now a deterministic operator inspection surface for the internal Preserves agent-run receipt hash, but it is not yet owned by `determinism_replay_policy` or `determinism_surface_inventory`. Without policy ownership, coverage-audit and replay cannot prove the new surface stays deterministic.

## What Changes

- Add a replay policy case for `agent-receipt preserves-hash fixtures/agent-receipts/positive.json`.
- Add the new case to the `full` replay group without expanding the existing `agent` or `release` required preflight groups.
- Add the command to `determinism_surface_inventory`.
- Teach replay execution to run the Preserves hash case through the same internal receipt builder as the CLI.
- Add focused replay and coverage-audit tests for the new policy-owned surface.

## Non-Goals

- No public receipt schema replacement.
- No change to existing `agent-receipt validate` replay behavior.
- No expansion of Preserves beyond the existing agent-run receipt hash inspection command.
