## Why

Rothrock’s analysis shows the most actionable distinction is not pass/fail alone but whether a rejection is an omission, systematic error, incoherence, or infrastructure failure. Cairn needs policy-owned labels so review receipts can be aggregated without ad hoc prose parsing.

## What Changes

- Add policy-owned review decisions and finding categories.
- Require every non-approved semantic review finding to carry one category.
- Preserve reviewer prose while making category, severity, and scope machine-checkable.

## Impact

- **Files**: Cairn policy, pure core receipt/profile validators, CLI validation/analytics commands, fixtures, tests, and docs as applicable.
- **Testing**: Positive/negative receipt fixtures, policy export checks, focused CLI tests, validation/gate receipts, and Tracey coverage refs.
