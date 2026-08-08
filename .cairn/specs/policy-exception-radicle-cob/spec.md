# Policy Exception Radicle Cob Specification

## Purpose

Defines the `policy-exception-radicle-cob` capability.

## Requirements

### Requirement: Policy-exception policy is typed

r[cairn.policy_exception_cob.policy] Cairn MUST define typed Nickel profiles for exception scope grammar, rule identity, roles, approval rules, finite validity bounds, observation sources, lifecycle transitions, and named protocol limits.

#### Scenario: Valid profile exports deterministically

r[cairn.policy_exception_cob.policy.deterministic]
- GIVEN a valid policy-exception profile
- WHEN Cairn exports the runtime projection
- THEN the generated profile MUST be deterministic and bound to its Nickel source identity.

#### Scenario: Invalid profile fails closed

r[cairn.policy_exception_cob.policy.invalid]
- GIVEN a profile has ambiguous wildcard meaning, duplicate roles, impossible thresholds, unbounded validity, unknown observation sources, or invalid transitions
- WHEN Cairn validates the Nickel source
- THEN export MUST fail and MUST NOT replace the generated runtime profile.

### Requirement: Policy exception has a distinct typed profile

r[cairn.policy_exception_cob.profile] Cairn MUST represent a policy exception with the custom type `xyz.onixresearch.policy-exception`, a stable exception ID, exact policy identity, stable rule ID, typed scope, rationale, compensating controls, finite validity bounds, role sets, and lifecycle facts.

#### Scenario: Valid root preserves exact inputs

r[cairn.policy_exception_cob.profile.root]
- GIVEN valid policy, rule, scope, rationale, control, validity, role, workspace, and typed base revision inputs
- WHEN Cairn initializes policy-exception state
- THEN it MUST preserve every exact admitted value and derive deterministic BLAKE3 request, content, and state identities.

#### Scenario: Policy-exception type rejects another workflow kind

r[cairn.policy_exception_cob.profile.kind]
- GIVEN an operation or stored object declares a verification obligation, release decision, document, Cairn change, or unknown kind under the policy-exception type
- WHEN Cairn admits or evaluates the input
- THEN it MUST reject the mismatch without materializing trusted exception state.

#### Scenario: Provider object ID remains separate

r[cairn.policy_exception_cob.profile.identity]
- GIVEN an exception exists in portable or direct-COB storage
- WHEN Cairn reports or exports it
- THEN the stable exception ID, policy ID, rule ID, evidence IDs, provider object ID, and operation IDs MUST remain separately typed.

### Requirement: Exception scope is exact and canonical

r[cairn.policy_exception_cob.scope] Cairn MUST bind every request root to one policy identity, one stable rule ID, canonical typed subject and action selectors, rationale, compensating controls, owners, and finite validity bounds.

#### Scenario: Exact scope round trips

r[cairn.policy_exception_cob.scope.exact]
- GIVEN a valid bounded exception scope
- WHEN Cairn canonicalizes, stores, and reloads the request
- THEN every selector and control MUST remain exact and deterministically ordered without scope expansion.

#### Scenario: Implicit wildcard scope fails closed

r[cairn.policy_exception_cob.scope.wildcard]
- GIVEN a request omits a required selector or uses a wildcard not defined by the selected policy profile
- WHEN Cairn validates the scope
- THEN validation MUST fail and MUST NOT interpret the input as an unrestricted scope.

#### Scenario: Policy or rule drift is visible

r[cairn.policy_exception_cob.scope.drift]
- GIVEN an approved request names one policy content identity and rule ID
- WHEN a consumer supplies a different current policy identity or missing rule
- THEN Cairn MUST classify the request as not applicable to that supplied policy without rewriting exception history.

### Requirement: Policy-exception authority is role-specific

r[cairn.policy_exception_cob.authority] New exception roots MUST bind bounded canonical immutable requester, reviewer, approver, revoker, and validity-observer public-key sets and MUST enforce the policy-required role for every action.

#### Scenario: Authorized roles perform bounded actions

r[cairn.policy_exception_cob.authority.allowed]
- GIVEN valid exception state and an actor in the required immutable role set
- WHEN the actor signs a policy-permitted request, review, decision, observation, revocation, closure, or resolution action
- THEN the reducer MUST admit the action if all state and policy preconditions pass.

#### Scenario: Requester cannot approve an exception

r[cairn.policy_exception_cob.authority.denied]
- GIVEN an actor is a requester but is not a required approver
- WHEN the actor signs an approval decision
- THEN the reducer MUST reject the action even if its signature and request content are valid.

#### Scenario: Ambient authority is ignored

r[cairn.policy_exception_cob.authority.ambient]
- GIVEN a key appears only in Git configuration, a Radicle profile, an identity document, a caller argument, or a later local file
- WHEN the key signs an exception action
- THEN those ambient facts MUST NOT grant an exception role.

### Requirement: Approval binds one exact request root

r[cairn.policy_exception_cob.approval] Cairn MUST bind every review finding and decision to one exact request root, and an approval MUST satisfy the selected policy's named role and threshold rules without unresolved request or decision conflicts.

#### Scenario: Complete approval binds exact scope and controls

r[cairn.policy_exception_cob.approval.allowed]
- GIVEN a conflict-free request root has required review evidence and sufficient authorized approval facts
- WHEN Cairn evaluates the lifecycle
- THEN it MUST preserve the approved state and bind the exact policy, rule, scope, rationale, controls, validity bounds, and decision basis.

#### Scenario: Request revision makes prior approval inapplicable

r[cairn.policy_exception_cob.approval.stale]
- GIVEN an exception has approval facts for one request root
- WHEN an authorized requester creates a meaning-changing request revision
- THEN Cairn MUST retain the prior decisions but MUST NOT apply them to the new request root.

#### Scenario: Conflicting decisions block applicability

r[cairn.policy_exception_cob.approval.conflict]
- GIVEN concurrent authorized decisions create incompatible approval and rejection heads
- WHEN the reducer evaluates both operations
- THEN both decisions MUST remain visible and exception applicability MUST remain blocked until complete-basis resolution.

### Requirement: Validity uses explicit observations without ambient time

r[cairn.policy_exception_cob.validity] Cairn MUST validate finite exception bounds deterministically and MUST evaluate current validity only from explicit typed observations containing source identity, observed value, evidence identity, actor, and authorization facts.

#### Scenario: Pure reduction does not read a clock

r[cairn.policy_exception_cob.validity.no_clock]
- GIVEN exception operations and explicit observation facts
- WHEN the pure reducer validates bounds or evaluates validity
- THEN it MUST return the same result without filesystem, environment, process, network, or clock access.

#### Scenario: Valid observation supports bounded applicability

r[cairn.policy_exception_cob.validity.current]
- GIVEN an approved request and a policy-accepted observation within its finite validity bounds
- WHEN Cairn evaluates validity
- THEN it MUST classify the bounds as currently satisfied for that supplied observation only.

#### Scenario: Missing or malformed observation fails closed

r[cairn.policy_exception_cob.validity.invalid]
- GIVEN current applicability requires an observation that is absent, malformed, unauthorized, from an unknown source, or outside the validity bounds
- WHEN Cairn evaluates validity
- THEN it MUST classify the exception as not currently applicable and report deterministic reasons.

#### Scenario: Conflicting validity observations block applicability

r[cairn.policy_exception_cob.validity.conflict]
- GIVEN concurrent policy-accepted observations are non-equivalent and the selected profile cannot order them
- WHEN Cairn evaluates validity
- THEN every observation MUST remain visible as an explicit conflict and applicability MUST remain blocked.

#### Scenario: Expiration cannot be reversed by an older observation

r[cairn.policy_exception_cob.validity.terminal]
- GIVEN a policy-accepted observation at or beyond the expiration bound has created a terminal expiration fact
- WHEN an actor later supplies an older within-bounds observation
- THEN Cairn MUST preserve expiration and MUST NOT restore applicability.

### Requirement: Applicability remains separate from policy enforcement

r[cairn.policy_exception_cob.applicability] Cairn MUST classify an exception as applicable only when policy identity, rule, scope, approved request root, decision basis, validity observation, revocation state, expiration state, and conflict state all satisfy the selected profile.

#### Scenario: Exact approved exception can be classified applicable

r[cairn.policy_exception_cob.applicability.allowed]
- GIVEN all required current facts match one conflict-free approved exception
- WHEN Cairn evaluates applicability for an exact supplied policy, subject, action, and observation
- THEN it MUST return an applicable classification with the complete decision basis and bounded non-claims.

#### Scenario: Expired or revoked exception is inapplicable

r[cairn.policy_exception_cob.applicability.terminal]
- GIVEN an exception has an authorized revocation or a valid observation at or beyond its expiration bound
- WHEN Cairn evaluates applicability
- THEN it MUST classify the exception as inapplicable while preserving its prior approval history.

#### Scenario: Approved state does not grant authority

r[cairn.policy_exception_cob.applicability.non_authority]
- GIVEN an exception has a valid approved and applicable classification
- WHEN a policy consumer inspects the result
- THEN Cairn MUST state that the consumer retains enforcement authority and the COB does not grant a capability or bypass policy by itself.

### Requirement: Policy-exception backends preserve neutral semantics

r[cairn.policy_exception_cob.backends] Cairn MUST support policy-exception portable JSON and optional direct `radicle-cob` storage while preserving equal neutral BLAKE3 identities for equivalent state.

#### Scenario: Portable and direct backends preserve semantic identity

r[cairn.policy_exception_cob.backends.roundtrip]
- GIVEN equivalent valid policy-exception operations are admitted through portable and direct-COB providers
- WHEN Cairn reloads both states
- THEN exact requests, roles, findings, decisions, observations, lifecycle facts, conflicts, content identity, and state identity MUST match.

#### Scenario: Runtime and transport remain absent

r[cairn.policy_exception_cob.backends.boundary]
- GIVEN a policy exception uses the direct COB backend
- WHEN dependencies and command surfaces are audited
- THEN no Radicle node, profile, identity document, discovery, synchronization, seeding, or transport behavior MUST enter the maintained product path.

### Requirement: Policy-exception commands are preview-first

r[cairn.policy_exception_cob.cli] Cairn MUST provide bounded request, status, update, review, approve, reject, observe-validity, revoke, expire, close, conflict, resolve, supersede, and export operations with explicit backend, policy, and observation inputs.

#### Scenario: Preview writes nothing

r[cairn.policy_exception_cob.cli.preview]
- GIVEN valid policy-exception command inputs
- WHEN the operator omits `--execute`
- THEN Cairn MUST emit a deterministic plan and receipt without mutating stores, refs, locks, policy files, observation sources, or exported files.

#### Scenario: Execution rechecks reviewed inputs

r[cairn.policy_exception_cob.cli.execute]
- GIVEN a valid reviewed exception plan
- WHEN the operator requests execution
- THEN Cairn MUST recheck policy, state, roles, scope, request root, decisions, conflicts, observations, destination safety, and the exact plan before mutation.

### Requirement: Policy-exception receipts preserve bounded claims

r[cairn.policy_exception_cob.receipts] Cairn MUST emit deterministic BLAKE3-bound receipts for identities, scope, controls, roles, findings, decisions, observations, applicability, conflicts, policy, provider observations, blockers, execution mode, mutation results, and non-claims.

#### Scenario: Applicability receipt does not become enforcement proof

r[cairn.policy_exception_cob.receipts.applicability]
- GIVEN valid signed exception history and a locally applicable classification
- WHEN Cairn emits a receipt
- THEN it MUST distinguish signature validity, configured role authorization, supplied time observation, applicability, consumer enforcement, and control effectiveness.

#### Scenario: Backend success is not remote proof

r[cairn.policy_exception_cob.receipts.backend]
- GIVEN a passing portable or direct-COB exception operation
- WHEN Cairn states the result
- THEN it MUST NOT claim network publication, synchronization, remote trust, globally current time, organization-wide approval, policy bypass, or release eligibility.

### Requirement: Policy-exception COB interoperability is fixture-backed

r[cairn.policy_exception_cob.interop] Cairn MUST maintain provenance-bound external producer and independent consumer fixtures for the pinned `xyz.onixresearch.policy-exception` COB format.

#### Scenario: Independent consumer reads exception history

r[cairn.policy_exception_cob.interop.outward]
- GIVEN a signed exception created and updated through Cairn
- WHEN a separate direct consumer evaluates its namespaced refs and action history
- THEN it MUST recover the exact type, exception ID, policy and rule IDs, scope, controls, roles, findings, decisions, observations, lifecycle facts, operation IDs, and current conflicts.

#### Scenario: Malformed history fails closed

r[cairn.policy_exception_cob.interop.negative]
- GIVEN an exception fixture has drifted provenance, an unsupported manifest, unsigned history, a mismatched actor, stale approval, malformed observation, unauthorized terminal fact, or mixed type data
- WHEN Cairn or the direct consumer evaluates the fixture
- THEN evaluation MUST fail before reporting compatibility or trusted exception state.

### Requirement: Policy-exception support is release-gated

r[cairn.policy_exception_cob.validation] The policy-exception implementation MUST pass dependency, positive, negative, authority, scope, approval, validity, applicability, conflict, backend, shell, receipt, interoperability, compatibility, traceability, Tiger Style, and supported-system checks before archive.

#### Scenario: Maintained checks pass

r[cairn.policy_exception_cob.validation.release]
- GIVEN the completed policy-exception implementation and fixtures
- WHEN the maintained focused and repository-wide validation rails run
- THEN every required check MUST pass without missing traceability or weakened non-claims.
