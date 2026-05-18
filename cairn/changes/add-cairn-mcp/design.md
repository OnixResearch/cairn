## Context

Cairn already owns deterministic CLI receipts for lifecycle reads, validation, gates, sync/archive planning, and migration planning. Agents would benefit from the same capability through MCP, but a naive MCP wrapper could weaken Cairn's guarantees by allowing arbitrary file reads, ambiguous mutation, unstructured prose responses, or filesystem effects inside the pure core.

## Decisions

### 1. MCP starts read-only by default

**Choice:** The server exposes read-only tools for `status`, `list_content`, `read_spec`, `change_show`, `doctor`, `validate`, `gate`, and `gate_context` without requiring a mutation capability. r[cairn.supersede.mcp_server.read_tools]

**Rationale:** These tools map directly to existing operator questions and are safe for autonomous agents. They should carry MCP annotations equivalent to `readOnlyHint=true` and `destructiveHint=false`.

### 2. Planning tools expose dry-run receipts, not mutations

**Choice:** `sync_plan`, `archive_plan`, and `migrate_plan` expose deterministic dry-run receipts only. r[cairn.supersede.mcp_server.plan_tools]

**Rationale:** Cairn's existing lifecycle commands are dry-run first. MCP should preserve that operator contract and make planned actions visible before any state change is possible.

### 3. Mutation tools require an explicit server flag

**Choice:** `init`, `change_create`, `sync_execute`, `archive_execute`, and `migrate_execute` are unavailable unless the MCP server is started with an explicit mutation capability such as `--allow-mutations`. r[cairn.supersede.mcp_server.mutation_guard]

**Rationale:** MCP clients can be autonomous and long-lived. Mutating lifecycle tools should be impossible to call accidentally from a default server configuration.

### 4. Bounded repository access only

**Choice:** MCP tools accept a selected root and lifecycle identifiers, but do not expose arbitrary path reads. Source/spec reads are bounded to native Cairn lifecycle artifacts and explicit migration inputs. r[cairn.supersede.mcp_server.boundary]

**Rationale:** Agents need lifecycle evidence, not ambient repository secrets. Path traversal, dotfile reads, build artifacts, and unrelated project files are outside the MCP contract.

### 5. Structured receipts are the public result format

**Choice:** MCP tool responses return structured content containing Cairn's existing JSON receipt fields and concise text summaries only as secondary presentation. r[cairn.supersede.mcp_server.verification]

**Rationale:** Agents and automation need stable machine-readable evidence. Prose-only MCP responses would lose Cairn's BLAKE3 provenance and schema-stability benefits.

## Risks / Trade-offs

- Adding MCP as a thin shell around the CLI is faster, but it may duplicate parsing and error handling; prefer shared shell helpers or pure core models where practical.
- Adding mutation tools improves agent ergonomics, but the default-deny flag is necessary to avoid accidental archives or migrations.
- Bounded reads may require extra helper tools for common workflows, but the safer default is to add specific lifecycle tools instead of exposing arbitrary files.
