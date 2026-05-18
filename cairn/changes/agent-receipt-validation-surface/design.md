# Design: Agent receipt validation surface

## CLI surface

**Choice:** Add a read-only CLI command such as `cairn agent-receipt validate <path>` that reads one bounded JSON receipt file, evaluates the existing pure `evaluate_agent_run_completion` gate, and emits deterministic JSON. r[cairn.supersede.agent_receipt_validation_surface.cli]

**Rationale:** The core gate already owns deterministic validation. A thin CLI shell makes it useful to agents and humans without moving filesystem or JSON transport effects into `cairn-core`.

**Receipt shape:** The command should expose at least `command`, `path`, `valid`, `verdict`, `issues`, `agent_receipt_hash`, `input_hash`, and `receipt_hash`. `agent_receipt_hash` comes from the agent-run receipt content; top-level `receipt_hash` hashes the Cairn validation receipt itself. r[cairn.supersede.agent_receipt_validation_surface.cli.schema]

## MCP tool

**Choice:** Add a read-only MCP tool such as `cairn_agent_receipt_validate` that accepts either a bounded receipt path under the selected root or an inline receipt object. r[cairn.supersede.agent_receipt_validation_surface.mcp]

**Rationale:** Agents using MCP should be able to gate their own completion claims through the same deterministic validation path already available from the CLI.

**Safety:** Path input must reject traversal/escape. Inline input must be bounded before validation. The tool must be present in default read-only MCP catalogs and must not require `--allow-mutations`. r[cairn.supersede.agent_receipt_validation_surface.mcp.read_only]

## Dogfood fixtures

**Choice:** Maintain source-controlled positive and negative receipt fixtures. Positive fixtures should validate to exact repeated JSON output; negative fixtures should fail deterministically with expected issue codes. r[cairn.supersede.agent_receipt_validation_surface.fixtures]

**Rationale:** The feature is about determinism, so the acceptance evidence must be replayable rather than only covered by unit tests.

## Documentation

**Choice:** Extend the agent-run receipt and/or MCP docs with a minimal wrapper flow. r[cairn.supersede.agent_receipt_validation_surface.docs]

**Rationale:** The target user is an agent wrapper/operator; docs should show how to close the loop without reading Rust internals.

## Non-goals

- No transcript storage, streaming log ingestion, or model-provider integration.
- No lifecycle mutation during validation.
- No replacement for existing lifecycle gates; this validates agent-run receipts only.
