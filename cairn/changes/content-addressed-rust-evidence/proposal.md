## Why

Octet already supports selective Rust function fingerprints, but Valence-style downstream consumers need a stricter lifecycle contract before those fingerprints can become portable evidence objects. Cairn should define the receipt and replay requirements that make Octet-emitted Rust definition identities usable by Trellis admission models, Crunch/Mantle proof rows, and Aspen CI/dogfood receipts without overstating them as correctness proofs.

## What Changes

- Define a native Cairn specification for content-addressed Rust evidence objects emitted by Octet.
- Require canonical object hashing, explicit caveats, name-pointer movement receipts, export bundles, and replayable validation evidence.
- Establish compiler-IR evidence boundaries for future HIR/MIR summaries while preserving source-token objects as the first supported level.
- Add downstream interop requirements for bounded Crunch/Mantle and Aspen pilots.

## Non-Goals

- Implement Octet extraction, HIR/MIR analysis, or object storage in Cairn.
- Claim semantic equivalence, behavioral correctness, or compiler-version-independent identity from matching hashes.
- Replace Cargo, Git, Crunch/Mantle stores, Aspen CI, or Trellis verification.

## Impact

- **Files**: Adds native Cairn change artifacts under `cairn/changes/content-addressed-rust-evidence/` and later accepted specs under `cairn/specs/content-addressed-rust-evidence/`.
- **APIs**: Future Cairn receipt validators and replay policies may consume Octet bundle receipts, but this change first defines the lifecycle contract.
- **Testing**: `cairn validate --root .`, proposal/design/tasks gates, Tracey coverage after implementation, and later deterministic fixtures for object canonicalization, pointer movement, and downstream pilot receipts.
