## Context

Gate, sync, archive, and migration commands already emit canonical JSON receipts with BLAKE3 provenance fields. The existing reproducibility test proves repeated runs produce identical JSON and valid hashes, but it does not state which fields are part of the supported public contract.

## Decisions

### 1. Lock the public top-level schema in CLI tests

**Choice:** Add a fixture-backed CLI regression that asserts exact top-level key sets for gate, sync, archive, and migration receipts, plus action key sets for lifecycle and migration action arrays.

**Rationale:** The CLI is the public shell boundary. Exact key-set assertions catch accidental removals/renames without introducing runtime schema dependencies or moving JSON presentation into the pure core.

**Alternative:** Generate JSON Schema files and validate receipts against them at runtime. Rejected for this slice because it adds dependency and maintenance surface before the contract stabilizes further.

### 2. Keep schema stability separate from receipt reproducibility

**Choice:** Preserve the existing byte-for-byte reproducibility test and add a focused schema-shape test rather than overloading the reproducibility rail.

**Rationale:** Reproducibility proves determinism; schema assertions prove compatibility. Keeping them separate makes failures easier to diagnose.

## Risks / Trade-offs

- Exact key-set assertions intentionally make public receipt additions a test update. That is acceptable because new public fields should be deliberate and documented.
- This slice locks top-level receipt/action fields, not every nested issue or reason object. Deeper schemas can be added later if downstream consumers need them.
