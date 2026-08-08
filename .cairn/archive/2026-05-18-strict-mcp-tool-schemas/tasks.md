## Tasks

- [x] r[cairn.supersede.mcp_server.strict_schemas.catalog] [parallel] Replace open MCP input schemas with deterministic per-tool schemas for read, plan, validation, and mutation tools.
- [x] r[cairn.supersede.mcp_server.strict_schemas.validation] [parallel] Add shell-side argument validation before lifecycle reads, plans, or mutations dispatch.
- [x] r[cairn.supersede.mcp_server.strict_schemas.errors] [parallel] Return deterministic invalid-argument errors and preserve default mutation denial with `mutated=false`.
- [x] r[cairn.supersede.mcp_server.strict_schemas.verification] [parallel] Add positive/negative MCP tests for schema catalog shape and malformed arguments.
- [x] r[cairn.supersede.mcp_server.strict_schemas.docs] [parallel] Document MCP tool argument schemas, selector expectations, and invalid-argument behavior.
- [x] r[cairn.supersede.mcp_server.strict_schemas.verification] [serial] Run focused MCP tests, Cairn validation/gates, Tracey coverage, and the maintained repository gate before sync/archive.
