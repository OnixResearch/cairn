## ADDED Requirements

### Requirement: Split MCP CLI integration tests into focused modules

r[cli_maintainability.mcp_cli_tests_followup.focused_modules] The system MUST split the `mcp_cli` integration-test target into focused modules while preserving the existing Cargo test target façade.

#### Scenario: Focused MCP test boundaries are visible

r[cli_maintainability.mcp_cli_tests_followup.focused_modules.visible_boundaries]
- GIVEN the MCP CLI integration-test split is implemented
- WHEN a reviewer inspects `crates/cairn-cli/tests/mcp_cli.rs` and its module tree
- THEN agent-receipt, catalog/read-tool, drain-plan, validation/mutation-denial, and JSON-RPC checks are separated into named modules
- AND `mcp_cli.rs` remains a small façade for the test target.

#### Scenario: Shared MCP test helpers stay narrow

r[cli_maintainability.mcp_cli_tests_followup.focused_modules.narrow_bridges]
- GIVEN sibling MCP integration-test modules need common setup
- WHEN helpers are shared between them
- THEN helper visibility MUST remain limited to the module tree rather than becoming broad public test APIs.

### Requirement: Split MCP CLI integration tests remains behavior preserving

r[cli_maintainability.mcp_cli_tests_followup.behavior_preserved] The system MUST preserve existing MCP CLI integration-test behavior, fixtures, commands, receipts, and issue expectations while splitting the test target.

#### Scenario: Focused MCP CLI tests pass

r[cli_maintainability.mcp_cli_tests_followup.behavior_preserved.focused_checks]
- GIVEN the split is implemented
- WHEN the operator runs the focused `mcp_cli` integration-test target
- THEN the focused checks pass without intentional receipt, command, fixture, or assertion drift.

#### Scenario: Repository checks pass

r[cli_maintainability.mcp_cli_tests_followup.behavior_preserved.repository_checks]
- GIVEN the split is implemented
- WHEN the operator runs repository validation, Tracey coverage, formatting, linting, workspace tests, and diff checks
- THEN all checks pass and the repository remains ready for archival and commit.
