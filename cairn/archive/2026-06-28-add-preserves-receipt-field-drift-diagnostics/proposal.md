# Proposal: Preserves receipt field drift diagnostics

## Why

Cairn's accepted `format_strategy.preserves_candidate` gate
(`cairn.structured_lifecycle.format_strategy.preserves_candidate`) requires a
"diagnostic mapping to source IDs" before Preserves can be promoted to an
authoritative receipt/lifecycle encoding. Today Cairn has:

- `cairn agent-receipt preserves-hash <path>` — emits a single aggregate
  Preserves hash for one receipt, with no field-level breakdown.
- `cairn preserves-corpus explain` — maps drift to corpus *case IDs* and hash
  *shape*, not to receipt *field paths* or *source requirement IDs*.

So when a stored receipt's Preserves hash changes between two snapshots,
operators must diff JSON by hand to find the responsible field. That is
error-prone, unreviewed, and is the one unmet clause of the promotion gate.

## What Changes

- Add a pure-core helper that computes a deterministic structural diff between
  two receipt-shaped JSON values, returning sorted, stable entries keyed by
  RFC 6901 JSON Pointer field paths with a drift class
  (`added`, `removed`, `changed`, `type_mismatch`).
- Map each differing field path to a source requirement ID through a
  policy-owned path→source-id table loaded from generated policy JSON, so the
  mapping is reviewed rather than a hidden literal. Unmapped paths are reported
  with `source_id: null` (still actionable).
- Add a read-only CLI `cairn receipt-preserves-diff <baseline.json>
  <candidate.json>` that emits both input hashes, both Preserves hashes, the
  sorted drift list with source IDs, policy/policy hashes, and its own
  `receipt_hash`. It fails closed on malformed inputs and never mutates.
- Add positive, negative, malformed, and boundary fixtures and Tracey refs.
- Add a determinism replay case and a policy-inventory entry, following the
  ROI-03 precedent for operator-facing deterministic rails.

## Impact

- **Files**: `crates/cairn-core/src/verified/` (new drift helper + tests),
  `crates/cairn-cli/src/` (new read-only command + tests), a generated-policy
  path→source-id table under `cairn-policy/`, fixtures, `docs/preserves-receipts.md`,
  and the determinism replay policy inventory.
- **Testing**: focused CLI tests for identical/drift/malformed/boundary inputs,
  Cairn `validate` + proposal/design/tasks gates, `preserves-corpus-fresh`,
  Slotcar purity smoke, and a determinism replay case.
- **Compatibility**: additive and read-only. No stored receipt schema, public
  `receipt_hash`, evidence bundle, or verifier contract changes.

## Non-Goals

- Do not modify stored receipt schemas or the public JSON `receipt_hash` field.
- Do not promote Preserves to an authoritative content hash (separate change).
- Do not mutate receipts, call networks/clocks/processes, or run `--execute`.
