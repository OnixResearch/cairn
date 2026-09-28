# Incremental agent observation tasks

## Contract and implementation

- [ ] [serial] [task:baseline] Capture complete readiness, targeted-context, and active-list baselines across positive and negative fixtures. r[cairn.incremental_observations.conformance]
- [ ] [serial] [task:contracts] [after:baseline] Define typed cohort, snapshot, page, cursor, difference, completeness, and named-bound contracts. r[cairn.incremental_observations.identity] r[cairn.incremental_observations.pages] r[cairn.incremental_observations.delta]
- [ ] [serial] [task:identity] [after:contracts] Implement complete cohort identity and prior-observation admission with explicit unavailable, stale, and invalid states. r[cairn.incremental_observations.identity] r[cairn.incremental_observations.delta]
- [ ] [serial] [task:core] [after:identity] Implement pure whole-entry pages, unchanged results, and exact added, changed, and removed entry differences. r[cairn.incremental_observations.pages] r[cairn.incremental_observations.delta]
- [ ] [serial] [task:shell] [after:core] Add bounded explicit snapshot adapters and CLI/MCP requests without hidden session writes or new mutation authority. r[cairn.incremental_observations.storage]

## Acceptance and handoff

- [ ] [serial] [task:positive] [after:shell] Test page reconstruction, cursor replay, edits, removals, empty sets, restarted processes, and warm/cold semantic parity. r[cairn.incremental_observations.pages] r[cairn.incremental_observations.delta] r[cairn.incremental_observations.storage]
- [ ] [serial] [task:negative] [after:positive] Test mixed cohorts, stale cursors, partial baselines, forged counts, corrupt caches, unsafe paths, and fresh mutation preflight. r[cairn.incremental_observations.identity] r[cairn.incremental_observations.storage] r[cairn.incremental_observations.conformance]
- [ ] [serial] [task:benchmark] [after:negative] Record paired resumed-loop tokens and oracle outcomes with every continuation, snapshot read, cache state, and refresh. r[cairn.incremental_observations.conformance]
- [ ] [serial] [task:docs] [after:benchmark] Document snapshot ownership, continuation grammar, replay behavior, completeness, stale recovery, and non-atomic observation limits. r[cairn.incremental_observations.pages] r[cairn.incremental_observations.storage]
- [ ] [serial] [task:verify] [after:docs] Run focused tests, policy freshness, formatting, workspace tests, Clippy, validation, traceability, and Nix checks. Retain current-source evidence. r[cairn.incremental_observations.conformance]
- [ ] [serial] [task:handoff] [after:verify] Prepare reviewed specification sync and archive previews after implementation evidence passes. r[cairn.incremental_observations.conformance]

All tasks remain open. Save implementation evidence under `evidence/implementation/`.
Planning receipts do not satisfy implementation tasks or authorize archive execution.
The shared verification and measurement rules are in `docs/agent-efficiency-roadmap.md`.
