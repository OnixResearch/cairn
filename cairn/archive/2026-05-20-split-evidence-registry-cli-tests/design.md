## Context

Recent Cairn work added a complete portable evidence-chain flow: export, verify, import, republish, index, registry build/verify, mirror verification, selection, fetch planning, and local fetch execution. The implementation is covered, but the primary proof lives inside the catch-all `crates/cairn-cli/tests/cli.rs` file.

## Goals

- Isolate evidence-registry chain coverage in a named integration-test target.
- Make future registry/evidence changes easier to run and review with focused cargo filters.
- Reduce `cli.rs` size without changing behavior or weakening assertions.

## Decisions

### 1. Use a dedicated integration test file

**Choice:** Create `crates/cairn-cli/tests/evidence_registry.rs` for the moved chain.

**Rationale:** Cargo integration tests are already split by feature family (`determinism.rs`, `content_addressed_evidence.rs`). A dedicated file gives a stable focused target: `cargo test -p cairn-cli --test evidence_registry`.

**Alternative rejected:** Extract a shared test-support crate first. That is broader than this behavior-preserving split and would mix helper architecture with test movement.

### 2. Duplicate only minimal local helpers

**Choice:** Copy the small helper functions needed by the moved test into the new file when they are not already available as public test support.

**Rationale:** Local helper duplication keeps the patch reviewable and avoids broad helper churn.

### 3. Preserve the end-to-end assertion set

**Choice:** Move the existing evidence-chain assertions intact, including positive and negative checks for mirror verification, selection, fetch-plan, fetch-execute, safe paths, missing artifacts, and hash mismatch behavior.

**Rationale:** The split must be behavior-preserving; this is a test topology change, not a semantic change.

## Risks / Trade-offs

- **Helper drift:** Duplicated helpers can drift later. Mitigation: keep helpers minimal and local to this test family.
- **Missed moved assertion:** A mechanical move can accidentally drop a negative case. Mitigation: compare focused test behavior before/after and run the full workspace test suite.
