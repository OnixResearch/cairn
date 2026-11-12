# Add advisory prose checks

## Why

Authors need local grammar feedback on specifications, proposals, and public documentation without promoting grammar observations to lifecycle authority.
The requested english-rs integration provides reusable lint semantics and precise diagnostics, not requirement verification.

## What Changes

- Add the opt-in `cairn prose check [--root DIR] FILE...` command and machine-envelope support.
- Check explicit, bounded, root-relative UTF-8 files with the pinned upstream englishc library.
- Emit structured advisory diagnostics, byte spans, Unicode columns, and review-only suggestions.
- Never rewrite text or change validation, readiness, gate, or completion verdicts.
- Pin the upstream Cargo source and Nix content hash, preserving the MIT license boundary.
- Remove obsolete post-release wording/source-text tests exposed by full verification; preserve only declared document relationships, not verification claims.

## Impact

Cairn CLI maintainers own this authoring integration; english-rs owns grammar rules.
The immediate consumer is the specification and public-document authoring loop.
The durable capability is one repeatable, offline, read-only report with explicit non-claims.
Consumer regression cases cover advisory severity, Markdown exclusions, source locations, machine transport, non-mutation, and input confinement.
No policy schema, mandatory gate, EARS semantics, or evidence identity changes.
