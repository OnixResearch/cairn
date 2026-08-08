# Evidence: downstream trigger for review metrics regression rail

This evidence note records the safe downstream signal that motivated the generic Cairn rail.

## Source

- Downstream repository: `/home/brittonr/git/clankers`
- Downstream package: `cairn/changes/harden-review-metrics-regression-rail`
- Downstream evidence kind: sanitized review metrics snapshot

## Safe repeated finding classes

The downstream evidence identified repeated review findings by category/stage/count only. Representative high-count classes included:

- `omission|tasks|auto-fix`
- `omission|tasks|deterministic-check`
- `omission|tasks|prompt`
- `incoherence|design|conflict`
- `incoherence|proposal|conflict`

## Boundary

This Cairn package does not import downstream private transcripts, hidden prompts, provider payloads, account identifiers, tokens, credentials, connection strings, or raw review logs.

## Design consequence

The generic Cairn rail should validate synchronization invariants for declared categories. Downstream projects remain responsible for their project-specific diagnostic vocabulary and fixture contents.
