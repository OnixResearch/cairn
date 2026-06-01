# Add change dependency graph

## Why

Cairn already has artifact dependencies inside one change package and task-level `[depends:...]` markers, but it has no first-class dependency model between active change packages. That leaves repository drains to infer order from prose and makes agents likely to pick a dependent change before its prerequisite.

A small optional change-package graph would let Cairn validate dependency declarations, report cycles deterministically, and produce a safe drain order without turning Cairn into a project manager. r[cairn.change_dependency_graph.metadata] r[cairn.change_dependency_graph.graph_validation] r[cairn.change_dependency_graph.drain_order]

## Scope

- Add optional `depends_on` declarations to change metadata.
- Validate unknown dependencies, self-dependencies, duplicate edges, and cycles in pure core logic.
- Expose deterministic graph receipts for CLI/MCP planning, including `blocked_by`, `blocks`, and topological drain order.
- Block sync/archive execution for a dependent change until declared dependencies are satisfied.
- Use `../trellis/` graph/scheduling primitives as the reference model where topological ordering, cycle detection, and dependency readiness logic are introduced. r[cairn.change_dependency_graph.trellis_primitives]

## Non-goals

- No automatic mutation, Git operation, branch orchestration, remote issue tracking, or background scheduler.
- No mandatory dependencies for repositories that do not need ordering.
- No runtime dependency on a sibling checkout; Trellis is a source/reference for verified primitives unless a reviewed portable dependency path is added.
- No replacement for task-level `[depends:...]` markers inside `tasks.md`.
