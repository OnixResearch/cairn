# Design: Add ADR Radicle COB collaboration

## Context

The RFD change adds the shared `cairn.document-collaboration.v1` semantic state, portable envelope, conflict model, direct COB storage boundary, and reusable CLI shell.

ADR documents use similar collaboration machinery but have different lifecycle semantics. An accepted decision is a historical record. Replacing that decision requires an explicit new ADR and a visible supersession relation.

## Decisions

### Decision: Depend on the shared document foundation

**Choice:** Implement ADR as a profile over the neutral document collaboration core from `add-rfd-radicle-cob`. Reuse typed state, revision sets, links, evidence, portable envelopes, direct COB storage, locks, and receipt primitives.

**Rationale:** A second reducer or storage implementation can drift from RFD behavior and duplicate security-sensitive code.

### Decision: Give ADR a separate protocol type

**Choice:** Expose ADR objects as `xyz.onixresearch.adr`. The adapter and reducer will reject RFD, Cairn change, and unknown schemas under this type.

**Rationale:** ADR lifecycle and immutability rules differ from RFD discussion rules. Separate type identity prevents accidental protocol conversion.

### Decision: Keep exact source and structured lifecycle separate

**Choice:** Preserve exact bounded ADR Markdown and attachments as source artifacts. Keep title, stable ID, optional number, status, roles, typed relations, policy identity, and evidence as structured state.

A deterministic materialized view will project structured metadata into the policy-selected ADR format without silently rewriting accepted source.

**Rationale:** Exact source supports existing renderers and Git review. Structured lifecycle data supports deterministic policy without parsing arbitrary prose for authority.

### Decision: Freeze accepted decision content

**Choice:** Authors can update draft ADR artifacts. After an authorized acceptance action binds a conflict-free content root, later source-artifact edits will fail.

Review comments, evidence, and permitted lineage metadata can remain append-only. Replacing the decision requires a new ADR that declares supersession.

**Rationale:** An accepted ADR records a decision at one point in history. In-place edits can erase the reason for later architecture.

### Decision: Use explicit ADR roles

**Choice:** Root state will bind bounded canonical immutable sets for authors, reviewers, approvers, and publishers.

Authors can edit drafts. Reviewers can add bounded review records. Approvers can accept, reject, deprecate, or approve supersession transitions. Publishers can assign a number and materialize policy-selected publication paths.

**Rationale:** A valid signature identifies an actor. It does not grant architecture approval or publication authority.

### Decision: Separate identity, number, and provider object ID

**Choice:** Each ADR will have a stable document ID that is independent of the COB object ID. The optional human ADR number will be set once through a publisher action at the policy-selected lifecycle stage.

The shell will bind number assignment to an explicit local registry snapshot. Receipts will state that local comparison does not prove global uniqueness.

**Rationale:** Public ADR identity must survive backend migration. Offline peers need a separate allocator to prevent all number collisions.

### Decision: Model supersession as typed cross-object lineage

**Choice:** ADR links will include typed source and target kind, stable ID, relation, and optional expected content root. Supported relations will include supersedes, superseded-by, related-decision, RFD-origin, and implementation evidence.

Supersession planning will validate available local target state and reciprocal relation expectations. Two separate object mutations will not be described as atomic.

**Rationale:** Cross-object lineage must remain visible. One local reducer cannot prove that another object exists, accepts the reciprocal link, or is remotely available.

### Decision: Reject conflicted acceptance and publication

**Choice:** Acceptance, number assignment, publication, deprecation, and supersession will require conflict-free policy-selected state. Conflict resolution must explicitly supersede every current head.

**Rationale:** An architecture decision cannot bind an ambiguous source or implicit conflict winner.

### Decision: Keep backends and effects explicit

**Choice:** ADR will support the portable JSON backend before the optional direct `radicle-cob` backend. Signed mutation will require an explicit Git repository and keystore.

Pure ADR admission, reduction, authority, lifecycle, lineage, hashing, and planning will remain in `cairn-core`. Filesystem, Git, keystore, registry, lock, publication, and output effects will remain in the CLI.

**Rationale:** ADR support must preserve backend neutrality, the FCIS boundary, and the no-Radicle-runtime contract.

### Decision: Require independent compatibility evidence

**Choice:** Add an upstream-produced static ADR COB fixture and a separate direct read-only consumer.

Positive fixtures will cover draft edits, review, acceptance, publication, deprecation, and supersession. Negative fixtures will cover mixed types, post-acceptance edits, unauthorized transitions, conflicting acceptance, broken lineage, malformed history, stale previews, and unsafe publication paths.

**Rationale:** Internal round trips cannot validate independent format consumption or failure behavior.

## Risks and Trade-offs

- Accepted-content immutability requires a new ADR for corrections that change decision meaning.
- Separate object updates cannot make supersession reciprocal in one atomic operation.
- Immutable role sets require a replacement object when authority changes.
- A local number registry cannot prove global uniqueness.
- Reusing a new shared core makes ADR implementation depend on the RFD foundation change.

## Claim Boundary

Passing ADR validation proves only the recorded state, configured role checks, deterministic lifecycle rules, selected local backend observations, and bounded publication plans.

It does not prove architectural correctness, organizational approval outside configured roles, remote publication, reciprocal remote lineage, global number uniqueness, source correctness, or release eligibility.
