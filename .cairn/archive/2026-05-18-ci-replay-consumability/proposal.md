# ci-replay-consumability

## Why

`cairn determinism replay` is now broad and policy-owned, but CI and agent operators still need targeted failure workflows. A single replay mismatch should be easy to isolate, summarize in CI logs, and attach as a deterministic artifact without forcing humans to inspect a large JSON receipt.

## What changes

- Add deterministic replay case selection with `--case <id>`.
- Add bounded machine-readable failure receipts for unknown case IDs and replay mismatches.
- Add CI-friendly replay output modes: full JSON, JSONL case events, and compact summary.
- Add `--out <path>` so CI can write the full replay receipt as an artifact while stdout remains concise.
- Add stable mismatch ergonomics: failing case IDs, mismatch class, field path, expected/actual hashes, and reproduction command.

## Impact

Agents and CI can run one policy replay case, get short logs, store full deterministic evidence, and deterministically fail builds when replay evidence fails.
