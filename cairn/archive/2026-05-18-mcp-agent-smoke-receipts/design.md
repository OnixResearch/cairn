## Context

The accepted Cairn MCP and agent-run receipt requirements already define strict tool schemas, read-only default behavior, structured MCP `tools/call` responses, and agent receipt validation. The missing piece is an operator-facing smoke receipt that proves these public surfaces continue to work together after future MCP/schema/receipt changes.

## Goals

- Provide one maintained, deterministic smoke scenario for the default read-only MCP server.
- Exercise catalog description, agent receipt validation through MCP, and invalid-argument rejection in one coherent workflow.
- Produce durable evidence with stable fields and explicit non-mutation assertions.

## Non-Goals

- No new mutation capability.
- No long-running external MCP client dependency.
- No broad release command yet; this change is the smoke-receipt seam that a later release bundle can reuse.

## Decisions

### Smoke flow shape

**Choice:** The smoke workflow should be runnable in-process through existing CLI test seams such as `cairn mcp --describe`, `cairn mcp --call`, or JSON-RPC stdin fixtures. It must avoid a network service or external MCP client. r[cairn.supersede.mcp_agent_smoke_receipts.flow]

**Rationale:** A pure local smoke path is deterministic, fast, and suitable for flake/CI checks.

### Evidence contract

**Choice:** The smoke evidence must assert stable, public fields rather than compare whole transient output. Required assertions include tool name/schema presence, `structuredContent`, receipt validity, `receipt_hash` presence where applicable, `mutated=false` for the invalid/default-deny branch, and JSON-RPC error determinism for malformed arguments. r[cairn.supersede.mcp_agent_smoke_receipts.evidence]

**Rationale:** Field-level assertions catch contract breaks while avoiding unnecessary coupling to human summaries.

### Read-only default

**Choice:** The smoke workflow must run without `--allow-mutations` and must use fixture paths under the repository root. Invalid calls must be rejected before lifecycle reads/plans/mutations are dispatched. r[cairn.supersede.mcp_agent_smoke_receipts.read_only]

**Rationale:** Operator smoke evidence should be safe to run before release and during local debugging.

## Risks

- Whole-JSON golden comparisons could become noisy; prefer targeted stable-field assertions.
- A smoke helper could duplicate existing integration tests; keep the first implementation small and fixture-backed.
