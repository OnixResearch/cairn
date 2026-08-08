# Design: Preserves corpus update rail

## Command

Add:

```text
cairn preserves-corpus update --path <corpus.json> [--execute]
```

Default mode is dry-run. The command reads the corpus, recomputes each case's `expected_hash` from `canonical_preserves_hash(value)`, and emits a deterministic receipt. With `--execute`, it rewrites the corpus file only when at least one hash differs.

## Receipt shape

The receipt reports:

- `command`: `preserves-corpus update`
- `path`: display path provided by the operator
- `valid`: false for malformed corpus or hash computation failures
- `execute`: whether mutation was requested
- `mutated`: true only when `--execute` rewrote stale hashes
- `stale_count`
- `case_count`
- `cases`: sorted/in-file-order case summaries with `id`, `expected_hash`, `computed_hash`, and `stale`
- `issues`: deterministic issue objects for malformed input

## Mutation boundary

The command is an imperative-shell rail in `cairn-cli`; hashing remains owned by `cairn-core::canonical_preserves_hash`. Dry-run never writes. Execute preserves all non-hash fields and rewrites pretty JSON with one trailing newline.

## Validation

Focused integration tests use a temp copy of the checked-in corpus, deliberately corrupt one expected hash, verify dry-run detects stale state without mutation, then verify `--execute` repairs the hash and a subsequent dry-run reports no stale cases. A negative test rejects a case with no `value`.
