## Why

Stack changes often depend on work landing in another repo first. Today that ordering is implicit in branch timing, sibling paths, or notes in proposals. Cairn should let a change declare dependency on another repo's change, gate receipt, or archive receipt.

## What Changes

- Add cross-repo change dependency metadata.
- Support dependencies on proposal/design/tasks gates, validation receipts, sync receipts, or archive receipts.
- Fail closed when required upstream dependencies are missing, stale, failed, or from the wrong repo/change.

## Impact

- Stack work can drain in deterministic order.
- Repos stay independent while coordinating through receipts.
- Release readiness can explain blocked cross-repo dependencies.
