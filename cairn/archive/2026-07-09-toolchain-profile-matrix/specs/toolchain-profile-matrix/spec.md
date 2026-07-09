# Toolchain Profile Matrix Specification

## Purpose

Defines a workspace matrix for repo validation profiles and toolchain expectations.

## Requirements

### Requirement: Toolchain profile matrix contract
r[cairn.toolchain_profile_matrix.contract] Cairn MUST define a matrix contract for repo profiles, toolchain pins, validation commands, expected artifacts, purpose, and non-claim boundaries.

#### Scenario: Complete profile passes
r[cairn.toolchain_profile_matrix.fixtures.positive]
- GIVEN a profile row names repo, profile ID, purpose, toolchain pins, validation command, expected artifact class, and non-claims
- WHEN matrix validation runs
- THEN the row MUST pass and preserve the profile ID.

#### Scenario: Incomplete profile fails
r[cairn.toolchain_profile_matrix.fixtures.negative]
- GIVEN a profile row omits toolchain pins, command, purpose, or non-claims
- WHEN matrix validation runs
- THEN validation MUST fail with deterministic diagnostics.

### Requirement: Matrix validation receipt
r[cairn.toolchain_profile_matrix.validation] Cairn MUST emit a deterministic receipt for profile matrix validation.

#### Scenario: Receipt summarizes profile health
r[cairn.toolchain_profile_matrix.validation.receipt]
- GIVEN a profile matrix
- WHEN validation runs
- THEN the receipt MUST include profile count, invalid profile count, matrix hash, policy hash, and receipt hash.

### Requirement: Profile docs
r[cairn.toolchain_profile_matrix.docs] Cairn documentation MUST distinguish dev, CI, release, proof, and compatibility profiles.

#### Scenario: Profile non-claim is visible
r[cairn.toolchain_profile_matrix.docs.non_claims]
- GIVEN a profile validation receipt passes
- WHEN the supported claim is rendered
- THEN it MUST state that profile declaration does not prove command success unless a separate execution receipt exists.

### Requirement: Final validation
r[cairn.toolchain_profile_matrix.final_validation] The change MUST include positive and negative fixtures plus focused validation evidence before archive.

#### Scenario: Fixture suite covers profile classes
r[cairn.toolchain_profile_matrix.final_validation.fixtures]
- GIVEN valid and invalid profile fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass and invalid fixtures MUST fail closed.
