## Why

The Cairn MCP server now exposes a useful lifecycle tool catalog, but every tool currently advertises an open object schema. MCP-capable agents cannot distinguish required arguments from optional ones, cannot preflight selector formats, and receive less deterministic feedback when arguments are malformed. That weakens the safety story for read-only lifecycle access and mutation-gated tools.

## What Changes

- Specify per-tool MCP input schemas for read, planning, validation, and mutation tools instead of a blanket `additionalProperties: true` contract.
- Require the server to validate calls against the advertised schema before filesystem lifecycle operations or mutation planning begins.
- Preserve the current mutation-capability guard: strict schemas improve argument safety but do not make mutating tools available by default.
- Document schema expectations and add focused positive/negative tests for catalog shape and invalid-argument behavior.

## Impact

- **Files**: `crates/cairn-cli/src/main.rs`, `crates/cairn-cli/tests/cli.rs`, `docs/cairn-mcp.md`, accepted Cairn MCP requirements after sync.
- **Testing**: MCP catalog schema assertions; invalid params for representative read, plan, validation, and mutation tools; existing Cairn validate/gate/Tracey/full Rust checks.
