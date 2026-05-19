# Design: Deterministic Surface Coverage Audit

## Context

Cairn's deterministic confidence is strongest when public receipt-producing commands are replayed by policy-owned cases and grouped into operator rails. Recent slices added replay verification, bundle verification, manifest verification, and manifest generation. The remaining gap is catalog completeness: a future deterministic surface could be omitted from replay policy.

## Data Model

Add `determinism_surface_inventory` to the generated policy. Each entry is a stable object:

- `id`: stable surface id.
- `command`: user-facing command label.
- `surface_kind`: `cli` or `mcp`.
- `receipt_command`: stable receipt command string when applicable.
- `required_replay_cases`: sorted replay case ids that cover the surface.
- `required_replay_groups`: sorted replay group ids that must include those cases.
- `determinism_claim`: short stable claim category such as `receipt_stability`, `read_only_replay`, or `publication_chain`.

The policy remains the source of truth. The CLI audit reads generated policy and verifies references against `determinism_replay_policy.cases` and `.groups`.

## CLI

Add:

```sh
cairn determinism coverage-audit --root .
```

The command emits a JSON receipt:

- `command = "determinism coverage-audit"`
- `root`
- `valid`
- `verdict`
- `issues`
- `current_replay_policy_hash`
- `surface_inventory_hash`
- `surface_count`
- `covered_surfaces`
- `missing_surfaces`
- `receipt_hash`

`valid=true` only when every inventory entry has existing required replay cases and groups and each required group contains each required case for that surface.

## Determinism and Safety

The audit is read-only, policy-only, and does not inspect host environment values. Failure receipts use stable issue codes and surface ids only; they do not echo raw roots, paths, credentials, or environment values.

## Tests

Add a focused CLI test that checks the real policy passes and exercises a pure helper against a synthetic broken policy fragment to prove deterministic missing-case/group failure behavior. The existing public schema test guards stable receipt fields.
