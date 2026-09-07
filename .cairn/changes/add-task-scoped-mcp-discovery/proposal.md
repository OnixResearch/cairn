# Add task-scoped MCP discovery

## Why

The current `tools/list` returns the complete admitted catalog without task selection.
Some agent hosts inject all returned schemas into model context, while other hosts already discover tools lazily.
A smaller catalog can reduce tokens only for hosts that expose those schemas to the model.

## What Changes

- Add explicit policy-owned catalog profiles for common inspection, authoring, and verification tasks.
- Add bounded catalog lookup that returns one exact existing tool schema when requested.
- Add deterministic catalog pagination with identity-bound continuation.
- Preserve strict typed schemas and the existing full-catalog compatibility path.
- Keep profile selection separate from mutation capability admission.

## Impact

Current consumer: Cairn MCP hosts that otherwise inject the full tool catalog.
Immediate outcome: the host can expose only the tools relevant to the selected task.
Durable contribution: a versioned catalog projection with exact schema parity and supported discovery behavior.
Maintenance owner: Cairn CLI/MCP and policy maintainers.

Reuse `mcp_tools`, `mcp_input_schema`, input validation, and mutation-denial rules.
Expected changes include Nickel catalog profiles, MCP startup and discovery adapters, tests, and host integration documentation.
No new generic command dispatcher is necessary.

## Repeatability evidence

Compare each discovered schema with the same tool in the full catalog.
Cover supported profiles, unknown profiles and tools, stale cursors, missing capabilities, and malformed discovery requests.
Run the token benchmark through both an eager-schema host and a lazy-discovery host.
Account for discovery calls as well as the schemas that reach the model.

## Out of Scope

- A loosely typed catch-all execution tool
- Runtime tool loading, third-party plugins, or mutation permission from a catalog profile
- Assuming every MCP host injects every schema
- Hiding diagnostic or repair tools required by the selected task

Dependency: `measure-agent-lifecycle-token-cost`. Implementation remains open.
