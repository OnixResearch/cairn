# Cairn MCP server

Cairn exposes a native MCP-compatible JSON-RPC tool surface through `cairn mcp`. The server is deterministic and read-only by default: read and planning tools return structured Cairn receipts with stable hashes, and mutating tools are unavailable unless the server is started with `--allow-mutations`.

## Startup

```sh
nix run .#cairn -- mcp
```

For inspection or tests without a long-running stdio session:

```sh
nix run .#cairn -- mcp --describe
nix run .#cairn -- mcp --call cairn_status '{"root":"fixtures/basic"}'
nix run .#cairn -- agent-receipt smoke-mcp --root .
nix run .#cairn -- release-readiness --root .
```

The `agent-receipt smoke-mcp` command is the maintained operator smoke receipt for the MCP/agent-receipt seam. It proves that the default MCP catalog includes `cairn_agent_receipt_validate` with a strict schema, validates `fixtures/agent-receipts/positive.json` through the MCP tool, rejects a malformed `cairn_gate` call before dispatch, and denies a default mutation request with `mutated=false`. Its JSON output includes stable `checks`, `receipt_tool_schema_hash`, `agent_receipt_validation_hash`, `agent_receipt_hash`, and top-level `receipt_hash` fields. r[verify cairn.supersede.mcp_agent_smoke_receipts.docs]

The `release-readiness` command emits a local JSON bundle that cites Cairn validation, Cairn-owned traceability coverage, the MCP agent smoke receipt, optional stack-provenance disposition, optional function-address disposition, and the maintained heavyweight repository gate policy. Use it as the concise operator artifact before release or agent handoff, then still run the listed repository gates (`cargo fmt --check`, workspace tests, Clippy, Slotcar, and `nix flake check`) as the heavyweight proof. Stable bundle fields include `checks`, `cairn_validation_hash`, `traceability_coverage_hash`, `mcp_smoke_receipt_hash`, optional `stack_provenance_hash`, optional `function_address_hash`, `repository_gate_summary`, and top-level `receipt_hash`. Traceability marker coverage does not prove behavioral correctness. r[verify cairn.supersede.release_readiness_bundle.docs]

When a Valence stack-provenance receipt and compact graph summary are supplied, Cairn checks release/lifecycle policy conformance over that evidence and records the result as `stack_provenance`; Mantle remains responsible for build/release bundle evidence, and Valence remains responsible for stack-provenance semantics. A passing Cairn stack-provenance gate does not prove source-code correctness, Valence verifier soundness, Mantle build correctness, or Trellis/Octet proof validity. r[verify cairn.stack_provenance_gates.boundary.visible]

When Valence, Mantle, and optional Kamacite function-address evidence are supplied, Cairn checks lifecycle/release policy conformance over that evidence and records the result as `function_address`; Octet remains responsible for Rust extraction, Kamacite remains responsible for portable receipts, Valence remains responsible for evidence identity/linkage/non-claims, and Mantle remains responsible for release artifact binding. The release-readiness shell accepts documented lowercase BLAKE3 spellings (`<hex>` and `b3:<hex>`) for Valence, Mantle, Kamacite, and graph digest fields, normalizes accepted inputs before pure readiness evaluation, and rejects unsupported prefixes, uppercase variants, wrong lengths, and non-hex content. Direct function-address inputs may declare versioned contracts such as `valence.function-address-evidence.v1`, `mantle.function-address-binding.v1`, `kamacite.function-address-receipt.v1`, and `valence.function-address-graph.v1`; when present, Cairn validates those schema versions and direct Mantle disposition fields before readiness evaluation while keeping wrapper compatibility bounded. Digest spelling normalization is not proof: role/domain checks still reject syntactically valid digests placed in the wrong readiness field. A passing function-address gate does not prove Rust semantic correctness, compiler correctness, whole-program safety, verifier soundness, or release eligibility. r[verify cairn.stack_provenance_gates.function_address_digest_normalization.contract] r[verify cairn.stack_provenance_gates.function_address_input_schema.contract]

Mutation-capable startup is explicit:

```sh
nix run .#cairn -- mcp --allow-mutations
```

## Default read-only and planning tools

All default tools include MCP annotations equivalent to `readOnlyHint=true` and `destructiveHint=false`. Each tool advertises a strict object `inputSchema` with explicit properties, required arguments, and `additionalProperties=false` unless noted. Invalid arguments are rejected before lifecycle files are read or lifecycle plans are built. r[verify cairn.supersede.mcp_server.strict_schemas.docs]

- `cairn_status`: optional `root`; list selected lifecycle state for a root.
- `cairn_list_content`: optional `root`; list accepted specs, active changes, and archived changes.
- `cairn_read_spec`: required `spec`, optional `root`; read a bounded accepted spec by spec id.
- `cairn_change_show`: required `change`, optional `root`; read bounded active-change metadata by change id.
- `cairn_doctor`: optional `root`, `policy`; run diagnostics and return a structured receipt.
- `cairn_validate`: optional `root`, `policy`; run validation and return a structured receipt.
- `cairn_gate`: required `change` and `stage`, optional `root`, `policy`; `stage` must be `proposal`, `design`, or `tasks`.
- `cairn_gate_context`: required `change`, optional `root`; return bounded gate context for a change.
- `cairn_agent_receipt_validate`: either `path` plus optional `root`, or inline `receipt`; validate a bounded agent-run receipt. r[verify cairn.supersede.agent_receipt_validation_surface.mcp]
- `cairn_drain_plan`: optional `root`, `policy`; return a read-only repository-wide drain plan for all active changes, including task counts, proposal/design/tasks gate summaries, blockers, and explicit next actions. It recommends `cairn_sync_execute`/`cairn_archive_execute` for ready changes but never performs mutations itself. r[verify cairn.mcp_drain_all.plan.read_only] r[verify cairn.mcp_drain_all.plan.readiness] r[verify cairn.mcp_drain_all.plan.next_actions] r[verify cairn.mcp_drain_all.plan.mutation_boundary]
- `cairn_sync_plan`: required `change`, optional `root`, `policy`; return a dry-run sync plan.
- `cairn_archive_plan`: required `change`, optional `root`, `policy`; return a dry-run archive plan.
- `cairn_migrate_plan`: required `source`, optional `root`, `policy`; `source` must be `cairn`, `openspec`, or `ironspec`.

## Mutation guard

Without `--allow-mutations`, calls to `cairn_init`, `cairn_change_create`, `cairn_sync_execute`, `cairn_archive_execute`, and `cairn_migrate_execute` return a structured capability error with `mutated=false`; this default denial does not depend on otherwise-valid tool arguments.

With `--allow-mutations`, execution tools are listed with `readOnlyHint=false`, `destructiveHint=true`, and strict schemas: `cairn_init` accepts optional `root`/`policy`, `cairn_change_create` requires `change`, sync/archive execute require `change` plus optional `root`/`policy`, and migrate execute requires `source` (`cairn`, `openspec`, or `ironspec`) plus optional `root`/`policy`. Mutation-enabled calls use the same Cairn receipt fields as the CLI execution paths.

## Boundaries

MCP artifact selectors are lifecycle identifiers, not arbitrary paths. Selectors that are absolute, dot-prefixed, contain `..`, or use backslash traversal are rejected. Returned spec content and agent receipt inputs are bounded to 64 KiB per call. Transport, filesystem access, path bounding, process IO, and JSON-RPC handling live in `cairn-cli`; `cairn-core` remains transport-independent.

## Response shape

`tools/call` responses contain:

- `content`: a short text summary for humans.
- `structuredContent`: the Cairn receipt payload.
- `isError`: false for successful tool execution.

Structured receipts include Cairn fields such as `layout`, `valid`, `actions`, `blocked`, `mutated`, `policy_hash`, `plan_hash`, and `receipt_hash` where applicable, plus an `mcp` object describing read/destructive behavior.

## Minimal JSON-RPC example

```jsonl
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"cairn_validate","arguments":{"root":"fixtures/basic"}}}
```
