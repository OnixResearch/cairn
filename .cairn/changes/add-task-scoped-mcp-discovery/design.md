# Task-scoped MCP discovery design

## Context

`mcp/schema.rs` builds the complete typed tool catalog and `mcp/transport.rs` returns it from tools/list.
Some hosts already perform lazy discovery, so a smaller catalog is not a universal token improvement.
This package preserves the full default and introduces explicit bounded views of the same descriptors.

## Decisions

### Decision: Select an explicit tool profile

Add `cairn mcp --tool-profile PROFILE` with policy-owned full, inspect, author, and verify profiles.
The default is full. The selected profile remains fixed for the server instance.
Each profile names existing tool IDs and required diagnostic or repair routes.
Reject unknown IDs, duplicate IDs, unsupported profiles, and inconsistent capability declarations during policy admission.
Filter the selected catalog by the existing mutation capability. A profile never supplies capability by itself.

### Decision: Reuse exact descriptors

Add `cairn mcp --describe-tool NAME` and a strict read-only `cairn_tool_describe` tool for one selected descriptor.
Lookup returns the same schema, annotations, and command-contract identity as the full catalog.
It cannot call another tool or forward arbitrary arguments.
A tool outside the selected catalog remains unavailable for lookup and dispatch.
The describe tool is explicitly included in each supported profile and does not recursively expand its catalog.

### Decision: Support standard paged listing

Keep MCP tools/list semantics and add its optional cursor flow with explicit nextCursor only when more descriptors remain.
The catalog identity binds policy, command schemas, profile, mutation capability, and deterministic tool order.
Nickel owns named descriptor, page-byte, entry, and cursor limits.
A cursor binds that identity and its position. Unknown fields, stale identities, and oversized requests fail before product reads.
Default full listing retains the existing unpaged shape unless a bounded listing mode is explicitly selected.
Single-tool lookup supplies an alternative when a host already knows the desired name.

### Decision: Preserve authority and dispatch order

Capability-first mutation denial still occurs before argument-dependent product effects under every profile.
For nonmutating or capability-admitted tools, profile admission precedes normal tool argument validation and observation.
The profile is a restriction, not an execution grant. Original sync and archive preflight remains unchanged.
Keep strict schemas rather than one generic execute operation.
The shell owns transport. Pure catalog admission, identity, filtering, and paging consume explicit policy facts.

## Alternatives rejected

- A loosely typed catch-all command reduces schema visibility at the cost of reliable admission.
- Automatic profile expansion makes discovery behavior depend on hidden session state.
- Custom JSON-RPC methods for schema lookup reduce compatibility without a required capability.
- Removing all diagnostic tools from a small profile can increase failed tasks and retries.

## Tests and evidence

Capture the current full catalog and malformed-call behavior before edits.
Compare every selected and discovered descriptor with its full-catalog source.
Test all profiles with and without mutation capability, unknown tools, schema drift, invalid cursors, empty pages, and page reconstruction.
Retain denial-before-argument controls and prove lookup causes no product reads or mutation.
Run host adapters that expose schemas eagerly and discover them lazily through the common benchmark.
Include discovery calls, profile restarts, missing-tool failures, and actual model-visible schemas in token totals.

## Risks and non-claims

A host can ignore pagination or inject previously cached schemas. Report the declared host behavior and actual exposure.
Catalog identity proves selected descriptor equality, not tool correctness or execution authority.
MCP maintainers own the host compatibility matrix. Policy maintainers own profile completeness.

## Implemented interfaces and admission

The standalone Nickel root `cairn-policy/mcp-discovery.ncl` exports to
`cairn-policy/generated/mcp-discovery.json`. No field selection or runtime
evaluator is used. `mcp-discovery-policy-fresh` compares the native export with
the generated file. Every supported profile explicitly declares ordered tool
membership, mutation requirements, and required read-only diagnostic routes.
Pure admission rejects duplicate/unknown tool IDs, duplicate/unsupported or
missing profiles, capability disagreements with the descriptor/dispatch
authority, missing routes, incomplete full coverage, and inconsistent bounds.

`cairn_core::verified::mcp_discovery` owns policy admission, descriptor bounds,
filtering, exact descriptor lookup, canonical BLAKE3 identity, and tools/list
paging. `mcp/discovery.rs` only loads generated policy and supplies existing
`schema.rs` descriptors plus mutation facts; it does not create another schema.
The stdio shell holds one admitted `ToolCatalog` for its lifetime. Internal
full-catalog consumers also use admission and propagate its errors.

The opt-in listing mode is `--paged-tools`. List params are absent or a strict
object containing only an optional nonempty valid cursor. No page-size or
custom MCP method is introduced. The current named limits are 65536 descriptor
bytes, 131072 serialized result bytes, 8 entries, 256 cursor bytes, and 128 name
bytes. Continuations use a versioned identity/position encoding, reject stale
identity and noncanonical/out-of-range offsets, and are emitted only when more
tools remain. Identical server restarts do not stale an unchanged identity.

Single lookup uses `--describe-tool NAME` or
`cairn_tool_describe {"name":"NAME"}`. The CLI emits one descriptor directly;
the MCP read tool emits it in normal structuredContent. No root, policy,
arguments, or generic forwarding is admitted. Nonmembers are unavailable even
when they are otherwise valid tool names. Mutation capability denial remains
first; profile admission then precedes input validation and lifecycle reads.
No response-profile selection or projection is introduced here: future result
presentation composes after this catalog/authority boundary.

## Evidence status

Consumer tests author all four profiles with both capability states, strict
lookup, unavailable dispatch before argument effects, admitted status calls,
bounded pages, duplicate-free enumeration, invalid/terminal/stale cursors, and
unpaged compatibility. Pure tests author policy admission, schema/policy/order
identity drift, oversized descriptors, and byte-bound page splitting. These
tests have not been executed in this implementation worktree. Descriptor
equality replay and repository gates belong to parent acceptance. No live host
campaign has occurred; eager/lazy exposure and savings remain unavailable.

Baseline capture and paired representation replay are supplied separately by
the integration owner. They do not retroactively establish live-host evidence.

