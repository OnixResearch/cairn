## Why

Active Cairn packages were grouped only by path, slug convention, or free-form proposal text. That made deterministic questions such as "which Cairns are current?", "which are future-dated?", or "which belong to a feature stream?" depend on local convention.

## What Changes

- Add policy-owned change metadata for active Cairn packages.
- Support deterministic grouping labels such as `current`, `future`, and feature-specific groups.
- Support optional target dates for planned/future work.
- Extend read-side change listing/detail receipts so agents can filter or inspect groups without parsing prose.

## Impact

- **Files**: active change packages may declare `metadata.json`.
- **CLI**: `cairn change list` and `cairn change show` expose metadata; list filtering selects by group and target-date bounds.
- **Compatibility**: existing Cairns without metadata remain valid and default to an empty metadata set.
