# MCP receipt projection design

## Context

`crates/cairn-cli/src/mcp/schema.rs::mcp_summary` embeds the whole receipt in text.
`mcp/mod.rs::mcp_call_tool` also places it in structuredContent.
A host can expose both copies, one copy, or neither copy correctly. Compatibility requires an explicit choice.

## Decisions

### Decision: Use an explicit response profile

Add `cairn mcp --response-profile legacy|structured-summary` to server startup and direct-call mode.
The policy owns admitted profile IDs and summary bounds. The default remains `legacy`.
The selected profile stays fixed for that server instance. A client name or protocol version does not imply support.
Unknown values fail before tool dispatch. Generated agent guidance can recommend the new profile only for declared compatible hosts.

### Decision: Preserve the original receipt

Keep tool execution and receipt builders unchanged.
The legacy profile emits the current response. The structured-summary profile uses the same structuredContent with a short text projection.
Do not change nested receipt hashes or manufacture a different lifecycle decision.
The summary includes the tool name, admitted state fields, bounded issue counts, and a reference to structured detail.
It does not copy arbitrary diagnostic strings or source content.

### Decision: Derive summaries from explicit semantics

Use a closed per-result projection for known receipt families.
Keep transport completion separate from valid, classification, verdict, blocked, and mutation denial.
Unknown receipt families produce an unclassified summary rather than a success inference.
Required status and boundary fields take priority over optional display text.
Reject impossible summary budgets or emit the policy-defined bounded fallback without truncating the complete receipt.

### Decision: Keep effects at the existing shell boundary

The shell selects the profile and renders JSON-RPC. Pure summary decisions consume explicit result values and admitted policy.
No new port or shared crate is necessary for a presentation transform.
Reuse existing strict schemas, capability-first mutation denial, and original mutation preflight.

## Alternatives rejected

- An immediate default switch can deprive text-only hosts of complete evidence.
- Removing structuredContent breaks structured consumers and typed receipt handling.
- Sending minified JSON in both fields retains the duplicate semantic payload.
- Inferring success from isError or process status hides lifecycle failure.

## Tests and evidence

Record the current catalog and successful, blocked, invalid, unavailable, and mutation-denied response fixtures.
Compare both profiles for exact structured receipt equality and unchanged hashes.
Assert the legacy text remains complete and the new text excludes the full serialized payload.
Exercise long diagnostics, unknown receipt families, invalid profile IDs, and denied mutation before argument admission.
Run the common token benchmark through a structured host, a text-only host, and a host that already removes duplication.
Include host routing and all tool calls in the comparison. Wire-byte reduction alone is not acceptance evidence for token savings.

## Risks and claim boundary

A host can ignore structuredContent. The explicit profile and compatibility default limit that risk but do not prove host support.
Receipt semantics remain Cairn-owned. A response profile grants no task completion, evidence truth, or execution authority.
MCP maintainers own compatibility fixtures and the documented host support matrix.
