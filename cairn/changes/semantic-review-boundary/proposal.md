## Why

Adding semantic review support risks muddying Cairn’s deterministic contract. Cairn should explicitly define the boundary: external humans or agents produce semantic judgments, while Cairn validates, hashes, aggregates, and gates their evidence.

## What Changes

- Document and enforce the external-reviewer boundary.
- Add validation errors for receipts that claim Cairn executed a remote model or unreviewed hidden process.
- Keep core validation pure and filesystem/network effects in CLI shells only.

## Impact

- **Files**: Cairn policy, pure core receipt/profile validators, CLI validation/analytics commands, fixtures, tests, and docs as applicable.
- **Testing**: Positive/negative receipt fixtures, policy export checks, focused CLI tests, validation/gate receipts, and Tracey coverage refs.
