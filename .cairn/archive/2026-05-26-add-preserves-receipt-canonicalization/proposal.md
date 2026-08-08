# Proposal: Preserves receipt canonicalization spike

## Summary

Add a narrow Preserves-backed canonical byte/hash helper for Cairn receipt material so Cairn can evaluate Preserves as an internal deterministic artifact encoding without replacing existing JSON receipt surfaces.

## Motivation

Cairn currently hashes canonical JSON for many receipt contracts. JSON remains the public and human-reviewable surface, but it conflates some data-model choices and does not exercise Preserves' typed canonical machine encoding. A small pure-core seam lets us test Preserves on receipt-shaped values before committing to broader schema or CLI changes.

## Scope

- Add a pure core helper that converts receipt JSON values into Preserves values and emits packed canonical bytes.
- Add a BLAKE3 hash helper over those Preserves bytes.
- Add focused tests proving stable object-order canonicalization and content-sensitive hashing.
- Keep existing JSON receipt schemas, CLI outputs, Nickel policy, and public APIs unchanged.

## Non-Goals

- Replacing JSON output or stored fixtures with Preserves in this slice.
- Adding Preserves Schema generation or `.prs` source files.
- Introducing binary CLI artifact formats, signing, transport, or registry migration.
