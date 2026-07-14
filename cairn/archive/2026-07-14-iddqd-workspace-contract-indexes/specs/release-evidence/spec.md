# Release Evidence Specification

## Purpose

Adds an iddqd-backed implementation boundary for workspace contract matrix edge and observation indexes without changing the public matrix or receipt schemas.

## Requirements

### Requirement: Workspace contract identity maps
r[cairn.workspace_contract_matrix.iddqd_indexes.contract] Cairn MUST index workspace contract edges and observed artifacts by ids derived from the values rather than by duplicated external map keys.

#### Scenario: Unique edge observations pass
r[cairn.workspace_contract_matrix.iddqd_indexes.fixtures.positive]
- GIVEN each declared workspace contract edge id is unique and each observed artifact record names a matching unique edge id
- WHEN workspace contract validation builds identity indexes
- THEN validation MUST compare observed and declared artifact identities without changing public receipt shape.

#### Scenario: Duplicate identity fails closed
r[cairn.workspace_contract_matrix.iddqd_indexes.fixtures.negative]
- GIVEN declared edges or observed artifact records contain duplicate edge ids, missing observations, stale artifact identities, unsafe paths, or weakened non-claims
- WHEN workspace contract validation builds identity indexes
- THEN validation MUST fail closed with deterministic diagnostics and MUST NOT silently overwrite a prior record.

### Requirement: Receipt schema compatibility
r[cairn.workspace_contract_matrix.iddqd_indexes.receipt_compatibility] Cairn MUST preserve the existing workspace contract matrix, validation receipt, and release-readiness receipt schemas while using iddqd only inside the pure validation core.

#### Scenario: Public receipt fields remain stable
r[cairn.workspace_contract_matrix.iddqd_indexes.validation]
- GIVEN a valid workspace contract matrix and matching observed artifact identities
- WHEN validation emits a receipt
- THEN the receipt MUST keep the existing public fields, matrix hash behavior, checked edge list, stale edge count, and non-claim boundary.

### Requirement: Final validation evidence
r[cairn.workspace_contract_matrix.iddqd_indexes.final_validation] The change MUST include positive and negative workspace contract coverage plus focused Cairn validation evidence before archive.

#### Scenario: Identity-index fixtures cover duplicate handling
r[cairn.workspace_contract_matrix.iddqd_indexes.final_validation.fixtures]
- GIVEN valid and invalid workspace contract matrix fixtures
- WHEN focused workspace contract tests and Cairn gates run
- THEN unique edge identities MUST pass and duplicate or stale identities MUST fail closed.
