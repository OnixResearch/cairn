# ROI 04: Preserves corpus explain CLI

## Why

Operators can check and update the Preserves corpus, but they lack a read-only structured explanation of case ids, values, expected hashes, and review rationale. An explain command would improve drift reviews without changing hash semantics.

## What changes

- Add `cairn preserves-corpus explain --path <corpus.json>`.
- Emit deterministic JSON describing corpus cases, expected hash fields, duplicate/malformed case issues, and non-secret review guidance.
- Keep the command read-only.

## Non-goals

- Do not recompute or rewrite hashes in explain mode.
- Do not replace the update rail.
- Do not expose secret-bearing fixture values.
