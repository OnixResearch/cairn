# Design: CI replay consumability

## CLI contract

Extend `cairn determinism replay` with:

- `--case <id>`: run only the matching policy-owned replay case. Repeated use is not required for this slice; if multiple case selection is needed later it can be added without changing the receipt shape.
- `--format json|jsonl|summary`: choose stdout format. Default remains `json` for compatibility.
- `--out <path>`: write the full JSON replay receipt to a path while stdout uses the requested format.

## Receipt behavior

The full JSON receipt remains the canonical artifact and keeps the stable fields from the replay oracle. This change adds stable CI-facing fields:

- `selected_cases`: sorted selected case IDs.
- `available_cases`: sorted policy case IDs.
- `summary`: `{ total_cases, passed_cases, failed_cases, skipped, mismatch_count, nondeterminism_failure_count }`.
- mismatch entries include a stable `reproduce` command string.

Unknown case IDs produce a deterministic failure receipt instead of an ad hoc error string. The process exits nonzero for invalid replay receipts, unknown cases, hidden entropy, and mismatches.

## Output modes

- `json`: pretty/full receipt to stdout, unless `--out` is used with another format.
- `summary`: deterministic compact lines suitable for CI logs.
- `jsonl`: one sorted JSON object per event: summary, selected cases, failures, mismatches, and artifact path when present.

`--out` writes the full receipt before stdout rendering. Parent directories are created by the CLI shell.

## Implementation notes

Reuse the policy-owned replay plan and replay case executor. Keep parsing and rendering in the CLI shell. Avoid adding filesystem or formatting behavior to `cairn-core`.
