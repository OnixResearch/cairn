## Why

Agents currently understand “drain all Cairns in a repo” as an operator workflow, but MCP exposes only per-change status, gates, sync, and archive tools. That leaves clients to rediscover ordering, readiness blockers, and safe next calls for every active change.

Cairn should expose a first-class MCP drain planning tool that summarizes every active change, reports which gates are blocking, and emits a deterministic next-action plan without mutating the repository.

## What Changes

- Add an MCP read-only tool for repository-wide drain planning.
- The tool lists all active changes in deterministic order with task counts, proposal/design/tasks gate outcomes, and sync/archive readiness blockers.
- The tool returns safe next MCP/CLI calls rather than claiming it can implement arbitrary product work by itself.
- Preserve the existing explicit mutation boundary: sync/archive execution remains behind existing mutation-capable MCP tools.

## Non-Goals

- Automatically implement arbitrary change tasks.
- Automatically run external verification commands, commit, or push.
- Add network, credential, or Git mutation behavior to MCP drain planning.
- Replace the existing per-change sync/archive execute tools.

## Impact

- **Files**: MCP schema/dispatch/read-tool code, MCP CLI tests, and this native Cairn package.
- **Testing**: focused MCP integration tests, Cairn validation/gates, and formatting/lint checks as feasible.
