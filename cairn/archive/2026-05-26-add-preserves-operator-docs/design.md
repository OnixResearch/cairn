# Design: Preserves operator docs

## Document shape

Add `docs/preserves-receipts.md` as a concise operator guide covering:

- Preserves is internal/additive; JSON public/stored receipts and JSON `receipt_hash` remain the compatibility surface.
- `cairn agent-receipt preserves-hash <path>` is a read-only inspection command that emits `agent_receipt_preserves_hash` in its own deterministic receipt.
- `cairn preserves-corpus update --path fixtures/preserves-canonicalization/corpus.json` is dry-run by default and fails closed on stale hashes.
- `--execute` is the explicit reviewed mutation rail for intentional Preserves/serde drift.
- Nix checks `preserves-corpus-fresh` and `preserves-corpus-stale-detects-drift` make dependency-bump drift visible in CI.

## Links

- Add a short section/link from `docs/agent-run-receipts.md` near replay/manual flow.
- Add a README command/example pointer so maintainers can discover the document.

## Non-goals

- No public/stored receipt schema changes.
- No new CLI behavior.
- No dependency bump.
- No automatic corpus mutation in CI.
