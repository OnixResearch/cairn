## Design

### MCP surface

Add `cairn_drain_plan` to the read-only MCP catalog. The input schema accepts optional `root` and `policy`, matching existing repository-wide validation tools.

The structured receipt includes:

- `command`: `mcp cairn_drain_plan`
- `layout`, `root`, `policy`
- `active_change_count`
- `ready_change_count`
- `changes[]`, sorted by change name, with task counts, gate verdicts, sync/archive blockers, and recommended next MCP calls
- `next_actions[]`, sorted deterministic suggestions across the repo
- `mcp.read_only=true`, `mcp.destructive=false`
- portable receipt hash

### Semantics

A change is drain-ready only when:

- proposal, design, and tasks gates all produce valid passing receipts;
- all tasks are marked done;
- the existing sync plan is not blocked;
- the existing archive plan is not blocked after sync/archive policy blockers are evaluated.

If a change is not ready, the receipt must explain the blocker class (`proposal_gate`, `design_gate`, `tasks_gate`, `unfinished_tasks`, `sync_blocked`, `archive_blocked`) and include the next safe MCP calls an agent should run.

### Mutation boundary

`cairn_drain_plan` is read-only. It must not call `sync --execute`, `archive --execute`, Git, verification commands, or process execution. Clients that decide to mutate must call the existing `cairn_sync_execute` and `cairn_archive_execute` tools with `--allow-mutations`, preserving the explicit mutation capability boundary.
