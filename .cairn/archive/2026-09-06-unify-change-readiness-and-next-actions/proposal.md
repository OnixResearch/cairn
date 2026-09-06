# Unify change readiness and next actions

## Why

Cairn already reports dependency order, artifact readiness, gate outcomes, completion evidence, and mutation blockers.
Operators must combine those reports and distinguish transport success from lifecycle readiness.
A failed gate can currently produce another gate command rather than the exact artifact repair that removes its cause.

## What Changes

- Add one pure change-readiness model over current, explicitly scoped observations.
- Compose existing workflow, gate, dependency, probe, completion, sync, and archive decisions without duplicating their rules.
- Return stable blocker causes, missing evidence, exact repair or diagnostic actions, and explicit mutation requirements.
- Add matching CLI and MCP report projections with a concise human view.
- Keep stale, missing, contradictory, and unavailable observations distinct from ready state.

The proposed command is `cairn readiness CHANGE`. It is a report, not a task runner or mutation grant.

## Impact

Maintenance owner: Cairn lifecycle-core and CLI maintainers.
Current consumers: Cairn operators and direct CLI or MCP agents.
Immediate outcome: one report identifies why a selected change cannot advance and which action can remove each blocker.
Durable contribution: a shared, tested readiness projection for existing and future consumer evidence adapters.

The implementation touches existing verified lifecycle models, shell observation adapters, machine output, MCP schemas, and report fixtures.
Stable receipts from existing commands remain unchanged. Their validators retain authority.
The shared stack is reused through existing Cairn and Valence evidence boundaries, not through a new scheduler dependency.

## Repeatability evidence

Run the same positive and negative repository fixtures through CLI and MCP.
Compare their semantic receipts after transport metadata is excluded.
Retain cases for stale evidence, malformed task order, blocked dependencies, invalid policy, unavailable observations, and denied mutation.
Measure diagnostic calls needed to reach a correct next action against the existing command sequence.

## Out of Scope

- Automatic task execution, file repair, sync, archive, or review approval
- Replacing Kiln CI state, Lattice orchestration, or Choregraph execution semantics
- Treating artifact presence, transport success, or task counts as completion
- New GUI infrastructure, storage backends, or implicit workspace discovery
- Claims of product value, source correctness, or release eligibility from the report alone
