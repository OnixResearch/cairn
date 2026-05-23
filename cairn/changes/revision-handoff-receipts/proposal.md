## Why

Rothrock’s weakest interface was the revision cycle after a rejection. Cairn should model the handoff from review findings to revision attempts so agents can prove which findings were addressed, which remain, and whether new findings appeared.

## What Changes

- Define a revision handoff receipt that binds prior review receipt, revision inputs, and follow-up review.
- Track finding disposition: addressed, unchanged, superseded, deferred, or new.
- Validate chains without executing reviewers or mutating the workspace.

## Impact

- **Files**: Cairn policy, pure core receipt/profile validators, CLI validation/analytics commands, fixtures, tests, and docs as applicable.
- **Testing**: Positive/negative receipt fixtures, policy export checks, focused CLI tests, validation/gate receipts, and Tracey coverage refs.
