# RFD Radicle COB Specification Delta

## ADDED Requirements

### Requirement: RFD has a distinct typed collaboration profile

r[cairn.rfd_cob.profile] Cairn MUST represent an RFD with the custom type `xyz.onixresearch.rfd`, an RFD-specific schema, exact policy-selected artifacts, typed links, bounded discussion, and lifecycle transitions.

#### Scenario: Valid RFD root preserves exact inputs

r[cairn.rfd_cob.profile.root]
- GIVEN valid RFD artifacts, metadata, role sets, links, policy identity, workspace identity, and typed base revision
- WHEN Cairn initializes the RFD state
- THEN it MUST preserve every exact admitted value and derive deterministic BLAKE3 content and state identities.

#### Scenario: RFD type rejects another document kind

r[cairn.rfd_cob.profile.kind]
- GIVEN an operation or stored object declares an ADR, Cairn change, or unknown document kind under the RFD type
- WHEN Cairn admits or evaluates the input
- THEN it MUST reject the mismatch without materializing RFD state.

### Requirement: RFD action authority is role-specific

r[cairn.rfd_cob.authority] New RFD roots MUST bind bounded canonical immutable author, participant, reviewer, and publisher public-key sets and MUST enforce the required role for each action.

#### Scenario: Authorized roles perform bounded actions

r[cairn.rfd_cob.authority.allowed]
- GIVEN a valid RFD root and an actor in the required immutable role set
- WHEN the actor signs an artifact, discussion, review, numbering, or publication action permitted by that role
- THEN the reducer MUST admit the action if all state and policy preconditions pass.

#### Scenario: Author cannot accept or publish

r[cairn.rfd_cob.authority.denied]
- GIVEN an actor is an author but is not a required reviewer or publisher
- WHEN the actor signs an acceptance, number assignment, or publication action
- THEN the reducer MUST reject the action even if its signature and content are valid.

#### Scenario: Ambient authority is ignored

r[cairn.rfd_cob.authority.ambient]
- GIVEN a key appears only in Git configuration, a Radicle profile, an identity document, a caller argument, or a later local file
- WHEN the key signs an RFD action
- THEN those ambient facts MUST NOT grant an RFD role.

### Requirement: RFD identity and publication number are separate

r[cairn.rfd_cob.identity] Cairn MUST keep the stable RFD document ID independent of the provider object ID and MUST treat an optional human publication number as a set-once publisher action.

#### Scenario: Provider object ID cannot become the RFD number

r[cairn.rfd_cob.identity.provider]
- GIVEN an RFD exists in a portable envelope or direct COB store
- WHEN Cairn reports or materializes its identity
- THEN the provider object ID MUST remain separately labeled and MUST NOT become the RFD document ID or publication number.

#### Scenario: Publication number uses explicit registry evidence

r[cairn.rfd_cob.identity.registry]
- GIVEN an accepted unnumbered RFD and an authorized publisher
- WHEN Cairn plans number assignment
- THEN the plan MUST bind the selected number, explicit local registry snapshot identity, current content root, policy identity, and publisher actor.

#### Scenario: Local number validation has bounded claims

r[cairn.rfd_cob.identity.non_claims]
- GIVEN a local registry comparison finds no duplicate number
- WHEN Cairn emits the numbering receipt
- THEN the receipt MUST NOT claim global uniqueness, remote reservation, publication, or future collision prevention.

### Requirement: RFD CLI mutations are preview-first

r[cairn.rfd_cob.cli] Cairn MUST provide bounded RFD import, status, checkout, update, conflict, resolution, numbering, and publication operations for portable and direct-COB backends with explicit backend inputs.

#### Scenario: Preview writes nothing

r[cairn.rfd_cob.cli.preview]
- GIVEN valid RFD command inputs
- WHEN the operator omits `--execute`
- THEN Cairn MUST emit a deterministic plan and receipt without mutating stores, refs, locks, registries, or published files.

#### Scenario: Execution rechecks reviewed inputs

r[cairn.rfd_cob.cli.execute]
- GIVEN a valid reviewed RFD plan
- WHEN the operator requests execution
- THEN Cairn MUST recheck policy, state, actor, roles, conflicts, source bytes, registry snapshot, destination safety, and the exact plan before mutation.

### Requirement: RFD materialization preserves canonical Git publication

r[cairn.rfd_cob.materialization] Cairn MUST derive deterministic exact-byte RFD checkout and publication plans while keeping policy-selected Markdown, attachments, registry entries, and Git commits as canonical publication content.

#### Scenario: Conflict-free RFD materializes exactly

r[cairn.rfd_cob.materialization.exact]
- GIVEN a conflict-free RFD state and safe absent or byte-equal destinations
- WHEN Cairn plans checkout or publication
- THEN the plan MUST map only policy-selected artifacts to safe paths with exact content and deterministic order.

#### Scenario: Conflicted RFD cannot publish

r[cairn.rfd_cob.materialization.conflict]
- GIVEN one or more RFD artifacts, links, or lifecycle fields have multiple current heads
- WHEN Cairn plans number assignment or publication
- THEN planning MUST fail with deterministic conflict details and MUST NOT produce a partial publication plan.

#### Scenario: Unsafe destination fails closed

r[cairn.rfd_cob.materialization.unsafe]
- GIVEN a destination is symlinked, divergent, partial, oversized, non-UTF-8, outside the selected root, or changed after preview
- WHEN RFD execution is requested
- THEN Cairn MUST report a blocker and MUST NOT overwrite local work.

### Requirement: RFD receipts preserve bounded claims

r[cairn.rfd_cob.receipts] Cairn MUST emit deterministic BLAKE3-bound RFD receipts for backend observations, document and provider identities, roles, conflicts, links, lifecycle, numbering, publication plans, policy, blockers, execution mode, and mutation results.

#### Scenario: Signed history does not become acceptance proof

r[cairn.rfd_cob.receipts.acceptance]
- GIVEN a valid signed RFD history and a locally accepted lifecycle state
- WHEN Cairn emits a receipt
- THEN it MUST distinguish signature validity, configured role authorization, lifecycle state, evidence truth, and semantic review quality.

#### Scenario: Backend success is not remote proof

r[cairn.rfd_cob.receipts.backend]
- GIVEN a passing portable or direct-COB RFD operation
- WHEN Cairn states the result
- THEN it MUST NOT claim network publication, synchronization, peer convergence, remote trust, global numbering, link truth, source correctness, or release eligibility.

### Requirement: RFD COB interoperability is fixture-backed

r[cairn.rfd_cob.interop] Cairn MUST maintain provenance-bound external producer and independent consumer fixtures for the pinned `xyz.onixresearch.rfd` COB format.

#### Scenario: Independent consumer reads RFD history

r[cairn.rfd_cob.interop.outward]
- GIVEN a signed RFD object created and updated through Cairn
- WHEN a separate direct consumer evaluates its namespaced refs and action history
- THEN it MUST recover the exact type, document ID, roles, artifacts, discussion, links, lifecycle, operation IDs, and current conflict state.

#### Scenario: Malformed history fails closed

r[cairn.rfd_cob.interop.negative]
- GIVEN an RFD fixture has drifted provenance, an unsupported manifest, unsigned history, a mismatched actor, malformed actions, or mixed type data
- WHEN Cairn or the direct consumer evaluates the fixture
- THEN evaluation MUST fail before reporting compatibility or trusted RFD state.

### Requirement: RFD support is release-gated

r[cairn.rfd_cob.validation] The RFD implementation MUST pass positive and negative core, shell, authority, conflict, publication, receipt, interoperability, compatibility, lifecycle, traceability, Tiger Style, and supported-system checks before archive.

#### Scenario: Maintained checks pass

r[cairn.rfd_cob.validation.release]
- GIVEN the completed RFD implementation and fixtures
- WHEN the maintained focused and repository-wide validation rails run
- THEN every required check MUST pass without missing traceability or weakened non-claims.
