# MCP receipt projection tasks

## Contract and implementation

- [ ] [serial] [task:baseline] Capture legacy catalog, response, and host-exposure baselines with passing and failing fixtures before edits. r[cairn.mcp_projection.conformance]
- [ ] [serial] [task:policy] [after:baseline] Define admitted response profiles, status projections, mandatory fields, and named summary bounds in typed Nickel. r[cairn.mcp_projection.profile] r[cairn.mcp_projection.status]
- [ ] [serial] [task:summary] [after:policy] Implement bounded pure summaries for known result families and explicit unclassified fallback behavior. r[cairn.mcp_projection.status] r[cairn.mcp_projection.single_payload]
- [ ] [serial] [task:transport] [after:summary] Add explicit startup and direct-call response profiles without changing receipt builders or structured payload hashes. r[cairn.mcp_projection.profile] r[cairn.mcp_projection.single_payload]

## Acceptance and handoff

- [ ] [serial] [task:positive] [after:transport] Test exact structured payload parity, full legacy text, and one complete payload in the structured-summary profile. r[cairn.mcp_projection.single_payload] r[cairn.mcp_projection.compatibility]
- [ ] [serial] [task:negative] [after:positive] Test unknown profiles, long diagnostics, unknown receipt families, invalid calls, blocked lifecycles, and capability-first mutation denial. r[cairn.mcp_projection.profile] r[cairn.mcp_projection.status] r[cairn.mcp_projection.compatibility]
- [ ] [serial] [task:hosts] [after:negative] Validate declared structured, text-only, and deduplicating host presentations with complete evidence retrieval. r[cairn.mcp_projection.compatibility] r[cairn.mcp_projection.conformance]
- [ ] [serial] [task:benchmark] [after:hosts] Record paired token and oracle results through the prerequisite benchmark, including host-visible tool output and failed attempts. r[cairn.mcp_projection.conformance]
- [ ] [serial] [task:docs] [after:benchmark] Document profile selection, text-only fallback, host support, status distinctions, and bounded savings claims. r[cairn.mcp_projection.profile] r[cairn.mcp_projection.compatibility]
- [ ] [serial] [task:verify] [after:docs] Run focused tests, policy freshness, formatting, workspace tests, Clippy, validation, traceability, and Nix checks. Retain current-source evidence. r[cairn.mcp_projection.conformance]
- [ ] [serial] [task:handoff] [after:verify] Prepare reviewed specification sync and archive previews after implementation evidence passes. r[cairn.mcp_projection.conformance]

All tasks remain open. Save implementation evidence under `evidence/implementation/`.
Planning receipts do not satisfy implementation tasks or authorize archive execution.
The shared verification and measurement rules are in `docs/agent-efficiency-roadmap.md`.
