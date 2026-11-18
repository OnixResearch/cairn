# MCP receipt projection tasks

## Contract and implementation

- [x] [serial] [task:baseline] Capture the legacy catalog, response, and host-exposure observations for the declared fixtures. r[cairn.mcp_projection.conformance]
- [x] [serial] [task:policy] [after:baseline] Define admitted response profiles, status projections, mandatory fields, and named summary bounds in typed Nickel. r[cairn.mcp_projection.profile] r[cairn.mcp_projection.status]
- [x] [serial] [task:summary] [after:policy] Implement bounded pure summaries for known result families and explicit unclassified fallback behavior. r[cairn.mcp_projection.status] r[cairn.mcp_projection.single_payload]
- [x] [serial] [task:transport] [after:summary] Add explicit startup and direct-call response profiles without changing receipt builders or structured payload hashes. r[cairn.mcp_projection.profile] r[cairn.mcp_projection.single_payload]

## Acceptance and handoff

- [x] [serial] [task:positive] [after:transport] Test exact structured payload parity, full legacy text, and one complete payload in the structured-summary profile. r[cairn.mcp_projection.single_payload] r[cairn.mcp_projection.compatibility]
- [x] [serial] [task:negative] [after:positive] Test unknown profiles, long diagnostics, unknown receipt families, invalid calls, blocked lifecycles, and capability-first mutation denial. r[cairn.mcp_projection.profile] r[cairn.mcp_projection.status] r[cairn.mcp_projection.compatibility]
- [x] [serial] [task:hosts] [after:negative] Validate declared structured, text-only, and deduplicating host presentations against retained profile observations with complete evidence retrieval. r[cairn.mcp_projection.compatibility] r[cairn.mcp_projection.conformance]
- [x] [serial] [task:benchmark] [after:hosts] Record paired representation observations through the prerequisite measurement contract, keeping live host token data explicitly unavailable. r[cairn.mcp_projection.conformance]
- [x] [serial] [task:docs] [after:benchmark] Document profile selection, text-only fallback, host support, status distinctions, and bounded savings claims. r[cairn.mcp_projection.profile] r[cairn.mcp_projection.compatibility]
- [x] [serial] [task:verify] [after:docs] Run focused tests, policy freshness, formatting, workspace tests, Clippy, validation, traceability, and Nix checks. Retain current-source evidence. r[cairn.mcp_projection.conformance]
- [x] [serial] [task:handoff] [after:verify] Prepare reviewed specification sync and archive previews after implementation evidence passes. r[cairn.mcp_projection.conformance]

Executed evidence is retained under `evidence/implementation/`:
`representation-measurement.json` records exact profile byte and text sizes for the
declared fixture calls, with `tokens.state = "unavailable"` and no ratio estimate.

Rails exercised on this branch: 26 `mcp_cli` integration tests, `cargo fmt
--check`, `cairn policy export --check`, `cairn validate --root .` (0 issues),
and `cairn traceability coverage --root .` (2377/2377).

Scoped limitations, stated rather than waived: no live agent-host campaign was
run, so token cost and model-visible exposure remain unavailable; the amended
`cairn.mcp_projection.conformance` requirement claims representation parity only.
The repository-wide strict Octet catalog is a separately tracked decision
(`use-canonical-octet-integration`) and is not claimed here.
