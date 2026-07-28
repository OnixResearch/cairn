# Design: Add RFD Radicle COB collaboration

## Context

Cairn change collaboration uses a neutral semantic state, a portable JSON envelope, and an optional direct Radicle COB adapter. Accepted files and Git commits remain canonical.

RFD documents need similar collaboration properties. They also need document identity, discussion, typed links, publication numbers, and authority that does not treat every valid signature as acceptance.

The first implementation must create reusable document infrastructure without weakening Cairn change compatibility.

## Decisions

### Decision: Add a shared neutral document core

**Choice:** Add `cairn.document-collaboration.v1` as a backend-neutral state. The state will carry a document kind, stable document ID, workspace identity, typed base revision, policy hash, exact source artifacts, revision DAGs, discussion records, typed links, lifecycle revisions, evidence, conflicts, and explicit role facts.

The pure core will derive BLAKE3 content and state identities. Provider object IDs will remain adapter observations.

**Rationale:** RFD and ADR collaboration need the same deterministic state machinery. A shared core prevents copied reducers and provider-specific semantics.

### Decision: Give RFD a separate protocol type

**Choice:** Expose the RFD profile as `xyz.onixresearch.rfd`. The RFD reducer will accept only the RFD schema and policy profile.

ADR objects will use another type. An operation cannot convert one document kind into another.

**Rationale:** Separate type names prevent mixed lifecycle rules and permit independent protocol evolution.

### Decision: Keep published Git content canonical

**Choice:** The COB state will preserve exact bounded draft artifacts and deterministic materialization plans. Publication will write only policy-selected RFD paths after full preview and execution rechecks.

Published Markdown, attachments, registry entries, and Git commits will remain the canonical publication record. A COB will remain collaboration history and evidence input.

**Rationale:** Existing Git review, rendering, indexing, and archival tools must remain usable without Radicle.

### Decision: Separate document identity from publication numbering

**Choice:** Root state will contain a stable document ID that is independent of the COB object ID. An optional human RFD number will use a set-once publisher operation.

The shell will compare a publication plan with an explicit local registry snapshot. The receipt will bind that snapshot and state that local validation does not prove global uniqueness.

**Rationale:** Provider IDs are unsuitable public identifiers. Offline peers cannot prove that a number is globally unique without a separate allocation authority.

### Decision: Use explicit role authority

**Choice:** Root state will bind bounded canonical sets for authors, participants, reviewers, and publishers. These role sets will be immutable in the first schema.

Authors can edit draft artifacts. Participants can add bounded discussion records. Reviewers can perform policy-selected review transitions. Publishers can assign a number and publish accepted content.

A signature will identify the actor only. The reducer will enforce the required role for each action.

**Rationale:** Signed history does not make an author a reviewer or publisher. Immutable root roles avoid ambient and time-dependent authority.

### Decision: Preserve whole-artifact conflicts

**Choice:** Source artifacts will use history-preserving multi-head revision sets. Concurrent updates to one artifact will remain explicit conflicts until an authorized resolution supersedes every current head.

The first schema will not claim character-level or line-level text merging. A later profile can split documents into stable section artifacts after evidence shows that whole-file conflicts are too frequent.

**Rationale:** Explicit conflicts are deterministic and safe. Silent last-writer-wins or unproved text merging can discard reviewed content.

### Decision: Keep links and discussion bounded

**Choice:** Typed links will name the target document kind, stable target ID, relation, and optional expected content root. Discussion records will use stable message IDs, actor IDs, bounded text, and reply references.

Local validation will check shape and known local targets. It will not prove remote target existence, target acceptance, or link truth.

**Rationale:** RFD review needs discussion and lineage without turning local object evaluation into a global database claim.

### Decision: Provide portable and COB backends

**Choice:** Implement portable JSON round trips before direct COB storage. The direct adapter will use `radicle-cob`, cryptographic, OID, Git, and ref-format primitives only.

The adapter will use one explicit Git repository and explicit keystore. It will not load Radicle profiles or identity documents and will not run a node.

**Rationale:** The neutral backend validates semantics without Radicle. The direct adapter adds signed Git-backed operation history without runtime ownership.

### Decision: Preserve the functional core and imperative shell

**Choice:** Pure parsing, admission, reduction, hashing, conflict inspection, resolution, authority, publication planning, and receipt material will live in `cairn-core`.

The CLI will own file reads, repository access, keystore access, locks, atomic publication, Git inspection, output, and exit status. Every mutation will remain preview-first.

**Rationale:** RFD support must retain Cairn's FCIS boundary and Tiger Style checks.

### Decision: Require independent interoperability evidence

**Choice:** Add an upstream-produced static COB fixture and a direct read-only consumer. Positive fixtures will cover create, update, discussion, review, resolution, and publication planning.

Negative fixtures will cover malformed history, unauthorized roles, duplicate actions, stale bases, mixed types, conflicting artifacts, duplicate local numbers, stale links, unsafe destinations, and drift after preview.

**Rationale:** Internal round trips do not prove format compatibility or correct failure behavior.

## Risks and Trade-offs

- Whole-document edits can create frequent explicit conflicts.
- Immutable role sets require a superseding object when authority changes.
- A local number registry cannot prove global uniqueness.
- Generalizing the adapter can affect Cairn change compatibility unless tests bind existing bytes and hashes.
- Discussion storage increases protocol bounds and denial-of-service risk.

## Claim Boundary

Passing RFD validation proves only the recorded neutral state, selected local backend observations, configured role checks, deterministic reduction, and bounded materialization plan.

It does not prove publication, synchronization, global number uniqueness, remote trust, semantic review quality, link truth, source correctness, or release eligibility.
