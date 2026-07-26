# Requirement ID Registry Specification Delta

## ADDED Requirements

### Requirement: Cairn exports a content-bound requirement registry

r[cairn.requirement_registry.content_bound_export]

Cairn MUST export a versioned registry that binds each accepted requirement to repository identity, declared revision, specification content, policy identity, and a safe source location.

#### Scenario: Current accepted requirements export deterministically

- GIVEN valid accepted specifications, generated policy, repository identity, and declared revision
- WHEN the operator exports the requirement registry twice
- THEN both exports MUST contain identical sorted rows and BLAKE3 identities
- AND each row MUST bind its requirement ID and requirement-body content

#### Scenario: Duplicate requirement identity fails

- GIVEN two accepted requirement sections declare the same requirement ID
- WHEN the registry core evaluates the supplied specifications
- THEN evaluation MUST fail with a deterministic duplicate-ID issue
- AND Cairn MUST NOT emit a successful registry

### Requirement: Cairn verifies a registry against the current root

r[cairn.requirement_registry.current_root_verification]

Cairn MUST recompute current accepted-specification and policy identities before accepting a supplied requirement registry.

#### Scenario: Current registry verifies

- GIVEN a registry exported from the current accepted specifications and policy
- WHEN the operator verifies it against the same capability root
- THEN verification MUST pass with matching row, specification, policy, and registry identities
- AND the receipt MUST identify the declared repository revision class

#### Scenario: Stale requirement body fails

- GIVEN an accepted requirement body changed after registry export
- WHEN the operator verifies the old registry against the current root
- THEN verification MUST fail with a stale-requirement issue
- AND the diagnostic MUST identify the requirement without printing unbounded source text

#### Scenario: Wrong repository binding fails

- GIVEN a valid registry is presented under a different expected repository identity
- WHEN verification compares the registry with the explicit expected identity
- THEN verification MUST fail before downstream evidence consumption
- AND no same-named requirement ID MUST bypass the mismatch

### Requirement: Registry use preserves claim boundaries

r[cairn.requirement_registry.downstream_boundary]

Cairn MUST label the registry as identity and linkage evidence only, and evidence publication MUST preserve that boundary.

#### Scenario: Registry is carried in an evidence manifest

- GIVEN a valid requirement registry is included in a Cairn evidence bundle
- WHEN the evidence manifest is generated and verified
- THEN the manifest MUST bind the exact registry bytes and receipt identity
- AND the artifact role MUST remain distinct from implementation or verification evidence

#### Scenario: Revision declaration does not become VCS proof

- GIVEN a registry carries a caller-supplied Git, Jujutsu, or opaque revision value
- WHEN the registry passes structural verification
- THEN Cairn MUST retain a non-claim that revision authenticity was not established
- AND it MUST NOT report requirement satisfaction, source correctness, or release eligibility
