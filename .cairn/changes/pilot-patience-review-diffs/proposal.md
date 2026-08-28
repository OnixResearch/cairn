## Why

Cairn renders lifecycle plans, changes, diagnostics, and review material. Reviewers need stable diffs that make meaningful Markdown changes easy to find.

The current workflow relies on normal text diffs. It has no measured patience or word-refinement candidate for lifecycle review output.

Jane Street provides two MIT-licensed references:

- `patdiff` at `db7c0c244f9c7e88aa6d91ecf68201ab1c1889b0`
- `patience_diff` at `773eba0f4264f89c6e1c88fe32d75128159b2a02`

Patdiff also supports numeric-tolerance display. Cairn must reject that behavior because exact numbers can change policy and evidence meaning.

Cairn needs a bounded pilot before it changes any default review output.

## What Changes

- Add a deterministic lifecycle review-diff corpus.
- Compare the current baseline, patience line matching, and optional word refinement.
- Keep source bytes and exact changes visible in every candidate.
- Add typed Nickel policy for bounds, required fixtures, structural metrics, and owner review.
- Reject numeric tolerance, hidden whitespace, lossy normalization, binary input, and unbounded output.
- Record accept-candidate, reject-candidate, or insufficient-evidence.
- Require a separate Cairn before any default CLI or review output changes.

## Impact

The pilot adds test-only comparison logic, fixtures, metrics, review records, and documentation. It does not change sync plans, hashes, receipts, or default rendering.

## Dependencies

The pilot uses current Cairn parsers and lifecycle-view inputs. It must not use a mutable external command at runtime.

## Non-goals

- Do not change default CLI output.
- Do not use approximate numeric equality.
- Do not alter lifecycle semantics, sync plans, or receipt hashes.
- Do not hide exact bytes, whitespace changes, or moved requirements.
- Do not treat reviewer preference as proof of correctness.
- Do not add OCaml to the Cairn runtime.
