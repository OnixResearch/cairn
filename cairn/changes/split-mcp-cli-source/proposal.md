## Why

The MCP server now contains JSON-RPC transport, schema catalog, argument validation, read tools, mutation tools, and smoke receipts in one file. Splitting reduces risk for future tool additions.

Current target: `crates/cairn-cli/src/mcp.rs` (~922 lines).

## What Changes

- Add a behavior-preserving refactor package for MCP transport, schema, read-tool, mutation-tool, and agent-receipt bridge code.
- Define module/test boundaries before implementation so each split can be landed as a narrow verified slice.
- Keep public CLI behavior, deterministic JSON receipts, issue codes, fixture output, policy hashes, and Tracey references stable unless a later explicit Cairn changes them.

## Impact

- **Files**: `crates/cairn-cli/src/mcp.rs` (~922 lines) and any newly introduced sibling modules/test targets.
- **Testing**: focused MCP integration tests plus agent receipt smoke and determinism bridge checks, then repository validation, Tracey coverage, Clippy, and diff checks.
- **Compatibility**: No public command, schema, or receipt semantics are intended to change.
