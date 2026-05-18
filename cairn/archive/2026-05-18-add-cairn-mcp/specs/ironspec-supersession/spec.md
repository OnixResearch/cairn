# Cairn Supersession MCP Delta

## Purpose

Specify a native MCP surface for Cairn lifecycle inspection, dry-run planning, and explicitly guarded lifecycle mutation while preserving deterministic receipts and FCIS boundaries.

## Requirements

### Requirement: Cairn MCP read-only tools
r[cairn.supersede.mcp_server.read_tools] The system MUST expose bounded read-only MCP tools for Cairn lifecycle inspection without enabling filesystem mutation.

#### Scenario: List lifecycle state through MCP
r[cairn.supersede.mcp_server.read_tools.status]
- GIVEN a repository with native `cairn/` lifecycle content
- WHEN an MCP client calls the Cairn status or content-listing tool
- THEN the tool returns structured content with accepted specs, active changes, archived changes, selected layout, and validation summary fields
- AND the tool is annotated as read-only and non-destructive

#### Scenario: Read selected lifecycle artifacts through MCP
r[cairn.supersede.mcp_server.read_tools.artifacts]
- GIVEN a repository with an accepted spec or active change artifact
- WHEN an MCP client requests that spec or change by lifecycle identifier
- THEN the tool returns bounded structured content for the selected lifecycle artifact
- AND it does not expose arbitrary repository paths outside the lifecycle contract

#### Scenario: Run diagnostics and gates through MCP
r[cairn.supersede.mcp_server.read_tools.gates]
- GIVEN a repository with a selected Cairn lifecycle root
- WHEN an MCP client calls doctor, validate, gate, or gate-context tools
- THEN each tool returns the corresponding deterministic Cairn receipt fields as structured content
- AND no repository files are mutated

### Requirement: Cairn MCP dry-run planning tools
r[cairn.supersede.mcp_server.plan_tools] The system MUST expose MCP tools for sync, archive, and migration planning that return dry-run receipts by default.

#### Scenario: Sync and archive plans remain non-mutating
r[cairn.supersede.mcp_server.plan_tools.lifecycle]
- GIVEN an active Cairn change
- WHEN an MCP client calls sync-plan or archive-plan
- THEN the tool returns a deterministic dry-run lifecycle receipt with planned actions, blockers, mutation state, and BLAKE3 evidence hashes
- AND no filesystem content is modified

#### Scenario: Migration plans remain non-mutating
r[cairn.supersede.mcp_server.plan_tools.migration]
- GIVEN a repository with legacy `openspec/` or `ironspec/` lifecycle content
- WHEN an MCP client calls the migration planning tool
- THEN the tool returns a deterministic dry-run migration receipt with source and destination manifest hashes
- AND no native lifecycle destination file is created or overwritten

### Requirement: Cairn MCP mutation guard
r[cairn.supersede.mcp_server.mutation_guard] The system MUST disable mutating MCP tools by default and require an explicit server capability before lifecycle mutation can be requested.

#### Scenario: Default server denies mutation tools
r[cairn.supersede.mcp_server.mutation_guard.default_deny]
- GIVEN the Cairn MCP server is started without the mutation capability flag
- WHEN an MCP client lists or calls init, change-create, sync-execute, archive-execute, or migrate-execute tools
- THEN the server either omits those tools or returns a structured capability error
- AND no lifecycle files are mutated

#### Scenario: Explicit capability allows guarded mutation
r[cairn.supersede.mcp_server.mutation_guard.enabled]
- GIVEN the Cairn MCP server is started with an explicit mutation capability flag
- WHEN an MCP client calls a mutating lifecycle tool with valid inputs
- THEN the tool applies the same validation, gate, blocker, and receipt semantics as the equivalent Cairn CLI command
- AND the response reports whether mutation occurred

### Requirement: Cairn MCP FCIS boundary
r[cairn.supersede.mcp_server.boundary] The system MUST keep MCP transport, filesystem access, path bounding, and JSON-RPC effects in the imperative shell while preserving pure deterministic logic in `cairn-core`.

#### Scenario: Core remains transport independent
r[cairn.supersede.mcp_server.boundary.core]
- GIVEN MCP server support is implemented
- WHEN `cairn-core` builds lifecycle models, diagnostics, gates, or plans
- THEN it consumes caller-supplied snapshots and policy data only
- AND it performs no MCP transport, filesystem, environment, process, time, stdout/stderr, or JSON-RPC effects

#### Scenario: Shell bounds lifecycle access
r[cairn.supersede.mcp_server.boundary.shell]
- GIVEN an MCP client supplies a root, lifecycle identifier, or relative artifact selector
- WHEN the server resolves the request
- THEN the shell rejects traversal, absolute artifact escapes, dotfile reads, build output reads, and unrelated repository file reads
- AND accepted reads are size-bounded before content is returned

### Requirement: Cairn MCP verification and documentation
r[cairn.supersede.mcp_server.verification] The system MUST verify and document the MCP tool contract, including schemas, annotations, safety flags, and structured receipt behavior.

#### Scenario: Tool registration is tested
r[cairn.supersede.mcp_server.verification.registration]
- GIVEN the maintained MCP test suite starts the Cairn MCP server
- WHEN it initializes the MCP session and lists tools
- THEN the expected read-only and planning tools are registered with stable names, input schemas, and read/destructive annotations

#### Scenario: Safety failures are tested
r[cairn.supersede.mcp_server.verification.negative]
- GIVEN the maintained MCP test suite exercises unsafe inputs
- WHEN it sends traversal selectors, oversized read requests, or mutation calls without the mutation capability
- THEN each call returns a structured error
- AND the fixture repository remains unchanged

#### Scenario: Documentation covers agent workflow
r[cairn.supersede.mcp_server.verification.docs]
- GIVEN an operator wants to configure Cairn for MCP-capable agents
- WHEN they inspect repository documentation
- THEN the documentation lists available tools, mutation gating, response receipt fields, and example read-only and mutation-enabled workflows
