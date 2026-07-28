# ADR Radicle COB Specification Delta

## ADDED Requirements

### Requirement: ADR has a distinct typed collaboration profile

r[cairn.adr_cob.profile] Cairn MUST represent an Architecture Decision Record with the custom type `xyz.onixresearch.adr`, an ADR-specific schema, exact policy-selected artifacts, typed lineage, review records, and lifecycle transitions.

#### Scenario: Valid ADR root preserves exact inputs

r[cairn.adr_cob.profile.root]
- GIVEN valid ADR artifacts, metadata, role sets, links, policy identity, workspace identity, and typed base revision
- WHEN Cairn initializes the ADR state
- THEN it MUST preserve every exact admitted value and derive deterministic BLAKE3 content and state identities.

#### Scenario: ADR type rejects another document kind

r[cairn.adr_cob.profile.kind]
- GIVEN an operation or stored object declares an RFD, Cairn change, or unknown document kind under the ADR type
- WHEN Cairn admits or evaluates the input
- THEN it MUST reject the mismatch without materializing ADR state.

### Requirement: ADR action authority is role-specific

r[cairn.adr_cob.authority] New ADR roots MUST bind bounded canonical immutable author, reviewer, approver, and publisher public-key sets and MUST enforce the required role for each action.

#### Scenario: Authorized roles perform bounded actions

r[cairn.adr_cob.authority.allowed]
- GIVEN a valid ADR root and an actor in the required immutable role set
- WHEN the actor signs an artifact, review, approval, numbering, publication, deprecation, or supersession action permitted by that role
- THEN the reducer MUST admit the action if all state and policy preconditions pass.

#### Scenario: Author cannot accept or publish

r[cairn.adr_cob.authority.denied]
- GIVEN an actor is an author but is not a required approver or publisher
- WHEN the actor signs an acceptance, number assignment, publication, deprecation, or supersession action
- THEN the reducer MUST reject the action even if its signature and content are valid.

#### Scenario: Ambient authority is ignored

r[cairn.adr_cob.authority.ambient]
- GIVEN a key appears only in Git configuration, a Radicle profile, an identity document, a caller argument, or a later local file
- WHEN the key signs an ADR action
- THEN those ambient facts MUST NOT grant an ADR role.

### Requirement: Accepted ADR decision content is immutable

r[cairn.adr_cob.immutability] An authorized ADR acceptance action MUST bind one conflict-free content root, after which decision source artifacts MUST NOT change in that object.

#### Scenario: Acceptance binds exact decision content

r[cairn.adr_cob.immutability.accept]
- GIVEN a draft ADR has conflict-free policy-required artifacts and valid review evidence
- WHEN an authorized approver accepts the ADR
- THEN the lifecycle revision MUST bind the exact current content root and acceptance evidence.

#### Scenario: Accepted decision content cannot change

r[cairn.adr_cob.immutability.reject_edit]
- GIVEN an ADR has an accepted content root
- WHEN any actor attempts to replace or delete a decision source artifact
- THEN the reducer MUST reject the complete operation and MUST preserve the accepted state.

#### Scenario: Replacement uses supersession

r[cairn.adr_cob.immutability.supersede]
- GIVEN an accepted decision needs a meaning-changing correction or replacement
- WHEN an authorized actor plans the change
- THEN Cairn MUST require a distinct ADR identity and explicit supersession relation instead of an in-place source edit.

### Requirement: ADR identity and publication number are separate

r[cairn.adr_cob.identity] Cairn MUST keep the stable ADR document ID independent of the provider object ID and MUST treat an optional human ADR number as a set-once publisher action.

#### Scenario: Provider object ID cannot become the ADR number

r[cairn.adr_cob.identity.provider]
- GIVEN an ADR exists in a portable envelope or direct COB store
- WHEN Cairn reports or materializes its identity
- THEN the provider object ID MUST remain separately labeled and MUST NOT become the ADR document ID or publication number.

#### Scenario: Number assignment binds registry evidence

r[cairn.adr_cob.identity.registry]
- GIVEN a valid unnumbered ADR at the policy-selected lifecycle stage and an authorized publisher
- WHEN Cairn plans number assignment
- THEN the plan MUST bind the number, explicit local registry snapshot identity, current content root, policy identity, and publisher actor.

#### Scenario: Local number validation has bounded claims

r[cairn.adr_cob.identity.non_claims]
- GIVEN a local registry comparison finds no duplicate number
- WHEN Cairn emits the numbering receipt
- THEN the receipt MUST NOT claim global uniqueness, remote reservation, publication, or future collision prevention.

### Requirement: ADR lineage is typed and explicit

r[cairn.adr_cob.lineage] Cairn MUST represent ADR supersession, superseded-by, related-decision, RFD-origin, and implementation-evidence relations with typed target kind, stable target ID, relation, and optional expected content root.

#### Scenario: Available reciprocal supersession is validated

r[cairn.adr_cob.lineage.reciprocal]
- GIVEN a new ADR supersedes an available local accepted ADR
- WHEN Cairn plans publication or supersession completion
- THEN the plan MUST identify both object states, expected content roots, relation directions, and any missing reciprocal update.

#### Scenario: Supersession does not claim cross-object atomicity

r[cairn.adr_cob.lineage.non_atomic]
- GIVEN supersession requires signed updates to two separate ADR objects
- WHEN one or both local updates succeed
- THEN the receipt MUST report each observed mutation separately and MUST NOT claim atomic remote lineage or peer convergence.

#### Scenario: Unknown target has bounded meaning

r[cairn.adr_cob.lineage.unknown]
- GIVEN an ADR relation names a syntactically valid target that is not available locally
- WHEN the pure reducer evaluates the source object
- THEN it MAY preserve the relation but MUST NOT claim target existence, acceptance, reciprocal linkage, or remote availability.

### Requirement: ADR backends preserve neutral semantics

r[cairn.adr_cob.backends] Cairn MUST support ADR portable JSON and optional direct `radicle-cob` storage while preserving equal neutral BLAKE3 identities for equivalent state.

#### Scenario: Portable and direct backends preserve semantic identity

r[cairn.adr_cob.backends.roundtrip]
- GIVEN equivalent valid ADR operations are admitted through portable and direct-COB providers
- WHEN Cairn reloads both states
- THEN exact artifacts, roles, lifecycle, lineage, conflicts, content identity, and state identity MUST match.

#### Scenario: Runtime and transport remain absent

r[cairn.adr_cob.backends.boundary]
- GIVEN ADR uses the direct COB backend
- WHEN dependencies and command surfaces are audited
- THEN no Radicle node, profile, identity document, discovery, synchronization, seeding, or transport behavior MUST enter the maintained product path.

### Requirement: ADR CLI mutations are preview-first

r[cairn.adr_cob.cli] Cairn MUST provide bounded ADR import, status, checkout, update, review, acceptance, conflict, resolution, numbering, publication, deprecation, and supersession operations with explicit backend inputs.

#### Scenario: Preview writes nothing

r[cairn.adr_cob.cli.preview]
- GIVEN valid ADR command inputs
- WHEN the operator omits `--execute`
- THEN Cairn MUST emit a deterministic plan and receipt without mutating stores, refs, locks, registries, target ADRs, or published files.

#### Scenario: Execution rechecks reviewed inputs

r[cairn.adr_cob.cli.execute]
- GIVEN a valid reviewed ADR plan
- WHEN the operator requests execution
- THEN Cairn MUST recheck policy, state, roles, conflicts, accepted root, source bytes, registry, lineage targets, destination safety, and the exact plan before mutation.

### Requirement: ADR materialization preserves canonical Git publication

r[cairn.adr_cob.materialization] Cairn MUST derive deterministic exact-byte ADR checkout and publication plans while keeping policy-selected Markdown, attachments, registry entries, and Git commits as canonical publication content.

#### Scenario: Conflict-free ADR materializes exactly

r[cairn.adr_cob.materialization.exact]
- GIVEN a conflict-free ADR state and safe absent or byte-equal destinations
- WHEN Cairn plans checkout or publication
- THEN the plan MUST map only policy-selected artifacts to safe paths with exact content and deterministic order.

#### Scenario: Conflicted ADR cannot be accepted

r[cairn.adr_cob.materialization.conflict]
- GIVEN one or more required ADR artifacts, lifecycle fields, or lineage fields have multiple current heads
- WHEN Cairn plans acceptance, number assignment, publication, deprecation, or supersession
- THEN planning MUST fail with deterministic conflict details and MUST NOT select an implicit winner.

#### Scenario: Unsafe destination fails closed

r[cairn.adr_cob.materialization.unsafe]
- GIVEN a destination is symlinked, divergent, partial, oversized, non-UTF-8, outside the selected root, or changed after preview
- WHEN ADR execution is requested
- THEN Cairn MUST report a blocker and MUST NOT overwrite local work.

### Requirement: ADR receipts preserve bounded claims

r[cairn.adr_cob.receipts] Cairn MUST emit deterministic BLAKE3-bound ADR receipts for backend observations, identities, roles, immutability, conflicts, lifecycle, lineage, numbering, publication, policy, blockers, execution mode, and mutation results.

#### Scenario: Accepted state is not correctness proof

r[cairn.adr_cob.receipts.acceptance]
- GIVEN a valid signed ADR history and locally accepted lifecycle state
- WHEN Cairn emits a receipt
- THEN it MUST distinguish signature validity, configured role authorization, lifecycle state, evidence truth, and architectural correctness.

#### Scenario: Backend success is not remote proof

r[cairn.adr_cob.receipts.backend]
- GIVEN a passing portable or direct-COB ADR operation
- WHEN Cairn states the result
- THEN it MUST NOT claim network publication, synchronization, reciprocal remote lineage, peer convergence, remote trust, global numbering, source correctness, or release eligibility.

### Requirement: ADR COB interoperability is fixture-backed

r[cairn.adr_cob.interop] Cairn MUST maintain provenance-bound external producer and independent consumer fixtures for the pinned `xyz.onixresearch.adr` COB format.

#### Scenario: Independent consumer reads ADR history

r[cairn.adr_cob.interop.outward]
- GIVEN a signed ADR object created and updated through Cairn
- WHEN a separate direct consumer evaluates its namespaced refs and action history
- THEN it MUST recover the exact type, document ID, roles, artifacts, review, lifecycle, lineage, operation IDs, accepted root, and current conflicts.

#### Scenario: Malformed history fails closed

r[cairn.adr_cob.interop.negative]
- GIVEN an ADR fixture has drifted provenance, an unsupported manifest, unsigned history, a mismatched actor, post-acceptance edits, malformed actions, or mixed type data
- WHEN Cairn or the direct consumer evaluates the fixture
- THEN evaluation MUST fail before reporting compatibility or trusted ADR state.

### Requirement: ADR support is release-gated

r[cairn.adr_cob.validation] The ADR implementation MUST pass dependency, positive, negative, authority, immutability, lineage, backend, materialization, receipt, interoperability, lifecycle, traceability, Tiger Style, and supported-system checks before archive.

#### Scenario: Maintained checks pass

r[cairn.adr_cob.validation.release]
- GIVEN the completed ADR implementation and fixtures
- WHEN the maintained focused and repository-wide validation rails run
- THEN every required check MUST pass without missing traceability or weakened non-claims.
