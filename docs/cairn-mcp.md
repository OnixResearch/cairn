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

The `release-readiness` command emits a local JSON bundle that cites Cairn validation, Cairn-owned traceability coverage, the MCP agent smoke receipt, optional stack-provenance disposition, optional function-address disposition, and the maintained heavyweight repository gate policy. Use it as the concise operator artifact before release or agent handoff, then still run the listed repository gates (`cargo fmt --check`, workspace tests, Clippy, Octet, and `nix flake check`) as the heavyweight proof. Stable bundle fields include `checks`, `cairn_validation_hash`, `traceability_coverage_hash`, `mcp_smoke_receipt_hash`, optional `stack_provenance_hash`, optional `function_address_hash`, `repository_gate_summary`, and top-level `receipt_hash`. Traceability marker coverage does not prove behavioral correctness. r[verify cairn.supersede.release_readiness_bundle.docs]

When a Valence stack-provenance receipt and compact graph summary are supplied, Cairn checks release/lifecycle policy conformance over that evidence and records the result as `stack_provenance`; Mantle remains responsible for build/release bundle evidence, and Valence remains responsible for stack-provenance semantics. A passing Cairn stack-provenance gate does not prove source-code correctness, Valence verifier soundness, Mantle build correctness, or Trellis/Octet proof validity. r[verify cairn.stack_provenance_gates.boundary.visible]

When Valence, Mantle, and optional Kamacite function-address evidence are supplied, Cairn checks lifecycle/release policy conformance over that evidence and records the result as `function_address`; Octet remains responsible for Rust extraction, Kamacite remains responsible for portable receipts, Valence remains responsible for evidence identity/linkage/non-claims, and Mantle remains responsible for release artifact binding. The release-readiness shell accepts documented lowercase BLAKE3 spellings (`<hex>` and `b3:<hex>`) for Valence, Mantle, Kamacite, and graph digest fields, normalizes accepted inputs before pure readiness evaluation, and rejects unsupported prefixes, uppercase variants, wrong lengths, and non-hex content. Direct function-address inputs may declare versioned contracts such as `valence.function-address-evidence.v1`, `mantle.function-address-binding.v1`, `kamacite.function-address-receipt.v1`, and `valence.function-address-graph.v1`; when present, Cairn validates those schema versions and direct Mantle disposition fields before readiness evaluation while keeping wrapper compatibility bounded. Digest spelling normalization is not proof: role/domain checks still reject syntactically valid digests placed in the wrong readiness field. A passing function-address gate does not prove Rust semantic correctness, compiler correctness, whole-program safety, verifier soundness, or release eligibility. r[verify cairn.stack_provenance_gates.function_address_digest_normalization.contract] r[verify cairn.stack_provenance_gates.function_address_input_schema.contract]

Mutation-capable startup is explicit:

```sh
nix run .#cairn -- mcp --allow-mutations
```

## Task-scoped discovery

The server selects `--tool-profile full|inspect|author|verify` once at startup.
`full` is the default. A profile restricts both listing and dispatch; it cannot
grant mutation capability or expand itself after a missing-tool call. Restart
with the appropriate profile when a task needs a tool outside its catalog.
The standalone source `cairn-policy/mcp-discovery.ncl` owns membership, ordering,
required diagnostic/repair routes, and named bounds. Runtime reads only its
checked generated JSON. Invalid policy or an unknown profile fails startup.

| Profile | Task and completeness boundary |
| --- | --- |
| `full` | Every existing read, planning, and capability-admitted mutation tool, plus descriptor lookup. |
| `inspect` | Status, content/spec/change reads, context, workflow status/instructions, doctor, gate context, change graph, readiness, implementation-ready selection, and descriptor lookup. No validation, execution, or migration. |
| `author` | `inspect` plus validate, gate, all three dry-run plans, and the five mutation tools when separately enabled. Retains workflow instructions and dry-run repair routes. |
| `verify` | Status, spec/change reads, context, doctor, validate, gate/context, change graph, receipt/completion validation, readiness, verification impact, adoption report, drain plan, implementation-ready selection, sync/archive plans, and descriptor lookup. No authoring or migration. |

Each profile retains `cairn_doctor` and `cairn_gate_context`. Authoring also
retains all mutation dry-run counterparts; verification retains sync/archive
plans. The exact ordered catalog is available through `--describe`, which adds
`tool_profile` and `catalog_identity` without changing existing descriptors.

```sh
cairn mcp --tool-profile inspect --describe
cairn mcp --tool-profile inspect --describe-tool cairn_status
cairn mcp --tool-profile inspect --call cairn_tool_describe '{"name":"cairn_status"}'
cairn mcp --tool-profile author --allow-mutations
cairn mcp --tool-profile verify --paged-tools
```

`--describe-tool NAME` returns the exact one descriptor, not a catalog wrapper.
The read-only `cairn_tool_describe` tool requires only the nonempty string `name`
and rejects all extra arguments, including `root`, `policy`, or forwarded tool
arguments. It returns that same descriptor as `structuredContent` in the normal
MCP envelope. Lookup never reads product files, runs a tool, or recursively
expands the catalog; self-description is just one descriptor. Unknown names,
profile nonmembers, and capability-filtered tools return `tool_unavailable`.
The existing descriptors remain the sole schema and annotation authority.

### Standard bounded tools/list

Without `--paged-tools`, `tools/list` retains its unpaged `{"tools":[...]}`
result and does not return `nextCursor`. With the flag, the standard optional
`params.cursor` flow returns at most 8 descriptors and 131072 serialized result
bytes per page. Individual descriptors are bounded to 65536 bytes, tool names
to 128 bytes, and cursor strings to 256 bytes by the current policy. The byte
bound includes the continuation and result wrapper. A page never silently
drops a descriptor to fit, and a valid admitted catalog has no empty terminal
page. Only pages with remaining tools contain `nextCursor`.

Send `tools/list` without `params` or with `{}` for the first page. Continue by
sending `{"cursor":"<the exact returned nextCursor>"}` as `params`; stop when
the result omits `nextCursor`. No custom JSON-RPC method or `limit` field is
required. Unknown fields, non-object params, non-string/oversized cursors,
noncanonical or out-of-range positions, and cursors when paging is disabled
fail with JSON-RPC invalid params (`-32602`). Explicit `null` is not an object.

Cursors are opaque client-side. They bind the BLAKE3 catalog identity and next
position. Identity includes the admitted policy, complete descriptor authority
(including schemas and annotations), selected profile, mutation capability,
and deterministic tool order. Policy, schema, order, profile, or capability
drift rejects a stale continuation instead of mixing catalogs. Identical
server restarts admit the same cursor; cursors are not credentials. The CLI
catalog identity is diagnostic metadata, not an execution permission.

### Host compatibility and evidence limits

Hosts using the full unpaged catalog need no configuration changes. Eager
hosts may explicitly select a smaller profile; hosts supporting standard MCP
cursor discovery may opt into bounded listing. Lazy hosts that already expose
only requested tools may gain nothing and can pay extra discovery overhead.
Hosts must consume every returned continuation when they intend to enumerate
the complete selected catalog. A host that ignores `nextCursor` will see only
the first page, so do not opt it into paging. Descriptor lookup is available
when the host already knows the desired tool name.

No live eager/lazy host campaign has been run for this change. Representation
replays do not establish model-visible exposure or end-to-end savings. Record
actual schemas exposed, discovery calls, profile restarts, missing-tool
failures, token totals, and oracle outcomes in paired host measurements before
claiming improvements. Separate response-profile projection, if selected,
applies to tool result presentation, not profile membership or descriptor
authority. r[verify cairn.mcp_discovery.conformance]

Policy freshness is checked by the Nix `mcp-discovery-policy-fresh` check. The
standalone export is the entire source root, with no field selection:

```sh
nickel export --format json cairn-policy/mcp-discovery.ncl > cairn-policy/generated/mcp-discovery.json
nix build .#checks.x86_64-linux.mcp-discovery-policy-fresh --no-link
```

Profile policy changes must keep every tool ID and capability declaration in
agreement with the descriptor authority; full coverage and required read-only
routes are admitted before a server starts. Profiles do not bypass sync or
archive preflight. Absent capability, malformed mutation calls still receive
the original denial with `mutated=false` before argument-dependent product
effects. With capability, profile admission precedes argument validation and
product observation. r[verify cairn.mcp_discovery.authority]


## Default read-only and planning tools

All default tools include MCP annotations equivalent to `readOnlyHint=true` and `destructiveHint=false`. Each tool advertises a strict object `inputSchema` with explicit properties, required arguments, and `additionalProperties=false` unless noted. Invalid arguments are rejected before lifecycle files are read or lifecycle plans are built. String arguments must be non-empty. r[verify cairn.supersede.mcp_server.strict_schemas.docs]

- `cairn_status`: optional `root`; list selected lifecycle state for a root.
- `cairn_list_content`: optional `root`; list accepted specs, active changes, and archived changes.
- `cairn_read_spec`: required `spec`, optional `root`; read a bounded accepted spec by spec id.
- `cairn_change_show`: required `change`, optional `root`; read bounded active-change metadata by change id.
- `cairn_observe`: required `view` (`active_changes`, `readiness`, or `context`), optional `root`, `policy`, `change`, readiness budgets, context scope/spec/artifact/selectors, `mode`, page limits, `cursor`, and one bounded `prior` object or root-relative `prior_path`; return a stateless complete, paged, unchanged, or difference observation. It never writes an output file.
- `cairn_selected_work`: required `change` and exactly one of `root` or `lifecycle_store`; optional `task`, `policy`, `contract`, `results`, `subject_hash`, `cross_repo_evidence`, `obligation_definition`, `obligation_observations` (requires `obligation_definition`), `disclosure`, `expected_cohort` and `detail` (requires `expected_cohort`). Returns the same `cairn.selected-work.v1` snapshot as `cairn selected-work`; see [selected-work snapshots](selected-work.md).
- `cairn_doctor`: optional `root`, `policy`; run diagnostics and return a structured receipt.
- `cairn_validate`: optional `root`, `policy`; run validation and return a structured receipt.
- `cairn_gate`: required `change` and `stage`, optional `root`, `policy`; `stage` must be `proposal`, `design`, or `tasks`.
- `cairn_gate_context`: required `change`, optional `root`; return bounded gate context for a change.
- `cairn_agent_receipt_validate`: either `path` plus optional `root`, or inline `receipt`; validate a bounded agent-run receipt. r[verify cairn.supersede.agent_receipt_validation_surface.mcp]
- `cairn_drain_plan`: optional `root`, `policy`, `cross_repo_evidence`; return a read-only repository-wide drain plan for all active changes, including task counts, proposal/design/tasks gate summaries, blockers, and explicit next actions. It recommends `cairn_sync_execute`/`cairn_archive_execute` for ready changes but never performs mutations itself. r[verify cairn.mcp_drain_all.plan.read_only] r[verify cairn.mcp_drain_all.plan.readiness] r[verify cairn.mcp_drain_all.plan.next_actions] r[verify cairn.mcp_drain_all.plan.mutation_boundary]
- `cairn_drain_plan` with `"view": "readiness"` returns the additive `cairn.drain-readiness.v1` projection instead: one shared `cairn.readiness.v1` report per active change, the declared dependency order, and selected advisory next actions. The default receipt and its hashes are unchanged. `evidence_manifest` selects a repository-relative JSON manifest keyed by exact active change identifiers; each entry may supply `contract`, `results`, `subject_hash`, and `cross_repo_evidence` under the same rules as `cairn_readiness`. Missing selectors stay `unavailable`; the view never discovers evidence, infers subjects, or executes actions. r[verify cairn.agent_workflow_readiness.compatibility] r[verify cairn.agent_workflow_readiness.shared] r[verify cairn.agent_workflow_readiness.order]
- `cairn_implementation_ready`: optional `root`, `policy`, `cross_repo_evidence`, `limit` (1–1000, default 100). Returns `cairn.implementation-ready.v1` eligibility distinct from archive readiness: per-change ready/completed/blocked tasks, stable `task:<id>` or legacy `change:<name>` selectors, requirement references, gates, blockers, and a deterministic per-change snapshot digest. `limit` truncation returns `outcome: incomplete_limit` with `complete: false`; it never mutates or dispatches.
- `cairn_convergence_inspect`: required `case`, optional `root`, `policy`; return the same `cairn.convergence.v1` receipt as `cairn convergence inspect` for the same inputs. Reads are bounded and link-free; unknown arguments are rejected before any input is read, and an unadmitted schema returns the CLI's rejection receipt. See [convergence](convergence.md).
- `cairn_sync_plan`: required `change`, optional `root`, `policy`; return a dry-run sync plan.
- `cairn_archive_plan`: required `change`, optional `root`, `policy`; return a dry-run archive plan.
- `cairn_migrate_plan`: required `source`, optional `root`, `policy`; `source` must be `cairn`, `openspec`, or `ironspec`. The receipt includes the reviewed `plan_hash`.

## Mutation guard

Without `--allow-mutations`, calls to `cairn_init`, `cairn_change_create`, `cairn_sync_execute`, `cairn_archive_execute`, and `cairn_migrate_execute` return a structured capability error with `mutated=false`; this default denial does not depend on otherwise-valid tool arguments.

With `--allow-mutations`, execution tools are listed with `readOnlyHint=false`, `destructiveHint=true`, and strict schemas: `cairn_init` accepts optional `root`/`policy`, `cairn_change_create` requires `change`, sync/archive execute require `change` plus optional `root`/`policy`, and migrate execute requires `source` (`cairn`, `openspec`, or `ironspec`) and `plan_hash` plus optional `root`/`policy`. Mutation-enabled calls use the same Cairn receipt fields as the CLI execution paths.

## Boundaries

MCP artifact selectors are lifecycle identifiers, not arbitrary paths. Selectors that are absolute, dot-prefixed, contain `..`, or use backslash traversal are rejected. Returned spec content and agent receipt inputs are bounded to 64 KiB per call. Transport, filesystem access, path bounding, process IO, and JSON-RPC handling live in `cairn-cli`; `cairn-core` remains transport-independent.

## Response shape

`tools/call` responses contain `content` text, the complete `structuredContent`
receipt, and the existing `isError` transport/tool classification. Transport
success does not establish lifecycle readiness or authorize mutation.

The default `legacy` response profile retains the complete receipt in both text
and structured content for text-only hosts. A host that consumes
`structuredContent` can explicitly select a single complete payload:

```sh
cairn mcp --response-profile structured-summary
cairn mcp --response-profile structured-summary --call cairn_readiness '{"root":"fixtures/basic","change":"demo"}'
```

`structured-summary` keeps the structured receipt, its hashes, error behavior,
and capability checks unchanged. Only text changes: a policy-bounded summary of
admitted status/boundary fields and a reference to `structuredContent`. Raw
diagnostics, source, and paths are not copied into summary text. Unknown receipt
families are explicitly unclassified; unavailable lifecycle state never becomes
success. Oversized summaries use an omission fallback without truncating the
structured payload.

The profile is fixed at server startup, also applies to direct `--call`, and is
not a tool argument. Unknown, missing, repeated, or unsupported profile options
fail before dispatch. `--response-profile legacy` preserves the default; neither
profile grants `--allow-mutations` or alters tool schemas. Select legacy for a
host that ignores structured content. A smaller representation alone is not an
end-to-end efficiency result.

Structured receipts include Cairn fields such as `layout`, `valid`, `actions`, `blocked`, `mutated`, `policy_hash`, `plan_hash`, and `receipt_hash` where applicable, plus an `mcp` object describing read/destructive behavior.

## Minimal JSON-RPC example

```jsonl
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"cairn_validate","arguments":{"root":"fixtures/basic"}}}
```

Discovery unavailability in JSON-RPC uses error code `-32000` and typed
`error.data` with `code: "tool_unavailable"`, the requested `tool`,
`allowed: false`, and `mutated: false`. A capability-denied mutation keeps its
existing structured capability receipt instead, including for profile
nonmembers; this precedence never discloses the denied descriptor.
