# Implementation-ready projection

`cairn implementation-ready` reports which active changes or task units are eligible for implementation work. It composes the existing drain plan, workflow profiles, gates, task ordering, dependency evidence, and cross-repository evidence. It is read-only: it does not execute checks, mutate lifecycle files, or authorize archive.

```bash
nix run path:$PWD#cairn -- implementation-ready --root .
nix run path:$PWD#cairn -- implementation-ready --root . --limit 20
cairn mcp --call cairn_implementation_ready '{"root":".","limit":20}'
```

The CLI and MCP tool return the same `cairn.implementation-ready.v1` receipt. The machine envelope uses `cairn --machine implementation-ready ...`.

## Selection units

- With explicit task ordering, each selectable unit is `task:<id>`. Completed tasks remain visible in `completed_tasks` but are not selected.
- With a legacy unnumbered task list, the change is selected as one `change:<name>` unit.
- Fully completed changes report `completion_only` with no selection. Archive readiness remains a separate `archive_ready` observation.
- Invalid task definitions, unsatisfied dependencies, and cross-repository evidence failures appear as blockers instead of selectable units.

Each selected unit carries the requirement references found in its task range and the change-level `snapshot_digest`. The digest changes when the projected change facts change.

## Outcomes and bounds

`outcome` is one of:

| Outcome | Meaning |
|---|---|
| `ready` | At least one implementation unit is selectable |
| `completion_only` | Processed changes are complete and have no implementation unit |
| `blocked` | Processed changes have blockers and no selectable unit |
| `empty` | No active changes were observed |
| `incomplete_limit` | More active changes exist than the requested limit |

`--limit` defaults to 100 and accepts 1 through 1000. When the limit truncates enumeration, `complete` is `false` and `outcome` is `incomplete_limit`; the receipt never reports a truncated queue as empty or complete. `active_change_count` still reports the full observed count.

## Claim boundary

The projection reports selected structural work only. It does not prove artifact truth, task completion, implementation correctness, archive readiness, or release eligibility. Blockers are advisory observations, not approval or mutation authority.
