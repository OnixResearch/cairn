## Phase 1: Source or test split

- [x] [serial] I1 Inspect dependencies and choose the smallest safe module/test-target boundaries for `crates/cairn-cli/src/mcp.rs` (~922 lines). r[cli_maintainability.mcp_cli_source.focused_modules.visible_boundaries]
- [x] [serial] I2 Move MCP transport, schema, read-tool, mutation-tool, and agent-receipt bridge code into focused modules or test targets while keeping facade dispatch and bridge APIs narrow. r[cli_maintainability.mcp_cli_source.focused_modules.visible_boundaries] r[cli_maintainability.mcp_cli_source.focused_modules.narrow_bridges]
- [x] [serial] I3 Preserve command behavior, deterministic receipts, issue codes, fixture output, and Tracey references. r[cli_maintainability.mcp_cli_source.behavior_preserved.focused_checks]

## Phase 2: Verification

- [x] [serial] V1 Run focused verification for MCP transport, schema, read-tool, mutation-tool, and agent-receipt bridge code. r[cli_maintainability.mcp_cli_source.behavior_preserved.focused_checks]
- [x] [serial] V2 Run repository validation, Tracey coverage, formatting, linting, and diff checks. r[cli_maintainability.mcp_cli_source.behavior_preserved.repository_checks]

## Verification Coverage

| Scenario | Verification |
| --- | --- |
| r[cli_maintainability.mcp_cli_source.focused_modules.visible_boundaries] | I1, I2 |
| r[cli_maintainability.mcp_cli_source.focused_modules.narrow_bridges] | I2 |
| r[cli_maintainability.mcp_cli_source.behavior_preserved.focused_checks] | I3, V1 |
| r[cli_maintainability.mcp_cli_source.behavior_preserved.repository_checks] | V2 |
