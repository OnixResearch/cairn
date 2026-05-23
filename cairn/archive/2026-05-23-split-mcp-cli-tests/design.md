# Design: Split MCP CLI integration tests

## Boundaries

- `crates/cairn-cli/tests/mcp_cli.rs`: tiny Cargo integration-test façade that loads the focused module tree.
- `crates/cairn-cli/tests/mcp_cli/mod.rs`: module registry and Tracey implementation/verification references.
- `common.rs`: shared command construction, JSON execution, hash assertion, repository fixture copy helper.
- `agent_receipts.rs`: MCP agent receipt smoke and bounded validation checks.
- `catalog.rs`: MCP catalog, read-only annotations, mutation tool exposure, and structured receipt checks.
- `drain_plan.rs`: read-only drain-plan and ready-change mutation recommendation checks.
- `validation.rs`: strict schema, traversal, and default mutation-denial negative checks.
- `json_rpc.rs`: stdio JSON-RPC tools/list and tools/call smoke checks.

## Invariants

- Preserve existing test functions and assertions.
- Preserve CLI commands and fixture paths.
- Keep helper visibility narrow (`pub(super)` only where sibling modules need helper access).
- Do not introduce production-code changes.

## Verification

Run focused MCP integration tests, formatting, validation, Tracey coverage, workspace tests, clippy, diff checks, and final flake check before commit.
