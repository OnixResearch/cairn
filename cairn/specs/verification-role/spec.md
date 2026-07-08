# Verification Role Specification

## Purpose

Defines the `verification-role` capability.

## Requirements

### Requirement: Verification role classification
r[cairn.verification_role] The system MUST classify every evidence artifact with a `VerificationRole` of `Property`, `RecordedOnly`, `Boundary`, or `Unknown`.

#### Scenario: Classifies known roles
r[cairn.verification_role.classify_known]
- GIVEN the role strings "property", "recorded_only", and "boundary"
- WHEN classified
- THEN they map to `Property`, `RecordedOnly`, and `Boundary` respectively

#### Scenario: Classifies unknown role as Unknown
r[cairn.verification_role.classify_unknown]
- GIVEN a role string that is not one of the known values
- WHEN classified
- THEN it maps to `Unknown`

### Requirement: Verified count counts only Property artifacts
r[cairn.verification_role.verified_count] The system MUST set `verified_count` equal to `property_count` so that a "0 verified" receipt means "nothing was verified," not "all good."

#### Scenario: Zero verified means nothing was verified
r[cairn.verification_role.verified_count.zero]
- GIVEN a summary with only `recorded_only` and `boundary` artifacts
- WHEN the summary is computed
- THEN `verified_count` is 0

#### Scenario: Only Property counts as verified
r[cairn.verification_role.verified_count.property_only]
- GIVEN a summary with mixed roles
- WHEN the summary is computed
- THEN `verified_count` equals `property_count` and no other count

### Requirement: Evidence verify emits verification role summary
r[cairn.verification_role.evidence_verify] The `evidence verify` receipt MUST include a `verification_role_summary` field counting artifacts by role, and each verified artifact MUST carry its `verification_role`.

#### Scenario: Evidence verify includes summary
r[cairn.verification_role.evidence_verify.summary]
- GIVEN an evidence export bundle with artifacts carrying `verification_role` fields
- WHEN `evidence verify` runs
- THEN the receipt includes a `verification_role_summary` with `property_count`, `recorded_only_count`, `boundary_count`, `unknown_count`, `total`, `verified_count`, and `boundary`

### Requirement: Release readiness emits verification role boundary
r[cairn.verification_role.release_readiness] The `release-readiness` receipt MUST include a `verification_role_boundary` non-claim field documenting that only `property` artifacts count as verified.

#### Scenario: Boundary field present
r[cairn.verification_role.release_readiness.boundary]
- GIVEN a `release-readiness` receipt
- WHEN inspected
- THEN it includes `verification_role_boundary` text stating that only `property` artifacts count as verified
