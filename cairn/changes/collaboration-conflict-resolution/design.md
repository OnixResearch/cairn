# Design: Explicit Collaboration Conflict Resolution

## Context

Collaboration state stores artifact and lifecycle values in revision DAGs. A revision is current when no later revision names it in its basis. Concurrent operations therefore produce multiple current heads without deleting either candidate. Ordinary update intentionally refuses such state, and both portable and Radicle providers already expose serialized mutation shells that can execute a reviewed neutral operation.

## Decisions

### Decision: inspect current heads in the pure core

**Choice:** Derive a sorted inspection containing state identity, every conflicted artifact, and each current head's revision ID, actor, deletion status, byte length, and BLAKE3 content hash. Report lifecycle heads with their exact typed records. A separate explicit `(artifact, revision)` selector may return one bounded exact head value for review.

**Rationale:** Metadata remains bounded and deterministic while the operator can retrieve one exact candidate without provider-specific storage inspection or an unbounded all-content receipt.

### Decision: preserve history through complete-basis revisions

**Choice:** Resolution requires exactly one selected current head for every conflicted artifact. Each emitted `SetArtifact` copies the selected value and names the complete current head set as its basis. Missing, duplicate, extra, stale, or non-head selections fail before a plan exists.

**Rationale:** One new revision makes the selected value current while retaining every competing revision and its actor/basis history. Rewriting state or deleting losing heads would destroy evidence; using local files would create an implicit winner.

### Decision: resolve lifecycle conflicts only through explicit conservative reset

**Choice:** When lifecycle has multiple current heads, the operator must explicitly request `open`. The planner emits one `SetLifecycle` action for the post-artifact content root with no evidence and the complete lifecycle head set as basis. Other lifecycle statuses are rejected in this slice.

**Rationale:** Selecting an approval-like head after content resolution could promote stale evidence or authority. Returning to `open` preserves history while making no review, approval, archive, or evidence claim.

### Decision: require complete atomic resolution

**Choice:** A resolution plan must cover every current artifact conflict and any lifecycle conflict in one operation. The planner simulates the operation, requires conflict-free content and one lifecycle head afterward, and binds input/result state and content identities into a deterministic BLAKE3 operation and plan hash.

**Rationale:** Partial resolution leaves ambiguous state and complicates receipts. Complete selection gives portable and Radicle providers one exact operation to recheck and apply.

### Decision: reuse provider serialization without transferring authority claims

**Choice:** Portable execution uses caller-declared creator identity, portable-store locking, the shared repository-change lock, envelope validation, prepared replacement, and reload verification. Radicle execution reloads current canonical identity, derives creator-or-delegate authority from the profile signer, uses the per-change update lock, creates one signed typed operation, and reloads the same object. `cairn collaboration conflicts|resolve` is backend-neutral; `cairn radicle conflicts|resolve` remains a compatibility entry point.

**Rationale:** Semantics remain provider-neutral while signatures, identity documents, object IDs, and locks remain Radicle adapter observations.

## Considered Alternatives

- **Rewrite the state to one head:** rejected because it erases losing revisions and invalidates historical identity.
- **Use local active-change bytes as the winner:** rejected because ordinary filesystem state is not an explicit conflict choice. Operators may select a head, then use ordinary update for a later reviewed edit.
- **Allow partial resolution:** rejected because receipts could appear successful while unresolved ambiguity remains.
- **Carry an approval lifecycle head forward:** rejected because selected content may make its subject/evidence stale; explicit `open` is the conservative first slice.

## Risks / Trade-offs

- Choosing an existing head does not perform semantic or textual merge. A later ordinary update can publish reviewed merged bytes after conflict resolution.
- Exact head display may reveal one bounded artifact value to the invoking operator; it is opt-in and does not dump every head.
- Portable actor identity remains a caller declaration and local atomic rename does not establish fsync-level crash durability.
- Local signed Radicle resolution does not establish node publication, peer convergence, availability, or remote trust.

## Verification

- Positive core tests cover deterministic inspection, one-head disclosure, complete multi-artifact resolution, lifecycle reset, exact bases, and stable hashes.
- Negative core tests cover missing/extra/duplicate/stale selectors, unauthorized actors, stale policy/change identity, unsupported lifecycle status, no-conflict input, and post-plan state drift.
- Provider tests cover preview no-mutation, exact execution/reload, store/object/state/policy/authority drift, stale locks, unsafe stores, missing objects, and explicit profile signer execution.
- CLI tests cover strict backend arguments, bounded inspection, conflict choices, receipts, compatibility routing, and non-claims.
- Full lifecycle, traceability, purity, Cargo, and Nix rails must pass before archive.
