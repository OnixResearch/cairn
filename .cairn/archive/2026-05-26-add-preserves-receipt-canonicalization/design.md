# Design: Preserves receipt canonicalization spike

## Context

`crates/cairn-core/src/verified/evidence.rs` owns deterministic receipt hashing helpers. Existing validators depend on JSON-compatible values and public JSON fixtures. The first Preserves adoption should therefore sit beside `canonical_json_hash` and remain internal/pure.

## Decisions

### 1. Add an additive helper beside JSON canonicalization

**Choice:** Add `canonical_preserves_bytes(&serde_json::Value) -> Result<Vec<u8>, String>` and `canonical_preserves_hash(&serde_json::Value) -> Result<String, String>`.

**Rationale:** This gives receipt code a deterministic binary hash seam while preserving the existing JSON contract and avoiding broad validator churn.

**Alternative rejected:** Replace existing receipt hashes immediately. That would invalidate fixtures and consumer expectations before Preserves proves useful.

### 2. Use Preserves packed encoding through Serde

**Choice:** Convert `serde_json::Value` through Preserves' Serde value support and encode the resulting IO value with the packed writer.

**Rationale:** The spike tests the real Preserves data model and canonical machine syntax while reusing Cairn's existing receipt-shaped values.

### 3. Keep schema work deferred

**Choice:** Do not add `.prs` schemas in this slice.

**Rationale:** Schema/codegen is valuable only after the byte/hash seam proves deterministic and ergonomic.

## Validation Plan

- Unit tests cover positive canonicalization for differently ordered JSON objects.
- Unit tests cover negative/content-sensitive hash changes.
- Run focused `cargo test -p cairn-core evidence` plus Cairn validate/gates.
