# Strengthen Nickel Contract Ownership

## Why

Cairn already has a Nickel-authored policy boundary, but several public contracts are still enforced only by Rust literals, README prose, or CLI regression shape checks. This makes policy drift harder to catch before generated runtime JSON is accepted and leaves stable receipt schemas duplicated outside the policy source of truth.

## What Changes

- Add Nickel-authored contracts for stable public receipt schemas: gate, sync, archive, migration, lifecycle actions, migration actions, and BLAKE3 hash fields. r[cairn.supersede.nickel_contracts.receipt_schemas]
- Strengthen policy contracts from broad shape checks to semantic invariants for supported version, BLAKE3 algorithm/canonicalization, non-empty policy arrays, and duplicate-free vocabularies. r[cairn.supersede.nickel_contracts.semantic_invariants]
- Move task marker vocabulary and lifecycle artifact dependencies into generated policy data consumed by Rust rather than duplicating marker/artifact literals in core logic. r[cairn.supersede.nickel_contracts.generated_vocabularies]
- Parse and validate gate exemption marker structure against Nickel-owned exemption key/date contracts before accepting an exemption as policy-satisfying. r[cairn.supersede.nickel_contracts.exemption_markers]
- Add deterministic export/check rails and invalid fixtures proving malformed policy, stale generated policy, and invalid receipt examples fail before merge. r[cairn.supersede.nickel_contracts.contract_rails]

## Impact

Runtime remains Nickel-free: the CLI reads checked-in generated policy JSON, and `cairn-core` evaluates caller-supplied snapshots against parsed policy data. The main behavior change is earlier rejection of malformed policy/receipt/schema data and less duplicated policy vocabulary in Rust.
