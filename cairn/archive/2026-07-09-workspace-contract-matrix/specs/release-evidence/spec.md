# Release Evidence Specification

## Purpose

Adds a workspace contract matrix for cross-repo contract and coupling evidence.

## Requirements

### Requirement: Workspace contract matrix schema
r[cairn.workspace_contract_matrix.schema] Cairn MUST define a workspace contract matrix schema for provider/consumer repo edges, artifact identities, validation commands, fixture coverage, owners, and non-claim boundaries.

#### Scenario: Complete matrix row passes
r[cairn.workspace_contract_matrix.positive]
- GIVEN a matrix row names an edge ID, provider repo, consumer repo, artifact family, schema or contract ID, artifact identity, validation command, positive fixture, negative fixture or failure class, owner, and required non-claim boundary
- WHEN the matrix validator evaluates the row
- THEN validation MUST pass and preserve the edge ID in the receipt.

#### Scenario: Malformed matrix row fails closed
r[cairn.workspace_contract_matrix.negative]
- GIVEN a matrix row omits an owner, uses an unsafe path, lacks negative coverage, declares a stale artifact identity, or weakens required non-claims
- WHEN the matrix validator evaluates the row
- THEN validation MUST fail with deterministic diagnostics naming the invalid edge.

### Requirement: Matrix validation receipt
r[cairn.workspace_contract_matrix.validation] Cairn MUST emit a deterministic BLAKE3-bound receipt for workspace contract matrix validation.

#### Scenario: Receipt binds matrix and artifact identities
r[cairn.workspace_contract_matrix.validation.receipt]
- GIVEN a matrix file and referenced artifact identities
- WHEN Cairn validates the matrix
- THEN the receipt MUST include validity, verdict, issues, matrix hash, checked edge count, stale edge count, policy hash, and receipt hash.

#### Scenario: Receipt avoids raw host paths
r[cairn.workspace_contract_matrix.validation.privacy]
- GIVEN matrix validation runs from a local workspace
- WHEN Cairn emits the receipt
- THEN it MUST use safe repo-relative paths or stable labels and MUST NOT require raw host paths for public review.

### Requirement: Release-readiness integration
r[cairn.workspace_contract_matrix.release_readiness] Cairn release readiness SHOULD support disabled, optional, and required modes for workspace contract matrix receipts.

#### Scenario: Optional matrix records context
r[cairn.workspace_contract_matrix.release_readiness.optional]
- GIVEN matrix mode is optional and a valid matrix receipt is supplied
- WHEN release readiness runs
- THEN Cairn SHOULD include the matrix receipt hash as context without broadening upstream correctness claims.

#### Scenario: Required matrix fails stale edge
r[cairn.workspace_contract_matrix.release_readiness.required]
- GIVEN matrix mode is required and a configured edge is missing, stale, or invalid
- WHEN release readiness runs
- THEN Cairn MUST fail readiness with diagnostics naming the edge and failure class.

### Requirement: Provider and consumer boundaries
r[cairn.workspace_contract_matrix.docs] Cairn documentation MUST state that provider repos own local contract semantics and consumer repos own integration checks, while Cairn owns only the cross-repo edge inventory and validation receipt.

#### Scenario: Matrix non-claim is visible
r[cairn.workspace_contract_matrix.docs.non_claims]
- GIVEN a matrix receipt or release-readiness report mentions a passing edge
- WHEN the supported claim is rendered
- THEN it MUST state that the matrix proves declared edge health only and does not prove provider correctness, consumer behavioral correctness, verifier soundness, or release eligibility by itself.

### Requirement: Final validation evidence
r[cairn.workspace_contract_matrix.final_validation] The change MUST include positive and negative tests, policy freshness, and Cairn validation evidence before archive.

#### Scenario: Fixture suite covers edge health
r[cairn.workspace_contract_matrix.final_validation.fixtures]
- GIVEN positive and negative matrix fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass, invalid fixtures MUST fail closed, and saved evidence MUST bind matrix, policy, and receipt identities.
