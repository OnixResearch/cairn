# Design: Stack provenance release gates

## Functional core / imperative shell boundary

`cairn-core` owns pure validation of stack-provenance readiness inputs:
Valence verification receipt metadata, Mantle bundle linkage metadata, current
policy hash, artifact hashes, claim scope, and non-claim boundaries. The CLI
shell owns file reads, optional report loading, stdout/stderr, and JSON
presentation.

## Policy shape

A policy-owned release-readiness setting declares stack provenance as `disabled`,
`optional`, or `required`. Required mode fails readiness when the Valence receipt
or Mantle bundle linkage is missing, stale, malformed, or overclaiming. Optional
mode records present/absent disposition without making the whole readiness result
fail only because stack provenance is absent.

## Validation inputs

The pure core receives a normalized in-memory input containing:

- Valence stack-provenance verification receipt hash and verdict;
- Valence policy/schema/hash metadata;
- Mantle sidecar role, schema, claim scope, digest, and binary identity linkage;
- artifact hash list and expected BLAKE3 values;
- verification role summary if available;
- bounded non-claim text.

The core returns a deterministic readiness contribution and diagnostics.

## Graph summary

Cairn may include a compact graph summary copied from a Valence report: artifact
count, edge count, manual-review node count, unresolved-link count, and failed
link count. Cairn must not recompute stack semantics from the graph. The summary
is display and gate context over Valence-verified evidence.

## Failure model

Negative cases fail closed in required mode for missing receipt, stale policy,
digest mismatch, unsupported claim scope, missing Mantle linkage, manual-review
artifacts counted as verified, weakened non-claims, malformed graph summary, and
failed Valence verdict.

## What is explicitly not provided

- No Valence semantic verifier implementation in Cairn.
- No source-code correctness proof.
- No Mantle build correctness proof beyond cited Mantle receipts.
- No Trellis or Octet proof rerun; Cairn consumes receipts only.
