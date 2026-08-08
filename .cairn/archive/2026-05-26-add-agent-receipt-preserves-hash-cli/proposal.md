# Add agent receipt Preserves hash CLI

## Why

Cairn has an internal Preserves packed-byte/hash seam and a golden agent-run receipt consumer, but operators and agents can only exercise it through Rust tests. A read-only CLI command gives a concrete inspection surface for receipt files while preserving existing public JSON receipt contracts.

## What Changes

- Add `cairn agent-receipt preserves-hash <path>`.
- Read the same bounded JSON receipt files as `agent-receipt validate`.
- Emit a deterministic JSON receipt containing the internal Preserves hash, input path, input hash, bounded read limit, validity, verdict, issues, and receipt hash.
- Keep existing agent-run `receipt_hash` validation and public/stored receipt schemas unchanged.

## Non-Goals

- No mutation of receipt files.
- No replacement of JSON `receipt_hash`.
- No MCP exposure in this slice.
- No new signing, publication, or remote evidence behavior.
