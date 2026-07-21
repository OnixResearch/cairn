# Design: Preview-First Collaboration Update

## Context

The backend-neutral state already records exact artifacts as revision DAGs, and `ChangeCollaborationAction::SetArtifact` requires explicit superseded revision IDs. The CLI currently exposes only import, status, and checkout. Radicle's typed store has exercised signed updates in adapter tests, while the portable backend stores a validated state envelope that is currently create-only.

## Decisions

### Decision: derive updates in the pure neutral core

**Choice:** Compare a bounded local artifact snapshot with a validated conflict-free neutral state. For every changed, added, or removed artifact, emit one sorted `SetArtifact` action whose basis is the complete current head set. Bind the plan to current state, policy, actor, exact desired artifacts, and a deterministic BLAKE3 operation identity.

**Rationale:** Both providers receive the same semantic intent and conflict policy. A test can exercise update selection, removal, authorization, stale policy, conflicts, no-op behavior, and deterministic identity without filesystem, signer, or store setup.

### Decision: refuse conflicts in this slice

**Choice:** Update planning fails closed when artifact or lifecycle heads are conflicted. It does not use a local file as an implicit conflict winner.

**Rationale:** Resolution needs inspection of every competing revision and explicit supersession of all heads. Shipping that as a separate reviewed command prevents ordinary update from becoming hidden last-writer-wins behavior.

### Decision: preview and recheck before provider mutation

**Choice:** Portable execution locks and reloads the store, re-reads local artifacts and policy, recomputes the plan, writes a validated prepared envelope, and atomically replaces the expected store. Radicle execution rechecks object state and current creator-or-delegate authority under a per-change update lock, proves signer possession through signed typed mutation, reloads the object, and verifies the planned content root. Both paths also use the repository-scoped change lock while binding local artifacts to provider mutation.

**Rationale:** The preview remains useful evidence while execution refuses stale local files, changed policy, changed state, wrong actors, unsafe stores, or duplicate concurrent writers.

### Decision: keep provider authority claims distinct

**Choice:** Portable `--actor` remains a caller declaration and may update only state it created. Radicle derives the actor and delegate fact from the explicit profile and current canonical identity document; creator authority remains the reducer's signed-history rule. Receipts state the observed authority class.

**Rationale:** Backend-neutral semantics must not promote local labels into cryptographic identity or require Radicle-specific authority from every provider.

## Risks / Trade-offs

- Portable atomic rename does not establish fsync-level crash durability; stale prepared files fail closed and require deliberate recovery.
- A successful local Radicle update does not prove node publication, peer synchronization, remote availability, or remote trust.
- Ordinary update cannot resolve concurrent heads; a later explicit inspection/resolution slice remains required.
- Artifact edits make prior lifecycle/evidence observations stale by design; update does not automatically advance lifecycle state or attach evidence.

## Verification

- Positive core tests cover deterministic add/change/remove planning, exact result content, and no-op updates.
- Negative core tests cover stale policy/change identity, unauthorized actors, artifact conflicts, lifecycle conflicts, malformed IDs, and protocol limits.
- Provider tests cover preview no-mutation, successful execution/reload, serialized drift checks, stale locks, unsafe stores, wrong authority, and changed state/local files.
- Full Cairn lifecycle, traceability, purity, Cargo, and Nix rails must pass before archive.
