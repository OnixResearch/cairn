# Requirement ID Registry Specification

## Purpose

Defines a cross-repo registry for `r[...]` requirement IDs referenced by stack evidence.

## Requirements

### Requirement: Requirement ID registry contract
r[cairn.requirement_id_registry.contract] Cairn MUST support a registry contract that maps requirement IDs and namespaces to owning repos, accepted spec paths, status, aliases, and allowed consumer roles.

#### Scenario: Owned ID resolves
r[cairn.requirement_id_registry.fixtures.positive]
- GIVEN a registry row names an owned requirement ID, owner repo, accepted spec path, active status, and allowed consumer role
- WHEN registry validation runs
- THEN the row MUST pass and the ID MUST resolve to its owner.

#### Scenario: Unknown ID fails
r[cairn.requirement_id_registry.fixtures.negative]
- GIVEN evidence references an ID absent from the registry and not covered by an explicit local-owner policy
- WHEN registry validation runs
- THEN validation MUST fail with an unknown-requirement diagnostic.

### Requirement: Registry validation receipt
r[cairn.requirement_id_registry.validation] Cairn MUST emit a deterministic receipt for registry validation and reference resolution.

#### Scenario: Receipt binds registry identity
r[cairn.requirement_id_registry.validation.receipt]
- GIVEN a registry and evidence references
- WHEN validation runs
- THEN the receipt MUST include registry hash, checked reference count, dangling reference count, stale alias count, policy hash, and receipt hash.

### Requirement: Registry documentation
r[cairn.requirement_id_registry.docs] Cairn documentation MUST state that registry resolution proves ownership/linkage only and not requirement satisfaction.

#### Scenario: Non-claim is visible
r[cairn.requirement_id_registry.docs.non_claims]
- GIVEN a registry report says an ID resolved
- WHEN the supported claim is rendered
- THEN it MUST state that resolution does not prove implementation correctness, test coverage, or release eligibility.

### Requirement: Final validation
r[cairn.requirement_id_registry.final_validation] The change MUST include positive and negative fixtures plus focused validation evidence before archive.

#### Scenario: Fixture suite covers ID health
r[cairn.requirement_id_registry.final_validation.fixtures]
- GIVEN valid, unknown, wrong-owner, stale-alias, and namespace-overreach fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass and invalid fixtures MUST fail closed.

### Requirement: Traceability marker core contract
r[cairn.requirement_id_registry.traceability_marker_core.contract] Cairn MUST define pure traceability marker contracts for requirement ID syntax, marker verbs, link DTOs, optional registry inputs, and bounded non-claim text.

#### Scenario: Valid marker parses
r[cairn.requirement_id_registry.traceability_marker_core.fixtures.positive]
- GIVEN a marker uses supported requirement ID syntax, a supported verb, a valid link target, and an allowed namespace
- WHEN traceability marker validation runs
- THEN validation MUST accept the marker and return a typed link DTO.

#### Scenario: Invalid marker fails closed
r[cairn.requirement_id_registry.traceability_marker_core.fixtures.negative]
- GIVEN a marker has malformed ID syntax, unsupported verb, wrong namespace, dangling ID, duplicate link where disallowed, missing owner, or satisfaction overclaim
- WHEN traceability marker validation runs
- THEN validation MUST fail with deterministic diagnostics naming the invalid marker field.

### Requirement: Marker verbs are vocabulary-bound
r[cairn.requirement_id_registry.traceability_marker_core.verbs] Cairn MUST validate marker verbs against a declared vocabulary rather than treating all marker prefixes as equivalent.

#### Scenario: Unsupported verb is rejected
r[cairn.requirement_id_registry.traceability_marker_core.verbs.unsupported]
- GIVEN a marker uses a verb absent from the declared vocabulary
- WHEN the marker parser validates the marker
- THEN validation MUST reject the marker before it is used as implementation, verification, dependency, or related-evidence linkage.

### Requirement: Traceability marker validation is pure
r[cairn.requirement_id_registry.traceability_marker_core.validation] Cairn MUST implement traceability marker parsing and link validation as pure deterministic logic over in-memory marker strings, vocabulary rows, and optional registry rows.

#### Scenario: Shell owns marker discovery
r[cairn.requirement_id_registry.traceability_marker_core.validation.shell]
- GIVEN a CLI or downstream tool discovers marker strings in source files, lifecycle artifacts, or generated reports
- WHEN traceability marker validation is invoked
- THEN file walking, source span capture, policy loading, stdout/stderr, and report rendering MUST remain outside the marker core.

### Requirement: Compatibility adapters preserve current marker strings
r[cairn.requirement_id_registry.traceability_marker_core.adapters] Cairn SHOULD expose adapters for current Cairn, Octet, and Valence marker spellings.

#### Scenario: Existing marker spelling adapts
r[cairn.requirement_id_registry.traceability_marker_core.adapters.shape]
- GIVEN an existing source comment or lifecycle artifact uses a supported current marker spelling
- WHEN an adapter parses it
- THEN the adapter SHOULD preserve the public string while normalizing the marker into the shared DTO model.

### Requirement: Traceability marker core remains bounded
r[cairn.requirement_id_registry.traceability_marker_core.docs] Passing marker validation MUST NOT prove requirement satisfaction, implementation correctness, verification coverage, proof soundness, release eligibility, lifecycle readiness, or downstream behavioral correctness.

#### Scenario: Boundary is visible
r[cairn.requirement_id_registry.traceability_marker_core.final_validation]
- GIVEN a marker validation report passes
- WHEN an operator reads the supported claim
- THEN the report MUST state that validation proves only marker syntax, vocabulary conformance, and declared linkage health.
