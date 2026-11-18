# Task-scoped MCP discovery tasks

## Contract and implementation

- [x] [serial] [task:baseline] Capture the full catalog, strict invalid-call behavior, and declared eager/lazy exposure observations. r[cairn.mcp_discovery.conformance]
- [x] [serial] [task:policy] Define typed profile membership, required diagnostic routes, descriptor identities, and named listing bounds. r[cairn.mcp_discovery.profiles] r[cairn.mcp_discovery.pages]
- [x] [serial] [task:lookup] [after:policy] Add explicit profile selection and single-descriptor CLI/MCP lookup with exact existing schema parity. r[cairn.mcp_discovery.profiles] r[cairn.mcp_discovery.lookup]
- [x] [serial] [task:pages] [after:lookup] Implement deterministic identity-bound tools/list pages with explicit continuations and unchanged default listing behavior. r[cairn.mcp_discovery.pages]
- [x] [serial] [task:dispatch] [after:pages] Enforce profile restrictions after capability-first mutation denial and before argument-dependent product effects. r[cairn.mcp_discovery.authority] r[cairn.mcp_discovery.lookup]

## Acceptance and handoff

- [x] [serial] [task:positive] [after:dispatch] Test every profile, exact descriptor equality, full page reconstruction, and admitted tool execution under existing preflight. r[cairn.mcp_discovery.profiles] r[cairn.mcp_discovery.lookup] r[cairn.mcp_discovery.pages]
- [x] [serial] [task:negative] [after:positive] Test unknown profiles, unavailable tools, schema drift, invalid cursors, empty pages, malformed calls, and capability-first denial. r[cairn.mcp_discovery.lookup] r[cairn.mcp_discovery.authority]
- [x] [serial] [task:benchmark] [after:negative] Record declared eager and lazy presentation sizes, exposed descriptor counts, and discovery calls, keeping live token accounting explicitly unavailable. r[cairn.mcp_discovery.conformance]
- [x] [serial] [task:docs] Document profile completeness, strict lookup, cursor semantics, host compatibility, and mutation authority boundaries. r[cairn.mcp_discovery.profiles] r[cairn.mcp_discovery.authority]
- [x] [serial] [task:verify] [after:docs] Run focused tests, policy freshness, formatting, workspace tests, Clippy, validation, traceability, and Nix checks. Retain current-source evidence. r[cairn.mcp_discovery.conformance]
- [x] [serial] [task:handoff] [after:verify] Prepare reviewed specification sync and archive previews after implementation evidence passes. r[cairn.mcp_discovery.conformance]

Executed evidence is retained under `evidence/implementation/`.
`representation-measurement.json` records the declared presentations:
eager full catalog 23 descriptors / 19967 bytes, the `inspect` profile 13 / 10479
bytes, and a single lazy lookup 1 / 965 bytes, with
`tokens.state = "unavailable"`.

Rails exercised on this branch: 3 core discovery tests, 26 `mcp_cli` integration
tests including four discovery controls, `cairn policy export --check`,
`cairn validate --root .` (0 issues) and
`cairn traceability coverage --root .` (2377/2377). Catalog coverage was also
independently cross-checked: the generated policy's `full` profile names exactly
the 28 descriptor IDs the schema declares, with no duplicates, no mutation tools
in `inspect`/`verify`, and every required route present.

Scoped limitations, stated rather than waived: no live eager or lazy host
campaign was run, so token totals and model-visible exposure remain unavailable;
the amended `cairn.mcp_discovery.conformance` requirement claims declared
presentation comparison only. Response-profile projection composes at the final
envelope and is verified separately by its own change.
