## Tasks

- [x] r[cairn.supersede.mcp_agent_smoke_receipts.flow] [parallel] Add a maintained local smoke flow that exercises MCP catalog description and `cairn_agent_receipt_validate` against an existing positive agent receipt fixture.
- [x] r[cairn.supersede.mcp_agent_smoke_receipts.evidence] [parallel] Assert stable evidence fields for the smoke flow, including tool schema presence, `structuredContent`, validation verdict, and hash fields where applicable.
- [x] r[cairn.supersede.mcp_agent_smoke_receipts.read_only] [parallel] Add a negative/default-safe branch proving malformed MCP arguments are rejected deterministically without mutation or lifecycle dispatch.
- [x] r[cairn.supersede.mcp_agent_smoke_receipts.docs] [parallel] Document the operator smoke command or fixture in MCP/receipt docs.
- [x] r[cairn.supersede.mcp_agent_smoke_receipts.verification] [serial] Run focused smoke tests, Cairn validation/gates, Tracey coverage, and maintained repository checks before sync/archive.
