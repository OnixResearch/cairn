## Why

Agents can already use `cairn` by shelling out to the CLI, but that forces them to parse command text, walk repository files directly, and decide mutation safety themselves. A native MCP surface would expose Cairn's deterministic lifecycle receipts as structured tool results while preserving the existing dry-run-first and FCIS boundaries. r[cairn.supersede.mcp_server.read_tools]

## What Changes

- Specify a Cairn MCP server surface for bounded read-only lifecycle inspection: status, content listing, spec reads, change reads, doctor, validate, gate, and gate context. r[cairn.supersede.mcp_server.read_tools]
- Specify dry-run planning tools for sync, archive, and legacy migration without filesystem mutation. r[cairn.supersede.mcp_server.plan_tools]
- Specify opt-in mutation tools guarded by an explicit server capability flag. r[cairn.supersede.mcp_server.mutation_guard]
- Preserve Cairn's pure-core / imperative-shell split by keeping MCP transport, filesystem access, path bounding, and JSON-RPC effects outside `cairn-core`. r[cairn.supersede.mcp_server.boundary]
- Add verification expectations for MCP tool registration, annotations, structured receipts, path bounds, and mutation denial by default. r[cairn.supersede.mcp_server.verification]

## Impact

- **Files**: Future implementation likely touches `crates/cairn-cli` or a new `crates/cairn-mcp`, `crates/cairn-core` read/planning helpers only where pure models are missing, CLI/MCP tests, README/docs, and Nix flake app/package wiring.
- **Testing**: Focused MCP registration/tool-call tests, existing CLI regression tests, policy freshness, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, Slotcar purity, `cairn validate --root .`, and `nix flake check`.
