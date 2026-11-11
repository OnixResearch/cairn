# Add implementation-ready work projection

## Why

Cairn exposes archive-oriented drain readiness, but an external implementer needs a bounded, versioned snapshot of implementation-ready change and task units. Unfinished tasks are eligible work when their prerequisites pass; completion-only lifecycle readiness is a later condition.

Issue: OnixResearch/cairn#1. Consumers: OnixResearch/kiln#44 and OnixResearch/lattice#6.

## What Changes

- Add `cairn implementation-ready` and the `cairn_implementation_ready` MCP tool with `cairn.implementation-ready.v1`.
- Reuse the existing drain plan, gates, task ordering, dependency graph, cross-repository evidence, and workflow profile facts; do not duplicate their evaluators.
- Return stable `task:<id>` units for explicit task markers and one `change:<name>` unit for legacy task lists, with okay/completed/blocked tasks and requirement references.
- Distinguish `ready`, `blocked`, `completion_only`, `empty`, and `incomplete_limit`; archive readiness remains separate.
- Bound enumeration with an explicit `limit` and return `complete: false` plus `incomplete_limit` when truncating.
- Keep CLI and MCP projections identical and read-only.

## Impact

Current consumer: an external implementer selecting one eligible unit.
Immediate outcome: deterministic selection with requirement references and blockers.
Durable contribution: a versioned read-only implementation-readiness boundary.
Maintenance owner: Cairn CLI/MCP maintainers.

No scheduler, claim, worker, GitHub polling, merge authority, or replacement completion checker is added.

## Repeatability evidence

Fixtures cover unfinished tasks, explicit task markers with completed predecessors, completion-only changes, and enumeration limits. CLI and MCP results are compared on the same fixture. Existing drain-plan and task-ordering tests remain unchanged.
