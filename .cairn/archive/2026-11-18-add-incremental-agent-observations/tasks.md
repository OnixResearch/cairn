# Incremental agent observation tasks

## Contract and implementation

- [x] [serial] [task:baseline] Capture complete readiness, targeted-context, and active-list baselines across positive and negative fixtures. r[cairn.incremental_observations.conformance]
- [x] [serial] [task:contracts] [after:baseline] Define typed cohort, snapshot, page, cursor, difference, completeness, and named-bound contracts. r[cairn.incremental_observations.identity] r[cairn.incremental_observations.pages] r[cairn.incremental_observations.delta]
- [x] [serial] [task:identity] [after:contracts] Implement complete cohort identity and prior-observation admission with explicit unavailable, stale, and invalid states. r[cairn.incremental_observations.identity] r[cairn.incremental_observations.delta]
- [x] [serial] [task:core] [after:identity] Implement pure whole-entry pages, unchanged results, and exact added, changed, and removed entry differences. r[cairn.incremental_observations.pages] r[cairn.incremental_observations.delta]
- [x] [serial] [task:shell] [after:core] Add bounded explicit snapshot adapters and CLI/MCP requests without hidden session writes or new mutation authority. r[cairn.incremental_observations.storage]

## Acceptance and handoff

- [x] [serial] [task:positive] [after:shell] Test page reconstruction, cursor replay, edits, removals, empty sets, restarted processes, and warm/cold semantic parity. r[cairn.incremental_observations.pages] r[cairn.incremental_observations.delta] r[cairn.incremental_observations.storage]
- [x] [serial] [task:negative] [after:positive] Test mixed cohorts, stale cursors, partial baselines, forged counts, corrupt caches, unsafe paths, and fresh mutation preflight. r[cairn.incremental_observations.identity] r[cairn.incremental_observations.storage] r[cairn.incremental_observations.conformance]
- [x] [serial] [task:benchmark] [after:negative] Record the representation sizes of a complete observation and its first page, keeping token cost and any resumed-loop saving explicitly unavailable. r[cairn.incremental_observations.conformance]
- [x] [serial] [task:docs] [after:benchmark] Document snapshot ownership, continuation grammar, replay behavior, completeness, stale recovery, and non-atomic observation limits. r[cairn.incremental_observations.pages] r[cairn.incremental_observations.storage]
- [x] [serial] [task:verify] [after:docs] Run focused tests, policy freshness, formatting, workspace tests, Clippy, validation, traceability, and Nix checks. Retain current-source evidence. r[cairn.incremental_observations.conformance]
- [x] [serial] [task:handoff] [after:verify] Prepare reviewed specification sync and archive previews after implementation evidence passes. r[cairn.incremental_observations.conformance]

Executed evidence: 14 core `incremental_observations` tests and 10 CLI
`incremental_observations` tests pass. `evidence/implementation/representation-measurement.json`
records a complete readiness observation at 5264 bytes, its first page at 2892
bytes, and a complete active-change observation at 6058 bytes, with
`tokens.state = "unavailable"`.

Defects found and fixed during acceptance, each recorded in
`evidence/implementation/acceptance-state.md`:

- a committed `flake.nix` still held conflict markers, so the flake failed to
  parse;
- `ensure_complete` lacked the lifetime binding its return type required;
- a core test shadowed the `unchanged` helper with its own binding;
- `cairn observe` stored every option as a string, so all byte and entry bounds
  failed with "must be a non-negative integer" — numeric targets now parse;
- three pagination loops re-sent `--page-entries` with `--cursor`, which the
  implementation correctly rejects;
- the delta test assumed one changed entry after an edit, but inserting bytes
  shifts the following block's absolute span, so both selected entries legitimately
  change. The assertion now distinguishes the edited text from the span-only move.

Scoped limitations: no live agent campaign was run, so token cost, model-visible
exposure and any resumed-loop saving remain unavailable. Observation identity
binds admitted inputs; it does not prove source truth, secrecy, authenticity or
an atomic filesystem snapshot.
