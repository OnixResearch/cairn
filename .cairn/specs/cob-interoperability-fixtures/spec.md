# Cob Interoperability Fixtures Specification

## Purpose

Defines the `cob-interoperability-fixtures` capability.

## Requirements

### Requirement: Provenance-bound external producer fixture
r[cairn.cob_interop.fixture] The system MUST maintain an offline Git bundle produced without Cairn crates through the pinned upstream Radicle Git COB storage adapter, together with typed provenance that binds the retained producer source, exact library versions, expected refs and object identities, and BLAKE3 hashes.

#### Scenario: Fixture identity is current
r[cairn.cob_interop.fixture.current]
- GIVEN the committed producer source, Git bundle, Nickel provenance, and generated JSON
- WHEN the dedicated compatibility check recomputes source and bundle identities and exports the Nickel record
- THEN every declared BLAKE3 hash, version, ref, object ID, operation ID, actor, and generated JSON byte MUST match

#### Scenario: Generation remains outside product dependencies
r[cairn.cob_interop.fixture.external]
- GIVEN the retained reference producer mentions the upstream umbrella `radicle` crate
- WHEN Cairn's workspace and package dependency trees are audited
- THEN `radicle` and `radicle-node` MUST remain absent from maintained product and check dependency graphs

### Requirement: Cairn consumes and extends the external history
r[cairn.cob_interop.inward] Cairn MUST load the reference-produced signed root through its production adapter and append an authorized signed update while preserving the reference operation and exact declared state.

#### Scenario: External root receives a Cairn delegate update
r[cairn.cob_interop.inward.update]
- GIVEN a fresh repository populated from the valid static bundle and the deterministic fixture delegate key
- WHEN Cairn inspects the external change and appends a proposal update
- THEN the reloaded object MUST retain the reference root, include the Cairn update, preserve immutable authority, and match the planned content root

### Requirement: Direct consumer evaluates Cairn output
r[cairn.cob_interop.outward] A test-only direct consumer MUST discover namespaced refs, load signed changes, decode Cairn action JSON, and evaluate Cairn-produced create and update history without calling Cairn's COB repository, evaluator, inspection, or reducer implementation.

#### Scenario: Independent consumer reads Cairn history
r[cairn.cob_interop.outward.roundtrip]
- GIVEN a signed object created and updated through Cairn
- WHEN the direct consumer loads it through its own read-only storage and action reducer
- THEN it MUST recover the exact change ID, creator, delegates, artifacts, operation IDs, and current proposal

### Requirement: Malformed fixtures fail closed
r[cairn.cob_interop.negative] The interoperability suite MUST reject fixture identity drift, missing declared refs, unsigned operation history, and unsupported COB manifest versions before reporting compatibility.

#### Scenario: Bundle bytes drift
r[cairn.cob_interop.negative.hash]
- GIVEN fixture bytes that do not match typed provenance
- WHEN fixture identity is validated
- THEN validation MUST fail before the bundle is imported

#### Scenario: Unsigned history is injected
r[cairn.cob_interop.negative.unsigned]
- GIVEN a fixture repository whose declared namespaced ref targets an ordinary unsigned commit
- WHEN Cairn inspects the change
- THEN inspection MUST fail without ignoring the malformed actor history

#### Scenario: Manifest version is unsupported
r[cairn.cob_interop.negative.version]
- GIVEN an otherwise well-shaped root entry with an unsupported COB manifest version
- WHEN Cairn's evaluator initializes the object
- THEN evaluation MUST return an explicit version mismatch

### Requirement: Runtime boundary remains absent
r[cairn.cob_interop.boundary] The compatibility gate and documentation MUST preserve fixture provenance and bounded claims without requiring profiles, identity documents, RIDs, nodes, peers, sockets, discovery, seeding, announce, connect/fetch, synchronization, or convergence behavior.

#### Scenario: Dedicated gate stays offline
r[cairn.cob_interop.boundary.offline]
- GIVEN the repository source and vendored Cargo dependencies
- WHEN the dedicated Nix compatibility check runs
- THEN it MUST validate provenance and both interoperability directions without network access or Radicle runtime processes

### Requirement: Interoperability evidence is release-gated
r[cairn.cob_interop.validation] The maintained release rails MUST run positive and negative interoperability checks, dependency auditing, lifecycle validation, traceability, and Nix validation before the change is archived.

#### Scenario: Required checks pass
r[cairn.cob_interop.validation.release]
- GIVEN the completed fixture and test implementation
- WHEN the focused and repository-wide verification rails run
- THEN every required check MUST pass with explicit bounded non-claims and no missing or dangling traceability markers
