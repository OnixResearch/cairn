# Release Decision Radicle Cob Specification

## Purpose

Defines the `release-decision-radicle-cob` capability.

## Requirements

### Requirement: Release-decision policy is typed

r[cairn.release_decision_cob.policy] Cairn MUST define typed Nickel profiles for candidate fields, target labels, evidence roles, authority roles, finding and blocker classes, approval and rejection rules, lifecycle transitions, and named protocol bounds.

#### Scenario: Valid profile exports deterministically

r[cairn.release_decision_cob.policy.deterministic]
- GIVEN a valid release-decision profile
- WHEN Cairn exports the runtime projection
- THEN the generated profile MUST be deterministic and bound to its Nickel source identity.

#### Scenario: Ambiguous terminal rules fail closed

r[cairn.release_decision_cob.policy.invalid]
- GIVEN a profile has duplicate roles, unnamed bounds, unknown evidence roles, impossible thresholds, ambiguous overlapping approval and rejection predicates, or invalid transitions
- WHEN Cairn validates the Nickel source
- THEN export MUST fail and MUST NOT replace the generated runtime profile.

### Requirement: Release decision has a distinct typed profile

r[cairn.release_decision_cob.profile] Cairn MUST represent a release decision with the custom type `xyz.onixresearch.release-decision`, a stable decision ID, one immutable candidate root, exact policy identity, target labels, role sets, evidence references, findings, blockers, votes, and lifecycle facts.

#### Scenario: Valid root preserves exact inputs

r[cairn.release_decision_cob.profile.root]
- GIVEN valid candidate, policy, target, role, workspace, and typed base revision inputs
- WHEN Cairn initializes release-decision state
- THEN it MUST preserve every exact admitted value and derive deterministic BLAKE3 candidate, content, and state identities.

#### Scenario: Release-decision type rejects another workflow kind

r[cairn.release_decision_cob.profile.kind]
- GIVEN an operation or stored object declares a verification obligation, policy exception, document, Cairn change, or unknown kind under the release-decision type
- WHEN Cairn admits or evaluates the input
- THEN it MUST reject the mismatch without materializing trusted release-decision state.

### Requirement: Candidate identity is immutable and separate

r[cairn.release_decision_cob.candidate] Cairn MUST bind one immutable candidate descriptor containing policy-required source revision, artifact or manifest identities, target labels, and release labels, and it MUST keep the candidate root separate from decision, provider, operation, evidence, and artifact identities.

#### Scenario: Provider object ID cannot become the candidate root

r[cairn.release_decision_cob.candidate.provider]
- GIVEN a release decision exists in portable or direct-COB storage
- WHEN Cairn reports or exports it
- THEN the COB object ID MUST remain separately labeled and MUST NOT become the candidate root, decision ID, release label, or artifact identity.

#### Scenario: Candidate substitution fails closed

r[cairn.release_decision_cob.candidate.immutable]
- GIVEN a decision root binds one candidate
- WHEN any actor attempts to replace a candidate-defining source revision, artifact manifest, target, or release label in that object
- THEN the reducer MUST reject the complete operation and preserve the original candidate.

#### Scenario: Replacement candidate uses supersession

r[cairn.release_decision_cob.candidate.supersede]
- GIVEN a release candidate changes after review begins
- WHEN an authorized actor plans a replacement
- THEN Cairn MUST require a distinct decision ID and explicit supersession relation instead of an in-place candidate edit.

### Requirement: Release-decision authority is role-specific

r[cairn.release_decision_cob.authority] New decision roots MUST bind bounded canonical immutable proposer, reviewer, approver, withdrawer, and invalidator public-key sets and MUST enforce the policy-required role for every action.

#### Scenario: Authorized roles perform bounded actions

r[cairn.release_decision_cob.authority.allowed]
- GIVEN valid decision state and an actor in the required immutable role set
- WHEN the actor signs a policy-permitted evidence, review, blocker, vote, withdrawal, invalidation, supersession, or resolution action
- THEN the reducer MUST admit the action if all state and policy preconditions pass.

#### Scenario: Proposer cannot approve a candidate

r[cairn.release_decision_cob.authority.denied]
- GIVEN an actor is a proposer but is not a required approver
- WHEN the actor signs an approval vote
- THEN the reducer MUST reject the action even if its signature and candidate data are valid.

#### Scenario: Ambient authority is ignored

r[cairn.release_decision_cob.authority.ambient]
- GIVEN a key appears only in Git configuration, a Radicle profile, an identity document, a caller argument, or a later local file
- WHEN the key signs a release-decision action
- THEN those ambient facts MUST NOT grant a decision role.

### Requirement: Release evidence remains external and role-preserving

r[cairn.release_decision_cob.evidence] Cairn MUST store bounded typed references and validation summaries for policy-known release evidence, and existing Cairn validators MUST remain authoritative for Cairn-owned receipt families.

#### Scenario: Validated evidence summary binds exact input

r[cairn.release_decision_cob.evidence.valid]
- GIVEN the shell validates an external receipt through its authoritative validator
- WHEN the pure decision core admits the resulting summary
- THEN the summary MUST bind the evidence identity, schema, verification role, claim scope, validator identity, verdict, policy identity, and validation receipt identity.

#### Scenario: Recorded-only evidence cannot satisfy property evidence

r[cairn.release_decision_cob.evidence.role]
- GIVEN a required evidence slot needs `Property` evidence
- WHEN the current review basis contains only `RecordedOnly`, `Boundary`, or `Unknown` evidence for that slot
- THEN the evidence rule MUST fail and approval MUST remain blocked.

#### Scenario: Missing or stale evidence fails closed

r[cairn.release_decision_cob.evidence.stale]
- GIVEN required evidence is absent, invalid, bound to another candidate root, or no longer matches its validation summary
- WHEN Cairn evaluates the current review basis
- THEN it MUST report deterministic evidence blockers and MUST NOT preserve an approved classification for that basis.

### Requirement: Findings and blockers are stable and conflict-preserving

r[cairn.release_decision_cob.blockers] Cairn MUST represent findings and blockers with stable IDs, candidate-root binding, policy-known category and severity, bounded description, optional source span, typed evidence links, responses, and append-only disposition revisions.

#### Scenario: Unresolved blocker prevents approval

r[cairn.release_decision_cob.blockers.unresolved]
- GIVEN the current review basis has an unresolved policy-blocking finding or blocker
- WHEN Cairn evaluates approval
- THEN approval MUST remain blocked and the exact blocker identity and current heads MUST remain visible.

#### Scenario: Concurrent blocker dispositions remain visible

r[cairn.release_decision_cob.blockers.conflict]
- GIVEN authorized actors create incompatible dispositions from the same blocker head
- WHEN the reducer evaluates both operations
- THEN every disposition head MUST remain visible as an explicit conflict and no disposition MUST become an implicit winner.

#### Scenario: Blocker cannot be silently deleted

r[cairn.release_decision_cob.blockers.delete]
- GIVEN a finding or blocker exists in decision history
- WHEN an actor attempts to remove it without a policy-permitted append-only disposition
- THEN the reducer MUST reject the complete operation and preserve the adverse fact.

### Requirement: Votes bind one exact review basis

r[cairn.release_decision_cob.decision] Cairn MUST derive a BLAKE3 review-basis root from the candidate, policy, targets, required evidence summaries, findings, blocker dispositions, and relevant exception references, and every approval or rejection vote MUST bind that exact root.

#### Scenario: Current votes can satisfy a decision rule

r[cairn.release_decision_cob.decision.current]
- GIVEN a conflict-free review basis has valid required evidence, no unresolved blocking fact, and sufficient authorized current votes under the named policy rule
- WHEN Cairn evaluates the decision
- THEN it MUST derive the matching verdict and bind its exact review-basis and vote identities.

#### Scenario: Review-basis drift makes earlier votes inapplicable

r[cairn.release_decision_cob.decision.stale]
- GIVEN votes bind one review-basis root
- WHEN evidence, findings, blocker dispositions, targets, relevant exceptions, or policy identity change the current review basis
- THEN the prior votes MUST remain in history but MUST NOT count toward the new basis.

#### Scenario: Conflicting terminal decisions remain visible

r[cairn.release_decision_cob.decision.conflict]
- GIVEN valid concurrent decision facts satisfy incompatible terminal predicates or create incompatible lifecycle heads
- WHEN the reducer evaluates them
- THEN Cairn MUST preserve an explicit decision conflict and MUST NOT select a verdict by arrival order or provider identity.

### Requirement: Release-decision lifecycle preserves terminal history

r[cairn.release_decision_cob.lifecycle] Cairn MUST preserve draft, in-review, approved, rejected, withdrawn, invalidated, conflicted, and superseded lifecycle facts according to policy without deleting prior review or decision history.

#### Scenario: Withdrawal terminates a pending decision

r[cairn.release_decision_cob.lifecycle.withdraw]
- GIVEN a decision is not approved, rejected, invalidated, or superseded
- WHEN an authorized withdrawer records a bounded withdrawal reason
- THEN Cairn MUST classify the decision as withdrawn and preserve all earlier review facts.

#### Scenario: Invalidation preserves prior approval

r[cairn.release_decision_cob.lifecycle.invalidate]
- GIVEN a decision has a valid approved verdict
- WHEN an authorized invalidator records a bounded reason and evidence reference
- THEN Cairn MUST classify the decision as invalidated while preserving the earlier approval and exact review basis.

#### Scenario: Supersession is not cross-object atomicity

r[cairn.release_decision_cob.lifecycle.supersede]
- GIVEN a replacement decision links to an earlier decision
- WHEN one or both object updates are observed
- THEN Cairn MUST report each mutation separately and MUST NOT claim atomic remote lineage or peer convergence.

### Requirement: Release-decision backends preserve neutral semantics

r[cairn.release_decision_cob.backends] Cairn MUST support release-decision portable JSON and optional direct `radicle-cob` storage while preserving equal neutral BLAKE3 identities for equivalent state.

#### Scenario: Portable and direct backends preserve semantic identity

r[cairn.release_decision_cob.backends.roundtrip]
- GIVEN equivalent valid release-decision operations are admitted through portable and direct-COB providers
- WHEN Cairn reloads both states
- THEN exact candidate, roles, evidence, findings, blockers, votes, lifecycle facts, conflicts, content identity, and state identity MUST match.

#### Scenario: Runtime and transport remain absent

r[cairn.release_decision_cob.backends.boundary]
- GIVEN a release decision uses the direct COB backend
- WHEN dependencies and command surfaces are audited
- THEN no Radicle node, profile, identity document, discovery, synchronization, seeding, or transport behavior MUST enter the maintained product path.

### Requirement: Release-decision commands are preview-first

r[cairn.release_decision_cob.cli] Cairn MUST provide bounded create, status, evidence, finding, blocker, response, review, vote, withdraw, invalidate, conflict, resolve, supersede, and export operations with explicit backend and evidence inputs.

#### Scenario: Preview writes nothing

r[cairn.release_decision_cob.cli.preview]
- GIVEN valid release-decision command inputs
- WHEN the operator omits `--execute`
- THEN Cairn MUST emit a deterministic plan and receipt without mutating stores, refs, locks, evidence artifacts, release artifacts, registries, tags, deployments, or exported files.

#### Scenario: Execution rechecks reviewed inputs

r[cairn.release_decision_cob.cli.execute]
- GIVEN a valid reviewed release-decision plan
- WHEN the operator requests execution
- THEN Cairn MUST recheck policy, state, roles, candidate root, review basis, evidence summaries, blockers, conflicts, destination safety, and the exact plan before mutation.

### Requirement: Release-decision receipts preserve bounded claims

r[cairn.release_decision_cob.receipts] Cairn MUST emit deterministic BLAKE3-bound receipts for identities, candidate roots, roles, evidence, findings, blockers, review bases, votes, verdicts, lifecycle, provider observations, policy, blockers, execution mode, mutation results, and non-claims.

#### Scenario: Approved decision does not execute a release

r[cairn.release_decision_cob.receipts.approval]
- GIVEN valid signed history and a locally approved release decision
- WHEN Cairn emits a receipt
- THEN it MUST distinguish signature validity, configured role authorization, workflow approval, evidence validation summaries, evidence truth, release eligibility, and release execution.

#### Scenario: Backend success is not remote proof

r[cairn.release_decision_cob.receipts.backend]
- GIVEN a passing portable or direct-COB release-decision operation
- WHEN Cairn states the result
- THEN it MUST NOT claim artifact publication, tag movement, deployment, rollback, synchronization, peer convergence, remote trust, source correctness, or release eligibility.

### Requirement: Release-decision COB interoperability is fixture-backed

r[cairn.release_decision_cob.interop] Cairn MUST maintain provenance-bound external producer and independent consumer fixtures for the pinned `xyz.onixresearch.release-decision` COB format.

#### Scenario: Independent consumer reads release-decision history

r[cairn.release_decision_cob.interop.outward]
- GIVEN a signed release decision created and updated through Cairn
- WHEN a separate direct consumer evaluates its namespaced refs and action history
- THEN it MUST recover the exact type, decision ID, candidate root, roles, evidence summaries, findings, blockers, dispositions, votes, lifecycle facts, operation IDs, and current conflicts.

#### Scenario: Malformed history fails closed

r[cairn.release_decision_cob.interop.negative]
- GIVEN a decision fixture has drifted provenance, an unsupported manifest, unsigned history, a mismatched actor, candidate substitution, stale decision basis, unauthorized terminal fact, or mixed type data
- WHEN Cairn or the direct consumer evaluates the fixture
- THEN evaluation MUST fail before reporting compatibility or trusted release-decision state.

### Requirement: Release-decision support is release-gated

r[cairn.release_decision_cob.validation] The release-decision implementation MUST pass dependency, positive, negative, candidate, authority, evidence, blocker, vote, verdict, lifecycle, conflict, backend, shell, receipt, interoperability, compatibility, traceability, Tiger Style, and supported-system checks before archive.

#### Scenario: Maintained checks pass

r[cairn.release_decision_cob.validation.release]
- GIVEN the completed release-decision implementation and fixtures
- WHEN the maintained focused and repository-wide validation rails run
- THEN every required check MUST pass without missing traceability or weakened non-claims.
