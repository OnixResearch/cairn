# Proposal: Backend-Agnostic Change Collaboration

## Why

Cairn's active-change reducer, import planner, checkout planner, and public operator workflow currently use Radicle and Git terminology as semantic identities. That makes Radicle more than a transport adapter: even an offline or different collaboration backend must pretend to have a Radicle repository and Git object anchor. Cairn needs one transport-neutral collaboration contract so Radicle remains supported without being mandatory.

## What Changes

- Introduce a transport-neutral collaborative-change schema with explicit workspace and typed base-revision identities.
- Preserve existing `cairn.radicle-change-cob.v1` objects and their BLAKE3 identities through an explicit legacy compatibility path.
- Expose `cairn collaboration import|status|checkout` with a selected backend rather than a transport-named top-level workflow.
- Add a capability-bounded `portable-json.v1` backend that provides preview-first import, read-only status, and preview-first exact checkout without Radicle context.
- Keep Radicle as a built-in adapter and retain `cairn radicle ...` as a compatibility command.
- Bind receipts to backend identity, semantic state, exact artifacts, blockers, execution mode, and explicit authority/non-claim observations.

## Impact

The pure reducer and planners gain neutral semantic names and versioned compatibility logic. The CLI gains a backend dispatcher and portable store shell. The Radicle adapter maps current identity and signed operation facts into the same reducer while preserving legacy wire behavior. Accepted specs, implementation source, and executed archives remain canonical Git content.
