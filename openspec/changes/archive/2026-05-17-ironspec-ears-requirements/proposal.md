## Why

`cairn` specs already use EARS-like GIVEN/WHEN/THEN scenarios, but validation does not yet enforce that requirements and scenarios are structured enough to be testable. If `cairn` is going to supersede OpenSpec, it should make requirement quality machine-checkable instead of relying on convention.

## What Changes

- **EARS-style validation**: Validate accepted specs and active change delta specs for structured requirement/scenario language.
- **Deterministic issue receipts**: Extend `cairn validate` receipts with spec validation issues alongside existing change/task issues.
- **Pure validation core**: Keep markdown classification and validation decisions in `cairn-core`; keep filesystem reads and JSON presentation in the CLI shell.

## Capabilities

### Modified Capabilities
- `cairn-supersession`: `cairn validate` becomes a stronger requirements-quality gate using EARS-style structure.

## Impact

- **Files**: core validation, CLI validation shell, tests/goldens, README, self-dogfood, accepted supersession spec.
- **APIs**: `cairn validate` JSON receipts include deterministic spec issue fields.
- **Dependencies**: No new dependencies.
- **Testing**: Unit tests for pure validation; CLI integration fixtures for valid and invalid EARS-style specs; flake self-dogfood remains green.
