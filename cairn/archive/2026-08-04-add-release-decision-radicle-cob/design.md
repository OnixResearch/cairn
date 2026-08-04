# Design: Add release-decision Radicle COB collaboration

## Context

The verification-obligation change introduces `cairn.workflow-collaboration.v1`, provider-neutral workflow identities, conflict-preserving reduction, external evidence references, portable envelopes, and optional direct-COB storage.

Cairn already validates release-readiness and release-evidence artifacts. A collaborative release decision should reference those artifacts and preserve human or policy-defined decision history. It must not duplicate their validators or gain artifact mutation authority.

## Decisions

### Decision: Depend on the shared workflow foundation

**Choice:** Implement release decisions as a profile over the neutral workflow collaboration core from `add-verification-obligation-radicle-cob`. Reuse typed state, field revisions, evidence references, conflicts, portable envelopes, direct-COB storage, locks, and receipt primitives.

**Rationale:** A second workflow reducer or storage implementation can drift and duplicate security-sensitive behavior.

### Decision: Give release decisions a distinct protocol type

**Choice:** Expose release decisions as `xyz.onixresearch.release-decision`. The adapter and reducer will reject other workflow, document, Cairn change, and unknown schemas under this type.

**Rationale:** Release authority, evidence rules, and lifecycle semantics must not mix with verification obligations or policy exceptions.

### Decision: Separate decision, candidate, artifact, and provider identities

**Choice:** Root state will contain a stable decision ID and one immutable candidate descriptor. The descriptor will bind source revision, artifact or manifest identities, target labels, and any policy-required release label into a BLAKE3 candidate root.

COB object IDs, operation IDs, evidence receipt IDs, artifact IDs, and release labels will remain separately typed.

**Rationale:** Provider migration must not rename the decision or candidate. Human labels and storage IDs are not content identities.

### Decision: Use one immutable candidate per decision object

**Choice:** Candidate fields will be immutable after object creation. Evidence, findings, blockers, and decisions can evolve around that candidate.

A changed source revision, artifact manifest, target, or candidate-defining label will require another decision ID with an explicit supersession relation.

**Rationale:** Approvals for one candidate must not transfer to another candidate through an in-place edit.

### Decision: Use immutable role authority

**Choice:** Root state will bind bounded canonical immutable public-key sets for proposers, reviewers, approvers, withdrawers, and invalidators. Policy will define action permissions and named approval or rejection rules.

Signatures and ambient local identity facts will not grant roles. Authority changes require a superseding decision object.

**Rationale:** Deterministic offline evaluation cannot depend on mutable profiles or external identity lookup.

### Decision: Keep evidence external and typed

**Choice:** The object will store bounded references and validation summaries for release-readiness, release-evidence, verification-obligation, policy-exception, source, artifact, and other policy-known evidence roles.

Existing Cairn validators will remain authoritative for Cairn receipt families. The shell will load and validate external artifacts, then pass typed summaries and exact identities to the pure release-decision core.

**Rationale:** Copying full evidence into the COB increases size and can create a second validator with weaker semantics.

### Decision: Model stable findings and blockers

**Choice:** Review findings and blockers will use stable IDs, policy-known categories and severities, bounded descriptions, optional source spans, candidate-root binding, typed evidence links, responses, and append-only dispositions.

Concurrent dispositions will remain explicit conflicts. No finding or blocker will be silently deleted.

**Rationale:** A release verdict must not erase unresolved adverse review facts.

### Decision: Bind votes to one review-basis root

**Choice:** A review-basis root will include the immutable candidate root, exact release policy identity, target labels, required evidence identities and validation summaries, findings, blocker dispositions, and relevant exception references. It will exclude votes themselves.

Every approval or rejection vote will bind that exact review-basis root. A meaning-changing evidence, finding, blocker, or policy update will produce a new review basis and make prior votes inapplicable without deleting them.

**Rationale:** Evidence or blocker drift must trigger fresh review without recursive vote hashing.

### Decision: Derive verdicts from explicit policy rules

**Choice:** Nickel policy will define named approval and rejection rules, required roles, evidence roles, blocker classes, and decision thresholds. Profiles will reject ambiguous overlapping terminal predicates or classify unresolved overlap as a decision conflict.

Approval will require a conflict-free current review basis, current valid required evidence, no unresolved blocking finding, and sufficient current approvals. Rejection will use its policy rule. The reducer will not use clocks or operation arrival order.

**Rationale:** A deterministic verdict must be reproducible from explicit facts and must not conceal contradictory signed decisions.

### Decision: Preserve withdrawal, invalidation, and supersession

**Choice:** An authorized withdrawer can terminate a pending decision. An authorized invalidator can record why a prior approval no longer applies. Neither transition will delete prior review or approval history.

A replacement candidate or decision will use a distinct object and typed supersession link. Cross-object updates will not be described as atomic.

**Rationale:** Release history must show why a candidate stopped being approved or was replaced.

### Decision: Keep release execution outside this type

**Choice:** Release decisions will not call build tools, move tags, sign artifacts, publish evidence, deploy systems, rotate retention, or mutate registries. They can reference receipts from those separate systems.

**Rationale:** Collaboration state is evidence for an operator or policy gate. It is not a release executor.

### Decision: Preserve the functional core and imperative shell

**Choice:** Pure candidate admission, evidence-summary validation, reduction, authority, finding and blocker logic, review-basis hashing, vote evaluation, verdict classification, conflict inspection, and receipt material will live in `cairn-core`.

The CLI will own files, external receipt reads, Git, keystores, locks, atomic writes, output, and process status. Every mutation will remain preview-first and will recheck exact inputs during execution.

**Rationale:** This keeps logic testable and preserves Cairn's FCIS and Tiger Style boundaries.

### Decision: Require independent interoperability evidence

**Choice:** Add a provenance-bound static COB fixture and a separate direct read-only consumer. Positive fixtures will cover creation, evidence attachment, review, blockers, dispositions, approval, rejection, withdrawal, invalidation, resolution, and supersession.

Negative fixtures will cover mixed types, candidate substitution, stale evidence, evidence-role promotion, stale votes, unresolved blockers, conflicting decisions, unauthorized terminal facts, tampered history, and weakened non-claims.

**Rationale:** Internal round trips do not prove format compatibility or fail-closed behavior.

## Risks and Trade-offs

- One immutable candidate per object creates more objects when a candidate changes.
- External evidence can become unavailable even when its recorded identity remains stable.
- Explicit blocker and vote conflicts require operator resolution.
- Immutable roles require a new object when authority changes.
- A policy-valid approval can still be wrong if evidence or human judgment is wrong.

## Claim Boundary

Passing release-decision validation proves only recorded workflow state, exact candidate and review-basis binding, configured role and decision rules, supplied evidence-reference validation summaries, deterministic reduction, and selected local backend observations.

It does not prove evidence truth, source correctness, build reproducibility, artifact integrity beyond supplied receipts, publication, deployment, rollback, remote trust, peer convergence, or release eligibility.
