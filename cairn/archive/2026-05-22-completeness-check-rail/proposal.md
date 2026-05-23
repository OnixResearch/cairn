## Why

The strongest lesson from the gate-analysis article is that agents usually forget work rather than contradict themselves. Cairn already links tasks to requirements, but it does not yet provide a dedicated completeness receipt that maps requirements, acceptance criteria, tasks, implementation refs, and verification refs into explicit omission findings.

## What Changes

- Add a read-only completeness check receipt for active changes and accepted specs.
- Map requirements to tasks, implementation refs, and verification refs.
- Emit omission findings for unmapped acceptance criteria or missing verification evidence.

## Impact

- **Files**: Cairn policy, pure core receipt/profile validators, CLI validation/analytics commands, fixtures, tests, and docs as applicable.
- **Testing**: Positive/negative receipt fixtures, policy export checks, focused CLI tests, validation/gate receipts, and Tracey coverage refs.
