# Change: Split MCP CLI integration tests

## Summary

Split the monolithic `crates/cairn-cli/tests/mcp_cli.rs` integration-test target into focused test modules while keeping the Cargo integration-test target name and public CLI behavior unchanged.

## Rationale

`mcp_cli.rs` was the largest Rust file after the latest core splits and mixed helper code with unrelated MCP surfaces: agent-receipt validation, catalog/structured receipt checks, drain-plan checks, strict argument rejection, mutation denial, and JSON-RPC transport smoke coverage. Focused modules reduce review surface and make future MCP changes easier to verify without changing behavior.

## Compatibility

The split is behavior-preserving:

- keep `tests/mcp_cli.rs` as the Cargo test target façade;
- move only test organization and shared helpers;
- preserve test names, assertions, fixtures, and CLI commands;
- add Tracey references for maintainability coverage.
