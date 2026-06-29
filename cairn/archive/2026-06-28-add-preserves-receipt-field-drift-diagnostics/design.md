# Design: Preserves receipt field drift diagnostics

## Context

The Preserves canonicalization seam lives in
`crates/cairn-core/src/verified/evidence.rs`:
`canonical_preserves_bytes`/`canonical_preserves_hash` convert a
`serde_json::Value` to a packed Preserves value and BLAKE3-hash it. The CLI
already exposes `agent-receipt preserves-hash` and `preserves-corpus
update|explain`. None of these map a hash delta back to receipt field paths or
source requirement IDs, which is the unmet clause of the accepted
`cairn.structured_lifecycle.format_strategy.preserves_candidate` requirement.

## Decisions

### 1. Functional core, imperative shell

**Choice:** The drift computation is a pure function in the verified core;
the CLI is a thin shell that reads two files, loads the policy source map,
prints JSON, and exits.

**Rationale:** Keeps the Slotcar purity boundary intact (no fs/env/process in
core) and makes the diff unit-testable with plain `serde_json::Value` inputs.

### 2. JSON Pointer field paths

**Choice:** Differing leaves are addressed by RFC 6901 JSON Pointer strings
(`/replay_evidence/replay_receipt_hash`).

**Rationale:** Standard, stable, and reviewable. Sorting pointers lexically
gives byte-stable drift ordering across runs and machines.

### 3. Source-ID mapping from reviewed policy

**Choice:** A path→source-id table is authored in Nickel policy and exported
into generated policy JSON. The core helper receives the map as a parameter;
the CLI loads it from the generated bundle.

**Rationale:** Satisfies the gate's "mapping to source IDs" clause without
hardcoding requirement IDs in Rust literals. Unmapped paths are reported with
`source_id: null` so unmapped drift is still visible.

### 4. Ordering-insensitive, byte-aware

**Choice:** The diff walks objects key-by-key (so key reordering is not a
drift) and emits a top-level `canonical_bytes_differ` boolean derived from the
two Preserves hashes. Leaf comparison uses exact equality; floats are compared
by value with no epsilon.

**Rationale:** The authoritative identity comparison is the canonical Preserves
byte hash; the field diff is the human/CI diagnostic. Reporting both keeps the
diff field-level while acknowledging that two semantically-equal-but-reordered
objects are not drift.

### 5. Read-only, fail closed

**Choice:** The command never mutates inputs. Malformed JSON, missing files,
or non-object roots produce deterministic issue codes and a nonzero exit.

**Rationale:** Matches the `preserves-corpus explain` and `preserves-hash`
precedent.

## Risks / Trade-offs

- The source-map table is only as good as its coverage; unmapped paths still
  report drift with `source_id: null`. Acceptable: the gate requires a mapping
  exists, not that every path is mapped.
- Diffing large receipts has linear cost; bound input reads with the existing
  bounded-read limit used by `preserves-hash`.
- The diff reports raw leaf values. Cairn receipts are not secret-bearing by
  contract, but the docs will warn against pointing the command at
  secret-bearing fixtures.
