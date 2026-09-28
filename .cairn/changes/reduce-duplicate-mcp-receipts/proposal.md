# Reduce duplicate MCP receipts

## Why

At `cb0be66e0e992c0f4070956bb9d451d8a8fe7573`, `mcp_summary()` serializes the complete receipt into text.
`mcp_call_tool()` also returns that receipt in `structuredContent`.
The initial installed-binary probe confirmed equal payloads in both locations.
The documentation describes the text as a short summary, but the source does not enforce that description.

## What Changes

- Add an explicit MCP response profile for structured-content clients.
- Keep the complete receipt once in `structuredContent` and a bounded, accurate text summary in that profile.
- Retain the existing response as the compatibility default for unknown and text-only clients.
- Keep receipt fields and hashes unchanged across response profiles.
- Define summary behavior for ready, blocked, unavailable, invalid, and mutation-denied results.
- Measure the actual host-visible token effect through the shared benchmark contract.

## Impact

Current consumer: Cairn MCP agents whose hosts expose structured results.
Immediate outcome: those hosts receive one complete receipt instead of duplicate payloads.
Durable contribution: an explicit, tested response projection with a text-only compatibility path.
Maintenance owner: Cairn CLI/MCP maintainers.

Expected surfaces are `crates/cairn-cli/src/mcp/schema.rs`, `mcp/mod.rs`, transport startup, generated policy, MCP tests, and documentation.
Reuse the existing receipt builders and strict input schemas. Do not change lifecycle decisions.

## Repeatability evidence

Record a compatibility baseline before edits.
Compare both profiles across status, gates, readiness, denied mutation, and malformed calls.
Prove unchanged structured payloads and receipt identities with deterministic tests.
Measure representative host presentations, including a host that exposes only text.
A smaller wire response alone does not establish lower model token use.

## Out of Scope

- Changing the default response without a separately reviewed compatibility decision
- Removing full diagnostics, changing receipt hashes, or treating successful transport as lifecycle success
- Guessing client support from a name or protocol version
- A new serialization format or unrestricted tool dispatcher

Dependency: `measure-agent-lifecycle-token-cost`. Implementation remains open.
