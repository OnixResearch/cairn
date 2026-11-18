# Task-scoped MCP discovery tasks

## Contract and implementation

- [ ] [serial] [task:baseline] Capture the full catalog, strict invalid-call behavior, and eager/lazy host exposure before implementation. r[cairn.mcp_discovery.conformance]
- [x] [serial] [task:policy] Define typed profile membership, required diagnostic routes, descriptor identities, and named listing bounds. r[cairn.mcp_discovery.profiles] r[cairn.mcp_discovery.pages]
- [x] [serial] [task:lookup] [after:policy] Add explicit profile selection and single-descriptor CLI/MCP lookup with exact existing schema parity. r[cairn.mcp_discovery.profiles] r[cairn.mcp_discovery.lookup]
- [x] [serial] [task:pages] [after:lookup] Implement deterministic identity-bound tools/list pages with explicit continuations and unchanged default listing behavior. r[cairn.mcp_discovery.pages]
- [x] [serial] [task:dispatch] [after:pages] Enforce profile restrictions after capability-first mutation denial and before argument-dependent product effects. r[cairn.mcp_discovery.authority] r[cairn.mcp_discovery.lookup]

## Acceptance and handoff

- [ ] [serial] [task:positive] [after:dispatch] Test every profile, exact descriptor equality, full page reconstruction, and admitted tool execution under existing preflight. r[cairn.mcp_discovery.profiles] r[cairn.mcp_discovery.lookup] r[cairn.mcp_discovery.pages]
- [ ] [serial] [task:negative] [after:positive] Test unknown profiles, unavailable tools, schema drift, invalid cursors, empty pages, malformed calls, and capability-first denial. r[cairn.mcp_discovery.lookup] r[cairn.mcp_discovery.authority]
- [ ] [serial] [task:benchmark] [after:negative] Record paired eager and lazy host token totals, discovery calls, restarts, missing-tool failures, and oracle outcomes. r[cairn.mcp_discovery.conformance]
- [x] [serial] [task:docs] Document profile completeness, strict lookup, cursor semantics, host compatibility, and mutation authority boundaries. r[cairn.mcp_discovery.profiles] r[cairn.mcp_discovery.authority]
- [ ] [serial] [task:verify] [after:docs] Run focused tests, policy freshness, formatting, workspace tests, Clippy, validation, traceability, and Nix checks. Retain current-source evidence. r[cairn.mcp_discovery.conformance]
- [ ] [serial] [task:handoff] [after:verify] Prepare reviewed specification sync and archive previews after implementation evidence passes. r[cairn.mcp_discovery.conformance]

Implementation and documentation are authored; checked items record authoring
only, not verification. Positive/negative tests are authored but unexecuted.
The integration owner supplies baseline/descriptor replay, current-source gate
evidence, and reviewed lifecycle previews. No lifecycle execution occurred in
this worktree. The benchmark task stays open: live eager/lazy host measurement
is unavailable, and representation replay cannot close that evidence boundary.
Save exercised implementation evidence under `evidence/implementation/`.
Planning receipts do not satisfy implementation tests or authorize archive.
Shared verification and measurement rules: `docs/agent-efficiency-roadmap.md`.
