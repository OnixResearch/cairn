## Why

Cairn records task concurrency markers but does not build a task dependency graph. Agents can start dependent work before prerequisites finish.

## What Changes

- Add optional `[task:<id>]` identities and `[after:<task-id>]` predecessor markers.
- Validate task ordering with pure deterministic core logic.
- Reject missing tasks, duplicate identities or edges, self-dependencies, cycles, and invalid progress.
- Include stable task order and ready-task data in task-gate and drain-plan receipts.
- Keep existing `[depends:...]` linkage behavior unchanged.

## Impact

- **Files**: task validation core, Nickel marker policy, gate and MCP receipts, tests, specifications, and operator documentation
- **Testing**: positive and negative core graph tests, task-gate tests, drain-plan tests, policy checks, and repository gates
