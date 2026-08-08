# Design: Radicle Change COB

## Context

Radicle COBs are signed Git-backed operation DAGs evaluated by deterministic reducers. They are not synchronized directory snapshots. Cairn therefore needs an explicit operation schema and merge model rather than serializing the existing `cairn.structured-lifecycle.bundle.v1` value. That bundle omits proposal and design text, task descriptions and states, dependency metadata, and other bytes required to reconstruct an active change.

The implementation must preserve Cairn's functional-core / imperative-shell boundary. `cairn-core` owns the lifecycle state, validation, merge rules, hashes, and materialization plan. `cairn-radicle` owns Radicle types, identity-document reads, signed operation metadata, and trait integration.

## Decisions

### Decision: use one COB per active change

**Choice:** Define `xyz.onixresearch.cairn-change` as the stable Radicle type name. Each object represents one active change and records its repository identity, base Git object, creator, policy identity, artifacts, evidence, and lifecycle assertions.

**Rationale:** The active change is Cairn's collaboration, gate, sync, and archive unit. Per-change histories avoid coupling unrelated work and let accepted specs and source remain canonical Git content.

### Decision: preserve lifecycle files as bounded typed artifacts

**Choice:** Model proposal, design, tasks, metadata, and keyed delta specs as validated artifact identifiers with exact UTF-8 text values. The root action records the initial artifact set; later actions set or delete one artifact.

**Rationale:** Exact text preserves current Cairn behavior and comments while a future schema can add finer task-level operations. Treating the current structured lifecycle bundle as lossless would discard reviewed lifecycle information.

### Decision: make conflicts explicit through revision bases

**Choice:** Every artifact update names the revision IDs it supersedes. Artifact heads are derived from all revisions minus referenced bases. Disjoint artifact edits merge. Concurrent or stale edits to the same artifact remain multiple heads and block materialization until a later update names every conflicting head as its basis.

**Rationale:** This rule is independent of Radicle timestamp ordering and does not grant last-writer-wins authority to actor-controlled timestamps. It is deterministic under replay and operation arrival order.

### Decision: separate semantic identity from Radicle storage identity

**Choice:** Radicle object and entry IDs remain Git OIDs required by the protocol. Cairn computes a BLAKE3 content root over the conflict-free semantic change content and a separate BLAKE3 state hash over the complete reducer state.

**Rationale:** The dual identity preserves Radicle interoperability while retaining Cairn's stack-owned BLAKE3 content and receipt identity.

### Decision: make canonical JSON independent of dependency features

**Choice:** Canonical JSON recursively sorts object keys before serialization instead of relying on `serde_json::Map`'s compile-time backing map.

**Rationale:** Radicle enables `serde_json/preserve_order`. Cargo feature unification otherwise changes Cairn BLAKE3 identities only when the adapter and CLI build together, even though the semantic values are equal. Explicit sorting preserves existing fixture hashes under either feature set.

### Decision: exclude Cargo build output from cross-root replay copies

**Choice:** Determinism replay copies skip directories named `target` while retaining source-controlled lifecycle, fixture, evidence, and Git inputs.

**Rationale:** Building the Radicle adapter makes Cargo output large enough that copying `target` once per replay root exhausts temporary storage. Cargo output is derived state rather than a deterministic command input; excluding it preserves the replay claim while preventing resource amplification.

### Decision: keep evidence and lifecycle claims bounded

**Choice:** Evidence records are append-only, BLAKE3-keyed declarations. Lifecycle transitions name the current content root and their evidence hashes. Content edits automatically make older transitions stale. Creators or repository delegates may edit content and request review; delegate authority is required for approval or archive assertions.

**Rationale:** Signed history establishes operation authorship and reducer acceptance only. It does not establish gate validity, evidence truth, source correctness, archive execution, or release readiness.

### Decision: make protocol limits consensus constants

**Choice:** The wire schema uses named upper-bound constants for actions, artifacts, text bytes, evidence records, and identifiers. Local Nickel policy may impose stricter shell admission later but cannot relax reducer limits.

**Rationale:** Reducer limits must be identical at every peer for convergence and cannot depend on ambient mutable policy. Named schema limits avoid hidden magic numbers while preserving deterministic evaluation.

## Functional Core / Imperative Shell

```text
cairn-core::verified::radicle_change
  validate root and operations
  derive revision heads and conflicts
  compute BLAKE3 content/state identities
  authorize from explicit authority facts
  plan native lifecycle file materialization

cairn-radicle
  decode Radicle operations
  load operation identity documents
  derive creator/delegate authority facts
  implement Cob, CobWithType, and Evaluate
```

The adapter returns errors for malformed or unauthorized operations so Radicle prunes those operation branches. It does not read other change objects. Existing aggregate dependency graph validation remains responsible for unknown dependencies, cycles, and archive ordering after all active change states are materialized.

## Risks / Trade-offs

- Artifact-level conflicts are coarser than task-level CRDT edits, but they are lossless and fail visibly instead of silently choosing a writer.
- Radicle's generic typed decoder deserializes JSON before the custom reducer sees an action; schema bounds limit accepted state but do not provide a streaming pre-allocation bound.
- Historical identity documents authorize historical operations. A future execution shell must re-check current delegate authority before mutating Git or publishing archive claims.
- This slice creates the reusable COB type but does not expose operator CLI commands or claim end-to-end peer synchronization.

## Verification

- Replay disjoint and conflicting edits in different valid orders and compare exact state hashes.
- Verify conflict resolution, stale basis rejection, unauthorized mutation rejection, stale evidence, stale lifecycle transitions, and protocol limits.
- Verify exact native path/content materialization and rejection while conflicts exist.
- Compile the native Radicle trait adapter and verify action serialization plus type identity.
- Run Cairn validation, proposal/design/tasks gates, focused Cargo tests, formatting, Clippy, and purity checks.
