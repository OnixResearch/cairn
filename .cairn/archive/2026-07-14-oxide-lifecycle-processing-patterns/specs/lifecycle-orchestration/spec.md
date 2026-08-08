# Lifecycle Orchestration Specification

## Purpose

Adds Oxide-inspired lifecycle processing patterns for saga-style execution logs, document processors, golden snapshots, evidence bundle inspection, and study-only reference mining.

## Requirements

### Requirement: Oxide lifecycle reference inventory
r[cairn.lifecycle_orchestration.oxide_patterns.reference_inventory] Cairn MUST record reference-only intake notes for each Oxide lifecycle, test, bundle, or study-only system repository used to shape Cairn lifecycle processing.

#### Scenario: Reference intake records non-adoption boundary
r[cairn.lifecycle_orchestration.oxide_patterns.reference_inventory.non_adoption]
- GIVEN Cairn evaluates an Oxide repository as prior art
- WHEN the reference inventory is written
- THEN it MUST name the source repository, intended adaptation, license posture, and explicit non-adoption boundary.

#### Scenario: Study-only systems stay bounded
r[cairn.lifecycle_orchestration.oxide_patterns.reference_inventory.study_only]
- GIVEN a domain-specific Oxide system repository is studied
- WHEN Cairn records the reference
- THEN the record MUST limit the claim to operator, observability, archive, or diagnostic pattern mining and MUST NOT import domain correctness claims.

### Requirement: Saga-style lifecycle execution logs
r[cairn.lifecycle_orchestration.oxide_patterns.saga_logs] Cairn SHOULD model multi-step lifecycle operations with deterministic saga-style logs and explicit recovery or compensation decisions.

#### Scenario: Lifecycle saga records recoverable state
r[cairn.lifecycle_orchestration.oxide_patterns.saga_logs.recoverable]
- GIVEN a multi-step lifecycle operation runs through validation, sync, archive, or evidence export steps
- WHEN a step completes
- THEN Cairn SHOULD record step identity, input hash, output hash, status, and compensation eligibility in a deterministic log.

#### Scenario: Missing compensation is visible
r[cairn.lifecycle_orchestration.oxide_patterns.saga_logs.missing_compensation]
- GIVEN a failed step has no safe compensation action
- WHEN Cairn reports recovery status
- THEN the report MUST mark the operation operator-intervention-required instead of pretending the operation was fully undone.

### Requirement: RFD-style lifecycle document processing
r[cairn.lifecycle_orchestration.oxide_patterns.rfd_processing] Cairn MAY add local lifecycle document processing for source revisions, rendered assets, state transitions, search indexes, webhook inputs, and periodic scans.

#### Scenario: Source revision binds rendered asset
r[cairn.lifecycle_orchestration.oxide_patterns.rfd_processing.revision_asset]
- GIVEN a lifecycle document is rendered into an asset or index entry
- WHEN Cairn records the processing result
- THEN the result MUST bind the asset to source path, source revision identity, renderer profile, and output hash.

#### Scenario: Stale revision is rejected
r[cairn.lifecycle_orchestration.oxide_patterns.rfd_processing.stale_revision]
- GIVEN a processor input references a stale source revision
- WHEN Cairn plans a state transition or rendered update
- THEN the plan MUST fail closed or require an explicit refresh before mutation.

### Requirement: Golden snapshot lifecycle tests
r[cairn.lifecycle_orchestration.oxide_patterns.golden_snapshots] Cairn SHOULD support explicit golden snapshot validation and opt-in refresh for CLI output, receipts, and rendered lifecycle reports.

#### Scenario: Golden output mismatch is reviewable
r[cairn.lifecycle_orchestration.oxide_patterns.golden_snapshots.mismatch]
- GIVEN current output differs from a checked-in golden snapshot
- WHEN the snapshot test runs without refresh approval
- THEN the test MUST fail and show a deterministic diff suitable for review.

#### Scenario: Unreviewed refresh is rejected
r[cairn.lifecycle_orchestration.oxide_patterns.golden_snapshots.unreviewed_refresh]
- GIVEN a caller tries to refresh golden snapshots without the configured approval signal
- WHEN Cairn runs snapshot validation
- THEN the refresh MUST be rejected and no golden file MUST be mutated.

### Requirement: Evidence bundle inspection
r[cairn.lifecycle_orchestration.oxide_patterns.bundle_inspector] Cairn SHOULD provide read-only evidence bundle inspection for exported bundle contents, hashes, roles, non-claim boundaries, and validation status.

#### Scenario: Bundle inspector lists bounded evidence
r[cairn.lifecycle_orchestration.oxide_patterns.bundle_inspector.list]
- GIVEN a Cairn evidence bundle is supplied to the inspector
- WHEN inspection runs
- THEN the inspector SHOULD list bundle members, content hashes, receipt roles, non-claim boundaries, and validation status without mutating the bundle.

#### Scenario: Corrupt bundle fails closed
r[cairn.lifecycle_orchestration.oxide_patterns.bundle_inspector.corrupt]
- GIVEN a bundle member hash or required non-claim boundary is missing or invalid
- WHEN inspection runs
- THEN the inspector MUST report deterministic diagnostics and MUST NOT mark the bundle valid.

### Requirement: Fixture-backed lifecycle validation
r[cairn.lifecycle_orchestration.oxide_patterns.validation] Cairn MUST validate adopted lifecycle processing patterns with positive and negative fixtures before implementation completion.

#### Scenario: Positive fixtures cover accepted processing
r[cairn.lifecycle_orchestration.oxide_patterns.validation.positive]
- GIVEN recoverable saga logs, valid document-processing plans, accepted golden updates, and readable bundles are present
- WHEN the fixture suite runs
- THEN accepted cases MUST produce stable deterministic reports.

#### Scenario: Negative fixtures cover rejected processing
r[cairn.lifecycle_orchestration.oxide_patterns.validation.negative]
- GIVEN missing compensation, stale revisions, unreviewed golden refreshes, corrupt bundles, or overclaiming study references are present
- WHEN the fixture suite runs
- THEN rejected cases MUST fail with deterministic diagnostics and no mutation.
