## ADDED Requirements

### Requirement: Cairn change metadata

r[cairn.supersede.change_metadata.schema] The system MUST support optional policy-owned metadata for active Cairn change packages so agents and reviewers can classify work without parsing prose or relying on slug conventions.

#### Scenario: Accept valid metadata

r[cairn.supersede.change_metadata.schema.valid_metadata]
- GIVEN an active Cairn change package declares metadata with unique group labels, an optional `YYYY-MM-DD` target date, and an optional policy-owned planning status
- WHEN the operator runs Cairn validation or a read-side change command
- THEN the system accepts the metadata
- AND the normalized metadata is deterministic

#### Scenario: Preserve packages without metadata

r[cairn.supersede.change_metadata.schema.optional_metadata]
- GIVEN an active Cairn change package does not declare metadata
- WHEN the operator runs Cairn validation or a read-side change command
- THEN the package remains valid
- AND the normalized metadata contains no groups or optional scheduling fields

### Requirement: Cairn change metadata validation

r[cairn.supersede.change_metadata.validation] The system MUST validate active change metadata fail-closed against policy-owned shape, label, status, and date rules.

#### Scenario: Reject malformed metadata

r[cairn.supersede.change_metadata.validation.invalid_metadata]
- GIVEN an active Cairn change package declares malformed metadata, duplicate groups, unsupported group labels, unsupported planning status, or a malformed target date
- WHEN the operator runs `cairn validate --root <path>`
- THEN the CLI emits a deterministic JSON receipt with `valid=false`
- AND the issue collection identifies the metadata problem without parsing proposal prose

### Requirement: Cairn change metadata read receipts

r[cairn.supersede.change_metadata.read_receipts] The system MUST expose normalized change metadata through deterministic active-change read receipts.

#### Scenario: List and show include metadata

r[cairn.supersede.change_metadata.read_receipts.list_show]
- GIVEN active Cairn changes with and without metadata
- WHEN the operator runs `cairn change list --root <path>` or `cairn change show <name> --root <path>`
- THEN each returned change summary includes normalized metadata
- AND output ordering remains stable

### Requirement: Cairn change metadata filters

r[cairn.supersede.change_metadata.filters] The system MUST provide deterministic active-change list filters for metadata groups and target-date bounds.

#### Scenario: Filter by group

r[cairn.supersede.change_metadata.filters.group]
- GIVEN active Cairn changes assigned to groups such as `current`, `future`, or `feature:<name>`
- WHEN the operator runs `cairn change list --root <path> --group <label>`
- THEN the receipt includes only active changes whose normalized metadata contains that group
- AND the filtered output remains stably sorted

#### Scenario: Filter by target date

r[cairn.supersede.change_metadata.filters.target_date]
- GIVEN active Cairn changes with optional target dates
- WHEN the operator runs `cairn change list --root <path>` with target-date bounds
- THEN the receipt includes only changes whose target dates satisfy the requested bounds
- AND changes without target dates are excluded from target-date bounded results

### Requirement: Cairn change metadata verification

r[cairn.supersede.change_metadata.verification] The system MUST cover change metadata behavior with focused tests and repository lifecycle gates.

#### Scenario: Metadata gates pass

r[cairn.supersede.change_metadata.verification.gates]
- GIVEN change metadata support is implemented
- WHEN the operator runs focused CLI/core tests plus repository validation, Tracey coverage, formatting, linting, and diff checks
- THEN all checks pass and the feature is ready for archive
