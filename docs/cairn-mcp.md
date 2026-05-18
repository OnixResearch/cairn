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
```

Mutation-capable startup is explicit:

```sh
nix run .#cairn -- mcp --allow-mutations
```

## Default read-only and planning tools

All default tools include MCP annotations equivalent to `readOnlyHint=true` and `destructiveHint=false`.

- `cairn_status`: list selected lifecycle state for a root.
- `cairn_list_content`: list accepted specs, active changes, and archived changes.
- `cairn_read_spec`: read a bounded accepted spec by spec id.
- `cairn_change_show`: read bounded active-change metadata by change id.
- `cairn_doctor`: run diagnostics and return a structured receipt.
- `cairn_validate`: run validation and return a structured receipt.
- `cairn_gate`: run proposal/design/tasks gate evaluation.
- `cairn_gate_context`: return bounded gate context for a change.
- `cairn_sync_plan`: return a dry-run sync plan.
- `cairn_archive_plan`: return a dry-run archive plan.
- `cairn_migrate_plan`: return a dry-run legacy migration plan.

## Mutation guard

Without `--allow-mutations`, calls to `cairn_init`, `cairn_change_create`, `cairn_sync_execute`, `cairn_archive_execute`, and `cairn_migrate_execute` return a structured capability error with `mutated=false`.

With `--allow-mutations`, execution tools are listed with `readOnlyHint=false` and `destructiveHint=true`. Mutation-enabled calls use the same Cairn receipt fields as the CLI execution paths.

## Boundaries

MCP artifact selectors are lifecycle identifiers, not arbitrary paths. Selectors that are absolute, dot-prefixed, contain `..`, or use backslash traversal are rejected. Returned spec content is bounded to 64 KiB per call. Transport, filesystem access, path bounding, process IO, and JSON-RPC handling live in `cairn-cli`; `cairn-core` remains transport-independent.

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
