# Add incremental agent observations

## Why

Repeated context, readiness, and active-change requests often contain unchanged data.
Cairn already has BLAKE3 identities, but the reviewed interfaces do not provide a shared source-bound continuation contract.
An agent must retain large prior responses or read the same content again.

## What Changes

- Add identity-bound pages for active-change listings and the selected readiness or context views.
- Add an unchanged response and a deterministic difference against an explicitly supplied prior observation.
- Bind continuations to source, policy, evidence, tool, view, selectors, and ordering.
- Report added, changed, and removed entries with explicit completeness and omission metadata.
- Keep prior observations as read-only inputs rather than implicit session memory.

## Impact

Current consumers: long agent sessions and resumed Cairn diagnostic loops.
Immediate outcome: the next response contains only the requested page or changed observations.
Durable contribution: a bounded observation protocol that survives session restart without weaker freshness checks.
Maintenance owner: Cairn lifecycle-core and CLI/MCP maintainers.

Reuse the compact readiness and exact context selector contracts from the prerequisite changes.
Reuse existing BLAKE3 framing and bounded source observation. Do not introduce a product-neutral cache dependency without matching requirements.
The core computes identities, pages, and differences. Shells read explicit snapshots and retain any cache as disposable storage.

## Repeatability evidence

Reconstruct the full ordered observation from pages and compare it with an unpaged result.
Replay unchanged, changed, removed, and renamed entries after a fresh process start.
Reject stale cursors, changed policies, missing snapshots, mixed roots, unsafe snapshot paths, and forged completeness claims.
Measure repeated and resumed diagnostic loops with declared cache states.

## Out of Scope

- Skipping fresh mutation preflight or treating cached state as authorization
- Hidden durable session state, network snapshot storage, or automatic retention and deletion
- Atomic filesystem snapshot claims from repeated bounded reads
- Secret-bearing cursors or content hashes as authentication

Dependencies: `add-targeted-lifecycle-context-reads` and `add-compact-agent-readiness`. Implementation remains open.
