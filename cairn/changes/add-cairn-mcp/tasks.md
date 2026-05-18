## Phase 1: MCP server surface

- [ ] [serial] Define the MCP server entry point, package/app wiring, and default read-only startup mode. r[cairn.supersede.mcp_server.read_tools]
- [ ] [serial] Implement bounded read-only MCP tools for status, content listing, spec reads, change reads, doctor, validate, gate, and gate context. r[cairn.supersede.mcp_server.read_tools]
- [ ] [serial] Implement dry-run planning MCP tools for sync, archive, and legacy migration receipts. r[cairn.supersede.mcp_server.plan_tools]
- [ ] [serial] Add mutation-capability gating and default-deny errors for init, change creation, sync execution, archive execution, and migration execution. r[cairn.supersede.mcp_server.mutation_guard]

## Phase 2: Boundaries and safety

- [ ] [serial] Enforce root/path bounding, traversal rejection, bounded content sizes, and lifecycle-only read surfaces. r[cairn.supersede.mcp_server.boundary]
- [ ] [serial] Keep MCP transport, filesystem reads, process exits, and JSON-RPC effects out of `cairn-core`; add or reuse pure core models only for deterministic lifecycle classification/planning. r[cairn.supersede.mcp_server.boundary]
- [ ] [serial] Preserve structured Cairn receipt fields, BLAKE3 hashes, and MCP read/destructive annotations in tool responses. r[cairn.supersede.mcp_server.verification]

## Phase 3: Verification and docs

- [ ] [serial] Add focused MCP registration/tool-call tests, including annotation/schema assertions and structured receipt assertions. r[cairn.supersede.mcp_server.verification]
- [ ] [serial] Add negative tests for path traversal, unavailable default mutation tools, and blocked mutation attempts without `--allow-mutations`. r[cairn.supersede.mcp_server.mutation_guard]
- [ ] [serial] Document the MCP tools, safety flags, response schemas, and example agent workflows. r[cairn.supersede.mcp_server.verification]
- [ ] [serial] Run formatting, workspace tests, Clippy, Slotcar, `cairn validate --root .`, and `nix flake check`. r[cairn.supersede.mcp_server.verification]
