## ADDED Requirements

### Requirement: Strict MCP tool input schemas
r[cairn.supersede.mcp_server.strict_schemas] The system MUST advertise deterministic per-tool MCP input schemas and validate tool arguments against those schemas before lifecycle reads, plans, or mutations are dispatched.

#### Scenario: Tool catalog advertises bounded schemas
r[cairn.supersede.mcp_server.strict_schemas.catalog]
- GIVEN the Cairn MCP server exposes its tool catalog
- WHEN an MCP client lists tools or an operator runs `cairn mcp --describe`
- THEN each tool MUST advertise an object input schema with explicit properties for its accepted arguments
- AND the schema MUST reject unknown properties unless the tool intentionally accepts an inline receipt object
- AND mutation tool schemas MUST remain unavailable unless the server starts with the explicit mutation capability.

#### Scenario: Argument validation precedes lifecycle effects
r[cairn.supersede.mcp_server.strict_schemas.validation]
- GIVEN an MCP client calls a Cairn tool with missing required arguments, unknown arguments, invalid enum values, or non-string lifecycle selectors
- WHEN the server receives the call
- THEN it MUST reject the request before reading lifecycle artifacts, planning lifecycle actions, or attempting filesystem mutation.

#### Scenario: Invalid arguments produce deterministic errors
r[cairn.supersede.mcp_server.strict_schemas.errors]
- GIVEN an MCP client calls a tool with malformed arguments
- WHEN the call is evaluated through direct `--call` or JSON-RPC `tools/call`
- THEN the server MUST return a deterministic invalid-argument error identifying the tool and argument class
- AND mutation-denied responses MUST continue to report `mutated=false` without depending on schema validation success.

#### Scenario: Schema contract is documented and verified
r[cairn.supersede.mcp_server.strict_schemas.verification]
- GIVEN the strict schema behavior is implemented
- WHEN repository verification runs
- THEN focused MCP tests MUST assert catalog schemas for representative read, planning, validation, and mutation tools
- AND negative tests MUST cover missing required keys, unknown keys, invalid stage/source enum values, traversal selectors, and mutation calls without the capability flag.

#### Scenario: Operators can use schema guidance
r[cairn.supersede.mcp_server.strict_schemas.docs]
- GIVEN an operator or agent wrapper author reads the Cairn MCP documentation
- WHEN they inspect tool usage guidance
- THEN the documentation MUST describe each tool's required and optional arguments, selector bounds, invalid-argument behavior, and the continued default-deny mutation policy.
