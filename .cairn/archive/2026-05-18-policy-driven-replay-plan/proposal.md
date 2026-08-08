## Why

The workspace replay oracle now proves a narrow built-in smoke plan, but agents and CI need a maintained contract for the full deterministic public surface. Hard-coded cases in the CLI make it harder to review which command receipts are replayed, which hash fields are compared, and which path fields are display-only.

## What Changes

- Move replay case metadata into the generated Cairn policy so the replay plan is reviewable alongside receipt schemas.
- Expand the default replay plan beyond status/validate/MCP smoke to include gates, sync/archive dry-runs, migration dry-run, agent-receipt validation, and release-readiness where fixtures support them.
- Keep the CLI as the executor/comparator: it loads policy cases, runs only read-only/dry-run cases, and emits the same stable replay receipt.
- Add tests proving policy freshness, expanded case coverage, deterministic replay hashes, and mutation-denied behavior.

## Out of Scope

- Executing mutating replay cases.
- External service or network-dependent replay.
- Replacing release-readiness; replay remains an explicit operator/CI preflight unless run separately.

## Impact

- `cairn-policy/default.ncl`, `contracts.ncl`, and generated policy JSON gain replay plan schema/data.
- `crates/cairn-cli` loads replay cases from policy and executes the expanded plan.
- CLI tests and README docs cover policy-driven replay behavior.
