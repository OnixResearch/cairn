# Design: Add policy-exception Radicle COB collaboration

## Context

The verification-obligation change introduces `cairn.workflow-collaboration.v1`, provider-neutral workflow identities, conflict-preserving reduction, external evidence references, portable envelopes, and optional direct-COB storage.

Policy exceptions need this machinery, but their authority and claim boundary differ. An approved exception is review evidence for a named policy consumer. It is not a capability grant or an ambient policy bypass.

## Decisions

### Decision: Depend on the shared workflow foundation

**Choice:** Implement policy exceptions as a profile over the neutral workflow collaboration core from `add-verification-obligation-radicle-cob`. Reuse typed state, field revisions, evidence references, conflicts, portable envelopes, direct-COB storage, locks, and receipt primitives.

**Rationale:** A second workflow reducer or storage implementation can drift and duplicate security-sensitive behavior.

### Decision: Give policy exceptions a distinct protocol type

**Choice:** Expose policy exceptions as `xyz.onixresearch.policy-exception`. The adapter and reducer will reject other workflow, document, Cairn change, and unknown schemas under this type.

**Rationale:** Exception authority and lifecycle rules must not mix with verification or release decisions.

### Decision: Separate exception identity from related identities

**Choice:** Root state will contain a stable exception ID. Policy identity, rule ID, subject identities, evidence identities, portable envelope ID, COB object ID, and operation IDs will remain separately typed.

**Rationale:** Provider migration must not rename the exception or alias the policy rule it references.

### Decision: Bind exact policy and scope

**Choice:** Each request root will bind one policy schema and content identity, one stable rule ID, typed subject and action selectors, rationale, compensating controls, owner facts, and explicit validity bounds.

Scope will use a policy-known grammar with canonical ordering. An omitted selector will not become a wildcard. Wildcards will be rejected unless the selected policy profile names their meaning explicitly.

**Rationale:** Vague or drifting scope can turn a narrow exception into an unintended general bypass.

### Decision: Use immutable role authority

**Choice:** Root state will bind bounded canonical immutable public-key sets for requesters, reviewers, approvers, revokers, and validity observers. Policy will define required roles and named approval thresholds.

Signatures and ambient local identity facts will not grant roles. Authority changes require a superseding exception.

**Rationale:** Deterministic offline evaluation cannot depend on mutable profiles or external identity lookup.

### Decision: Bind approval to one exact request root

**Choice:** Review findings and approval facts will name the exact request root containing policy identity, rule, scope, rationale, controls, and validity bounds. Any meaning-changing revision will create a new request root and make prior approvals inapplicable.

Concurrent request revisions or incompatible approval decisions will remain explicit conflicts.

**Rationale:** Approval for one scope or control set must not transfer silently to another.

### Decision: Keep time as an explicit supplied observation

**Choice:** Every exception will have policy-valid finite validity bounds. The pure reducer will validate their syntax and ordering but will not read a clock.

An applicability or expiration evaluation will consume an explicitly selected typed time observation with source identity, observed value, evidence identity, actor, and role authorization. The reducer will preserve the observation without claiming that its source is globally trusted or accurate.

A policy-accepted observation at or beyond the expiration bound will create a terminal expiration fact. An older observation cannot restore applicability. Concurrent non-equivalent observations that policy cannot order will remain an explicit validity conflict and will block applicability.

**Rationale:** Hidden wall-clock access breaks determinism. A signed timestamp still needs a declared trust boundary.

### Decision: Separate lifecycle approval from consumer applicability

**Choice:** The object lifecycle will preserve requested, in-review, approved, rejected, revoked, expired, closed, conflicted, and superseded facts. A deterministic applicability projection will additionally require matching current policy identity, exact scope, approved request root, no revocation, no conflict, and a policy-acceptable current validity observation.

Consumers will decide whether that projection is sufficient for their own enforcement.

**Rationale:** An approved object is not an ambient capability and can become inapplicable through policy, subject, or observation mismatch.

### Decision: Preserve revocation and expiration history

**Choice:** Authorized revocation and admitted expiration observations will append terminal facts without deleting the approval basis. Closure can record completed follow-up work but cannot restore applicability.

A replacement needs a new exception ID and explicit supersession link.

**Rationale:** Audit history must show what was approved, when it was later observed as expired, and why it was revoked.

### Decision: Keep policy enforcement outside the adapter

**Choice:** The COB reducer will validate workflow semantics and configured role rules only. Basalt, Cairn gates, UCAN consumers, and other policy engines will compare the exception projection with their exact current policy and subject inputs.

**Rationale:** Storage and collaboration code must not become a universal policy authority.

### Decision: Preserve the functional core and imperative shell

**Choice:** Pure scope admission, reduction, authority, request-root hashing, conflict inspection, validity-observation evaluation, applicability classification, and receipt material will live in `cairn-core`.

The CLI will own files, policy snapshot reads, clocks or external observation collection, Git, keystores, locks, atomic writes, output, and process status. Every mutation will remain preview-first.

**Rationale:** This keeps logic testable without standing up the world and preserves Cairn's FCIS boundary.

### Decision: Require independent interoperability evidence

**Choice:** Add a provenance-bound static COB fixture and a separate direct read-only consumer. Positive fixtures will cover request, review, revision, approval, applicability, revocation, expiration, closure, and supersession.

Negative fixtures will cover mixed types, unsafe or wildcard scope, stale policy, stale approval, malformed validity bounds, untrusted observation shape, unauthorized transition, conflicting decisions, tampered history, and weakened non-claims.

**Rationale:** Internal round trips do not prove format compatibility or fail-closed behavior.

## Risks and Trade-offs

- Immutable roles require a new object when authority changes.
- A policy consumer can choose not to honor an otherwise valid exception.
- Explicit time observations add operator work but avoid hidden clock authority.
- Offline peers can disagree about observation trust while preserving the same signed history.
- Finite validity prevents permanent exceptions but requires deliberate renewal through a new object.

## Claim Boundary

Passing policy-exception validation proves only recorded workflow state, exact declared policy and scope binding, configured role checks, deterministic reduction, supplied validity observations, selected local backend observations, and bounded applicability classification.

It does not grant authority, bypass enforcement, prove current time, prove control effectiveness, prove organizational approval outside configured roles, establish remote trust, or prove release eligibility.
