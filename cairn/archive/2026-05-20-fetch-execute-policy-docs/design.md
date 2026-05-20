## Context

The registry consume rail already supports deterministic registry selection and metadata-only fetch plans. The newest slice added `evidence registry fetch-execute`, which consumes a verified fetch-plan receipt, materializes artifacts from a local/offline source directory, verifies BLAKE3 hashes before and after writing, rejects unsafe paths, and emits deterministic success/failure receipts. The implementation exists, but policy inventory and docs lag that public command surface.

## Decisions

### 1. Treat fetch execution as a policy-inventoried receipt surface

**Choice:** Add `evidence registry fetch-execute` to `receipt_schema_catalog` and `determinism_surface_inventory`.

**Rationale:** Operators and downstream agents should be able to audit the command's stable receipt fields and see that it is an intentional deterministic surface, not an undocumented side path.

**Alternative:** Leave the command covered only by integration tests. Rejected because receipt consumers use generated policy/docs to understand compatibility surfaces.

### 2. Document local/offline execution boundaries, not network fetching

**Choice:** Describe fetch-execute as local materialization from an already mirrored/source directory, with safe relative paths and BLAKE3 verification.

**Rationale:** This preserves the product boundary: Cairn proves local artifact materialization and hash validation; it does not perform network transport, credential handling, or remote trust decisions.

**Alternative:** Describe it as a generic download command. Rejected because that overclaims behavior and could mislead wrappers into expecting network I/O.

### 3. Add a schema assertion to the existing registry chain test

**Choice:** Extend the current positive fetch-execute integration test to assert its top-level receipt keys against policy.

**Rationale:** The test already produces a valid fetch-execute receipt and has the necessary fixture chain; adding schema coverage there prevents drift without creating another large fixture path.

## Risks / Trade-offs

- **Policy hash churn:** Generated policy hashes and dependent goldens may change. Mitigate by running policy export/check and full workspace tests; update only intentional generated artifacts.
- **Docs overclaim:** Keep docs explicit that fetch-execute is local/offline and never handles credentials or network transport.
- **Task-gate references:** Keep tasks tied only to requirement IDs declared by this change delta.
