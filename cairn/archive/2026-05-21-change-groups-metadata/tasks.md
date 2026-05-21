## Phase 1: Metadata contract

- [x] [serial] I1 Define the policy-owned change metadata schema, including group labels, optional target dates, and optional planning status. r[cairn.supersede.change_metadata.schema]
- [x] [serial] I2 Add deterministic validation for valid metadata and fail-closed issues for malformed metadata. r[cairn.supersede.change_metadata.validation]

## Phase 2: Read-side behavior

- [x] [serial] I3 Surface normalized change metadata in `cairn change list` and `cairn change show` receipts while preserving compatibility for packages without metadata. r[cairn.supersede.change_metadata.read_receipts]
- [x] [serial] I4 Add deterministic `change list` filters for group and target-date bounds. r[cairn.supersede.change_metadata.filters]

## Phase 3: Verification

- [x] [serial] V1 Add positive and negative tests for metadata parsing, validation, receipt output, and filter composition. r[cairn.supersede.change_metadata.validation] r[cairn.supersede.change_metadata.read_receipts] r[cairn.supersede.change_metadata.filters]
- [x] [serial] V2 Run focused CLI/core tests plus repository validation, Tracey coverage, formatting, linting, and diff checks. r[cairn.supersede.change_metadata.verification]

## Verification Coverage

| Scenario | Verification |
| --- | --- |
| r[cairn.supersede.change_metadata.schema.valid_metadata] | I1, I2, V1 |
| r[cairn.supersede.change_metadata.schema.optional_metadata] | I1, I3, V1 |
| r[cairn.supersede.change_metadata.validation.invalid_metadata] | I2, V1 |
| r[cairn.supersede.change_metadata.read_receipts.list_show] | I3, V1 |
| r[cairn.supersede.change_metadata.filters.group] | I4, V1 |
| r[cairn.supersede.change_metadata.filters.target_date] | I4, V1 |
| r[cairn.supersede.change_metadata.verification.gates] | V2 |
