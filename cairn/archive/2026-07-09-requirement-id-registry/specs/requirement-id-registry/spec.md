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
