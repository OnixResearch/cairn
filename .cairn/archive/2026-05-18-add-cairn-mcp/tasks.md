## Phase 1: MCP server surface

- [x] [serial] Define the MCP server entry point, package/app wiring, and default read-only startup mode. r[cairn.supersede.mcp_server.read_tools]
  - Evidence 2026-05-18T16:35:03Z: `cairn mcp` stdio JSON-RPC entry point added; `mcp --describe` and `mcp --call` added for deterministic inspection/tests.
- [x] [serial] Implement bounded read-only MCP tools for status, content listing, spec reads, change reads, doctor, validate, gate, and gate context. r[cairn.supersede.mcp_server.read_tools]
  - Evidence 2026-05-18T16:35:03Z: `cairn_status`, `cairn_list_content`, `cairn_read_spec`, `cairn_change_show`, `cairn_doctor`, `cairn_validate`, `cairn_gate`, and `cairn_gate_context` return structured receipts.
- [x] [serial] Implement dry-run planning MCP tools for sync, archive, and legacy migration receipts. r[cairn.supersede.mcp_server.plan_tools]
  - Evidence 2026-05-18T16:35:03Z: `cairn_sync_plan`, `cairn_archive_plan`, and `cairn_migrate_plan` return dry-run Cairn receipts with plan/manifest hashes.
- [x] [serial] Add mutation-capability gating and default-deny errors for init, change creation, sync execution, archive execution, and migration execution. r[cairn.supersede.mcp_server.mutation_guard]
  - Evidence 2026-05-18T16:35:03Z: mutating MCP tools are omitted by default and denied when called without `--allow-mutations`; mutation-enabled startup advertises destructive annotations.

## Phase 2: Boundaries and safety

- [x] [serial] Enforce root/path bounding, traversal rejection, bounded content sizes, and lifecycle-only read surfaces. r[cairn.supersede.mcp_server.boundary]
  - Evidence 2026-05-18T16:35:03Z: spec/change selectors reject absolute, dot-prefixed, `..`, and backslash escapes; spec reads are capped at 64 KiB.
- [x] [serial] Keep MCP transport, filesystem reads, process exits, and JSON-RPC effects out of `cairn-core`; add or reuse pure core models only for deterministic lifecycle classification/planning. r[cairn.supersede.mcp_server.boundary]
  - Evidence 2026-05-18T16:35:03Z: implementation is confined to `crates/cairn-cli`; `cairn-core` was not modified for transport or IO.
- [x] [serial] Preserve structured Cairn receipt fields, BLAKE3 hashes, and MCP read/destructive annotations in tool responses. r[cairn.supersede.mcp_server.verification]
  - Evidence 2026-05-18T16:35:03Z: tool calls return `structuredContent` receipts with `receipt_hash` and an `mcp` annotation object.

## Phase 3: Verification and docs

- [x] [serial] Add focused MCP registration/tool-call tests, including annotation/schema assertions and structured receipt assertions. r[cairn.supersede.mcp_server.verification]
  - Evidence 2026-05-18T16:35:03Z: `cargo test -p cairn-cli mcp -- --nocapture` passed 4 MCP tests.
- [x] [serial] Add negative tests for path traversal, unavailable default mutation tools, and blocked mutation attempts without `--allow-mutations`. r[cairn.supersede.mcp_server.mutation_guard]
  - Evidence 2026-05-18T16:35:03Z: `mcp_rejects_traversal_and_default_mutation` covers traversal rejection and default mutation denial.
- [x] [serial] Document the MCP tools, safety flags, response schemas, and example agent workflows. r[cairn.supersede.mcp_server.verification]
  - Evidence 2026-05-18T16:35:03Z: added `docs/cairn-mcp.md` and README MCP section.
- [x] [serial] Run formatting, workspace tests, Clippy, Slotcar, `cairn validate --root .`, and `nix flake check`. r[cairn.supersede.mcp_server.verification]
  - Evidence 2026-05-18T16:35:03Z: pre-archive `python3 tools/export-policy.py --check`, `cargo fmt --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo tigerstyle check --output-format json --artifact-dir artifacts/slotcar-run`, `cairn validate --root .`, and all gates passed; `nix flake check` was deferred until after sync/archive because accepted-spec traceability is only complete after promotion.
