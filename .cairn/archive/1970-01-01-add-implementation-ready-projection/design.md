## Decisions

The projection composes `mcp_drain_plan` rather than reimplementing gate, dependency, cross-repository, or workflow evaluation. It filters completion-only blockers (`unfinished_tasks`, sync/archive plan blockers) out of implementation eligibility and selects only ready task units or a legacy whole-change unit.

Task units use the existing explicit task IDs. Legacy task lists without markers produce one `change:<name>` unit, never list-position identities. Completed tasks are reported but never selected.

`limit` bounds the number of processed changes. Truncation is explicit (`complete: false`, `outcome: incomplete_limit`), never an empty-success queue.

The per-change snapshot digest is canonical JSON over the projection without the digest field; the outer receipt is portable over the lifecycle root. A relevant intent or policy edit changes the digest.

## Non-claims

The projection is observed intent and eligibility only. It does not claim work, execute a worker, prove acceptance, or modify lifecycle state. Completion-only and archive readiness remain distinct from implementation work.
