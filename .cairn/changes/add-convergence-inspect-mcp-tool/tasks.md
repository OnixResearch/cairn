# Tasks

- [x] [serial] [task:inspect] Extract the CLI policy read and rejection mapping into one `inspect_with_policy` function and route `cairn convergence inspect` through it. r[cairn.convergence.mcp_inspect]
- [x] [serial] [task:tool] [after:inspect] Add the read-only `cairn_convergence_inspect` tool with `case`, `root` and `policy`, a strict input schema, argument validation and dispatch through the shared inspect function. r[cairn.convergence.mcp_inspect]
- [x] [serial] [task:policy] [after:tool] Register the tool in the `mcp-discovery` inspect, full and verify profiles, regenerate generated policy, and pass `policy export --check`. r[cairn.convergence.mcp_inspect]
- [x] [serial] [task:positive] [after:policy] Assert on the synthetic corpus that the MCP receipt equals the CLI receipt, `receipt_blake3` matches, and repeated calls agree. r[cairn.convergence.mcp_inspect]
- [x] [serial] [task:negative] [after:positive] Add the missing argument, unknown argument, unadmitted schema and symlinked input cases, plus catalog/profile membership. r[cairn.convergence.mcp_inspect]
- [x] [serial] [task:docs] [after:negative] Document the tool in `docs/convergence.md` and `docs/cairn-mcp.md`, and record it in the CHANGELOG. r[cairn.convergence.mcp_inspect]

## Validation and completion

- [x] [serial] [task:focused] [after:docs] Run formatting, clippy, `cargo test -p cairn-cli` and the convergence core tests; record the transcripts under `evidence/`. r[cairn.convergence.mcp_inspect]
- [x] [serial] [task:gates] [after:focused] Run `policy export --check`, `validate --root .`, the proposal, design and tasks gates, and the full `nix flake check`. r[cairn.convergence.mcp_inspect]
- [x] [serial] [task:lifecycle] [after:gates] Sync and archive through native lifecycle, then re-run validate and traceability coverage. r[cairn.convergence.mcp_inspect]
