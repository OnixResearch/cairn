# Verification Obligation Radicle Cob Specification

## Purpose

Defines the `verification-obligation-radicle-cob` capability.

## Requirements

### Requirement: Verification obligation has a distinct typed profile

r[cairn.verification_obligation_cob.profile] Cairn MUST represent a verification obligation with the custom type `xyz.onixresearch.verification-obligation`, a stable obligation ID, exact structured claim, subject identities, verification criteria, role sets, evidence links, findings, and lifecycle facts.

#### Scenario: Valid root preserves exact inputs

r[cairn.verification_obligation_cob.profile.root]
- GIVEN valid claim, subject, criteria, roles, policy, workspace, and typed base revision inputs
- WHEN Cairn initializes verification-obligation state
- THEN it MUST preserve every exact admitted value and derive deterministic BLAKE3 claim, content, and state identities.

#### Scenario: Verification-obligation type rejects another workflow kind

r[cairn.verification_obligation_cob.profile.kind]
- GIVEN an operation or stored object declares a document, Cairn change, policy exception, release decision, or unknown kind under the verification-obligation type
- WHEN Cairn admits or evaluates the input
- THEN it MUST reject the mismatch without materializing trusted obligation state.

### Requirement: Obligation identity is separate from provider and evidence identity

r[cairn.verification_obligation_cob.identity] Cairn MUST keep the stable obligation ID independent of portable envelope IDs, COB object IDs, operation IDs, evidence artifact IDs, and external publication numbers.

#### Scenario: Provider object ID cannot become the obligation ID

r[cairn.verification_obligation_cob.identity.provider]
- GIVEN an obligation exists in portable or direct-COB storage
- WHEN Cairn reports, exports, or materializes the obligation
- THEN it MUST label provider and obligation identities separately and MUST NOT substitute one for the other.

### Requirement: Verification-obligation authority is role-specific

r[cairn.verification_obligation_cob.authority] New obligation roots MUST bind bounded canonical immutable submitter, contributor, verifier, and invalidator public-key sets and MUST enforce the policy-required role for every action.

#### Scenario: Authorized roles perform bounded actions

r[cairn.verification_obligation_cob.authority.allowed]
- GIVEN valid obligation state and an actor in the required immutable role set
- WHEN the actor signs a policy-permitted claim, evidence, finding, response, decision, invalidation, or resolution action
- THEN the reducer MUST admit the action if all state and policy preconditions pass.

#### Scenario: Submitter cannot discharge an obligation

r[cairn.verification_obligation_cob.authority.denied]
- GIVEN an actor is a submitter but is not a required verifier
- WHEN the actor signs a discharge or rejection decision
- THEN the reducer MUST reject the action even if its signature and content are valid.

#### Scenario: Ambient authority is ignored

r[cairn.verification_obligation_cob.authority.ambient]
- GIVEN a key appears only in Git configuration, a Radicle profile, an identity document, a caller argument, or a later local file
- WHEN the key signs an obligation action
- THEN those ambient facts MUST NOT grant an obligation role.

### Requirement: Claim revisions create explicit review epochs

r[cairn.verification_obligation_cob.claim_revision] Cairn MUST bind every evidence link, finding, verifier decision, and discharge to one exact claim root, and a new claim revision MUST preserve but make prior decision facts inapplicable to the new root.

#### Scenario: Claim revision preserves prior review history

r[cairn.verification_obligation_cob.claim_revision.history]
- GIVEN an obligation has findings and decisions for one claim root
- WHEN an authorized submitter creates a new complete-basis claim revision
- THEN Cairn MUST retain the earlier claim and review facts and MUST open a distinct review epoch for the new claim root.

#### Scenario: Claim revision makes an earlier discharge inapplicable

r[cairn.verification_obligation_cob.claim_revision.stale]
- GIVEN an obligation was discharged for an earlier claim root
- WHEN a later authorized claim revision becomes current
- THEN the earlier discharge MUST NOT classify the new claim root as discharged.

#### Scenario: Concurrent claim revisions conflict

r[cairn.verification_obligation_cob.claim_revision.conflict]
- GIVEN two claim revisions supersede the same prior claim head without superseding each other
- WHEN the reducer evaluates both revisions
- THEN both claim roots MUST remain visible and lifecycle decision planning MUST remain blocked.

### Requirement: Findings and dispositions are stable and conflict-preserving

r[cairn.verification_obligation_cob.findings] Cairn MUST represent findings with stable IDs, claim-root binding, policy-known category and severity, bounded description, optional source span, typed evidence links, responses, and append-only disposition revisions.

#### Scenario: Unresolved finding remains visible

r[cairn.verification_obligation_cob.findings.unresolved]
- GIVEN a verifier records a valid finding against the current claim root
- WHEN contributors add responses or evidence without an authorized disposition
- THEN the finding MUST remain unresolved and all responses MUST remain in history.

#### Scenario: Concurrent finding dispositions remain visible

r[cairn.verification_obligation_cob.findings.conflict]
- GIVEN authorized actors create incompatible dispositions from the same finding head
- WHEN the reducer evaluates both operations
- THEN it MUST preserve every disposition head as an explicit conflict and MUST NOT select a winner.

#### Scenario: Finding cannot be silently deleted

r[cairn.verification_obligation_cob.findings.delete]
- GIVEN a finding exists in obligation history
- WHEN an actor attempts to remove it without a policy-permitted append-only disposition
- THEN the reducer MUST reject the complete operation and preserve the finding.

### Requirement: Discharge is bound to current conflict-free facts

r[cairn.verification_obligation_cob.discharge] Cairn MUST classify an obligation as discharged only when the current claim root is conflict-free, policy-required evidence has current accepted validation summaries bound to exact evidence and claim identities, no blocking finding is unresolved or conflicted, and policy-named verifier decision rules pass.

#### Scenario: Complete current evidence permits discharge

r[cairn.verification_obligation_cob.discharge.allowed]
- GIVEN a conflict-free current claim has every required evidence role, no unresolved blocking finding, and sufficient authorized verifier decisions bound to its claim root
- WHEN Cairn evaluates the lifecycle
- THEN it MUST derive the discharged state and bind the exact decision basis.

#### Scenario: Unresolved blocking finding prevents discharge

r[cairn.verification_obligation_cob.discharge.blocker]
- GIVEN a current claim has an unresolved or conflicted policy-blocking finding
- WHEN an actor requests or the reducer evaluates discharge
- THEN discharge MUST fail with deterministic blocker details and MUST NOT select a disposition implicitly.

#### Scenario: Stale evidence prevents discharge

r[cairn.verification_obligation_cob.discharge.stale_evidence]
- GIVEN required evidence is bound only to an earlier claim root, has an incompatible claim scope, lacks a current accepted validation summary, or no longer matches the summary identity
- WHEN Cairn evaluates discharge for the current claim
- THEN that evidence MUST NOT satisfy the current discharge rule.

#### Scenario: Post-discharge invalidation preserves history

r[cairn.verification_obligation_cob.discharge.invalidation]
- GIVEN an obligation has a valid discharge decision
- WHEN an authorized invalidator records a bounded invalidation reason and evidence reference
- THEN Cairn MUST classify the obligation as invalidated while preserving the prior discharge and its exact basis.

### Requirement: Verification-obligation commands are preview-first

r[cairn.verification_obligation_cob.cli] Cairn MUST provide bounded import, status, update, evidence, finding, response, review, conflict, resolution, discharge, rejection, invalidation, and export operations for portable and direct-COB backends with explicit backend inputs.

#### Scenario: Preview writes nothing

r[cairn.verification_obligation_cob.cli.preview]
- GIVEN valid verification-obligation command inputs
- WHEN the operator omits `--execute`
- THEN Cairn MUST emit a deterministic plan and receipt without mutating stores, refs, locks, evidence artifacts, or exported files.

#### Scenario: Execution rechecks reviewed inputs

r[cairn.verification_obligation_cob.cli.execute]
- GIVEN a valid reviewed obligation plan
- WHEN the operator requests execution
- THEN Cairn MUST recheck policy, state, actor, roles, conflicts, claim root, evidence references, destination safety, and the exact plan before mutation.

### Requirement: Verification-obligation receipts preserve bounded claims

r[cairn.verification_obligation_cob.receipts] Cairn MUST emit deterministic BLAKE3-bound obligation receipts for semantic and provider identities, roles, claim roots, evidence references, findings, conflicts, lifecycle, decision basis, policy, blockers, execution mode, mutation results, and non-claims.

#### Scenario: Discharge receipt keeps bounded claims

r[cairn.verification_obligation_cob.receipts.discharge]
- GIVEN valid signed obligation history and a locally discharged lifecycle state
- WHEN Cairn emits a receipt
- THEN it MUST distinguish signature validity, configured role authorization, workflow discharge, evidence identity, evidence truth, and proof soundness.

#### Scenario: Backend success is not remote proof

r[cairn.verification_obligation_cob.receipts.backend]
- GIVEN a passing portable or direct-COB obligation operation
- WHEN Cairn states the result
- THEN it MUST NOT claim network publication, synchronization, peer convergence, remote trust, proof validity, source correctness, or release eligibility.

### Requirement: Verification-obligation COB interoperability is fixture-backed

r[cairn.verification_obligation_cob.interop] Cairn MUST maintain provenance-bound external producer and independent consumer fixtures for the pinned `xyz.onixresearch.verification-obligation` COB format.

#### Scenario: Independent consumer reads obligation history

r[cairn.verification_obligation_cob.interop.outward]
- GIVEN a signed obligation created and updated through Cairn
- WHEN a separate direct consumer evaluates its namespaced refs and action history
- THEN it MUST recover the exact type, obligation ID, roles, claim revisions, evidence references, findings, dispositions, lifecycle facts, operation IDs, and current conflicts.

#### Scenario: Malformed history fails closed

r[cairn.verification_obligation_cob.interop.negative]
- GIVEN an obligation fixture has drifted provenance, an unsupported manifest, unsigned history, a mismatched actor, malformed action, stale decision basis, or mixed type data
- WHEN Cairn or the direct consumer evaluates the fixture
- THEN evaluation MUST fail before reporting compatibility or trusted obligation state.

### Requirement: Verification-obligation support is release-gated

r[cairn.verification_obligation_cob.validation] The verification-obligation implementation MUST pass dependency, positive, negative, authority, evidence, finding, conflict, lifecycle, backend, shell, receipt, interoperability, compatibility, traceability, Tiger Style, and supported-system checks before archive.

#### Scenario: Maintained checks pass

r[cairn.verification_obligation_cob.validation.release]
- GIVEN the completed verification-obligation implementation and fixtures
- WHEN the maintained focused and repository-wide validation rails run
- THEN every required check MUST pass without missing traceability or weakened non-claims.
