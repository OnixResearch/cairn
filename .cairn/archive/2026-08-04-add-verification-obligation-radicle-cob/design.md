# Design: Add verification-obligation Radicle COB collaboration

## Context

Cairn already has backend-neutral collaboration for active changes. The RFD change extends reusable portable and direct-COB storage primitives for typed documents.

Verification obligations are workflow records, not documents. They need structured claim revisions, external evidence references, findings, dispositions, and discharge rules without copying the document model or treating collaboration history as proof.

## Decisions

### Decision: Add a shared workflow collaboration core

**Choice:** Add `cairn.workflow-collaboration.v1` as a backend-neutral state. It will carry an object kind, stable workflow ID, workspace identity, typed base revision, policy hash, root-bound roles, typed field revisions, evidence references, lifecycle assertions, conflicts, and deterministic receipt material.

The pure core will derive BLAKE3 content and state identities. Provider object IDs and signed operation IDs will remain adapter observations.

**Rationale:** Verification obligations, policy exceptions, and release decisions need common deterministic workflow machinery. They must not inherit document publication or source-artifact semantics.

### Decision: Reuse storage primitives without reusing document semantics

**Choice:** Depend on `add-rfd-radicle-cob` for generalized portable-envelope, direct-COB storage, locking, signing, and compatibility seams. Keep workflow admission and reduction in separate core modules and schemas.

**Rationale:** Reusing storage reduces security-sensitive duplication. Separate semantic models prevent a workflow record from becoming a fake RFD or ADR.

### Decision: Give verification obligations a distinct protocol type

**Choice:** Expose verification obligations as `xyz.onixresearch.verification-obligation`. The adapter and reducer will reject document, Cairn change, policy-exception, release-decision, and unknown schemas under this type.

**Rationale:** A separate type permits independent lifecycle evolution and fail-closed dispatch.

### Decision: Separate obligation identity from provider identity

**Choice:** Root state will contain a stable obligation ID. The portable envelope ID, COB object ID, operation IDs, and evidence artifact IDs will remain separately typed.

**Rationale:** Obligation identity must survive backend migration and must not alias a storage location or proof identity.

### Decision: Bind immutable role authority at the root

**Choice:** Root state will bind bounded canonical immutable public-key sets for submitters, contributors, verifiers, and invalidators. Policy will map each operation to one or more required roles.

A valid signature identifies the actor only. Ambient Git configuration, profiles, identity documents, caller claims, and later local files will not grant a role.

**Rationale:** Immutable root authority makes offline reduction deterministic. Authority changes require a superseding obligation instead of ambient reinterpretation.

### Decision: Make claim revision boundaries explicit

**Choice:** Each claim revision will bind exact structured claim text, subject identities, policy identity, verification criteria, and a BLAKE3 claim root. A revision will name the complete prior claim head basis.

Evidence, findings, votes, and discharge decisions will name the claim root they evaluated. A claim revision will preserve earlier review history but make earlier decisions inapplicable to the new claim root.

**Rationale:** Review evidence for one claim must not silently discharge another claim.

### Decision: Preserve findings and dispositions

**Choice:** Findings will use stable IDs, policy-known categories and severities, bounded descriptions, optional source spans, claim-root binding, and typed evidence links. Responses and dispositions will be append-only revisions.

Concurrent dispositions for one finding will remain explicit heads until an authorized complete-basis resolution supersedes them.

**Rationale:** Silent finding deletion or last-writer-wins can erase unresolved verification work.

### Decision: Derive discharge from exact current state

**Choice:** Policy will define named evidence roles, required verifier facts, decision thresholds, and blocking finding classes. Discharge will require a conflict-free current claim root, policy-accepted current evidence validation summaries bound to exact evidence and claim identities, no unresolved blocking finding, and sufficient verifier decisions bound to that root.

Rejection and post-discharge invalidation will preserve prior decisions. No reducer transition will depend on a clock or operation arrival order.

**Rationale:** Discharge must be reproducible from explicit facts while remaining a bounded workflow decision rather than proof truth.

### Decision: Preserve semantic conflicts

**Choice:** Concurrent claim revisions, finding dispositions, and incompatible lifecycle decisions will remain visible multi-head conflicts. Resolution must name every current head and create one new superseding revision.

The system will not claim textual, logical, or proof-level merging.

**Rationale:** Explicit conflict is safer than discarding signed review state.

### Decision: Provide portable and direct-COB backends

**Choice:** Implement BLAKE3-bound portable JSON first. The optional direct adapter will use `radicle-cob` over one explicit Git repository and explicit keystore.

It will not load Radicle profiles or identity documents and will not run a node.

**Rationale:** Portable state validates semantics without Radicle. Direct COB storage adds signed Git-backed operation history without runtime ownership.

### Decision: Preserve the functional core and imperative shell

**Choice:** Pure admission, reduction, hashing, authority checks, conflict inspection, discharge evaluation, invalidation, planning, and receipt material will live in `cairn-core`.

The CLI will own files, Git, keystores, locks, atomic writes, output, and process status. Every mutation will remain preview-first and will recheck exact inputs during execution.

**Rationale:** The feature must retain Cairn's FCIS and Tiger Style boundaries.

### Decision: Require independent interoperability evidence

**Choice:** Add a provenance-bound static COB fixture and a separate direct read-only consumer. Positive fixtures will cover creation, claim revision, evidence attachment, findings, responses, discharge, invalidation, and conflict resolution.

Negative fixtures will cover mixed types, malformed history, unauthorized operations, stale claim roots, evidence-role promotion, unresolved blockers, conflicting decisions, tampered provenance, and unsafe shell state.

**Rationale:** Internal round trips do not prove format compatibility or correct failure behavior.

## Risks and Trade-offs

- Immutable roles require a superseding object when authority changes.
- External evidence can become unavailable even when its recorded identity remains stable.
- Explicit claim and disposition conflicts require operator resolution.
- Generalizing shared storage can affect existing change and document compatibility unless fixtures bind existing bytes and hashes.
- A discharged obligation can still be wrong if evidence or verifier judgment is wrong.

## Claim Boundary

Passing verification-obligation validation proves only recorded workflow state, configured role checks, deterministic reduction, supplied evidence-reference identity, selected local backend observations, and bounded discharge rules.

It does not prove evidence truth, proof soundness, reviewer competence, source correctness, remote publication, synchronization, peer convergence, organizational trust, or release eligibility.
